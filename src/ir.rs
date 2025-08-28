use crate::ty::TypeMetadata;

/// # IrBlock
///
/// NOTE: This structure is not reconmended for building the ir use the high level functions/structures instead. -> for internal usage and optimization passes.
///
/// This enum houses the different types of an ir block (with possibly more advanced metadata)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IrBlock {
    /// The most basic block, consisting of a vector of ir instructions
    Basic {
        /// The name of the ir block
        name: String,
        /// The instructions of the ir block
        insts: Vec<IrInst>,
    },
}

/// Debug information for IR instructions
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IrInstDebugNote {
    /// The line which the debug note references
    pub line: Option<u32>,
    /// The coloumn which the debug note references
    pub column: Option<u32>,
    /// The file which the debug note references
    pub file: Option<String>,
}

/// An IrOperand
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IrOperand {
    /// A simple plain constant
    Const {
        /// The type of the constant
        ty: TypeMetadata,
        /// The value of the constant
        value: Vec<u8>,
    },

    /// A variable
    Variable {
        /// The name of the variable
        name: String,
        /// The type of the variable
        ty: TypeMetadata,
    },
}

impl IrOperand {
    /// Returns the type of the ir operand
    pub fn get_ty(&self) -> &TypeMetadata {
        match self {
            IrOperand::Const { ty, value: _ } => ty,
            IrOperand::Variable { name: _, ty } => ty,
        }
    }
}

impl std::fmt::Display for IrOperand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            IrOperand::Const { ty: _, value: _ } => todo!(),
            IrOperand::Variable { name, ty } => write!(f, "{ty:?} %{name}"),
        }
    }
}

/// For internal uses
pub trait IrInstTrait: std::fmt::Debug {
    /// Returns nth operand of the ir instructioion
    fn operand(&self, num: usize) -> Option<IrOperand>;
    /// Returns the number of operands of the ir instruction
    fn num_operands(&self) -> usize;
    /// Returns a formatted version of the ir code
    fn dump(&self) -> String;

    /// Returns an owned vector to all inputs (cloned)
    fn inputs(&self) -> Vec<IrOperand>;
    /// Returns a mutable reference vector to all inputs
    fn inputs_mut(&mut self) -> Vec<&mut IrOperand>;
    /// Returns the outputs as a list
    fn outputs(&self) -> Vec<IrOperand>;

    // ToDo: add more checker functions
    /// Checks if the ir instruction is an add instruction
    fn is_add(&self) -> bool {
        false
    }
    /// Checks if the ir instruction is a sub instruction
    fn is_sub(&self) -> bool {
        false
    }
    /// Checks if the ir instruction is a mul instruction
    fn is_mul(&self) -> bool {
        false
    }
    /// Checks if the ir instruction is an division instruction
    fn is_div(&self) -> bool {
        false
    }

    // These functions here are used for random ass trait implementations cuz magic idk
    /// Internal
    fn clone_box(&self) -> Box<dyn IrInstTrait>;
    /// Internal
    fn hash_value(&self) -> u64;
}

/// Useful for defining a standard ir instruction in the ir with 3 operands.
#[macro_export]
macro_rules! ir_inst_with3_ops {
    ($name:ident, $out_num:expr) => {
        ::paste::paste! {
            /// Auto Generated IR Node
            #[derive(Debug, Clone, PartialEq, Eq, Hash)]
            pub struct [<IrInst $name>] {
                /// Operand 1 (Maybe Input | Maybe Output)
                pub op1: IrOperand,
                /// Operand 2 (Maybe Input | Maybe Output)
                pub op2: IrOperand,
                /// Operand 3 (Maybe Input | Maybe Output)
                pub op3: IrOperand,
            }

            impl [<IrInst $name>] {
                /// Creates a new instance
                pub fn raw_new(op1: IrOperand, op2: IrOperand, op3: IrOperand) -> Box<Self> {
                    Box::new(Self { op1, op2, op3 })
                }
            }

            impl IrInstTrait for [<IrInst $name>] {
                fn [<is_ $name:lower>](&self) -> bool { true }
                fn num_operands(&self) -> usize { 3 }

                fn inputs(&self) -> Vec<IrOperand> {
                    vec![
                        self.op1.clone(),
                        self.op2.clone(),
                        self.op3.clone()
                    ]
                }

                fn inputs_mut(&mut self) -> Vec<&mut IrOperand> {
                    vec![
                        &mut self.op1,
                        &mut self.op2,
                        &mut self.op3
                    ]
                }

                fn operand(&self, num: usize) -> Option<IrOperand> {
                    match num {
                        0 => Some(self.op1.clone().into()),
                        1 => Some(self.op2.clone().into()),
                        2 => Some(self.op3.clone().into()),
                        _ => None,
                    }
                }

                fn outputs(&self) -> Vec<IrOperand> {
                    match $out_num {
                        Some(num) => vec![self.inputs()[num].clone()],
                        None => vec![],
                    }
                }

                fn dump(&self) -> String { format!("{self:?}") }

                fn clone_box(&self) -> Box<dyn IrInstTrait> {
                    Box::new(self.clone())
                }

                fn hash_value(&self) -> u64 {
                    use std::hash::{Hash, Hasher};
                    let mut hasher = std::collections::hash_map::DefaultHasher::new();
                    self.op1.hash(&mut hasher);
                    self.op2.hash(&mut hasher);
                    self.op3.hash(&mut hasher);
                    hasher.finish()
                }
            }
        }
    };
}

/// # IrInst
///
/// NOTE: This structure is not reconmended for building the ir use the high level functions/structures instead. -> for internal usage and optimization passes.
///
/// This structure houses an abstract type of an ir instruction which can have debug metadata
#[derive(Debug, Clone)]
pub struct IrInst {
    inst: Box<dyn IrInstTrait>,
    debug: Option<IrInstDebugNote>,
}

impl IrInst {
    /// Creates a new ir instruction
    pub fn new(inst: Box<dyn IrInstTrait>, debug: Option<IrInstDebugNote>) -> Self {
        Self { inst, debug }
    }
}

impl PartialEq for IrInst {
    fn eq(&self, other: &Self) -> bool {
        self.inst == other.inst.clone() && self.debug == other.debug
    }
}

impl Eq for IrInst {}

impl std::hash::Hash for IrInst {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.inst.hash(state);
        self.debug.hash(state);
    }
}

// Now come random ass trait implementations which magically make everything work

impl Clone for Box<dyn IrInstTrait> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl PartialEq for Box<dyn IrInstTrait> {
    fn eq(&self, other: &Self) -> bool {
        self.dump() == other.dump()
    }
}

impl Eq for Box<dyn IrInstTrait> {}

impl std::hash::Hash for Box<dyn IrInstTrait> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u64(self.hash_value());
    }
}
