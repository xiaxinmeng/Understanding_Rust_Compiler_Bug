rust
pub mod outer {
  mod inner {
    #[doc(hidden)] struct FooBar;
    struct FooBaz;
  }
  /// FooBar will be inlined here, because it's explicitly named.
  pub use inner::FooBar;
  /// FooBaz, of course, will also be inlined here.
  pub use inner::FooBaz;
}
/// FooBar *WON'T* be inlined here, because it's #[doc(hidden)]
/// However, FooBaz will be inlined here, because it's just private.
pub use outer::inner::*;
