rust
#[doc(cfg(feature = "foobar"))]
mod imp_priv {
    /// Feature on `struct` in private module is never shown
    pub struct BarPriv {}
    impl BarPriv {
        /// Oddly enough the feature guard _is_ shown here
        pub fn test() {}
    }
}
#[doc(cfg(feature = "foobar"))]
pub use crate::imp_priv::*;
