use crate::{ir::ir::IrInst, target_trait::McInstHandler, x86::X86Target};

impl McInstHandler for X86Target {
    fn custom_handled(&self, inst: &IrInst) -> bool {
        false
    }

    fn handle(&self, inst: &IrInst) -> crate::mc::McNode {
        todo!()
    }
}
