plain
---- src/ptr/mod.rs - ptr (line 192) stdout ----
error[E0658]: use of unstable library feature 'strict_provenance'
  --> src/ptr/mod.rs:204:22
   |
15 |     let tagged = ptr.map_addr(|addr| addr | HAS_DATA);
   |
   = note: see issue #95228 <https://github.com/rust-lang/rust/issues/95228> for more information
   = note: see issue #95228 <https://github.com/rust-lang/rust/issues/95228> for more information
   = help: add `#![feature(strict_provenance)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'strict_provenance'
  --> src/ptr/mod.rs:207:15
   |
   |
18 |     if tagged.addr() & HAS_DATA != 0 {
   |
   = note: see issue #95228 <https://github.com/rust-lang/rust/issues/95228> for more information
   = note: see issue #95228 <https://github.com/rust-lang/rust/issues/95228> for more information
   = help: add `#![feature(strict_provenance)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'strict_provenance'
  --> src/ptr/mod.rs:209:28
   |
   |
20 |         let data = *tagged.map_addr(|addr| addr & FLAG_MASK);
   |
   = note: see issue #95228 <https://github.com/rust-lang/rust/issues/95228> for more information
   = note: see issue #95228 <https://github.com/rust-lang/rust/issues/95228> for more information
   = help: add `#![feature(strict_provenance)]` to the crate attributes to enable
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/ptr/non_null.rs - ptr::non_null::NonNull<[T]>::as_mut_ptr (line 487) stdout ----
error[E0308]: mismatched types
 --> src/ptr/non_null.rs:492:1
  |
8 | assert_eq!(slice.as_mut_ptr(), NonNull::<i8>::dangling());
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected *-ptr, found struct `NonNull`
  |
  = note: expected raw pointer `*mut i8`
                  found struct `NonNull<i8>`
  = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
Couldn't compile the test.
