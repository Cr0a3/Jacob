//! The aarch64 backend

use crate::{
    aarch64::regs::*,
    codegen::{Allocation, ArchBackend, BackendDecompiler, Reg},
};

mod asmprinter;
mod lowering;

/// The registers used in aarch64
#[allow(dead_code)]
pub mod regs;

/// The aarch64 backend
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Aarch64Backend {}

impl ArchBackend for Aarch64Backend {
    fn name(&self) -> &'static str {
        "aarch64"
    }

    fn caller_gpr(&self) -> Vec<Box<dyn crate::codegen::Reg>> {
        let regs: Vec<Aarch64Reg> = vec![X0, X1, X2, X3, X4, X5, X6, X7];
        regs.iter()
            .map(|x| Box::new(*x) as Box<dyn crate::codegen::Reg>)
            .collect()
    }

    fn callee_gpr(&self) -> Vec<Box<dyn crate::codegen::Reg>> {
        let regs: Vec<Aarch64Reg> = vec![
            X8, X9, X10, X11, X12, X13, X14, X15, X16, X17, X18, X19, X20, X21, X22, X23, X24, X25,
            X26, X27, X28,
        ];
        regs.iter()
            .map(|x| Box::new(*x) as Box<dyn crate::codegen::Reg>)
            .collect()
    }

    fn grps(&self) -> Vec<Box<dyn crate::codegen::Reg>> {
        let regs: Vec<Aarch64Reg> = vec![
            X0, X1, X2, X3, X4, X5, X6, X7, X8, X9, X10, X11, X12, X13, X14, X15, X16, X17, X18,
            X19, X20, X21, X22, X23, X24, X25, X26, X27, X28,
        ];
        regs.iter()
            .map(|x| Box::new(*x) as Box<dyn crate::codegen::Reg>)
            .collect()
    }

    fn callconv_argpos(
        &self,
        num: usize,
        ty: crate::ir::TypeMetadata,
    ) -> crate::codegen::Allocation {
        if num < 8 {
            return Allocation::Register {
                id: match num {
                    0 => X0,
                    1 => X1,
                    2 => X2,
                    3 => X3,
                    4 => X4,
                    5 => X5,
                    6 => X6,
                    7 => X7,
                    _ => unreachable!(),
                }
                .id(),
                ty,
            };
        }

        Allocation::Stack { slot: num - 5, ty } // ToDo: find out: why 5?
    }

    fn ret_reg(&self) -> crate::codegen::Allocation {
        X0.alloc()
    }
}

impl BackendDecompiler for Aarch64Backend {
    fn num_for_arg(&self, op: &Allocation) -> usize {
        if let Allocation::Register { id, .. } = op {
            return match *id {
                X0_ID => 0,
                X1_ID => 1,
                X2_ID => 2,
                X3_ID => 3,
                X4_ID => 4,
                X5_ID => 5,
                X6_ID => 6,
                X7_ID => 7,
                _ => panic!("Given register (id: {id}) cannot be an argument"),
            };
        }

        if let Allocation::Stack { slot, .. } = op {
            return *slot + 5; // ToDo: find out why we do +5
        }

        panic!()
    }
}
