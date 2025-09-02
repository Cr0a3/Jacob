//! Code Generation library experiment

#![forbid(missing_docs)]
#![forbid(non_snake_case)]

use crate::ir::{Function, Module};

pub mod codegen;
pub mod ir;
pub mod opt;
pub mod x86;

fn main() {
    let mut module = Module::new();

    let mut func = Function::new("add");

    let x = func.add_arg(ir::TypeMetadata::Int64);
    let y = func.add_arg(ir::TypeMetadata::Int64);

    let result = func.add(&x, &y);
    func.ret(&result);

    module.add_func(func);

    let asm = module.compile(codegen::TargetArch::X86);

    println!("{}", asm.asm());
}
