//! The x86 backend

use procmacro::backend;

use crate::codegen::{Allocation, ArchBackend, ArchInfos, BackendDecompiler, Reg};

mod asmprinter;
mod lowering;

backend! {
    name: X86,
    ret_reg: RAX,
    stack_reg: RSP,

    caller_saved: [ RAX, RCX, RDX, RSI, RDI, R8, R9, R10, R11 ],
    callee_saved: [
        RBX, R12, R13, R14, R15
        ],

    gprs: [
        RAX, RCX, RDX, RSI, RDI, R8, R9, R10, R11, RBX, R12, R13, R14, R15,
    ],

    arg_reg_map: {
        0 -> RDI,
        1 -> RSI,
        2 -> RCX,
        3 -> RDX,
        4 -> R8,
        5 -> R9,
    },

    stack_off: -5,
}
