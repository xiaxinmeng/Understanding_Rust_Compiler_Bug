plain
running 618 tests
.................................................................................................... 100/618
.................................................................................................... 200/618
..........................................................i......................................... 300/618
....................................................FF..................F........................... 400/618
.................................................................................................... 500/618
.F..................................................................................FF.F...FF.FF..F. 600/618
failures:

---- src/slice.rs - slice::[T]::try_to_vec (line 536) stdout ----
error[E0658]: use of unstable library feature 'more_fallible_allocation_methods'
error[E0658]: use of unstable library feature 'more_fallible_allocation_methods'
 --> src/slice.rs:538:11
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
---- src/slice.rs - slice::[T]::try_to_vec_in (line 578) stdout ----
error[E0658]: use of unstable library feature 'more_fallible_allocation_methods'
 --> src/slice.rs:584:11
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
---- src/str.rs - str::str::try_to_owned (line 601) stdout ----
error[E0658]: use of unstable library feature 'more_fallible_allocation_methods'
 --> src/str.rs:603:20
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
---- src/sync.rs - sync::Arc<[T]>::try_from_vec (line 2517) stdout ----
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
---- src/vec/mod.rs - vec::Vec<T,A>::try_into_boxed_slice (line 1126) stdout ----
error[E0658]: use of unstable library feature 'more_fallible_allocation_methods'
 --> src/vec/mod.rs:1129:15
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
---- src/vec/mod.rs - vec::Vec<T,A>::try_into_boxed_slice (line 1134) stdout ----
error[E0658]: use of unstable library feature 'more_fallible_allocation_methods'
 --> src/vec/mod.rs:1139:17
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
---- src/vec/mod.rs - vec::Vec<T,A>::try_extend_from_slice (line 2451) stdout ----
error[E0658]: use of unstable library feature 'more_fallible_allocation_methods'
 --> src/vec/mod.rs:2453:5
error: test failed, to rerun pass '--doc'
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
---- src/vec/mod.rs - vec::Vec<T,A>::try_push (line 1884) stdout ----
error[E0658]: use of unstable library feature 'more_fallible_allocation_methods'
 --> src/vec/mod.rs:1886:5
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
---- src/vec/mod.rs - vec::Vec<T,A>::try_shrink_to_fit (line 1039) stdout ----
error[E0658]: use of unstable library feature 'more_fallible_allocation_methods'
 --> src/vec/mod.rs:1043:5
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
---- src/vec/mod.rs - vec::Vec<T,A>::try_with_capacity_in (line 671) stdout ----
error[E0658]: use of unstable library feature 'more_fallible_allocation_methods'
 --> src/vec/mod.rs:676:15
  |
8 | let mut vec = Vec::try_with_capacity_in(10, System).unwrap();
  |
  = note: see issue #86942 <https://github.com/rust-lang/rust/issues/86942> for more information
  = note: see issue #86942 <https://github.com/rust-lang/rust/issues/86942> for more information
  = help: add `#![feature(more_fallible_allocation_methods)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'more_fallible_allocation_methods'
  --> src/vec/mod.rs:694:18
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
---- src/vec/mod.rs - vec::Vec<T,A>::try_resize (line 2389) stdout ----
error[E0658]: use of unstable library feature 'more_fallible_allocation_methods'
 --> src/vec/mod.rs:2391:5
  |
5 | vec.try_resize(3, "world").unwrap();
  |
  = note: see issue #86942 <https://github.com/rust-lang/rust/issues/86942> for more information
  = note: see issue #86942 <https://github.com/rust-lang/rust/issues/86942> for more information
  = help: add `#![feature(more_fallible_allocation_methods)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'more_fallible_allocation_methods'
 --> src/vec/mod.rs:2395:5
  |
  |
9 | vec.try_resize(2, 0).unwrap();
  |
  = note: see issue #86942 <https://github.com/rust-lang/rust/issues/86942> for more information
  = note: see issue #86942 <https://github.com/rust-lang/rust/issues/86942> for more information
  = help: add `#![feature(more_fallible_allocation_methods)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'more_fallible_allocation_methods'
  --> src/vec/mod.rs:2399:18
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
---- src/vec/mod.rs - vec::Vec<T>::try_with_capacity (line 488) stdout ----
error[E0658]: use of unstable library feature 'more_fallible_allocation_methods'
 --> src/vec/mod.rs:489:15
  |
4 | let mut vec = Vec::try_with_capacity(10).unwrap();
  |
  = note: see issue #86942 <https://github.com/rust-lang/rust/issues/86942> for more information
  = note: see issue #86942 <https://github.com/rust-lang/rust/issues/86942> for more information
  = help: add `#![feature(more_fallible_allocation_methods)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'more_fallible_allocation_methods'
  --> src/vec/mod.rs:507:18
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
test result: FAILED. 605 passed; 12 failed; 1 ignored; 0 measured; 0 filtered out; finished in 15.58s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "alloc" "--" "--quiet"


Build completed unsuccessfully in 0:19:01
