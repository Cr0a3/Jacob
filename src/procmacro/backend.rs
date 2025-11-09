use indexmap::IndexMap;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Expr, ExprArray, Ident, Lit, LitInt, Token, braced, parse::Parse, parse_macro_input};

const BACKEND_FIELDS: usize = 8;

#[derive(Debug, Default)]
pub struct RegArgMap {
    map: IndexMap<usize, Ident>,
}

#[derive(Debug)]
struct BackendInput {
    name: Option<Ident>,
    ret_reg: Option<Ident>,
    stack_reg: Option<Ident>,

    caller_saved: Vec<Ident>,
    callee_saved: Vec<Ident>,
    gprs: Vec<Ident>,

    arg_reg_map: RegArgMap,
    stack_off: isize,
}

impl Parse for RegArgMap {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        braced!(content in input);

        let mut items = IndexMap::new();

        while !content.is_empty() {
            let key: LitInt = content.parse()?;
            let key_val = key.base10_parse::<usize>()?;

            let _: Token![->] = content.parse()?;

            let value_expr: Expr = content.parse()?;
            let value_name = match value_expr {
                Expr::Path(path) => path
                    .path
                    .get_ident()
                    .map(|x| x.to_owned())
                    .ok_or_else(|| content.error("expected identifier after '->'"))?,
                _ => return Err(content.error("expected identifier after '->'")),
            };

            items.insert(key_val, value_name);

            let _ = content.parse::<Token![,]>();
        }

        Ok(Self { map: items })
    }
}

impl Parse for BackendInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut out = BackendInput {
            name: None,
            ret_reg: None,
            stack_reg: None,
            caller_saved: Vec::new(),
            callee_saved: Vec::new(),
            gprs: Vec::new(),
            arg_reg_map: RegArgMap::default(),
            stack_off: 0,
        };

        let convert_array = |array: ExprArray| -> Vec<Ident> {
            array
                .elems
                .into_iter()
                .filter_map(|expr| {
                    if let Expr::Path(expr_path) = expr {
                        expr_path.path.get_ident().map(|ident| ident.to_owned())
                    } else {
                        None
                    }
                })
                .collect()
        };

        for _ in 0..BACKEND_FIELDS {
            let _ = input.parse::<Token![,]>();

            let field_name: Ident = match input.parse() {
                Ok(v) => v,
                Err(_) => break,
            };
            let _: Token![:] = input.parse()?;
            let basic_ident = input.parse::<Ident>();
            let num = input.parse::<Lit>();
            let array = input.parse::<ExprArray>();
            let map = input.parse::<RegArgMap>();

            if let Ok(array) = array {
                match field_name.to_string().to_lowercase().as_str() {
                    "caller_saved" => out.caller_saved = convert_array(array),
                    "callee_saved" => out.callee_saved = convert_array(array),
                    "gprs" => out.gprs = convert_array(array),
                    unknown => panic!(
                        "Arrays are only suported for the fields: `caller_saved`, `callee_saved`, `gprs`. Field name: {unknown}"
                    ),
                }
            }

            if let Ok(num) = num
                && let Lit::Int(int) = num
            {
                match field_name.to_string().to_lowercase().as_str() {
                    "stack_off" => out.stack_off = int.base10_parse::<isize>()?,
                    unknown => panic!(
                        "Number literals are only supported for the field `stack_off`. Field name: {unknown}"
                    ),
                }
            }

            if let Ok(ident) = basic_ident {
                match field_name.to_string().to_lowercase().as_str() {
                    "name" => out.name = Some(ident),
                    "ret_reg" => out.ret_reg = Some(ident),
                    "stack_reg" => out.stack_reg = Some(ident),
                    _ => panic!(
                        "Standalone idents are only supported for the fields: `name`, `ret_reg`, `stack_reg`"
                    ),
                }
            }

            if let Ok(map) = map {
                if field_name.to_string().to_lowercase().as_str() != "arg_reg_map" {
                    panic!("Arg register maps are only supported for the field: `arg_reg_map`")
                }

                out.arg_reg_map = map;
            }

            let _ = input.parse::<Token![,]>();
        }

        Ok(out)
    }
}

