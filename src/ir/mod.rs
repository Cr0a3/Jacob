//! Internal representation

/// Function
pub mod function;
/// Compilation unit
pub mod module;
/// Ir nodes
pub mod node;
/// Ir Operand
pub mod operand;
/// Types
pub mod ty;

pub use function::*;
pub use module::*;
pub use node::*;
pub use operand::*;
pub use ty::*;
