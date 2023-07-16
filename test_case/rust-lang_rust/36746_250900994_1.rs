ignore
//! impl<I: Iterator> IntoIterator for I {
//!     type Item = I::Item;
//!     type IntoIter = I;
//! 
//!     fn into_iter(self) -> I {
//!         self
//!     }
//! }
//! 