use crate::asm_builder::MachineAssemblyBuilder;
use crate::code_gen::target::Target;
use crate::mcinst_builder::{McInstBuilder, RegisterAllocator};
use crate::module::Module;
use crate::x86::X86Target;

/// This function is used for compilation in the crate.
/// It compiles the given module into a Compilation Result
pub fn compile(module: &Module, target: Target) -> Compilation {
    /*
    Steps:
     - 1. Lower the input ir to McInsts
     - 2. Allocate registers
     - 3. Lower to machine assembly code
    */

    let arch = match target {
        Target::X86 => Box::from(X86Target::new()),
    };

    let mut builder = McInstBuilder::build(module, arch);
    builder.use_regalloc(RegisterAllocator::LinearScan, target);

    println!("{builder:#?}");

    let mut machine = MachineAssemblyBuilder::new(builder, target);
    machine.lower();

    machine.result()
}

/// # Compilation
///
/// This structure only stores the compilation output (generated assembly code).
/// It does not perform any compilation steps
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Compilation {}

impl Compilation {
    /// Outputs the generated assembly code (formatted as a string)
    pub fn asm(&self) -> String {
        todo!()
    }
}
