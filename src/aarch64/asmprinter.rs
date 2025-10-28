use crate::{aarch64::Aarch64Backend, codegen::AsmPrinter};

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

impl AsmPrinter for Aarch64Backend {
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
        reg_printer!(num, ty, 0, "x0");
        reg_printer!(num, ty, 1, "x1");
        reg_printer!(num, ty, 2, "x2");
        reg_printer!(num, ty, 3, "x3");
        reg_printer!(num, ty, 4, "x4");
        reg_printer!(num, ty, 5, "x5");
        reg_printer!(num, ty, 6, "x6");
        reg_printer!(num, ty, 7, "x7");
        reg_printer!(num, ty, 8, "x8");
        reg_printer!(num, ty, 9, "x9");
        reg_printer!(num, ty, 10, "x10");
        reg_printer!(num, ty, 11, "x11");
        reg_printer!(num, ty, 12, "x12");
        reg_printer!(num, ty, 13, "x13");
        reg_printer!(num, ty, 14, "x14");
        reg_printer!(num, ty, 15, "x15");
        reg_printer!(num, ty, 16, "x16");
        reg_printer!(num, ty, 17, "x17");
        reg_printer!(num, ty, 18, "x18");
        reg_printer!(num, ty, 19, "x19");
        reg_printer!(num, ty, 20, "x20");
        reg_printer!(num, ty, 21, "x21");
        reg_printer!(num, ty, 22, "x22");
        reg_printer!(num, ty, 23, "x23");
        reg_printer!(num, ty, 24, "x24");
        reg_printer!(num, ty, 25, "x25");
        reg_printer!(num, ty, 26, "x26");
        reg_printer!(num, ty, 27, "x27");
        reg_printer!(num, ty, 28, "x28");
        panic!("Impossible register id: {num}. Aarch64 supports 0-28");
    }

    fn print_const(&self, c: &crate::codegen::Constant) -> String {
        format!("c{}: {:?}", c.id, c.bytes)
    }

    fn print_code_section(&self) -> &'static str {
        ".text\n"
    }

    fn print_global(&self, func: &String) -> String {
        format!(".global {}\n", func)
    }
}
