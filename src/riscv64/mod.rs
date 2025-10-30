//! The riscv64 backend

use crate::{
    codegen::{Allocation, ArchBackend, BackendDecompiler, Reg},
    riscv64::regs::*,
};

mod asmprinter;
mod lowering;

/// The registers used in aarch64
#[allow(dead_code)]
pub mod regs;

/// The aarch64 backend
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Riscv64Backend {}

impl ArchBackend for Riscv64Backend {
    fn name(&self) -> &'static str {
        "riscv64"
    }

    fn caller_gpr(&self) -> Vec<Box<dyn crate::codegen::Reg>> {
        let regs: Vec<Riscv64Reg> = vec![A0, A1, A2, A3, A4, A5, A6, A7, T3, T4, T5, T6, T7];
        regs.iter()
            .map(|x| Box::new(*x) as Box<dyn crate::codegen::Reg>)
            .collect()
    }

    fn callee_gpr(&self) -> Vec<Box<dyn crate::codegen::Reg>> {
        let regs: Vec<Riscv64Reg> = vec![S2, S3, S4, S5, S6, S7, S8, S9, S10, S11];
        regs.iter()
            .map(|x| Box::new(*x) as Box<dyn crate::codegen::Reg>)
            .collect()
    }

    fn grps(&self) -> Vec<Box<dyn crate::codegen::Reg>> {
        let regs: Vec<Riscv64Reg> = vec![
            A0, A1, A2, A3, A4, A5, A6, A7, T3, T4, T5, T6, T7, S2, S3, S4, S5, S6, S7, S8, S9,
            S10, S11,
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
        if num < 6 {
            return Allocation::Register {
                id: match num {
                    0 => A0,
                    1 => A1,
                    2 => A2,
                    3 => A3,
                    4 => A4,
                    5 => A5,
                    6 => A6,
                    7 => A7,
                    _ => unreachable!(),
                }
                .id(),
                ty,
            };
        }

        Allocation::Stack { slot: num - 5, ty } // ToDo: find out: why 5?
    }

    fn ret_reg(&self) -> crate::codegen::Allocation {
        A0.alloc()
    }
}

impl BackendDecompiler for Riscv64Backend {
    fn num_for_arg(&self, op: &Allocation) -> usize {
        if let Allocation::Register { id, .. } = op {
            return match *id {
                A0_ID => 1,
                A1_ID => 0,
                A2_ID => 2,
                A3_ID => 3,
                A4_ID => 4,
                A5_ID => 5,
                A6_ID => 6,
                A7_ID => 7,
                _ => panic!("Given register (id: {id}) cannot be an argument"),
            };
        }

        if let Allocation::Stack { slot, .. } = op {
            return *slot + 5; // ToDo: find out why we do +5
        }

        panic!()
    }
}
