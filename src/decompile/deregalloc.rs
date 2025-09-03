use crate::{codegen::AllocatedIrNode, ir::IrNode};

/// This helper structure is used to reverse enginner a list of
/// `AllocatedIrNodes` into a
///
/// Note: it does not add dropping ir operands because they are
/// not neccessary for simple decompilation
pub struct DeRegAlloc<'a> {
    allocated_ir: &'a Vec<AllocatedIrNode>,
    ir: Vec<IrNode>,
}

impl<'a> DeRegAlloc<'a> {
    /// Creates a new deregalloc instance
    pub fn new(allocated_ir: &'a Vec<AllocatedIrNode>) -> Self {
        Self {
            allocated_ir,
            ir: Vec::new(),
        }
    }

    /// Deallocation time!
    pub fn dealloc(&mut self) {
        for inst in self.allocated_ir {
            todo!("Dealloc {:?}", inst.opcode);
        }
    }

    /// Returns the deallocated ir
    pub fn ir(&self) -> &Vec<IrNode> {
        &self.ir
    }

    /// Returns the deallocated ir (owned)
    pub fn ir_owned(self) -> Vec<IrNode> {
        self.ir
    }
}
