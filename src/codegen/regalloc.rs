use std::{cell::RefCell, rc::Rc};

use crate::{
    codegen::Reg,
    ir::{IrOpcode, IrOperand, TypeMetadata},
};

/// The resource to use for an allocation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Allocation {
    /// The register to use
    Register {
        /// Id of the register
        id: usize,
        /// Type of the register
        ty: TypeMetadata,
    },
}

/// same as `src/ir/operand.rs - IrOperand` but with a allocated dest
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AllocatedIrOperand {
    /// Argument
    Arg {
        /// The number of the arg
        num: usize,
        /// The type
        ty: TypeMetadata,
        /// Allocation
        alloc: Allocation,
    },
    /// The output of a previous instruction
    Out {
        /// Other instruction
        cell: Rc<RefCell<AllocatedIrOperand>>,
        /// Allocation
        alloc: Allocation,
    },
}

/// same as `src/ir/node.rs - IrNode` but with a allocated dest
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllocatedIrNode {
    pub(crate) opcode: IrOpcode,
    pub(crate) ops: Vec<IrOperand>,
    pub(crate) has_out: bool,
    pub(crate) ty: Option<TypeMetadata>,
    pub(crate) alloc: Allocation,
}
/// Helper structure for register allocation
pub struct RegAlloc {
    args: Vec<TypeMetadata>,
    ir: Vec<IrOperand>,

    allocated_ir: Vec<AllocatedIrNode>,
    free_regs: Vec<Allocation>,
    freed_mem: Vec<Allocation>,

    max_stack_poses_used: usize,
    used_callee_saved_regs: bool,
}

impl RegAlloc {
    /// Creates a register allocator
    pub fn new(args: Vec<TypeMetadata>, ir: Vec<IrOperand>) -> Self {
        Self {
            args,
            ir,
            allocated_ir: Vec::new(),
        }
    }

    /// Runs the register allocator
    pub fn run(&mut self) {
        for inst in &mut self.ir {}
    }

    /// Returns the new and allocated ir
    pub fn get_ir(&self) -> &Vec<AllocatedIrNode> {
        &self.allocated_ir
    }
}
