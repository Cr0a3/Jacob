use std::any::Any;

use crate::{
    codegen::{AllocatedIrNode, Allocation, AssemblyInst, Compilation},
    ir::{TypeMetadata, visibility::Visibilty},
};

/// The target architecture
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetArch {
    /// 64Bit x86
    X86,
}

impl TargetArch {
    /// Returns the backend for the architecture
    pub fn backend(&self) -> Box<dyn ArchBackend> {
        match self {
            TargetArch::X86 => Box::new(crate::x86::X86Backend {}),
        }
    }
}

/// The trait to implement when defining the backend for a custom architecture
pub trait ArchBackend: Any + BackendInst + AsmPrinter + BackendDecompiler {
    /// Returns the name of the backend
    fn name(&self) -> &'static str;

    /// Returns a list of all gpr registers to use which are caller saved
    fn caller_gpr(&self) -> Vec<Box<dyn Reg>>;

    /// Returns a list of all gpr registers to use which are callee saved
    fn callee_gpr(&self) -> Vec<Box<dyn Reg>>;

    /// Returns a list of all gpr registers
    fn grps(&self) -> Vec<Box<dyn Reg>>;

    /// Returns the position for an argument
    fn callconv_argpos(&self, num: usize, ty: TypeMetadata) -> Allocation;

    /// Returns the return register
    fn ret_reg(&self) -> Allocation;
}

/// The trait to implement for defining custom register
pub trait Reg: Any + std::fmt::Debug {
    /// Returns the name of the register
    fn name(&self) -> &'static str;

    /// Returns if the register is a general pourpuse register
    fn is_gpr(&self) -> bool;

    /// Returns if the register needs to be caller saved
    fn caller_saved(&self) -> bool;

    /// Returns if the register needs to be callee saved
    fn callee_saved(&self) -> bool {
        !self.caller_saved()
    }

    /// Returns the id of the register
    fn id(&self) -> usize;

    /// Returns the type of the register
    fn ty(&self) -> TypeMetadata;

    /// Returns the register as an allocation
    fn alloc(&self) -> Allocation {
        Allocation::Register {
            id: self.id(),
            ty: self.ty(),
        }
    }
}

/// This trait is used to lower ir nodes into ir
pub trait BackendInst {
    /// Lowers the given ir instruction
    fn lower_inst(&self, ir: &AllocatedIrNode) -> Vec<AssemblyInst>;

    /// Gets the ir for the given assembly instruction
    fn disasm_inst(&self, asm: &[AssemblyInst]) -> (usize, AllocatedIrNode);
}

/// This trait is used to implement asm printing for the given architecture
pub trait AsmPrinter {
    /// Prints a compilation result in assembly
    fn print_compilation(&self, compilation: &Compilation) -> String {
        let mut out = self.print_comment("Compilation output");

        out += self.print_code_section();

        for func in &compilation.funcs {
            if func.scope == Visibilty::Public {
                out += &format!("global {}\n", func.name);
            }

            out += &self.print_func_name(&func.name);
            for inst in &func.insts {
                out += &self.print_inst(inst);
            }
        }

        out
    }

    /// Prints a commit
    fn print_comment(&self, text: &str) -> String {
        format!("// {text}\n")
    }

    /// Prints a function name
    fn print_func_name(&self, name: &str) -> String {
        format!("{name}:\n")
    }

    /// Prints an operand
    fn print_op(&self, op: &Allocation) -> String;

    /// Prints the register from it's name
    fn print_reg(&self, num: &usize, ty: &TypeMetadata) -> String;

    /// Prints the code for an instruction
    fn print_inst(&self, inst: &AssemblyInst) -> String {
        let mut ops = String::new();

        for (index, op) in inst.ops.iter().enumerate() {
            if index != 0 {
                ops += ", ";
            }

            ops += &self.print_op(op);
        }

        format!("\t{} {}\n", inst.opcode, ops)
    }

    /// Prints the start for a code section
    fn print_code_section(&self) -> &'static str {
        "section .text\n\n"
    }
}

/// Trait to help with target specific decompilation stuff
pub trait BackendDecompiler {
    /// Returns the number for the given argument for the given position
    fn num_for_arg(&self, op: &Allocation) -> usize;
}
