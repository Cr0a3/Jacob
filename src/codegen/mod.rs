//! CodeGeneration

/// Structures for storing assembly code
pub mod asm;
/// Resource dropping
pub mod dropper;
/// Instruction selection
pub mod inst_selec;
/// Register allocation
pub mod regalloc;
/// Target enum and target trait
pub mod target;

pub use asm::*;
pub use dropper::*;
pub use inst_selec::*;
pub use regalloc::*;
pub use target::*;
