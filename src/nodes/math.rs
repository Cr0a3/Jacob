use crate::ir_inst_with3_ops;
use crate::ir::*;

ir_inst_with3_ops!(Add, Some(0));
ir_inst_with3_ops!(Sub, Some(0));
ir_inst_with3_ops!(Mul, Some(0));
ir_inst_with3_ops!(Div, Some(0));

impl std::fmt::Display for IrInstAdd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = {{{{name}}}} {} {}", self.op1, self.op2, self.op3)
    }
}

impl std::fmt::Display for IrInstSub {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = {{{{name}}}} {} {}", self.op1, self.op2, self.op3)
    }
}

impl std::fmt::Display for IrInstMul {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = {{{{name}}}} {} {}", self.op1, self.op2, self.op3)
    }
}

impl std::fmt::Display for IrInstDiv {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = {{{{name}}}} {} {}", self.op1, self.op2, self.op3)
    }
}