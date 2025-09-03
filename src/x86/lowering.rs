use procmacro::patterns;

use crate::{
    codegen::{BackendInst, Reg},
    x86::{X86Backend, regs::RAX},
};

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
            condition: in1 == RAX.alloc()
            asm: ret()
        }
        Ret(Any) {
            condition: in1 != RAX.alloc()
            asm: mov(RAX.alloc(), in1)
            asm: ret()
        }
    }
}
