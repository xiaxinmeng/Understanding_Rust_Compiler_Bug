plain
running 594 tests
.................................................................................................... 100/594
.................................................................................................... 200/594
..................................i................................................................. 300/594
..............................FF..................F................................................. 400/594
..............................................................................F..................... 500/594
............................................................FF.F..FFFF......F.................

---- src/slice.rs - slice::[T]::try_to_vec (line 531) stdout ----
error[E0658]: use of unstable library feature 'more_fallible_allocation_methods'
 --> src/slice.rs:533:11
 --> src/slice.rs:533:11
  |
5 | let x = s.try_to_vec().unwrap();
  |
  = note: see issue #86942 <https://github.com/rust-lang/rust/issues/86942> for more information
  = note: see issue #86942 <https://github.com/rust-lang/rust/issues/86942> for more information
  = help: add `#![feature(more_fallible_allocation_methods)]` to the crate attributes to enable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/slice.rs - slice::[T]::try_to_vec_in (line 573) stdout ----
error[E0658]: use of unstable library feature 'more_fallible_allocation_methods'
 --> src/slice.rs:579:11
  |
9 | let x = s.try_to_vec_in(System).unwrap();
  |
  = note: see issue #86942 <https://github.com/rust-lang/rust/issues/86942> for more information
  = note: see issue #86942 <https://github.com/rust-lang/rust/issues/86942> for more information
  = help: add `#![feature(more_fallible_allocation_methods)]` to the crate attributes to enable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/str.rs - str::str::try_to_owned (line 584) stdout ----
error[E0658]: use of unstable library feature 'more_fallible_allocation_methods'
 --> src/str.rs:586:20
  |
5 | let ss: String = s.try_to_owned().unwrap();
  |
  = note: see issue #86942 <https://github.com/rust-lang/rust/issues/86942> for more information
  = note: see issue #86942 <https://github.com/rust-lang/rust/issues/86942> for more information
  = help: add `#![feature(more_fallible_allocation_methods)]` to the crate attributes to enable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/sync.rs - sync::Arc<[T]>::try_from_vec (line 2456) stdout ----
error[E0599]: no function or associated item named `try_from` found for struct `Arc<_>` in the current scope
  |
  |
6 | let shared: Arc<[i32]> = Arc::try_from(unique).unwrap();
  |                               ^^^^^^^^ function or associated item not found in `Arc<_>`
  = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
  |
3 | use std::convert::TryFrom;
---
---- src/vec/mod.rs - vec::Vec<T, A>::try_into_boxed_slice (line 1119) stdout ----
error[E0658]: use of unstable library feature 'more_fallible_allocation_methods'
 --> src/vec/mod.rs:1122:15
  |
6 | let slice = v.try_into_boxed_slice().unwrap();
  |
  = note: see issue #86942 <https://github.com/rust-lang/rust/issues/86942> for more information
  = note: see issue #86942 <https://github.com/rust-lang/rust/issues/86942> for more information
  = help: add `#![feature(more_fallible_allocation_methods)]` to the crate attributes to enable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/vec/mod.rs - vec::Vec<T, A>::try_into_boxed_slice (line 1127) stdout ----
error[E0658]: use of unstable library feature 'more_fallible_allocation_methods'
 --> src/vec/mod.rs:1132:17
  |
8 | let slice = vec.try_into_boxed_slice().unwrap();
  |
  = note: see issue #86942 <https://github.com/rust-lang/rust/issues/86942> for more information
  = note: see issue #86942 <https://github.com/rust-lang/rust/issues/86942> for more information
  = help: add `#![feature(more_fallible_allocation_methods)]` to the crate attributes to enable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/vec/mod.rs - vec::Vec<T, A>::try_extend_from_slice (line 2380) stdout ----
error[E0658]: use of unstable library feature 'more_fallible_allocation_methods'
 --> src/vec/mod.rs:2382:5
  |
5 | vec.try_extend_from_slice(&[2, 3, 4]).unwrap();
  |
  = note: see issue #86942 <https://github.com/rust-lang/rust/issues/86942> for more information
  = note: see issue #86942 <https://github.com/rust-lang/rust/issues/86942> for more information
  = help: add `#![feature(more_fallible_allocation_methods)]` to the crate attributes to enable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/vec/mod.rs - vec::Vec<T, A>::try_push (line 1816) stdout ----
error[E0658]: use of unstable library feature 'more_fallible_allocation_methods'
 --> src/vec/mod.rs:1818:5
  |
