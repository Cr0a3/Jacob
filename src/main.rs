//! Code Generation library experiment

#![forbid(missing_docs)]
#![forbid(non_snake_case)]

use crate::{
    code_gen::target::Target,
    function::Function,
    module::Module,
    ty::{FunctionType, TypeMetadata},
};

mod code_gen;
pub use code_gen::*;

pub mod target;
pub use target::*;

pub mod ir;
pub use ir::*;

fn main() {
    let mut module = Module::new();

    let mut func = Function::new("add");

    let block = func.create_block("0");
    func.set_block(block);

    let mut ty = FunctionType::new();
    let x = ty.add_arg("x", TypeMetadata::i32);
    let y = ty.add_arg("y", TypeMetadata::i32);
    ty.set_ret(TypeMetadata::i32);
    func.set_function_type(ty);

    let add = func.add(&x, &y, None, None);

    func.ret(&add, None);

    module.add_func(func);

    let out = module.compile(Target::X86);
    println!("{}", out.asm());
}
