use std::{cell::RefCell, rc::Rc};

use crate::{
    codegen::{Compilation, FuncAsm, TargetArch},
    decompile::{deregalloc::DeRegAlloc, type_extractor::TypeExtractor},
    ir::{Function, IrOperand, Module},
};

/// Helper structure to make decompilation much easier
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecompilationHelper {
    asm: Compilation,
}

impl DecompilationHelper {
    /// Creates a new decompilation helper
    pub fn new(asm: Compilation) -> Self {
        Self { asm }
    }

    /// Parses the public constants and inserts them into the module
    pub fn add_symbols(&self, _module: &mut Module) {
        // ToDo
        // Currently the `Compilation` structure does not even support
        // constants and that kind of shit, so this function just
        // does nothing
    }

    /// Parses the functions and inserts them into the module
    pub fn add_funcs(&self, module: &mut Module) {
        for func in &self.asm.funcs {
            module.add_func(self.add_func(func, &self.asm.arch));
        }
    }

    /// Parses the function assembly of the given `FuncAsm` and returns an ir `Function`
    pub fn add_func(&self, asm: &FuncAsm, target: &TargetArch) -> Function {
        let mut func = Function::new(&asm.name);

        let back = self.asm.arch.backend();
        let mut alloc_ir = Vec::new();

        let mut insts = asm.insts.clone();

        while !insts.is_empty() {
            let (used_insts, ir) = back.disasm_inst(&insts);
            alloc_ir.push(ir);

            for i in 0..used_insts {
                insts.remove(i);
            }
        }

        let mut deregalloc = DeRegAlloc::new(&alloc_ir, *target);
        deregalloc.dealloc();

        eprintln!("{:#?}", deregalloc.ir()); // ToDo: remove log

        let mut ty_extractor = TypeExtractor::new(deregalloc.ir());
        ty_extractor.extract();

        func.args = ty_extractor.args().to_owned();
        func.ret = *ty_extractor.ret();

        // ToDo: do not make new rcs + refcells cuz then our ir will lose that
        // we can just borrow_mut one and change all
        func.ir = deregalloc
            .ir_owned()
            .iter()
            .map(|node| IrOperand::Out(Rc::new(RefCell::new(node.to_owned()))))
            .collect();

        func
    }
}