5 | vec.try_push(3).unwrap();
  |
  = note: see issue #86942 <https://github.com/rust-lang/rust/issues/86942> for more information
  = note: see issue #86942 <https://github.com/rust-lang/rust/issues/86942> for more information
  = help: add `#![feature(more_fallible_allocation_methods)]` to the crate attributes to enable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/vec/mod.rs - vec::Vec<T, A>::try_shrink_to_fit (line 1031) stdout ----
error[E0658]: use of unstable library feature 'more_fallible_allocation_methods'
 --> src/vec/mod.rs:1035:5
  |
7 | vec.try_shrink_to_fit().unwrap();
  |
  = note: see issue #86942 <https://github.com/rust-lang/rust/issues/86942> for more information
  = note: see issue #86942 <https://github.com/rust-lang/rust/issues/86942> for more information
  = help: add `#![feature(more_fallible_allocation_methods)]` to the crate attributes to enable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/vec/mod.rs - vec::Vec<T, A>::try_with_capacity_in (line 665) stdout ----
error[E0658]: use of unstable library feature 'more_fallible_allocation_methods'
 --> src/vec/mod.rs:670:15
  |
8 | let mut vec = Vec::try_with_capacity_in(10, System).unwrap();
error: test failed, to rerun pass '--doc'
  |
  = note: see issue #86942 <https://github.com/rust-lang/rust/issues/86942> for more information
  = note: see issue #86942 <https://github.com/rust-lang/rust/issues/86942> for more information
  = help: add `#![feature(more_fallible_allocation_methods)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'more_fallible_allocation_methods'
  --> src/vec/mod.rs:688:18
   |
   |
26 | let mut result = Vec::try_with_capacity_in(usize::MAX, System);
   |
   = note: see issue #86942 <https://github.com/rust-lang/rust/issues/86942> for more information
   = note: see issue #86942 <https://github.com/rust-lang/rust/issues/86942> for more information
   = help: add `#![feature(more_fallible_allocation_methods)]` to the crate attributes to enable
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/vec/mod.rs - vec::Vec<T, A>::try_resize (line 2318) stdout ----
error[E0658]: use of unstable library feature 'more_fallible_allocation_methods'
 --> src/vec/mod.rs:2320:5
  |
5 | vec.try_resize(3, "world").unwrap();
  |
  = note: see issue #86942 <https://github.com/rust-lang/rust/issues/86942> for more information
  = note: see issue #86942 <https://github.com/rust-lang/rust/issues/86942> for more information
  = help: add `#![feature(more_fallible_allocation_methods)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'more_fallible_allocation_methods'
 --> src/vec/mod.rs:2324:5
  |
  |
9 | vec.try_resize(2, 0).unwrap();
  |
  = note: see issue #86942 <https://github.com/rust-lang/rust/issues/86942> for more information
  = note: see issue #86942 <https://github.com/rust-lang/rust/issues/86942> for more information
  = help: add `#![feature(more_fallible_allocation_methods)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'more_fallible_allocation_methods'
  --> src/vec/mod.rs:2328:18
   |
   |
13 | let result = vec.try_resize(usize::MAX, 0);
   |
   = note: see issue #86942 <https://github.com/rust-lang/rust/issues/86942> for more information
   = note: see issue #86942 <https://github.com/rust-lang/rust/issues/86942> for more information
   = help: add `#![feature(more_fallible_allocation_methods)]` to the crate attributes to enable
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/vec/mod.rs - vec::Vec<T>::try_with_capacity (line 482) stdout ----
error[E0658]: use of unstable library feature 'more_fallible_allocation_methods'
 --> src/vec/mod.rs:483:15
  |
4 | let mut vec = Vec::try_with_capacity(10).unwrap();
  |
  = note: see issue #86942 <https://github.com/rust-lang/rust/issues/86942> for more information
  = note: see issue #86942 <https://github.com/rust-lang/rust/issues/86942> for more information
  = help: add `#![feature(more_fallible_allocation_methods)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'more_fallible_allocation_methods'
  --> src/vec/mod.rs:501:18
   |
   |
22 | let mut result = Vec::try_with_capacity(usize::MAX);
   |
   = note: see issue #86942 <https://github.com/rust-lang/rust/issues/86942> for more information
   = note: see issue #86942 <https://github.com/rust-lang/rust/issues/86942> for more information
   = help: add `#![feature(more_fallible_allocation_methods)]` to the crate attributes to enable
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
---
test result: FAILED. 581 passed; 12 failed; 1 ignored; 0 measured; 0 filtered out; finished in 13.72s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "alloc" "--" "--quiet"


Build completed unsuccessfully in 0:18:39
