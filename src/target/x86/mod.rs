use crate::{code_gen::target::Target, target_trait::TargetArchitecture};

/// X86 assembly transformation
pub mod asm_trans;
/// Implementation of x86 custom mc inst transformations
pub mod mc_inst;

/// The target for the x86 64 bit architecture
#[derive(Debug)]
pub struct X86Target {}

impl X86Target {
    /// Creates a new empty instance
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for X86Target {
    fn default() -> Self {
        X86Target::new()
    }
}

impl TargetArchitecture for X86Target {
    fn name(&self) -> &'static str {
        "x86"
    }

    fn is_target(&self, target: Target) -> bool {
        target == Target::X86
    }
}
