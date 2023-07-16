
pub use private::PubMarked as Pub;
mod private { pub struct PubMarked; }
pub fn f() -> private::PubMarked { ... } // Exported by EmbargoVisitor, because of `pub use`
