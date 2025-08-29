use crate::{
    code_gen::target::Target,
    compilation::{Compilation, compile},
    function::Function,
};

/// # A module
///
/// A module contains the ir of multiple different functions
/// It allows easy access to optimizations and compilation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module {
    pub(crate) funcs: Vec<Function>,
}

impl Default for Module {
    fn default() -> Self {
        Self::new()
    }
}

impl Module {
    /// Creates a new module
    pub fn new() -> Self {
        Self { funcs: Vec::new() }
    }

    /// Registers a function in the module
    pub fn add_func(&mut self, func: Function) {
        self.funcs.push(func);
    }

    /// Compiles the entire module to the given target architecture
    pub fn compile(&self, target: Target) -> Compilation {
        compile(self, target)
    }
}
