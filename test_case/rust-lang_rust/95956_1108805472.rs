plain
617 | impl Error for alloc::ffi::NulError {
    |                ^^^^^^^^^^^^^^^^^^^^
    |
    = note: see issue #94079 <https://github.com/rust-lang/rust/issues/94079> for more information
    = help: add `#![feature(alloc_ffi)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'alloc_ffi'
   --> library/std/src/error.rs:625:11
    |
    |
625 | impl From<alloc::ffi::NulError> for io::Error {
    |
    = note: see issue #94079 <https://github.com/rust-lang/rust/issues/94079> for more information
    = note: see issue #94079 <https://github.com/rust-lang/rust/issues/94079> for more information
    = help: add `#![feature(alloc_ffi)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'alloc_ffi'
   --> library/std/src/error.rs:644:16
    |
644 | impl Error for alloc::ffi::FromVecWithNulError {}
644 | impl Error for alloc::ffi::FromVecWithNulError {}
    |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: see issue #94079 <https://github.com/rust-lang/rust/issues/94079> for more information
    = help: add `#![feature(alloc_ffi)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'alloc_ffi'
   --> library/std/src/error.rs:647:16
    |
    |
647 | impl Error for alloc::ffi::IntoStringError {
    |
    = note: see issue #94079 <https://github.com/rust-lang/rust/issues/94079> for more information
    = note: see issue #94079 <https://github.com/rust-lang/rust/issues/94079> for more information
    = help: add `#![feature(alloc_ffi)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'alloc_ffi'
   --> library/std/src/error.rs:627:16
    |
    |
627 |     fn from(_: alloc::ffi::NulError) -> io::Error {
    |
    = note: see issue #94079 <https://github.com/rust-lang/rust/issues/94079> for more information
    = note: see issue #94079 <https://github.com/rust-lang/rust/issues/94079> for more information
    = help: add `#![feature(alloc_ffi)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'alloc_ffi'
   --> library/std/src/ffi/mod.rs:151:32
    |
151 | pub type FromVecWithNulError = alloc::ffi::FromVecWithNulError;
151 | pub type FromVecWithNulError = alloc::ffi::FromVecWithNulError;
    |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: see issue #94079 <https://github.com/rust-lang/rust/issues/94079> for more information
    = help: add `#![feature(alloc_ffi)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'alloc_ffi'
   --> library/std/src/ffi/mod.rs:154:20
    |
154 | pub type CString = alloc::ffi::CString;
154 | pub type CString = alloc::ffi::CString;
    |                    ^^^^^^^^^^^^^^^^^^^
    |
    = note: see issue #94079 <https://github.com/rust-lang/rust/issues/94079> for more information
    = help: add `#![feature(alloc_ffi)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'alloc_ffi'
   --> library/std/src/ffi/mod.rs:157:28
    |
    |
157 | pub type IntoStringError = alloc::ffi::IntoStringError;
    |
    = note: see issue #94079 <https://github.com/rust-lang/rust/issues/94079> for more information
    = note: see issue #94079 <https://github.com/rust-lang/rust/issues/94079> for more information
    = help: add `#![feature(alloc_ffi)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'alloc_ffi'
   --> library/std/src/ffi/mod.rs:160:21
    |
    |
160 | pub type NulError = alloc::ffi::NulError;
    |
    = note: see issue #94079 <https://github.com/rust-lang/rust/issues/94079> for more information
    = note: see issue #94079 <https://github.com/rust-lang/rust/issues/94079> for more information
    = help: add `#![feature(alloc_ffi)]` to the crate attributes to enable
For more information about this error, try `rustc --explain E0658`.
error: could not compile `std` due to 9 previous errors
Build completed unsuccessfully in 0:03:48
