//! This contains all things which have to do with the code gen (not for individual backends, but shared. For individual
//! backends the code is in the `Target` folder)

/// Builds the architecture dependend assembly code
pub mod asm_builder;
/// Compilation output
pub mod compilation;
/// Linear scan register allocator
pub mod linear_scan;
/// Cross architecture instrúctions
pub mod mc;
pub(crate) mod mc_dce;
/// Lowering the IR to the closest thing capable of being shared across multiple architecturs
pub mod mcinst_builder;
/// Control of the compilation target architecture
#[allow(dead_code)]
pub mod target;
