//! Ir optimizations

use std::any::Any;

use crate::ir::Function;

/// Dead code elemination
pub mod dce;

pub use dce::*;

/// Trait to implement to make an optimization pass
pub trait Optimization: Any {
    /// Returns the name of the optimization
    fn name(&self) -> &'static str;

    /// Runs the optimization pass on a singular function
    fn run(&self, func: &mut Function);

    /*
    /// Runs the optimization pass on an entire module
    fn run_module(&self, module: &mut Module);
    */
}
