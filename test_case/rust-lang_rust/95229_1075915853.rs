plain
.................................................................................................... 2700/3812
.................................................................................................... 2800/3812
.................................................................................................... 2900/3812
...................................................................................................i 3000/3812
ii...................................F......................................F..........F............ 3100/3812
........F........................................................................................... 3200/3812
.................................................................................................... 3400/3812
.....................i...............................................i..................i........... 3500/3812
............i.......................i.......................i.......................i............... 3600/3812
.....................i.......................i.......................i.......................i...... 3700/3812
---
---- src/intrinsics.rs - intrinsics::transmute (line 984) stdout ----
error[E0658]: use of unstable library feature 'strict_provenance'
  --> src/intrinsics.rs:991:40
   |
10 | let ptr_num_cast = (ptr as *const i32).addr();
   |
   = note: see issue #99999999 <https://github.com/rust-lang/rust/issues/99999999> for more information
   = note: see issue #99999999 <https://github.com/rust-lang/rust/issues/99999999> for more information
   = help: add `#![feature(strict_provenance)]` to the crate attributes to enable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/ptr/const_ptr.rs - ptr::const_ptr::*constT::offset_from (line 532) stdout ----
error[E0658]: use of unstable library feature 'strict_provenance'
 --> src/ptr/const_ptr.rs:535:18
  |
6 | let diff = (ptr2.addr() as isize).wrapping_sub(ptr1.addr() as isize);
  |
  = note: see issue #99999999 <https://github.com/rust-lang/rust/issues/99999999> for more information
  = note: see issue #99999999 <https://github.com/rust-lang/rust/issues/99999999> for more information
  = help: add `#![feature(strict_provenance)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'strict_provenance'
 --> src/ptr/const_ptr.rs:535:53
  |
  |
6 | let diff = (ptr2.addr() as isize).wrapping_sub(ptr1.addr() as isize);
  |
  = note: see issue #99999999 <https://github.com/rust-lang/rust/issues/99999999> for more information
  = note: see issue #99999999 <https://github.com/rust-lang/rust/issues/99999999> for more information
  = help: add `#![feature(strict_provenance)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'strict_provenance'
 --> src/ptr/const_ptr.rs:538:17
  |
  |
9 | assert_eq!(ptr2.addr(), ptr2_other.addr());
  |
  = note: see issue #99999999 <https://github.com/rust-lang/rust/issues/99999999> for more information
  = note: see issue #99999999 <https://github.com/rust-lang/rust/issues/99999999> for more information
  = help: add `#![feature(strict_provenance)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'strict_provenance'
 --> src/ptr/const_ptr.rs:538:36
  |
  |
9 | assert_eq!(ptr2.addr(), ptr2_other.addr());
  |
  = note: see issue #99999999 <https://github.com/rust-lang/rust/issues/99999999> for more information
  = note: see issue #99999999 <https://github.com/rust-lang/rust/issues/99999999> for more information
  = help: add `#![feature(strict_provenance)]` to the crate attributes to enable
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/ptr/mod.rs - ptr::zst_exists (line 299) stdout ----
error: unused import: `mem`
  |
  |
4 | use core::{ptr, mem};
  |
note: the lint level is defined here
 --> src/ptr/mod.rs:297:9
  |
  |
1 | #![deny(warnings)]
  |         ^^^^^^^^
  = note: `#[deny(unused_imports)]` implied by `#[deny(warnings)]`

error[E0658]: use of unstable library feature 'strict_provenance'
 --> src/ptr/mod.rs:303:19
  |
7 | let my_good_ptr = ptr::zst_exists::<()>(0xc001_add7);
  |
  = note: see issue #99999999 <https://github.com/rust-lang/rust/issues/99999999> for more information
  = note: see issue #99999999 <https://github.com/rust-lang/rust/issues/99999999> for more information
  = help: add `#![feature(strict_provenance)]` to the crate attributes to enable
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/ptr/mut_ptr.rs - ptr::mut_ptr::*mutT::offset_from (line 710) stdout ----
error[E0658]: use of unstable library feature 'strict_provenance'
 --> src/ptr/mut_ptr.rs:713:18
  |
6 | let diff = (ptr2.addr() as isize).wrapping_sub(ptr1.addr() as isize);
  |
  = note: see issue #99999999 <https://github.com/rust-lang/rust/issues/99999999> for more information
  = note: see issue #99999999 <https://github.com/rust-lang/rust/issues/99999999> for more information
  = help: add `#![feature(strict_provenance)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'strict_provenance'
 --> src/ptr/mut_ptr.rs:713:53
  |
  |
6 | let diff = (ptr2.addr() as isize).wrapping_sub(ptr1.addr() as isize);
  |
  = note: see issue #99999999 <https://github.com/rust-lang/rust/issues/99999999> for more information
  = note: see issue #99999999 <https://github.com/rust-lang/rust/issues/99999999> for more information
  = help: add `#![feature(strict_provenance)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'strict_provenance'
 --> src/ptr/mut_ptr.rs:716:17
  |
  |
9 | assert_eq!(ptr2.addr(), ptr2_other.addr());
  |
  = note: see issue #99999999 <https://github.com/rust-lang/rust/issues/99999999> for more information
  = note: see issue #99999999 <https://github.com/rust-lang/rust/issues/99999999> for more information
  = help: add `#![feature(strict_provenance)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'strict_provenance'
 --> src/ptr/mut_ptr.rs:716:36
  |
  |
9 | assert_eq!(ptr2.addr(), ptr2_other.addr());
  |
  = note: see issue #99999999 <https://github.com/rust-lang/rust/issues/99999999> for more information
  = note: see issue #99999999 <https://github.com/rust-lang/rust/issues/99999999> for more information
  = help: add `#![feature(strict_provenance)]` to the crate attributes to enable
error: aborting due to 4 previous errors

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
