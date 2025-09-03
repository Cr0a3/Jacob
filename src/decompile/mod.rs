//! Decompilation from assembly to ir

use std::{fs::File, path::Path};

// For all interressted people who just want to take a look around:

// The code in this folder does essentially only turn a `Vec<AllocatedIrNode>`
// into a vec ir node. The main assembly -> ir instruction code is auto generated
// using the backend macro (e.g: src/x86/lowering.rs).
// In this folder there a also just a few helper functions to make the pipeline
// but nothing worth taking a look

use crate::{codegen::Compilation, ir::Module};

/// Helper for decompilation
pub mod decomp;
/// Reverse engineers ir from allocated ir nodes
pub mod deregalloc;
/// Extraction of function type from ir
pub mod type_extractor;

impl Module {
    /// Decompiles the assembly of the given compilation result into a module
    pub fn decompile_comp(compilation: Compilation) -> Self {
        let decompiler = decomp::DecompilationHelper::new(compilation);

        let mut module = Module::new();
        decompiler.add_symbols(&mut module);
        decompiler.add_funcs(&mut module);

        module
    }

    /// Decompiles a given object file into a module
    #[inline]
    pub fn decompile<P: AsRef<Path>>(path: P) -> Self {
        Module::decompile_comp(Compilation::load(path).expect("Couldn't open given file"))
    }
}

impl Compilation {
    /// Loads an object file and disassembles it's instructions into a `Compilation`
    pub fn load<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let _file = File::open(path)?;
        todo!("implement disassembly of an object file into the `Compilation` struct")
    }
}
