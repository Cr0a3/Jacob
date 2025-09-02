use crate::codegen::Allocation;

/// Structure to store an assembly instruction
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssemblyInst {
    pub(crate) ops: Vec<Allocation>,
    pub(crate) opcode: String,
}

impl AssemblyInst {
    /// Creates a new assembly instruction with 0 operands
    pub fn with0(opcode: &str) -> Self {
        Self {
            ops: Vec::new(),
            opcode: opcode.to_owned(),
        }
    }

    /// Creates a new assembly instruction with 1 operand
    pub fn with1(opcode: &str, op0: &Allocation) -> Self {
        Self {
            ops: vec![*op0],
            opcode: opcode.to_owned(),
        }
    }

    /// Creates a new assembly instruction with 2 operands
    pub fn with2(opcode: &str, op0: &Allocation, op1: &Allocation) -> Self {
        Self {
            ops: vec![*op0, *op1],
            opcode: opcode.to_owned(),
        }
    }

    /// Creates a new assembly instruction with 3 operands
    pub fn with3(opcode: &str, op0: &Allocation, op1: &Allocation, op2: &Allocation) -> Self {
        Self {
            ops: vec![*op0, *op1, *op2],
            opcode: opcode.to_owned(),
        }
    }

    /// Creates a new assembly instruction with a variable amount of operands
    pub fn withn(opcode: &str, ops: Vec<&Allocation>) -> Self {
        Self {
            opcode: opcode.to_owned(),
            ops: ops.iter().map(|x| **x).collect(),
        }
    }
}
