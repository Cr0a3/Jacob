use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    codegen::ArchBackend,
    ir::{IrNode, IrOpcode, IrOperand, TypeMetadata},
};

/// The resource to use for an allocation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Allocation {
    /// The register to use
    Register {
        /// Id of the register
        id: usize,
        /// Type of the register
        ty: TypeMetadata,
    },
    /// A stock position
    Stack {
        /// The slot
        slot: usize,
        /// Type to store
        ty: TypeMetadata,
    },
}

impl Allocation {
    /// Returns if it's a register
    #[inline]
    pub fn is_gr(&self) -> bool {
        matches!(self, Allocation::Register { .. })
    }

    /// Returns if it's a stack var
    #[inline]
    pub fn is_mem(&self) -> bool {
        matches!(self, Allocation::Stack { .. })
    }

    /// Returns if it's a constant int
    #[inline]
    pub fn is_imm(&self) -> bool {
        false
    }

    /// Returns the type of the allocation (not Register/Stack but Int64 for example)
    pub fn get_ty(&self) -> TypeMetadata {
        match self {
            Allocation::Register { id: _, ty } => *ty,
            Allocation::Stack { slot: _, ty } => *ty,
        }
    }
}

/// same as `src/ir/operand.rs - IrOperand` but with a allocated dest
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AllocatedIrOperand {
    /// Argument
    Arg {
        /// The number of the arg
        num: usize,
        /// The type
        ty: TypeMetadata,
        /// Allocation
        alloc: Allocation,
    },
    /// The output of a previous instruction
    Inst {
        /// Opcode of the instruction
        opcode: IrOpcode,
        /// Allocation for output
        alloc: Option<Allocation>,
        /// Operands
        ops: Vec<Allocation>,
    },
}

/// same as `src/ir/node.rs - IrNode` but with a allocated dest
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllocatedIrNode {
    pub(crate) opcode: IrOpcode,
    pub(crate) ops: Vec<Allocation>,
    pub(crate) has_out: bool,
    pub(crate) ty: Option<TypeMetadata>,
    pub(crate) alloc: Option<Allocation>,
}
/// Helper structure for register allocation
pub struct RegAlloc<'a> {
    args: Vec<TypeMetadata>,

    allocated_ir: Vec<AllocatedIrNode>,
    free_regs: Vec<Allocation>,
    freed_mem: Vec<Allocation>,

    max_stack_poses_used: usize,
    used_callee_saved_regs: bool, // ToDo

    back: &'a dyn ArchBackend,
}

impl<'a> RegAlloc<'a> {
    /// Creates a register allocator
    pub fn new(args: Vec<TypeMetadata>, backend: &'a dyn ArchBackend) -> Self {
        // ToDo: remove args from free regs
        Self {
            args,
            allocated_ir: Vec::new(),

            free_regs: backend
                .grps()
                .iter()
                .map(|x| Allocation::Register {
                    id: x.id(),
                    ty: x.ty(),
                })
                .rev()
                .collect(),
            freed_mem: Vec::new(),
            max_stack_poses_used: 0,
            used_callee_saved_regs: false,
            back: backend,
        }
    }

    /// Runs the register allocator
    pub fn run(&mut self, ir: &Vec<IrOperand>) {
        let mut allocs = HashMap::new();

        for op in ir {
            if let IrOperand::Out(node) = op {
                self.make_node(op.hash_u64(), node, &mut allocs);
            }
        }
    }

    fn make_node(
        &mut self,
        operand_hash: u64,
        node: &Rc<RefCell<IrNode>>,
        allocs: &mut HashMap<u64, Allocation>,
    ) {
        let node = node.borrow();

        let mut alloc = None;
        let mut ops = Vec::new();

        if node.has_out {
            if let std::collections::hash_map::Entry::Vacant(e) = allocs.entry(operand_hash) {
                let new_alloc = self.alloc(node.ty);
                e.insert(new_alloc);

                alloc = Some(new_alloc);
            } else {
                alloc = allocs.get(&operand_hash).copied();
            }
        }

        for op in &node.ops {
            if op.is_drop() {
                if let Some(alloc) = allocs.get(&op.hash_u64()) {
                    println!("{:?}", operand_hash == op.hash_u64());
                    ops.push(*alloc);
                    self.free(*alloc);
                } else {
                    let op = op.force_op();
                    if op.is_out() {
                        self.make_node(op.hash_u64(), op.force_node(), allocs);
                        ops.push(
                            *allocs
                                .get(&op.hash_u64())
                                .expect("ToDo: Insert error message here"),
                        );
                    } else if op.is_arg() {
                        let (num, ty) = op.force_arg();
                        self.free_arg(num, ty);
                        ops.push(self.pos_for_arg(num, ty));
                    } else {
                        todo!("{op:?}")
                    }
                }
            } else {
                let (num, ty) = op.force_arg();
                ops.push(self.pos_for_arg(num, ty))
            }
        }

        self.allocated_ir.push(AllocatedIrNode {
            opcode: node.opcode,
            ops,
            has_out: node.has_out,
            ty: node.ty,
            alloc,
        });
    }

    /// Allocates a resource
    fn alloc(&mut self, _ty: Option<TypeMetadata>) -> Allocation {
        if let Some(reg) = self.free_regs.pop() {
            return reg;
        }
        if let Some(freed) = self.freed_mem.pop() {
            return freed;
        }

        let slot = self.max_stack_poses_used;
        self.max_stack_poses_used += 1;

        Allocation::Stack {
            slot,
            ty: _ty.unwrap(),
        }
    }

    /// Frees the given resource
    fn free(&mut self, res: Allocation) {
        match res {
            Allocation::Register { .. } => self.free_regs.push(res),
            Allocation::Stack { .. } => self.freed_mem.push(res),
        }
    }

    /// Returns the position for the given argument
    #[inline]
    fn pos_for_arg(&self, num: usize, ty: TypeMetadata) -> Allocation {
        self.back.callconv_argpos(num, ty)
    }

    /// Frees the given argument
    #[inline]
    fn free_arg(&mut self, num: usize, ty: TypeMetadata) {
        self.free(self.pos_for_arg(num, ty));
    }

    /// Returns the new and allocated ir
    pub fn get_ir(&self) -> &Vec<AllocatedIrNode> {
        &self.allocated_ir
    }
}
