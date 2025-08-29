use std::cell::RefCell;

use crate::ty::TypeMetadata;

/// Avaliable Opcodes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
}

/// A mc inst
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct McInstValue {
    pub(crate) node: RefCell<McNode>,
    pub(crate) result_id: usize,
}

/// A mc node
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct McNode {
    pub(crate) opcode: McInstOpCode,
    pub(crate) operands: Vec<McInstValue>,
    pub(crate) value_types: Vec<TypeMetadata>,
    pub(crate) node_id: usize,
}
