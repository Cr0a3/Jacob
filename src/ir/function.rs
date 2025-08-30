use crate::ir::{IrNode, operand::IrOperand, ty::TypeMetadata};

/// Saves the ir code for a function
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function {
    pub(crate) name: String,

    pub(crate) ret: Option<TypeMetadata>,
    pub(crate) args: Vec<TypeMetadata>,

    pub(crate) ir: Vec<IrOperand>,
}

impl Function {
    /// Creates a new function
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            ret: None,
            args: Vec::new(),

            ir: Vec::new(),
        }
    }

    /// Sets the return type of the function
    pub fn set_ret(&mut self, new: TypeMetadata) {
        self.ret = Some(new)
    }

    /// Adds an argument to the function
    pub fn add_arg(&mut self, ty: TypeMetadata) -> IrOperand {
        self.args.push(ty);
        IrOperand::Arg {
            num: self.args.len() - 1,
            ty,
        }
    }

    /// Adds two numbers
    pub fn add(&mut self, lhs: &IrOperand, rhs: &IrOperand) -> IrOperand {
        let node = IrNode::add(lhs, rhs);
        self.ir.push(node.to_owned());
        node
    }

    /// Subtracts two numbers
    pub fn sub(&mut self, lhs: &IrOperand, rhs: &IrOperand) -> IrOperand {
        let node = IrNode::sub(lhs, rhs);
        self.ir.push(node.to_owned());
        node
    }

    /// Returns the given constant
    pub fn ret(&mut self, op: &IrOperand) {
        self.ir.push(IrNode::ret(op));
    }
}
