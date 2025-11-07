/// The type of the instrinc
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InstrincType {
    /// Returns the stack pointer
    GetStackPointer,
}

/// Settings instrinc calling
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InstrincSettings {
    /// The type of the instrinc
    pub(crate) instrinc: InstrincType,
}

impl InstrincSettings {
    /// Returns the default settings for getting as tack ptr
    pub fn get_stack_ptr() -> Self {
        Self {
            instrinc: InstrincType::GetStackPointer,
        }
    }
}
