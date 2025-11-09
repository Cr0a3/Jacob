//! This crate contains all procedual macros used in the code generation library

mod backend;
mod patterns;

use proc_macro::TokenStream;

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
///    }
///}
///
/// ```
#[proc_macro]
pub fn patterns(input: TokenStream) -> TokenStream {
    patterns::patterns_impl(input)
}

/// This procmacro is used to define a backend
///
/// It automaticlly implemenets registers, you only need to add support for
/// asmprinting and compilation/decompilation (we reconmend the `patterns macro`)
#[proc_macro]
pub fn backend(input: TokenStream) -> TokenStream {
    backend::backend_impl(input)
}
