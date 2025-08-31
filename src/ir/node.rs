use std::{
    cell::RefCell,
    hash::{Hash, Hasher},
    rc::Rc,
};

use crate::ir::{operand::IrOperand, ty::TypeMetadata};

/// The opcode of the node
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IrOpcode {
    /// An addition
    Add,
    /// A subtraction
    Sub,
    /// Returns to the caller
    Ret,
}

/// An ir node
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IrNode {
    pub(crate) opcode: IrOpcode,
    pub(crate) ops: Vec<IrOperand>,
    pub(crate) has_out: bool,
    pub(crate) ty: Option<TypeMetadata>,
}

macro_rules! op2 {
    ($name:tt, $opcode:expr) => {
        /// Creates the node
        pub fn $name(lhs: &IrOperand, rhs: &IrOperand) -> IrOperand {
            let ty = lhs.get_ty();
            IrOperand::Out(Rc::new(RefCell::new(IrNode {
                opcode: $opcode,
                ops: vec![lhs.clone(), rhs.clone()],
                has_out: true,
                ty: Some(ty),
            })))
        }
    };
}
macro_rules! op1 {
    ($name:tt, $opcode:expr) => {
        /// Creates the node
        pub fn $name(op: &IrOperand) -> IrOperand {
            let ty = op.get_ty();
            IrOperand::Out(Rc::new(RefCell::new(IrNode {
                opcode: $opcode,
                ops: vec![op.clone()],
                has_out: true,
                ty: Some(ty),
            })))
        }
    };
}

impl IrNode {
    op2!(add, IrOpcode::Add);
    op2!(sub, IrOpcode::Sub);
    op1!(ret, IrOpcode::Ret);

    /// Returns the type of the node
    pub fn get_ty(&self) -> Option<TypeMetadata> {
        self.ty
    }

    /// Returns if the instruction has the `add` opcode
    pub fn is_add(&self) -> bool {
        matches!(self.opcode, IrOpcode::Add)
    }

    /// Returns if the instruction has the `sub` opcode
    pub fn is_sub(&self) -> bool {
        matches!(self.opcode, IrOpcode::Sub)
    }

    /// Returns if the instruction has the `ret` opcode
    pub fn is_ret(&self) -> bool {
        matches!(self.opcode, IrOpcode::Ret)
    }

    /// Gets the first operand (be carful, if there are no operands, this function will panic!)
    pub fn get_lhs(&self) -> &IrOperand {
        self.ops.first().unwrap()
    }

    /// Gets the second operand (be carful, if there are no operands, this function will panic!)
    pub fn get_rhs(&self) -> &IrOperand {
        self.ops.get(2).unwrap()
    }

    /// Returns if the node has 2 ops
    pub fn is_2ops(&self) -> bool {
        matches!(self.ops.len(), 2)
    }

    /// Returns if the node has 1 op
    pub fn is_1op(&self) -> bool {
        matches!(self.ops.len(), 1)
    }

    /// Returns the hash of the node
    pub fn hash_u64(&self) -> u64 {
        let mut hasher = std::hash::DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}
