use crate::{
    Target,
    function::Function,
    ir::ir::{IrBlock, IrInst, IrInstTrait},
    mc::{McInstOpCode, McInstValue, McNode},
    module::Module,
    target_trait::TargetArchitecture,
    ty::FunctionType,
};

/// Avaliable register allocators
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegisterAllocator {
    /// Simple Linear Scan Register optimization
    LinearScan,
}

/// Block handling in the start of the backend
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct McInstBlock {
    name: String,
    nodes: Vec<McNode>,
    entry_node: Option<McInstValue>,
    root: Option<McInstValue>,
}

/// Function handling in the start of the backend
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct McInstFunc {
    name: String,
    ty: FunctionType,
    blocks: Vec<McInstBlock>,
}

/// This structs helps lowering the IR to the closest thing capable of being shared across multiple architecturs
pub struct McInstBuilder {
    funcs: Vec<McInstFunc>,
    arch: Box<dyn TargetArchitecture>,
}

impl McInstBuilder {
    /// Builds the given module into the mc insts
    pub fn build(module: &Module, backend: Box<dyn TargetArchitecture>) -> Self {
        let mut builder = Self {
            funcs: Vec::new(),
            arch: backend,
        };
        for func in &module.funcs {
            builder.build_func(func);
        }

        builder.route_ops();

        builder
    }

    /// Builds the given function into mc insts
    pub fn build_func(&mut self, ir_func: &Function) {
        let mut func = McInstFunc {
            name: ir_func.name.to_owned(),
            ty: ir_func.function_type.to_owned(),
            blocks: Vec::new(),
        };

        for ir_block in &ir_func.ir {
            let block = match ir_block {
                IrBlock::Basic { name, insts } => {
                    self.build_basic_block(name, insts, ir_block, ir_func)
                }
            };

            func.blocks.push(block);
        }

        self.funcs.push(func);
    }

    fn build_basic_block(
        &self,
        name: &str,
        insts: &Vec<IrInst>,
        _ir_block: &IrBlock,
        _ir_func: &Function,
    ) -> McInstBlock {
        let mut block = McInstBlock {
            name: name.to_owned(),
            nodes: Vec::new(),
            entry_node: None,
            root: None,
        };

        for inst in insts {
            let node = if self.arch.custom_handled(inst) {
                self.arch.handle(inst)
            } else {
                self.handle_inst(inst)
            };

            block.nodes.push(node);
        }

        block
    }

    fn handle_inst(&self, inst: &IrInst) -> McNode {
        if inst.is_add() {
            return self.handle_math(McInstOpCode::Add, inst);
        }
        if inst.is_sub() {
            return self.handle_math(McInstOpCode::Sub, inst);
        }
        if inst.is_mul() {
            return self.handle_math(McInstOpCode::Mul, inst);
        }
        if inst.is_div() {
            return self.handle_math(McInstOpCode::Div, inst);
        }
        if inst.is_ret() {
            return self.handle_ret(inst);
        }

        todo!("Node: {inst:?} is not yet implemented")
    }

    fn handle_math(&self, opcode: McInstOpCode, inst: &IrInst) -> McNode {
        todo!()
    }

    fn handle_ret(&self, inst: &IrInst) -> McNode {
        todo!()
    }

    /// This function connects the different variables of different blocks
    fn route_ops(&mut self) {
        todo!()
    }

    /// Allocates the registers on the machine instructions
    pub fn use_regalloc(&mut self, allocator: RegisterAllocator, target: Target) {
        todo!()
    }
}
