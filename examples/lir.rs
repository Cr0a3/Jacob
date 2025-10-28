/*

This file is an example on how to use the low level ir
for building a simple add function and compilation

*/

use jacob::ir::{Function, Module};
use jacob::*;

fn main() {
    let mut module = Module::new();

    let mut func = Function::new("add");
    func.public();

    let x = func.add_arg(ir::TypeMetadata::Int64);
    let y = func.add_arg(ir::TypeMetadata::Int64);

    let result = func.add(&x, &y);
    func.ret(&result);

    module.add_func(func);

    let asm = module.compile(codegen::TargetArch::Aarch64, false);

    println!("{}", asm.asm());
}
