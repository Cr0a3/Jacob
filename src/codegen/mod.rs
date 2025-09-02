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

/// The result of an compilation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Compilation {
    /// The compilation result of the functions
    pub funcs: Vec<FuncAsm>,
    /// The target architecture
    pub arch: TargetArch,
}

impl Compilation {
    /// Creates a new empty compilation result
    pub fn new(arch: TargetArch) -> Self {
        Self {
            funcs: Vec::new(),
            arch,
        }
    }
    /// Adds the compilation result from a function
    pub fn add(&mut self, asm: FuncAsm) {
        self.funcs.push(asm);
    }

    /// Returns a fully formated assembly code ready to be printed
    pub fn asm(&self) -> String {
        self.arch.backend().print_compilation(self)
    }
}
