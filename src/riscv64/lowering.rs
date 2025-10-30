use procmacro::patterns;

use crate::{
    codegen::{BackendInst, Reg},
    riscv64::{Riscv64Backend, regs::A0},
};

impl BackendInst for Riscv64Backend {
    patterns! {
        Add(Gr, Gr) -> Gr {
            asm: add (out, in1, in2)
        }
        Ret(Gr) {
            condition: in1 == A0.alloc()
            asm: ret()
        }
        Ret(Any) {
            condition: in1 != A0.alloc()
            asm: mov(A0.alloc(), in1)
            asm: ret()
        }
    }
}
