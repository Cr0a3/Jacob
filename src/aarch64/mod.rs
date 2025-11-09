//! The aarch64 backend

use procmacro::backend;

use crate::codegen::{Allocation, ArchBackend, ArchInfos, BackendDecompiler, Reg};

mod asmprinter;
mod lowering;

backend! {
    name: Aarch64,
    ret_reg: X0,
    stack_reg: SP,

    caller_saved: [ X0, X1, X2, X3, X4, X5, X6, X7 ],
    callee_saved: [
        X8, X9, X10, X11, X12, X13, X14, X15, X16, X17, X18, X19, X20, X21, X22, X23, X24, X25,
        X26, X27, X28,
        ],

    gprs: [
            X0, X1, X2, X3, X4, X5, X6, X7, X8, X9, X10, X11, X12, X13, X14, X15, X16, X17, X18,
            X19, X20, X21, X22, X23, X24, X25, X26, X27, X28,
        ],

    arg_reg_map: {
        0 -> X0,
        1 -> X1,
        2 -> X2,
        3 -> X3,
        4 -> X4,
        5 -> X5,
        6 -> X6,
        7 -> X7,
    },

    stack_off: -5,
}
