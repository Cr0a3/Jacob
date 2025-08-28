use std::collections::BTreeMap;

/// # FunctionType
///
/// This struct saves the actual type of the function.
/// It includes:
///  - The arguments (saved as a name - type list)
///  - The (optional) return type
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FunctionType {
    args: BTreeMap<String, TypeMetadata>,
    ret: Option<TypeMetadata>,
}

impl FunctionType {
    /// Creates a new empty function type with no arguments
    /// and no return type
    pub fn new() -> Self {
        Self {
            args: BTreeMap::new(),
            ret: None,
        }
    }

    /// Sets the return type
    pub fn set_ret(&mut self, new_ty: TypeMetadata) {
        self.ret = Some(new_ty)
    }

    /// Adds an argument to the function type
    pub fn add_arg(&mut self, name: String, ty: TypeMetadata) {
        self.args.insert(name, ty);
    }
}

impl Default for FunctionType {
    fn default() -> Self {
        Self::new()
    }
}

/// # TypeMetadata
///
/// This enum houses the possible types to use
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum TypeMetadata {
    /// Signed 8 bit sized intenger
    i8,
    /// Unsigned 8 bit sized intenger
    u8,
    /// Signed 16 bit sized intenger
    i16,
    /// Unsigned 16 bit sized intenger
    u16,
    /// Signed 32 bit sized intenger
    i32,
    /// Unsigned 32 bit sized intenger
    u32,
    /// Signed 64 bit sized intenger
    i64,
    /// Unsigned 64 bit sized intenger
    u64,
}
