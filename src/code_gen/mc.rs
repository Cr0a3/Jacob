use std::{cell::RefCell, rc::Rc};

use crate::ty::TypeMetadata;

/// Avaliable Opcodes
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum McInstOpCode {
    /// Add
    Add,
    /// Sub
    Sub,
    /// Mul
    Mul,
    /// Div
    Div,
    /// Ret
    Ret,
    /// Constant
    Const(Vec<u8>),
    /// Unresolved variable
    Unresolved(String),
    /// Argument use
    Arg(usize),
}

/// A mc inst
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct McInstValue {
    pub(crate) node: Rc<RefCell<McNode>>,
    pub(crate) result_id: usize,
}

/// A allocation position
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Allocation {
    /// The position is in a register
    Register {
        /// The id (num) of the register
        num: usize,
        /// Size of the register
        ty: TypeMetadata,
    },
    /// The position is spilled to the stack
    Spill {
        /// The position in the stack (does not accomadate for bytes!)
        pos: usize,
        /// The size
        ty: TypeMetadata,
    },
}

/// A mc node
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct McNode {
    pub(crate) opcode: McInstOpCode,
    pub(crate) out: Option<String>,
    pub(crate) operands: Vec<McInstValue>,
    pub(crate) value_types: Vec<TypeMetadata>,
    pub(crate) node_id: usize,

    pub(crate) out_allocate: Option<Allocation>,
}

impl McNode {
    /// Returns if the node is crucial to the controll flow (e.g: ret is crucial)
    pub fn crucial(&self) -> bool {
        matches!(self.opcode, McInstOpCode::Ret)
    }

    /// Returns if the node has an output
    pub fn has_out(&self) -> bool {
        use McInstOpCode::*;
        match &self.opcode {
            Add | Sub | Mul | Div => true,
            Ret => false,

            Const(_) | Unresolved(_) | Arg(_) => false,
        }
    }
}

impl Allocation {
    /// Returns if the alloc is on the stack
    pub fn is_mem(&self) -> bool {
        matches!(self, Allocation::Spill { pos: _, ty: _ })
    }
}
