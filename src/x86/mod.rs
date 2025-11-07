//! The x86 backend

use crate::{
    codegen::{Allocation, ArchBackend, ArchInfos, BackendDecompiler, Reg},
    ir::TypeMetadata,
    x86::regs::*,
};

/// The registers to use in x86
#[allow(dead_code)]
pub mod regs;

mod asmprinter;
mod lowering;

/// This structure defines the entire x86 backend
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct X86Backend {}

impl ArchBackend for X86Backend {}

impl ArchInfos for X86Backend {
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

        Allocation::Stack { slot: num - 5, ty } // ToDo: find out: why 5?
    }

    fn ret_reg(&self) -> Allocation {
        RAX.alloc()
    }

    fn get_stack_ptr(&self) -> Allocation {
        RSP.alloc()
    }
}

impl BackendDecompiler for X86Backend {
    fn num_for_arg(&self, op: &Allocation) -> usize {
        if let Allocation::Register { id, .. } = op {
            return match *id {
                RDI_ID => 0,
                RSI_ID => 1,
                RCX_ID => 2,
                RDX_ID => 3,
                R8_ID => 4,
                R9_ID => 5,
                _ => panic!("Given register (id: {id}) cannot be an argument"),
            };
        }

        if let Allocation::Stack { slot, .. } = op {
            return *slot + 5; // ToDo: find out why we do +5
        }

        panic!()
    }
}
