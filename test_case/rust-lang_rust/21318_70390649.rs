
#[cfg(not(test))] pub use self::impl::{HEAP, Box};
#[cfg(test)] pub use std::boxed::{HEAP, Box};
