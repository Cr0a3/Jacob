use crate::codegen::{AllocatedIrNode, ArchBackend, AssemblyInst};

/// Stores the assembly for a function
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FuncAsm {
    insts: Vec<AssemblyInst>,
}

impl Default for FuncAsm {
    fn default() -> Self {
        FuncAsm::new()
    }
}

impl FuncAsm {
    /// Creates a new instance
    pub fn new() -> Self {
        Self { insts: Vec::new() }
    }

    pub(crate) fn add(&mut self, inst: AssemblyInst) {
        self.insts.push(inst);
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
            funcasm.add(inst);
        }
    }
}
