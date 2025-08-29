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

/// A mc node
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct McNode {
    pub(crate) opcode: McInstOpCode,
    pub(crate) out: Option<String>,
    pub(crate) operands: Vec<McInstValue>,
    pub(crate) value_types: Vec<TypeMetadata>,
    pub(crate) node_id: usize,
}

impl McNode {
    /// Returns if the given node is crucial to the controll flow (e.g: ret is crucial)
    pub fn crucial(&self) -> bool {
        matches!(self.opcode, McInstOpCode::Ret)
    }
}
