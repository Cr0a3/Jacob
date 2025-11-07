use procmacro::patterns;

use crate::{
    aarch64::{Aarch64Backend, regs::X0},
    codegen::{BackendInst, InstrincLowering, Reg},
};

impl BackendInst for Aarch64Backend {
    patterns! {
        Add(Gr, Gr) -> Gr {
            asm: add (out, in1, in2)
        }
        Ret(Gr) {
            condition: in1 == X0.alloc()
            asm: ret()
        }
        Ret(Any) {
            condition: in1 != X0.alloc()
            asm: mov(X0.alloc(), in1)
            asm: ret()
        }
        Copy(Gr) -> Gr {
            asm: mov (out, in1)
        }
    }
}

impl InstrincLowering for Aarch64Backend {}
