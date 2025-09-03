use crate::{
    codegen::{AllocatedIrNode, ArchBackend, AssemblyInst},
    ir::visibility::Visibilty,
};

/// Stores the assembly for a function
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FuncAsm {
    pub(crate) insts: Vec<AssemblyInst>,
    pub(crate) name: String,
    pub(crate) scope: Visibilty,
}
impl FuncAsm {
    /// Creates a new instance
    pub fn new(name: String, scope: &Visibilty) -> Self {
        Self {
            insts: Vec::new(),
            name,
            scope: *scope,
        }
    }

    pub(crate) fn add(&mut self, inst: &Vec<AssemblyInst>) {
        self.insts.extend_from_slice(&inst);
    }
}

/// Helper structure for instructiopn selection
pub struct InstSelector<'a, 'b> {
    ir: &'a Vec<AllocatedIrNode>,
    backend: &'b dyn ArchBackend,
}

impl<'a, 'b> InstSelector<'a, 'b> {
    /// Creates a new instance
    pub fn new(ir: &'a Vec<AllocatedIrNode>, backend: &'b dyn ArchBackend) -> Self {
        Self { ir, backend }
    }

    /// Runs the register selector
    pub fn run(&mut self, funcasm: &mut FuncAsm) {
        for inst in self.ir {
            let inst = self.backend.lower_inst(inst);
            funcasm.add(&inst);
        }
    }
}
