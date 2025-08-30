use std::{collections::VecDeque, os, rc::Rc};

use crate::ir::{IrOperand, TypeMetadata};

/// Helper structure to insert resource dropping instructions in the ir
/// which will make the register allocators work easier
pub struct Dropper {
    args: Vec<TypeMetadata>,
    ir: Vec<IrOperand>,
}

impl Dropper {
    /// Creates a new dropper
    pub fn new(args: Vec<TypeMetadata>, ir: Vec<IrOperand>) -> Self {
        Self { args, ir }
    }

    /// Inserts dropping instructions into the ir
    pub fn run(&mut self) {
        // Tracks how many times each Out is used
        use std::collections::HashMap;
        let mut use_count: HashMap<u64, usize> = HashMap::new();

        // First pass: count uses of each Out
        for op in &self.ir {
            if let IrOperand::Out(node) = op {
                for inner in &node.borrow().ops {
                    if inner.is_out() {
                        let id = inner.hash_u64();
                        *use_count.entry(id).or_insert(0) += 1;
                    }
                }
            }
        }

        // Second pass: replace last uses with Drop
        for op in &mut self.ir {
            if let IrOperand::Out(node) = op {
                let mut n = node.borrow_mut();
                for inner in n.ops.iter_mut() {
                    if inner.is_out() {
                        let id = inner.hash_u64();
                        if let Some(count) = use_count.get_mut(&id) {
                            if *count == 1 {
                                // last use: wrap in Drop
                                let old = inner.clone();
                                *inner = IrOperand::Drop(Rc::new(old));
                            }
                            *count -= 1;
                        }
                    }
                }
            }
        }
    }

    /// Returns the modified ir
    pub fn get_ir(&self) -> &Vec<IrOperand> {
        &self.ir
    }
}
