
mod private { pub struct PubMarked; }
pub fn f() -> private::PubMarked { ... } // Not exported by EmbargoVisitor, linker errors are possible
