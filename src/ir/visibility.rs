/// An enum which defines the visibility
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Visibilty {
    /// The code is only visible in the module and not publicly visible and usable from other modules/object files
    Internal,
    /// The code is visibila and callable/usable from all linked modules/object files
    Public,
}
