plain
   Compiling alloc v0.0.0 (/checkout/library/alloc)
   Compiling cfg-if v0.1.10
   Compiling adler v0.2.3
   Compiling rustc-demangle v0.1.21
error[E0658]: use of unstable library feature 'ptr_metadata'
 --> library/alloc/src/boxed/thin.rs:4:23
  |
4 | use core::ptr::{self, Pointee};
  |
  = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
  = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
  = help: add `#![feature(ptr_metadata)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'ptr_metadata'
error[E0658]: use of unstable library feature 'ptr_metadata'
  --> library/alloc/src/boxed/thin.rs:14:11
   |
14 |     meta: <Dyn as Pointee>::Metadata,
   |
   = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
   = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
   = help: add `#![feature(ptr_metadata)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'ptr_metadata'
error[E0658]: use of unstable library feature 'ptr_metadata'
  --> library/alloc/src/boxed/thin.rs:52:19
52 |         let ptr = ptr::from_raw_parts(value, metadata);
   |                   ^^^^^^^^^^^^^^^^^^^
   |
   = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
   = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
   = help: add `#![feature(ptr_metadata)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'ptr_metadata'
error[E0658]: use of unstable library feature 'ptr_metadata'
  --> library/alloc/src/boxed/thin.rs:34:51
   |
34 |         let (_, metadata) = (u_ref as *const Dyn).to_raw_parts();
   |
   = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
   = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
   = help: add `#![feature(ptr_metadata)]` to the crate attributes to enable
For more information about this error, try `rustc --explain E0658`.
error: could not compile `alloc` due to 4 previous errors
warning: build failed, waiting for other jobs to finish...
error: build failed
