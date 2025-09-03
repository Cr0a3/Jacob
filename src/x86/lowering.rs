use procmacro::patterns;

use crate::{codegen::BackendInst, x86::X86Backend};

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
            asm: ret(out)
        }
    }
}
