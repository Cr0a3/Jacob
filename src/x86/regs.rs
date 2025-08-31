use crate::codegen::Reg;

/// A register for the x86 platform
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct X86Reg {
    id: usize,
}

/// The rax register
pub const RAX: X86Reg = X86Reg { id: 0 };
/// The rcx register
pub const RCX: X86Reg = X86Reg { id: 1 };
/// The rdx register
pub const RDX: X86Reg = X86Reg { id: 2 };
/// The rbx register
pub const RBX: X86Reg = X86Reg { id: 3 };
/// The rsp register
pub const RSP: X86Reg = X86Reg { id: 4 };
/// The rbp register
pub const RBP: X86Reg = X86Reg { id: 5 };
/// The rsi register
pub const RSI: X86Reg = X86Reg { id: 6 };
/// The rdi register
pub const RDI: X86Reg = X86Reg { id: 7 };
/// The r8 register
pub const R8: X86Reg = X86Reg { id: 8 };
/// The r9 register
pub const R9: X86Reg = X86Reg { id: 9 };
/// The r10 register
pub const R10: X86Reg = X86Reg { id: 10 };
/// The r11 register
pub const R11: X86Reg = X86Reg { id: 11 };
/// The r12 register
pub const R12: X86Reg = X86Reg { id: 12 };
/// The r13 register
pub const R13: X86Reg = X86Reg { id: 13 };
/// The r14 register
pub const R14: X86Reg = X86Reg { id: 14 };
/// The r15 register
pub const R15: X86Reg = X86Reg { id: 15 };

impl Reg for X86Reg {
    fn id(&self) -> usize {
        self.id
    }

    fn ty(&self) -> crate::ir::TypeMetadata {
        crate::ir::TypeMetadata::Int64
    }

    fn name(&self) -> &'static str {
        match self.id {
            val if val == RAX.id() => "rax",
            val if val == RCX.id() => "rcx",
            val if val == RDX.id() => "rdx",
            val if val == RBX.id() => "rbx",
            val if val == RSP.id() => "rsp",
            val if val == RBP.id() => "rbp",
            val if val == RSI.id() => "rsi",
            val if val == RDI.id() => "rdi",
            val if val == R8.id() => "r8",
            val if val == R9.id() => "r9",
            val if val == R10.id() => "r10",
            val if val == R11.id() => "r11",
            val if val == R12.id() => "r12",
            val if val == R13.id() => "r13",
            val if val == R14.id() => "r14",
            val if val == R15.id() => "r15",
            _ => panic!(),
        }
    }

    fn is_gpr(&self) -> bool {
        !matches!(self.id(), val if val == RBP.id() | RSP.id())
    }

    fn caller_saved(&self) -> bool {
        !matches!(self.id(), val if val == RBX.id() | RBP.id() | R12.id() | R13.id() | R14.id() | R15.id() | RSP.id() )
    }
}
