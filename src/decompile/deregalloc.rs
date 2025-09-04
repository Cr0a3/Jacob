use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    codegen::{AllocatedIrNode, Allocation, TargetArch},
    ir::{IrNode, IrOpcode, IrOperand},
};

/// This helper structure is used to reverse enginner a list of
/// `AllocatedIrNodes` into a
///
/// Note: it does not add dropping ir operands because they are
/// not neccessary for simple decompilation
pub struct DeRegAlloc<'a> {
    allocated_ir: &'a Vec<AllocatedIrNode>,
    ir: Vec<IrNode>,
    inst_map: HashMap<Allocation, IrOperand>,
    target: TargetArch,
}

impl<'a> DeRegAlloc<'a> {
    /// Creates a new deregalloc instance
    pub fn new(allocated_ir: &'a Vec<AllocatedIrNode>, target: TargetArch) -> Self {
        Self {
            allocated_ir,
            ir: Vec::new(),
            inst_map: HashMap::new(),
            target,
        }
    }

    /// Deallocation time!
    pub fn dealloc(&mut self) {
        let back = self.target.backend();

        for inst in self.allocated_ir {
            let mut node = IrNode {
                opcode: inst.opcode,
                ops: Vec::new(),
                has_out: inst.has_out,
                ty: inst.ty,
            };

            if inst.opcode == IrOpcode::Ret // ToDo: won't work for good returns
                && let Some(op) = self.inst_map.get(&back.ret_reg())
            {
                node.ops.push(op.to_owned());
            }

            for op in &inst.ops {
                if let Some(operand) = self.inst_map.get(op) {
                    node.ops.push(operand.to_owned());
                } else if !op.is_imm() {
                    // Now it's an argument which we can insert here
                    node.ops.push(IrOperand::Arg {
                        num: back.num_for_arg(op),
                        ty: crate::ir::TypeMetadata::Int64,
                    });
                } else if let Allocation::Imm { num, ty } = op {
                    node.ops.push(IrOperand::ConstNum { num: *num, ty: *ty });
                }
            }

            if let Some(out) = &inst.alloc {
                let op = IrOperand::Out(Rc::new(RefCell::new(node.to_owned())));

                *self.inst_map.entry(*out).or_insert(op.to_owned()) = op.to_owned();
            }

            self.ir.push(node);
        }
    }

    /// Returns the deallocated ir
    pub fn ir(&self) -> &Vec<IrNode> {
        &self.ir
    }

    /// Returns the deallocated ir (owned)
    pub fn ir_owned(self) -> Vec<IrNode> {
        self.ir
    }
}
