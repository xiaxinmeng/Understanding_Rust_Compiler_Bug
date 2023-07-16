rust
/// bar docs
pub mod bar {
    /// the docs for Bar
    pub trait Bar {}
}

mod qux {
    #[doc(no_inline)]
    pub use crate::bar::Bar;
}

#[doc(inline_once)]
pub use qux::Bar;
