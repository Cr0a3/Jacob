use std::{any::TypeId, collections::HashMap};

use crate::{
    codegen::{self, ArchBackend, FuncAsm, TargetArch},
    ir::Function,
    opt::*,
    x86::X86Backend,
};

/// Includes multiple functions and easy access to optimizations/compilation
pub struct Module {
    funcs: Vec<Function>,
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

    /// Compiles the module
    pub fn compile(&mut self, target: TargetArch) {
        self.add_opt::<Dce>();
        self.run_opts();
        self.clear_opts();

        let backend: Box<dyn ArchBackend> = match target {
            TargetArch::X86 => Box::new(X86Backend {}),
        };

        for func in &self.funcs {
            let mut asm = FuncAsm::new();

            let mut dropper = codegen::Dropper::new(func.ir.clone());
            dropper.run();

            let mut regalloc = codegen::RegAlloc::new(func.args.clone(), &*backend);
            regalloc.run(dropper.get_ir());

            let mut inst = codegen::InstSelector::new(regalloc.get_ir(), &*backend);
            inst.run(&mut asm);

            println!("{:#?}", asm);
        }

        todo!()
    }
}
