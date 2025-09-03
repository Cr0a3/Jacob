extern crate proc_macro;
use std::collections::HashMap;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    ExprPath, Ident, Token, braced, parenthesized,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pos {
    Gr,
    Mem,
    Any,
    Imm,
}

impl Pos {
    fn func(&self) -> Ident {
        match self {
            Pos::Gr => format_ident!("is_gr"),
            Pos::Mem => format_ident!("is_mem"),
            Pos::Imm => format_ident!("is_imm"),
            Pos::Any => format_ident!("is_any"),
        }
    }
}

struct Pattern {
    ins: Vec<Pos>,
    out: Option<Pos>,
    condition: Vec<syn::Expr>,
    asm: Vec<syn::Expr>,
    name: String,
}

impl Parse for Pattern {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let parse_ty = |ident: Ident| -> Pos {
            match ident.to_string().to_lowercase().as_str() {
                "gr" => Pos::Gr,
                "imm" => Pos::Imm,
                "mem" => Pos::Mem,
                "any" => Pos::Any,
                inv => panic!("Invalid position: {inv}. Available are: gr, imm, mem"),
            }
        };

        let name: Ident = input.parse()?;

        // Parse inputs
        let content;
        parenthesized!(content in input);
        let ins: Punctuated<Ident, Token![,]> =
            content.parse_terminated(Ident::parse, Token![,])?;
        let ins = ins.into_iter().map(parse_ty).collect();

        // Optional output
        let out = if input.peek(Token![->]) {
            input.parse::<Token![->]>()?;
            let ty: Ident = input.parse()?;
            Some(parse_ty(ty))
        } else {
            None
        };

        // Parse body
        let body_content;
        braced!(body_content in input);

        let mut condition = Vec::new();
        let mut asm = Vec::new();

        while !body_content.is_empty() {
            let key: Ident = body_content.parse()?;
            body_content.parse::<Token![:]>()?;

            match key.to_string().as_str() {
                "condition" => {
                    let expr: syn::Expr = body_content.parse()?;
                    condition.push(expr);
                }
                "asm" => {
                    let expr: syn::Expr = body_content.parse()?;
                    asm.push(expr);
                }
                other => {
                    return Err(syn::Error::new_spanned(
                        key,
                        format!("expected `condition` or `asm`, got `{other}`"),
                    ));
                }
            }

            let _ = body_content.parse::<Token![;]>();
        }

        Ok(Pattern {
            ins,
            out,
            condition,
            asm,
            name: name.to_string(),
        })
    }
}

struct Patterns(Vec<Pattern>);

impl Parse for Patterns {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut patterns = Vec::new();
        while !input.is_empty() {
            patterns.push(input.parse()?);
        }
        Ok(Patterns(patterns))
    }
}

use syn::{Expr, visit_mut::VisitMut};

struct Rewriter;
struct RewriterAsm;

impl VisitMut for Rewriter {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        if let Expr::Path(p) = expr
            && p.path.segments.len() == 1
        {
            let name = p.path.segments[0].ident.to_string();
            let replacement = match name.as_str() {
                "in1" => Some(quote!(inst.ops[0])),
                "in2" => Some(quote!(inst.ops[1])),
                "in3" => Some(quote!(inst.ops[2])),
                "in4" => Some(quote!(inst.ops[3])),
                "out" => Some(quote!(inst.alloc.unwrap())),
                _ => None,
            };
            if let Some(rep) = replacement {
                *expr = syn::parse2::<Expr>(rep).expect("rewrite parse should succeed");
                return;
            }
        }
        syn::visit_mut::visit_expr_mut(self, expr);
    }
}

