//! CodeGeneration

/// Resource dropping
pub mod dropper;
/// Instruction selection
pub mod inst_selec;
/// Register allocation
pub mod regalloc;
/// Target enum and target trait
pub mod target;

pub use dropper::*;
pub use inst_selec::*;
pub use regalloc::*;
pub use target::*;
