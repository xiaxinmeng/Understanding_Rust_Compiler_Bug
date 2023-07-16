rust
#[rustc_per_edition]
pub use self::rust_0000::Range;

mod rust_2015 { pub use crate::iter::Range; }
mod rust_2018 { pub use crate::iter::Range; }
mod rust_2021 { pub struct Range<T> {...} ... }
