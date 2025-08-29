use crate::ir::ir::{IrInstTrait, IrOperand};
use crate::ir_inst_with1_op;

ir_inst_with1_op!(Ret, None);

impl std::fmt::Display for IrInstRet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ret {}", self.op1,)
    }
}
