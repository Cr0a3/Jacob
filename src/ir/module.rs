use std::{any::TypeId, collections::HashMap};

use crate::{
    codegen::{self, Compilation, FuncAsm, TargetArch},
    ir::Function,
    opt::*,
};

/// Includes multiple functions and easy access to optimizations/compilation
pub struct Module {
    /// The functions of the module
    pub funcs: Vec<Function>,
    registered_opts: HashMap<TypeId, Box<dyn Optimization>>,

    opts_to_run: Vec<TypeId>,
}

impl Default for Module {
    fn default() -> Self {
        Module::new()
    }
}

impl Module {
    /// Creates a new empty module
    pub fn new() -> Self {
        let mut opts: HashMap<TypeId, Box<dyn Optimization>> = HashMap::new();

        // ToDo: for each optimization which is newly implemented register it here like this
        // opts.insert(TypeId::of::<AmazingOptimization>, Box::from( AmazingOptimization {} ))
        opts.insert(TypeId::of::<Dce>(), Box::from(Dce {}));

        Self {
            funcs: Vec::new(),
            registered_opts: opts,
            opts_to_run: Vec::new(),
        }
    }

    /// Adds the function into the module
    pub fn add_func(&mut self, func: Function) {
        self.funcs.push(func);
    }

    /// Adds the given optimization to the queue
    pub fn add_opt<T: Optimization>(&mut self) {
        let id = TypeId::of::<T>();

        if !self.registered_opts.contains_key(&id) {
            panic!("The optimization pass to add is not int ");
        }

        self.opts_to_run.push(id);
    }

    /// Runs the optimizations in the queue
    pub fn run_opts(&mut self) {
        for type_id in &self.opts_to_run {
            let opt = self
                .registered_opts
                .get(type_id)
                .expect("This here should never happen");

            for func in &mut self.funcs {
                opt.run(func);
            }

            // ToDo: opt.run_module(self);
        }
    }

    /// Clears the optimization queue
    pub fn clear_opts(&mut self) {
        self.opts_to_run.clear();
    }

    /// Runs dead code elimination on the module without requiring the optimization pipeline
    pub(crate) fn dce(&mut self) {
        for func in &mut self.funcs {
            (Dce {}).run(func);
        }
    }

    /// Compiles the module
    ///
    /// Args:
    ///  - `target` the target to compile to
    ///  - `rich_comments` should comments be inserted into the assembly code
    ///
    /// Example:
    /// ```rust
    /// module.compile(codegen::TargetArch::X86, false);
    /// ```
    pub fn compile(&mut self, target: TargetArch, rich_comments: bool) -> Compilation {
        self.dce();

        let mut result = Compilation::new(target);
        let backend = target.backend();

        for func in &self.funcs {
            let mut asm = FuncAsm::new(func.name.to_owned(), &func.visibility);

            let mut dropper = codegen::Dropper::new(func.ir.clone());
            dropper.run();

            let mut regalloc = codegen::RegAlloc::new(func.args.clone(), &*backend);
            regalloc.run(dropper.get_ir());

            let mut inst = codegen::InstSelector::new(regalloc.get_ir(), &*backend, rich_comments);
            inst.run(&mut asm);

            result.add(asm);
        }

        // ToDo: add public constants and that shit

        result
    }
}
