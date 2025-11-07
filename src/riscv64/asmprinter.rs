use crate::{codegen::AsmPrinter, riscv64::Riscv64Backend};

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

impl AsmPrinter for Riscv64Backend {
    fn print_op(&self, op: &crate::codegen::Allocation) -> String {
        match op {
            crate::codegen::Allocation::Register { id, ty } => self.print_reg(id, ty),
            crate::codegen::Allocation::Stack { slot, ty: _ } => format!("[sp, #-{}]!", slot * 16), // ToDo: check alignment
            crate::codegen::Allocation::Imm { num, ty: _ } => format!("#{num:x}"),
            crate::codegen::Allocation::ConstUse { id } => format!("[c{id}]"),
        }
    }

    #[allow(unreachable_patterns)]
    fn print_reg(&self, num: &usize, ty: &crate::ir::TypeMetadata) -> String {
        reg_printer!(num, ty, 0, "a0");
        reg_printer!(num, ty, 1, "a1");
        reg_printer!(num, ty, 2, "a2");
        reg_printer!(num, ty, 3, "a3");
        reg_printer!(num, ty, 4, "a4");
        reg_printer!(num, ty, 5, "a5");
        reg_printer!(num, ty, 6, "a6");
        reg_printer!(num, ty, 7, "a7");
        reg_printer!(num, ty, 8, "s2");
        reg_printer!(num, ty, 9, "s3");
        reg_printer!(num, ty, 10, "s4");
        reg_printer!(num, ty, 11, "s5");
        reg_printer!(num, ty, 12, "s6");
        reg_printer!(num, ty, 13, "s7");
        reg_printer!(num, ty, 14, "s8");
        reg_printer!(num, ty, 15, "s9");
        reg_printer!(num, ty, 16, "s10");
        reg_printer!(num, ty, 17, "s11");
        reg_printer!(num, ty, 18, "t1");
        reg_printer!(num, ty, 20, "t2");
        reg_printer!(num, ty, 20, "t3");
        reg_printer!(num, ty, 21, "t4");
        reg_printer!(num, ty, 22, "t5");
        reg_printer!(num, ty, 23, "t6");
        reg_printer!(num, ty, 24, "t7");
        reg_printer!(num, ty, 25, "sp");
        panic!("Impossible register id: {num}. RiscV supports 0-25");
    }

    fn print_const(&self, c: &crate::codegen::Constant) -> String {
        format!("c{}: {:?}", c.id, c.bytes)
    }

    fn print_code_section(&self) -> &'static str {
        ".text\n"
    }

    fn print_global(&self, func: &String) -> String {
        format!(".globl {}\n", func)
    }
}
