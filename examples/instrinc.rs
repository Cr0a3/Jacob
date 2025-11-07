/*

This file is an example on how to use instrincs

*/

use jacob::ir::{Function, Module};
use jacob::*;

fn main() {
    let mut module = Module::new();

    let mut func = Function::new("add");
    func.public();

    let result = func.get_sp();
    func.ret(&result);

    module.add_func(func);

    let asm = module.compile(codegen::TargetArch::Aarch64, false);

    println!("{}", asm.asm());
}
