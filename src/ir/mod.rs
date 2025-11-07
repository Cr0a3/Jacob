//! Internal representation

/// Function
pub mod function;
/// Instrincs
pub mod instrinc;
/// Compilation unit
pub mod module;
/// Ir nodes
pub mod node;
/// Ir Operand
pub mod operand;
/// Types
pub mod ty;
/// Visibilty
pub mod visibility;

pub use function::*;
pub use instrinc::*;
pub use module::*;
pub use node::*;
pub use operand::*;
pub use ty::*;
