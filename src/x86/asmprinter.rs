use crate::{
    codegen::{AsmPrinter, AssemblyInst},
    x86::X86Backend,
};

macro_rules! reg_printer {
    ($num_var:tt, $ty_var:tt, $num_lit:literal, $b64_name:expr) => {
        if *$num_var == $num_lit {
            match $ty_var {
                crate::ir::TypeMetadata::Int64 => return $b64_name.to_string(),
                other => todo!("Implement handling for type: {:?}", other),
            }
        }
    };
}

impl AsmPrinter for X86Backend {
    fn print_op(&self, op: &crate::codegen::Allocation) -> String {
        match op {
            crate::codegen::Allocation::Register { id, ty } => self.print_reg(id, ty),
            crate::codegen::Allocation::Stack { slot, ty: _ } => format!("[rsp + {}]", slot * 16),
        }
    }

    fn print_comment(&self, text: &str) -> String {
        format!("; {text}\n")
    }

    #[allow(unreachable_patterns)]
    fn print_reg(&self, num: &usize, ty: &crate::ir::TypeMetadata) -> String {
        reg_printer!(num, ty, 0, "rax");
        reg_printer!(num, ty, 1, "rcx");
        reg_printer!(num, ty, 2, "rdx");
        reg_printer!(num, ty, 3, "rbx");
        reg_printer!(num, ty, 4, "rsp");
        reg_printer!(num, ty, 5, "rbp");
        reg_printer!(num, ty, 6, "rsi");
        reg_printer!(num, ty, 7, "rdi");
        reg_printer!(num, ty, 8, "r8");
        reg_printer!(num, ty, 9, "r9");
        reg_printer!(num, ty, 10, "r10");
        reg_printer!(num, ty, 11, "r11");
        reg_printer!(num, ty, 12, "r12");
        reg_printer!(num, ty, 13, "r13");
        reg_printer!(num, ty, 14, "r14");
        reg_printer!(num, ty, 15, "r15");
        panic!("Impossible register id: {num}. X86 supports 0-15");
    }

    fn print_inst(&self, inst: &AssemblyInst) -> String {
        if inst.opcode == "lea" {
            return format!(
                "\tlea {}, [{} + {}]\n",
                self.print_op(&inst.ops[0]),
                self.print_op(&inst.ops[1]),
                self.print_op(&inst.ops[2])
            );
        }

        let mut ops = String::new();

        for (index, op) in inst.ops.iter().enumerate() {
            if index != 0 {
                ops += ", ";
            }

            ops += &self.print_op(op);
        }

        format!("\t{} {}\n", inst.opcode, ops)
    }
}
