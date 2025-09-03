use crate::ir::{IrNode, TypeMetadata};

/// Helper structure to reverse enginner the function type
/// from the list of ir nodes
pub struct TypeExtractor<'a> {
    ir: &'a Vec<IrNode>,
    args: Vec<TypeMetadata>,
    ret: Option<TypeMetadata>,
}

impl<'a> TypeExtractor<'a> {
    /// Creates a new type extractor
    pub fn new(ir: &'a Vec<IrNode>) -> Self {
        Self {
            ir,
            args: Vec::new(),
            ret: None,
        }
    }

    /// Runs the extraction process
    pub fn extract(&mut self) {
        todo!()
    }

    /// Returns the extracted function type arguments
    pub fn args(&self) -> &Vec<TypeMetadata> {
        &self.args
    }

    /// Returns the extracted function return type
    pub fn ret(&self) -> &Option<TypeMetadata> {
        &self.ret
    }
}
