rust
mod a {
    /// baz
    pub struct Type;
}

mod b {
    /// bar
    pub use crate::a::Type;
}

/// foo
pub use b::Type;