pub fn backend_impl(input: TokenStream) -> TokenStream {
    let def = parse_macro_input!(input as BackendInput);

    let Some(name) = def.name else {
        panic!("expected backend name")
    };
    let Some(ret_reg) = def.ret_reg else {
        panic!("expected return register")
    };
    let Some(sp_reg) = def.stack_reg else {
        panic!("expected stack register")
    };
    let name_str = name.to_string();
    let struct_name = format_ident!("{}Backend", name);
    let reg_name = format_ident!("{}Reg", name);

    let caller_regs = def.caller_saved;
    let callee_regs = def.callee_saved;
    let gpr_regs = def.gprs;

    let reg_args = def.arg_reg_map.map.len();
    let stack_off = def.stack_off;

    let reg_map: Vec<proc_macro2::TokenStream> = def
        .arg_reg_map
        .map
        .iter()
        .map(|(num, val)| quote! { #num => #val })
        .collect();

    let reg_map_rev: Vec<proc_macro2::TokenStream> = def
        .arg_reg_map
        .map
        .iter()
        .map(|(num, val)| quote! { val if val == #val.id() => #num })
        .collect();

    let mut reg_consts: Vec<proc_macro2::TokenStream> = gpr_regs
        .iter()
        .enumerate()
        .map(|(index, reg)| {
            quote! {
                /// Register
                pub const #reg: #reg_name = #reg_name { id: #index };
            }
        })
        .collect();

    let index = reg_consts.len();
    reg_consts.push(quote! {
        /// Register
        pub const #sp_reg: #reg_name = #reg_name { id: #index };
    });

    let name_arms: Vec<proc_macro2::TokenStream> = gpr_regs
        .iter()
        .map(|reg| {
            let name = reg.to_string();
            quote! { val if val == #reg.id() => #name }
        })
        .collect();
    let caller_regs_iter: Vec<proc_macro2::TokenStream> = caller_regs
        .iter()
        .map(|x| {
            quote! { val if val == #x.id() => true }
        })
        .collect();

    quote! {
        /// Cool backend!
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct #struct_name {}

        impl ArchBackend for #struct_name {}

        impl ArchInfos for #struct_name {
            fn name(&self) -> &'static str {
                #name_str
            }

            fn caller_gpr(&self) -> Vec<Box<dyn crate::codegen::Reg>> {
                let regs: Vec<#reg_name> = vec![#(#caller_regs,)*];
                regs.iter()
                    .map(|x| Box::new(*x) as Box<dyn crate::codegen::Reg>)
                    .collect()
            }

            fn callee_gpr(&self) -> Vec<Box<dyn crate::codegen::Reg>> {
                let regs: Vec<#reg_name> = vec![#(#callee_regs,)*];
                regs.iter()
                    .map(|x| Box::new(*x) as Box<dyn crate::codegen::Reg>)
                    .collect()
            }


            fn grps(&self) -> Vec<Box<dyn crate::codegen::Reg>> {
                let regs: Vec<#reg_name> = vec![#(#gpr_regs,)*];
                regs.iter()
                    .map(|x| Box::new(*x) as Box<dyn crate::codegen::Reg>)
                    .collect()
            }

            fn ret_reg(&self) -> crate::codegen::Allocation {
                #ret_reg.alloc()
            }

            fn get_stack_ptr(&self) -> Allocation {
                #sp_reg.alloc()
            }

            fn callconv_argpos(
                &self,
                num: usize,
                ty: crate::ir::TypeMetadata,
            ) -> crate::codegen::Allocation {
                if num < #reg_args {
                    return Allocation::Register {
                        id: match num {
                            #(#reg_map,)*
                            _ => unreachable!(),
                        }
                        .id(),
                        ty,
                    };
                }

                Allocation::Stack { slot: (num as isize + #stack_off) as usize, ty } // ToDo: find out: why 5?
            }
        }

        impl BackendDecompiler for #struct_name {
            fn num_for_arg(&self, op: &Allocation) -> usize {
                if let Allocation::Register { id, .. } = op {
                    return match *id {
                        #(#reg_map_rev,)*
                        _ => panic!("Given register (id: {id}) cannot be an argument"),
                    };
                }

                if let Allocation::Stack { slot, .. } = op {
                    return (*slot as isize + #stack_off) as usize;
                }

                panic!()
            }
        }

        /// Register
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct #reg_name {
            id: usize,
        }

        #(#reg_consts)*

        impl Reg for #reg_name {
            fn id(&self) -> usize {
                self.id
            }

            fn ty(&self) -> crate::ir::TypeMetadata {
                crate::ir::TypeMetadata::Int64
            }

            fn name(&self) -> &'static str {
                match self.id {
                    #(#name_arms, )*
                    _ => panic!(),
                }
            }

            fn is_gpr(&self) -> bool {
                true
            }

            fn caller_saved(&self) -> bool {
                match self.id() {
                    #(#caller_regs_iter, )*
                    _ => false,
                }
            }
        }
    }
    .into()
}
