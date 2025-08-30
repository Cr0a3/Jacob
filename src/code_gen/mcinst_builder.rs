use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    Target,
    function::Function,
    ir::ir::{IrBlock, IrInst, IrInstTrait, IrOperand},
    linear_scan::LinearScan,
    mc::{McInstOpCode, McInstValue, McNode},
    mc_dce::mcinst_dce,
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
    pub(crate) name: String,
    pub(crate) nodes: Vec<Rc<RefCell<McNode>>>,
}

/// Function handling in the start of the backend
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct McInstFunc {
    pub(crate) name: String,
    pub(crate) ty: FunctionType,
    pub(crate) blocks: Vec<McInstBlock>,
    pub(crate) arg_insts: Vec<McNode>,
}

/// This structs helps lowering the IR to the closest thing capable of being shared across multiple architecturs
#[derive(Debug)]
pub struct McInstBuilder {
    pub(crate) funcs: Vec<McInstFunc>,
    arch: Box<dyn TargetArchitecture>,
    last_node_id: usize,
}

impl McInstBuilder {
    /// Builds the given module into the mc insts
    pub fn build(module: &Module, backend: Box<dyn TargetArchitecture>) -> Self {
        let mut builder = Self {
            funcs: Vec::new(),
            arch: backend,
            last_node_id: 0,
        };
        for func in &module.funcs {
            builder.build_func(func);
        }

        builder.route_ops();
        mcinst_dce(&mut builder);

        builder
    }

    /// Builds the given function into mc insts
    pub fn build_func(&mut self, ir_func: &Function) {
        let mut func = McInstFunc {
            name: ir_func.name.to_owned(),
            ty: ir_func.function_type.to_owned(),
            blocks: Vec::new(),
            arg_insts: Vec::new(),
        };

        for (index, (name, ty)) in ir_func.function_type.args.iter().enumerate() {
            let node = McNode {
                opcode: McInstOpCode::Arg(index),
                out: Some(name.to_owned()),
                operands: vec![],
                value_types: vec![*ty],
                node_id: self.last_node_id,
                out_allocate: None,
            };
            func.arg_insts.push(node);
            self.last_node_id += 1;
        }

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
        &mut self,
        name: &str,
        insts: &Vec<IrInst>,
        _ir_block: &IrBlock,
        _ir_func: &Function,
    ) -> McInstBlock {
        let mut block = McInstBlock {
            name: name.to_owned(),
            nodes: Vec::new(),
        };

        for inst in insts {
            let node = if self.arch.custom_handled(inst) {
                self.arch.handle(inst)
            } else {
                self.handle_inst(inst)
            };

            block.nodes.push(Rc::new(RefCell::new(node)));
        }

        block
    }

    fn handle_inst(&mut self, inst: &IrInst) -> McNode {
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

    fn handle_math(&mut self, opcode: McInstOpCode, inst: &IrInst) -> McNode {
        let op1 = self.handle_op(inst.operand(1).unwrap());
        let op2 = self.handle_op(inst.operand(2).unwrap());

        let node = McNode {
            opcode,
            operands: vec![op1, op2],
            value_types: vec![inst.get_ty(), inst.get_ty()],
            node_id: self.last_node_id,
            out: Some(inst.outputs()[0].force_name()),
            out_allocate: None,
        };
        self.last_node_id += 1;
        node
    }

    fn handle_ret(&mut self, inst: &IrInst) -> McNode {
        let value = self.handle_op(inst.operand(0).unwrap());

        let node = McNode {
            opcode: McInstOpCode::Ret,
            operands: vec![value],
            value_types: vec![inst.get_ty()],
            node_id: self.last_node_id,
            out: None,
            out_allocate: None,
        };
        self.last_node_id += 1;
        node
    }

    fn handle_op(&mut self, op: IrOperand) -> McInstValue {
        match &op {
            IrOperand::Const { ty, value } => {
                let node = McNode {
                    opcode: McInstOpCode::Const(value.clone()),
                    operands: vec![],
                    value_types: vec![*ty],
                    node_id: self.last_node_id,
                    out: None,
                    out_allocate: None,
                };
                self.last_node_id += 1;

                let node_rc = Rc::new(RefCell::new(node));

                let value = McInstValue {
                    node: node_rc,
                    result_id: self.last_node_id,
                };
                self.last_node_id += 1;

                value
            }
            IrOperand::Variable { name, ty } => {
                let node = McNode {
                    opcode: McInstOpCode::Unresolved(name.to_owned()),
                    operands: vec![],
                    value_types: vec![*ty],
                    node_id: self.last_node_id,
                    out: None,
                    out_allocate: None,
                };
                self.last_node_id += 1;

                let node_rc = Rc::new(RefCell::new(node));

                let value = McInstValue {
                    node: node_rc,
                    result_id: self.last_node_id,
                };
                self.last_node_id += 1;

                value
            }
        }
    }

    /// This function connects the variables and constants together in the mc insts for all blocks
    fn route_ops(&mut self) {
        for func in &mut self.funcs {
            let mut defs = HashMap::new();

            for arg in &func.arg_insts {
                if let Some(name) = &arg.out {
                    let rc = Rc::new(RefCell::new(arg.clone()));
                    defs.insert(
                        name.clone(),
                        McInstValue {
                            node: rc,
                            result_id: arg.node_id,
                        },
                    );
                }
            }

            for block in &func.blocks {
                for node in &block.nodes {
                    if let Some(name) = &node.borrow().out {
                        let rc = Rc::new(RefCell::new(node.borrow().clone()));
                        defs.insert(
                            name.clone(),
                            McInstValue {
                                node: rc,
                                result_id: node.borrow().node_id,
                            },
                        );
                    }
                }
            }

            for block in &mut func.blocks {
                for node in &mut block.nodes {
                    for op in &mut node.borrow_mut().operands {
                        let opcode = op.node.borrow().opcode.clone();
                        if let McInstOpCode::Unresolved(var) = opcode {
                            if let Some(def) = defs.get(&var) {
                                *op = def.clone();
                            } else {
                                panic!("Unresolved variable {var} has no definition");
                            }
                        }
                    }
                }
            }
        }
    }

    /// Allocates the registers on the machine instructions
    pub fn use_regalloc(&mut self, allocator: RegisterAllocator, target: Target) {
        match allocator {
            RegisterAllocator::LinearScan => {
                let mut regalloc = LinearScan::new(self, target);
                regalloc.run();
            }
        }
    }
}
