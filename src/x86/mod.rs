//! The x86 backend

use crate::{codegen::ArchBackend, x86::regs::*};

/// The registers to use in x86
pub mod regs;

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
            RAX, RCX, RDX, RBX, RSI, RDI, R8, R9, R10, R11, R12, R13, R14, R15,
        ];
        regs.iter()
            .map(|x| Box::new(*x) as Box<dyn crate::codegen::Reg>)
            .collect()
    }
}
