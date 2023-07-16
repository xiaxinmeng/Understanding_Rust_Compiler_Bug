rust
mod option {
     pub trait OptionExt<T> { /* our methods to put on all Option types */ }
     
     impl<T> OptionExt<T> for Option<T> { /* */ }
}

pub mod prelude {
    pub use crate::option::OptionExt as _;
}
