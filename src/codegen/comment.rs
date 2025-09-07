use std::fmt::Display;

use crate::codegen::{AllocatedIrNode, Allocation};

impl Display for AllocatedIrNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(out) = &self.alloc {
            write!(f, "{} = ", out)?;
        }

        write!(f, "{} ", format!("{:?}", self.opcode).to_lowercase())?;

        for (index, op) in self.ops.iter().enumerate() {
            if index != 0 {
                write!(f, ", ")?;
            }

            write!(f, "{}", op)?;
        }

        std::fmt::Result::Ok(())
    }
}

impl Display for Allocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Allocation::Register { id, .. } => write!(f, "reg({id})"),
            Allocation::Stack { slot, .. } => write!(f, "stack({slot})"),
            Allocation::Imm { num, .. } => write!(f, "{num}"),
        }
    }
}