impl VisitMut for RewriterAsm {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        // needs to turn add (inst.ops[0], inst.ops[1]) ...
        // into
        // AssemblyInst::with2("add", vec![inst.ops[0], inst.ops[1]])
        if let Expr::Call(call) = expr {
            let name = if let Expr::Path(ExprPath { path, .. }) = call.func.as_ref() {
                path.segments
                    .last()
                    .map(|segment| segment.ident.to_string())
                    .unwrap()
            } else {
                panic!("No function name was supplyed")
            };

            call.func = Box::new(syn::parse_quote!(crate::codegen::AssemblyInst::withn));

            let name_lit = syn::LitStr::new(&name, proc_macro2::Span::call_site());
            let args_vec: Vec<Expr> = call.args.iter().cloned().collect();

            let new_expr: Expr = syn::parse_quote! {
                crate::codegen::AssemblyInst::withn(#name_lit, vec![#(&#args_vec),*])
            };

            *expr = new_expr;
            return;
        }

        syn::visit_mut::visit_expr_mut(self, expr);
    }
}

fn force_call_name(expr: &Expr) -> String {
    if let Expr::Call(call) = expr
        && let Expr::Path(ExprPath { path, .. }) = call.func.as_ref()
    {
        return path
            .segments
            .last()
            .map(|segment| segment.ident.to_string())
            .unwrap();
    }

    panic!()
}

fn extract_name_from_path(path: &ExprPath) -> String {
    let path = &path.path;
    path.segments
        .last()
        .map(|segment| segment.ident.to_string())
        .unwrap()
}

fn rewrite_expr(expr: &syn::Expr) -> syn::Expr {
    let mut expr = expr.clone();
    let mut rewriter = Rewriter;
    rewriter.visit_expr_mut(&mut expr);
    expr
}

fn rewrite_asm_expr(expr: &syn::Expr) -> syn::Expr {
    let mut expr = expr.clone();
    let mut rewriter = Rewriter;
    rewriter.visit_expr_mut(&mut expr);
    let mut rewriter = RewriterAsm;
    rewriter.visit_expr_mut(&mut expr);
    expr
}

