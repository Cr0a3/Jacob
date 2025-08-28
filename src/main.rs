//! Code Generation library experiment

#![forbid(missing_docs)]
#![forbid(non_snake_case)]

use crate::{
    function::Function,
    ty::{FunctionType, TypeMetadata},
};

/// The function struct + ir building functions
pub mod function;
/// Raw Ir
pub mod ir;
/// Real ir nodes
pub mod nodes;
/// Type metadata
pub mod ty;

fn main() {
    let mut func = Function::new();
    func.set_name("add");

    let block = func.create_block("0");
    func.set_block(block);

    let mut ty = FunctionType::new();
    let x = ty.add_arg("x", TypeMetadata::i32);
    let y = ty.add_arg("y", TypeMetadata::i32);
    ty.set_ret(TypeMetadata::i32);
    func.set_function_type(ty);

    let add = func.add(&x, &y, None, None);

    func.ret(&add, None);

    println!("{func}")
}
