/// The target to compile to
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Target {
    /// Standard x86 64 bit with linux calling convention (used by **Most Computers**)
    X86,
}

pub const X86_RAX: usize = 0;
pub const X86_RCX: usize = 1;
pub const X86_RDX: usize = 2;
pub const X86_RBX: usize = 3;
pub const X86_RSP: usize = 4;
pub const X86_RBP: usize = 5;
pub const X86_RSI: usize = 6;
pub const X86_RDI: usize = 7;
pub const X86_R8: usize = 8;
pub const X86_R9: usize = 9;
pub const X86_R10: usize = 10;
pub const X86_R11: usize = 11;
pub const X86_R12: usize = 12;
pub const X86_R13: usize = 13;
pub const X86_R14: usize = 14;
pub const X86_R15: usize = 15;

impl Target {
    /// Returns the ids of general pourpus registers
    pub fn gpr_regs(&self) -> Vec<usize> {
        match self {
            Target::X86 => vec![
                X86_RAX, X86_RCX, X86_RDX, X86_RBX, X86_RSI, X86_RDI, X86_R8, X86_R9, X86_R10,
                X86_R11, X86_R12, X86_R13, X86_R14, X86_R15,
            ],
        }
    }

    /// Returns if the register needs to be saved by the caller
    pub fn saved_by_caller(&self, id: usize) -> bool {
        if *self == Target::X86 {
            match id {
                X86_RAX | X86_RCX | X86_RDX | X86_RSI | X86_RDI | X86_R8 | X86_R9 | X86_R10
                | X86_R11 => true,
                _ => panic!("Unknown register"),
            }
        } else {
            panic!("Unhandled target")
        }
    }

    /// Returns if the register needs to be saved by the callee
    pub fn saved_by_callee(&self, id: usize) -> bool {
        !self.saved_by_caller(id)
    }
}
