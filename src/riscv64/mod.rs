//! The riscv64 backend

use procmacro::backend;

use crate::codegen::{Allocation, ArchBackend, ArchInfos, BackendDecompiler, Reg};

mod asmprinter;
mod lowering;

backend! {
    name: Riscv64,
    ret_reg: A0,
    stack_reg: SP,

    caller_saved: [ A0, A1, A2, A3, A4, A5, A6, A7, T3, T4, T5, T6, T7 ],
    callee_saved: [
        S2, S3, S4, S5, S6, S7, S8, S9, S10, S11
        ],

    gprs: [
            A0, A1, A2, A3, A4, A5, A6, A7, T3, T4, T5, T6, T7, S2, S3, S4, S5, S6, S7, S8, S9,
            S10, S11,
        ],

    arg_reg_map: {
        0 -> A0,
        1 -> A1,
        2 -> A2,
        3 -> A3,
        4 -> A4,
        5 -> A5,
        6 -> A6,
        7 -> A7,
    },

    stack_off: -5,
}
