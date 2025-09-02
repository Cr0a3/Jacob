//! This crate contains all procedual macros used in the code generation library

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
    Imm,
}

struct Pattern {
    ins: Vec<Pos>,
    out: Option<Pos>,
    condition: Option<syn::Expr>,
    asm: Option<syn::Expr>,
    name: String,
}

impl Parse for Pattern {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let parse_ty = |ident: Ident| -> Pos {
            match ident.to_string().to_lowercase().as_str() {
                "gr" => Pos::Gr,
                "imm" => Pos::Imm,
                "mem" => Pos::Mem,
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

        let mut condition = None;
        let mut asm = None;

        while !body_content.is_empty() {
            let key: Ident = body_content.parse()?;
            body_content.parse::<Token![:]>()?;

            match key.to_string().as_str() {
                "condition" => {
                    let expr: syn::Expr = body_content.parse()?;
                    condition = Some(expr);
                }
                "asm" => {
                    let expr: syn::Expr = body_content.parse()?;
                    asm = Some(expr);
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

// -------------------------
// AST Rewriter for Idents
// -------------------------
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

fn get_asm_inst(asm: &Option<Expr>) -> String {
    if let Some(Expr::Call(c)) = asm {
        if let Expr::Path(ExprPath { path, .. }) = c.func.as_ref() {
            return path
                .segments
                .last()
                .map(|segment| segment.ident.to_string())
                .unwrap()
                .to_owned();
        }
        panic!("No function name was supplyed")
    }
    panic!("Asm needs to be parsed as an call instruction")
}

/// This procedual macro is used to build backend compilation support!
///
/// It automaticlly implements compilation and decompilation from the specified
/// patterns
///
/// Example:
/// ```rust no-run
/// impl BackendInst for X86Backend {
///    patterns! {
///        Add(Gr, Gr) -> Gr {
///            condition: in1 == out
///            asm: add (in1, in2)
///        }
///        Add(Gr, Gr) -> Gr {
///            condition: in2 == out
///            asm: add (in2, in1)
///        }
///        Add(Gr, Gr) -> Gr {
///            condition: in1 != out && in2 != out
///            asm: lea (out, in1, in2)
///        }
///        Ret(Gr) {
///            asm: ret(out)
///        }
///    }
///}
///
/// ```
#[proc_macro]
pub fn patterns(input: TokenStream) -> TokenStream {
    let patterns = parse_macro_input!(input as Patterns).0;

    let mut grouped: HashMap<String, Vec<&Pattern>> = HashMap::new();
    for pat in &patterns {
        grouped.entry(pat.name.clone()).or_default().push(pat);
    }

    let lower_inst_match = grouped.into_iter().map(|(name, pats)| {
        let name_ident = format_ident!("{}", name);
        let arms = pats.iter().map(|p| {
            let ins_check = p.ins.iter().enumerate().map(|(index, pos)| {
                let func_ident = match pos {
                    Pos::Gr => format_ident!("is_gr"),
                    Pos::Mem => format_ident!("is_mem"),
                    Pos::Imm => format_ident!("is_imm"),
                };
                quote! { && inst.ops[#index].#func_ident() }
            });

            let out_check = if let Some(out) = p.out {
                let func_ident = match out {
                    Pos::Gr => format_ident!("is_gr"),
                    Pos::Mem => format_ident!("is_mem"),
                    Pos::Imm => format_ident!("is_imm"),
                };
                quote! { && inst.has_out && inst.alloc.as_ref().unwrap().#func_ident() }
            } else {
                quote! {}
            };

            let cond = if let Some(cond_expr) = &p.condition {
                let rewritten = rewrite_expr(cond_expr);
                quote! { true #(#ins_check)* #out_check && (#rewritten) }
            } else {
                quote! { true #(#ins_check)* #out_check }
            };

            let asm_tokens = if let Some(asm_expr) = &p.asm {
                let rewritten = rewrite_asm_expr(asm_expr);
                quote! { #rewritten }
            } else {
                quote! { todo!("No asm provided for {}", stringify!(#name_ident)); }
            };

            quote! {
                if #cond {
                    return #asm_tokens;
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

    let mut dgrouped: HashMap<String, Vec<&Pattern>> = HashMap::new();
    for pat in &patterns {
        dgrouped
            .entry(get_asm_inst(&pat.asm))
            .or_default()
            .push(pat);
    }
    let disasm_inst_match = dgrouped.into_iter().map(|(name, pats)| {
        let arms = pats.iter().map(|p| {
            let num_ops = p.ins.len() + p.out.is_some() as usize;

            let out_check = if let Some(out) = p.out {
                let func_ident = match out {
                    Pos::Gr => format_ident!("is_gr"),
                    Pos::Mem => format_ident!("is_mem"),
                    Pos::Imm => format_ident!("is_imm"),
                };
                quote! {
                    && asm.ops[0].#func_ident()
                }
            } else {
                quote! {}
            };

            let ins_check = p.ins.iter().enumerate().map(|(index, pos)| {
                let func_ident = match pos {
                    Pos::Gr => format_ident!("is_gr"),
                    Pos::Mem => format_ident!("is_mem"),
                    Pos::Imm => format_ident!("is_imm"),
                };
                quote! { && asm.ops[#index].#func_ident() }
            });

            let opcode = format_ident!("{}", p.name);
            let has_out = p.out.is_some();
            let alloc = if has_out {
                quote! { Some(asm.ops[0]) }
            } else {
                quote! { None}
            };
            let ty = quote! { asm.get_ty() };

            let ops = p.ins.iter().enumerate().map(|(index, _)| {
                let mut starting_comma = quote! { , };
                if index == 0 {
                    starting_comma = quote! {}
                };

                quote! {
                    #starting_comma asm.ops[#index]
                }
            });

            quote! {
                if asm.operands() == #num_ops #out_check #(#ins_check)* {
                    return crate::codegen::AllocatedIrNode {
                        opcode: crate::ir::IrOpcode::#opcode,
                        ops: vec![#(#ops)*],
                        has_out: #has_out,
                        ty: #ty,
                        alloc: #alloc,
                    };
                }
            }
        });

        quote! {
            #name => {
                #(#arms)*
                panic!("no matching pattern for {:?}", asm);
            }
        }
    });

    quote! {
        fn lower_inst(&self, inst: &crate::codegen::AllocatedIrNode) -> crate::codegen::AssemblyInst {
            match inst.opcode {
                #(#lower_inst_match)*
                unhandled => todo!("Implement handling for {:?}", unhandled),
            }
        }

        fn disasm_inst(&self, asm: &crate::codegen::AssemblyInst) -> crate::codegen::AllocatedIrNode {
            match asm.opcode.as_str() {
                #(#disasm_inst_match)*
                unhandled => todo!("Unhandled asm instruction: {:?}", unhandled),
            }
        }
    }
    .into()
}
