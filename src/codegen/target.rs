use std::any::Any;

use crate::{codegen::Allocation, ir::TypeMetadata};

/// The target architecture
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TargetArch {
    /// 64Bit x86
    X86,
}

/// The trait to implement when defining the backend for a custom architecture
pub trait ArchBackend: Any {
    /// Returns the name of the backend
    fn name(&self) -> &'static str;

    /// Returns a list of all gpr registers to use which are caller saved
    fn caller_gpr(&self) -> Vec<Box<dyn Reg>>;

    /// Returns a list of all gpr registers to use which are callee saved
    fn callee_gpr(&self) -> Vec<Box<dyn Reg>>;

    /// Returns a list of all gpr registers
    fn grps(&self) -> Vec<Box<dyn Reg>>;

    /// Returns the position for an argument
    fn callconv_argpos(&self, num: usize, ty: TypeMetadata) -> Allocation;
}

/// The trait to implement for defining custom register
pub trait Reg: Any + std::fmt::Debug {
    /// Returns the name of the register
    fn name(&self) -> &'static str;

    /// Returns if the register is a general pourpuse register
    fn is_gpr(&self) -> bool;

    /// Returns if the register needs to be caller saved
    fn caller_saved(&self) -> bool;

    /// Returns if the register needs to be callee saved
    fn callee_saved(&self) -> bool {
        !self.caller_saved()
    }

    /// Returns the id of the register
    fn id(&self) -> usize;

    /// Returns the type of the register
    fn ty(&self) -> TypeMetadata;
}
