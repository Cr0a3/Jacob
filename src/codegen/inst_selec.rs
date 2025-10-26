use crate::{
    codegen::{AllocatedIrNode, ArchBackend, AssemblyInst, CommentedInst},
    ir::visibility::Visibilty,
};

/// Stores the assembly for a function
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FuncAsm {
    /// The instructions of the function
    pub insts: Vec<AssemblyInst>,
    /// The constants the function requires
    pub consts: Vec<Constant>,
    /// Commented instructions
    pub meta_insts: Vec<CommentedInst>,
    /// The name of the function
    pub name: String,
    /// The visibility of the function
    pub scope: Visibilty,
}

/// A constant
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Constant {
    /// The value of the constnat
    pub bytes: Vec<u8>,
    /// The id of the constant
    pub id: usize,
}

impl FuncAsm {
    /// Creates a new instance
    pub fn new(name: String, scope: &Visibilty) -> Self {
        Self {
            insts: Vec::new(),
            consts: Vec::new(),
            meta_insts: Vec::new(),
            name,
            scope: *scope,
        }
    }

    /// Adds new instructions to the `FuncAsm`
    pub fn add(&mut self, inst: &[AssemblyInst]) {
        self.insts.extend_from_slice(inst);
    }

    /// Adds a new constant to the `FuncAsm`
    pub fn add_const(&mut self, c: Constant) {
        self.consts.push(c);
    }
}

/// Helper structure for instructiopn selection
pub struct InstSelector<'a, 'b> {
    ir: &'a Vec<AllocatedIrNode>,
    backend: &'b dyn ArchBackend,
    rich_commenting: bool,
}

impl<'a, 'b> InstSelector<'a, 'b> {
    /// Creates a new instance
    pub fn new(
        ir: &'a Vec<AllocatedIrNode>,
        backend: &'b dyn ArchBackend,
        rich_commenting: bool,
    ) -> Self {
        Self {
            ir,
            backend,
            rich_commenting,
        }
    }

    /// Runs the register selector
    pub fn run(&mut self, funcasm: &mut FuncAsm) {
        for ir_inst in self.ir {
            let inst = self.backend.lower_inst(ir_inst);
            funcasm.add(&inst);

            if self.rich_commenting {
                funcasm.meta_insts.push(CommentedInst {
                    insts: inst,
                    comment: ir_inst.to_string(),
                });
            }
        }
    }
}
