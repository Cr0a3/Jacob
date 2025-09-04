use std::{
    cell::RefCell,
    hash::{Hash, Hasher},
    rc::Rc,
};

use crate::ir::{node::IrNode, ty::TypeMetadata};

/// An ir operand
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IrOperand {
    /// Argument
    Arg {
        /// The number of the arg
        num: usize,
        /// The type
        ty: TypeMetadata,
    },
    /// A constant number
    ConstNum {
        /// The number
        num: usize,
        /// The type
        ty: TypeMetadata,
    },

    /// The output of a previous instruction
    Out(Rc<RefCell<IrNode>>),

    /// Drop the used resource for the output of the given instruction
    Drop(Rc<IrOperand>),
}

impl IrOperand {
    /// Returns the type
    pub fn get_ty(&self) -> TypeMetadata {
        match self {
            IrOperand::Arg { num: _, ty } => *ty,
            IrOperand::ConstNum { num: _, ty } => *ty,
            IrOperand::Out(ref_cell) => ref_cell.borrow().get_ty().expect("Expected type"),
            IrOperand::Drop(ref_cell) => ref_cell.get_ty(),
        }
    }

    /// Returns if the operands type is an argument
    pub fn is_arg(&self) -> bool {
        matches!(self, IrOperand::Arg { num: _, ty: _ })
    }

    /// Returns if the operands type is a drop
    pub fn is_drop(&self) -> bool {
        matches!(self, IrOperand::Drop(_))
    }

    /// Returns if the operands type is the output of a previous node
    pub fn is_out(&self) -> bool {
        matches!(self, IrOperand::Out(_))
    }

    /// Force gets a node (if it's something else e.g: an arg it just panics)
    pub fn force_node(&self) -> &Rc<RefCell<IrNode>> {
        match self {
            IrOperand::Out(ref_cell) => ref_cell,
            _ => panic!(),
        }
    }

    /// Force gets the internal operand for the drop variant
    pub fn force_op(&self) -> &Rc<IrOperand> {
        match self {
            IrOperand::Drop(rc) => rc,
            _ => panic!(),
        }
    }

    /// Force gets the argument data fields for the arg variant
    pub fn force_arg(&self) -> (usize, TypeMetadata) {
        match self {
            IrOperand::Arg { num, ty } => (*num, *ty),
            _ => panic!(),
        }
    }
}

impl Hash for IrOperand {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

impl IrOperand {
    /// Returns the hash
    pub fn hash_u64(&self) -> u64 {
        let mut hasher = std::hash::DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}