pub(crate) fn patterns_impl(input: TokenStream) -> TokenStream {
    let patterns = parse_macro_input!(input as Patterns).0;

    let mut grouped: HashMap<String, Vec<&Pattern>> = HashMap::new();
    for pat in &patterns {
        grouped.entry(pat.name.clone()).or_default().push(pat);
    }

    let lower_inst_match = grouped.into_iter().map(|(name, pats)| {
        let name_ident = format_ident!("{}", name);
        let arms = pats.iter().map(|p| {
            let ins_check = p.ins.iter().enumerate().map(|(index, pos)| {
                let func_ident = pos.func();
                quote! { && inst.ops[#index].#func_ident() }
            });

            let out_check = if let Some(out) = p.out {
                let func_ident = out.func();
                quote! { && inst.has_out && inst.alloc.as_ref().unwrap().#func_ident() }
            } else {
                quote! {}
            };

            let mut cond = quote! { true #(#ins_check)* #out_check };

            for cond_expr in &p.condition {
                let rewritten = rewrite_expr(cond_expr);
                cond = quote! { #cond && (#rewritten) };
            }

            let mut asms = quote! {};

            for (index, asm_expr) in p.asm.iter().enumerate() {
                let rewritten = rewrite_asm_expr(asm_expr);
                if index > 0 {
                    asms = quote! { #asms, #rewritten };
                } else {
                    asms = quote! { #rewritten };
                }
            }

            quote! {
                if #cond {
                    return vec![#asms];
                }
            }
        });

        quote! {
            crate::ir::IrOpcode::#name_ident => {
                #(#arms)*
                panic!("no matching pattern for {:?}", inst);
            }
        }
    });

    let disasm_inst_match = patterns.iter().map(|pat| {
        // ToDo: sort by length of list `asm` (higher -> lower) so that the most complex patterns will get matched at first
        // Disassembly is a bit more tricky :(
        // we first need to check all opcodes and then the operands
        let mut asm_cond = quote! {};
        let asm_len = pat.asm.len();

        for (index, asm) in pat.asm.iter().enumerate() {
            let name = force_call_name(asm);

            asm_cond = quote! { #asm_cond && asm[#index].opcode == #name }
        }

        let mut ops_cond = quote! {};

        for op in &pat.ins {
            let func = op.func();

            for (asm_index, asm) in pat.asm.iter().enumerate() {
                let Expr::Call(asm) = asm else { panic!() };
                for (op_index, arg) in asm.args.iter().enumerate() {
                    let Expr::Path(path) = arg else {
                        if let Expr::MethodCall(c) = arg {
                            // e.g: RAX.alloc()
                            ops_cond = quote! { #ops_cond && asm[#asm_index].ops[#op_index] == #c}
                        }
                        continue;
                    };

                    match extract_name_from_path(path).as_str() {
                        "in1" => {
                            ops_cond = quote! { #ops_cond && asm[#asm_index].ops[#op_index].#func()}
                        }
                        "in2" => {
                            ops_cond = quote! { #ops_cond && asm[#asm_index].ops[#op_index].#func()}
                        }
                        "in3" => {
                            ops_cond = quote! { #ops_cond && asm[#asm_index].ops[#op_index].#func()}
                        }
                        "in4" => {
                            ops_cond = quote! { #ops_cond && asm[#asm_index].ops[#op_index].#func()}
                        }
                        "out" => {
                            ops_cond = quote! { #ops_cond && asm[#asm_index].ops[#op_index].#func()}
                        }
                        _ => panic!(),
                    }
                }
            }
        }

        let opcode = format_ident!("{}", pat.name);

        let mut has_out = quote! { false };
        let mut ty = quote! { None };
        let mut alloc = quote! { None };
        let mut ops = quote! {};
        let mut parsed_ops = 0;

        for (index, inst) in pat.asm.iter().enumerate().rev() {
            let Expr::Call(asm) = inst else { panic!() };

            for (i, arg) in asm.args.iter().enumerate() {
                let Expr::Path(path) = arg else {
                    continue;
                };
                let path = extract_name_from_path(path);

                if &path == "out" {
                    alloc = quote! { Some(asm[#index].ops[#i]) };
                    ty = quote! { Some(asm[#index].ops[#i].get_ty()) };
                    has_out = quote! { true };
                    continue;
                }

                parsed_ops += 1;

                if parsed_ops > 1 {
                    ops = quote! { #ops , };
                }
                match path.as_str() {
                    "in1" => ops = quote! { #ops asm[#index].ops[#i] },
                    "in2" => ops = quote! { #ops asm[#index].ops[#i] },
                    "in3" => ops = quote! { #ops asm[#index].ops[#i] },
                    "in4" => ops = quote! { #ops asm[#index].ops[#i] },
                    _ => {
                        parsed_ops -= 1;
                    }
                }
            }
        }

        quote! {
            if asm.len() >= #asm_len #asm_cond #ops_cond {
                let inst = crate::codegen::AllocatedIrNode {
                    opcode: crate::ir::IrOpcode::#opcode,
                    ops: vec![#ops],
                    has_out: #has_out,
                    ty: #ty,
                    alloc: #alloc,
                };
                return (#asm_len, inst);
            }
        }
    });

    quote! {
        fn lower_inst(&self, inst: &crate::codegen::AllocatedIrNode) -> Vec<crate::codegen::AssemblyInst> {
            match inst.opcode {
                #(#lower_inst_match)*
                unhandled => todo!("Implement handling for {:?}", unhandled),
            }
        }

        fn disasm_inst(&self, asm: &[crate::codegen::AssemblyInst]) -> (usize, crate::codegen::AllocatedIrNode) {
            if asm.is_empty() {
                panic!("Given assembly instructions are empty")
            }

            #(#disasm_inst_match)*
            todo!("Unhandled asm instruction: {asm:?}");
        }
    }
    .into()
}
