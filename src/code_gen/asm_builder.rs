use crate::{code_gen::target::Target, compilation::Compilation, mcinst_builder::McInstBuilder};

/// Lowers mc insts into assembly code
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MachineAssemblyBuilder {}

impl MachineAssemblyBuilder {
    /// Creates a new instance
    pub fn new(builder: McInstBuilder, target: Target) -> Self {
        todo!()
    }

    /// Lowers the insts into assembly code
    pub fn lower(&mut self) {
        todo!()
    }

    /// Returns the compilation result
    pub fn result(&self) -> Compilation {
        todo!()
    }
}
