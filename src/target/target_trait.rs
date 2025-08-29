use as_any::AsAny;

use crate::{code_gen::target::Target, ir::ir::IrInst, mc::McNode};

/// # Target Archictecture
///
/// This trait is used to describe a given target architecture
pub trait TargetArchitecture: McInstHandler + AssemblyTransformer + AsAny {
    /// Returns the name of the target Architecture
    fn name(&self) -> &'static str;
    /// Returns if the given target matches the architecture
    fn is_target(&self, target: Target) -> bool;
}

/// This trait defines target specific mc inst handling
pub trait McInstHandler {
    /// Returns if the given instruction should be custom handled for the given architecture
    fn custom_handled(&self, inst: &IrInst) -> bool;

    /// Handles the conversion of the ir instruction to mc inst (called after custom_handled check)
    fn handle(&self, inst: &IrInst) -> McNode;
}

/// This trait defines target specific assembly handling/creation
pub trait AssemblyTransformer {}
