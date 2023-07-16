plain
    Checking rustc-demangle v0.1.21
error[E0658]: use of unstable library feature 'maybe_uninit_uninit_array_transpose'
   --> library/alloc/src/vec/into_iter.rs:226:40
    |
226 |             return Ok(unsafe { raw_ary.transpose().assume_init() });
    |
    = note: see issue #96097 <https://github.com/rust-lang/rust/issues/96097> for more information
    = help: add `#![feature(maybe_uninit_uninit_array_transpose)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'maybe_uninit_uninit_array_transpose'
   --> library/alloc/src/vec/into_iter.rs:244:24
    |
244 |             Ok(raw_ary.transpose().assume_init())
    |
    = note: see issue #96097 <https://github.com/rust-lang/rust/issues/96097> for more information
    = help: add `#![feature(maybe_uninit_uninit_array_transpose)]` to the crate attributes to enable

