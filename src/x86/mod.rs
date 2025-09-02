//! The x86 backend

use procmacro::patterns;

use crate::{
    codegen::{Allocation, ArchBackend, BackendInst, Reg},
    ir::TypeMetadata,
    x86::regs::*,
};

/// The registers to use in x86
pub mod regs;

mod asmprinter;

/// This structure defines the entire x86 backend
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct X86Backend {}

impl ArchBackend for X86Backend {
    fn name(&self) -> &'static str {
        "x86"
    }

    fn caller_gpr(&self) -> Vec<Box<dyn crate::codegen::Reg>> {
        let regs: Vec<X86Reg> = vec![RAX, RCX, RDX, RSI, RDI, R8, R9, R10, R11];
        regs.iter()
            .map(|x| Box::new(*x) as Box<dyn crate::codegen::Reg>)
            .collect()
    }

    fn callee_gpr(&self) -> Vec<Box<dyn crate::codegen::Reg>> {
        let regs: Vec<X86Reg> = vec![RBX, R12, R13, R14, R15];
        regs.iter()
            .map(|x| Box::new(*x) as Box<dyn crate::codegen::Reg>)
            .collect()
    }

    fn grps(&self) -> Vec<Box<dyn crate::codegen::Reg>> {
        let regs: Vec<X86Reg> = vec![
            RAX, RCX, RDX, RSI, RDI, R8, R9, R10, R11, RBX, R12, R13, R14,
        ];
        regs.iter()
            .map(|x| Box::new(*x) as Box<dyn crate::codegen::Reg>)
            .collect()
    }

    fn callconv_argpos(&self, num: usize, ty: TypeMetadata) -> Allocation {
        if num < 6 {
            return Allocation::Register {
                id: match num {
                    0 => RDI,
                    1 => RSI,
                    2 => RCX,
                    3 => RDX,
                    4 => R8,
                    5 => R9,
                    _ => unreachable!(),
                }
                .id(),
                ty,
            };
        }

        Allocation::Stack { slot: num - 5, ty }
    }
}

impl BackendInst for X86Backend {
    patterns! {
        Add(Gr, Gr) -> Gr {
            condition: in1 == out
            asm: add (in1, in2)
        }
        Add(Gr, Gr) -> Gr {
            condition: in2 == out
            asm: add (in2, in1)
        }
        Add(Gr, Gr) -> Gr {
            condition: in1 != out && in2 != out
            asm: lea (out, in1, in2)
        }
        Ret(Gr) {
            asm: ret(out)
        }
    }
}
