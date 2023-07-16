plain
.................................................................................................... 9300/11690
.................................................................................................... 9400/11690
.................................................................................................... 9500/11690
.................................i......i........................................................... 9600/11690
...............................................................................iiiiiii..iiiiii.i.... 9700/11690
.................................................................................................... 9900/11690
.................................................................................................... 10000/11690
.................................................................................................... 10100/11690
.................................................................................................... 10200/11690
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 31 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii..

 finished in 0.090 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.34s

 finished in 2.393 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---

   Doc-tests core

running 2847 tests
iiiiii..................F....F...................................................................... 100/2847
....ii.............................................................................................. 300/2847
.....................................................i.............................................. 400/2847
.................................................................................................... 500/2847
..........................i..i...................iiii............................................... 600/2847
---
.................................................................................................... 2000/2847
.................................................................................................... 2100/2847
.................................................................................................... 2200/2847
.................................................................................................... 2300/2847
.....................................................................................F...F.......... 2400/2847
.......................................i................i.....................i..................... 2600/2847
i.....................i.....................i................................i.....................i 2700/2847
.....................i.....................i.....................i.................................. 2800/2847
...............................................
...............................................
failures:

---- src/array/mod.rs - array::[T; N]::split_array (line 546) stdout ----
error[E0658]: use of unstable library feature 'array_split_array': not intended for stabilization
  |
  |
7 |    let (left, right) = v.split_array::<0>();
  |
  = note: see issue #74674 <https://github.com/rust-lang/rust/issues/74674> for more information
  = help: add `#![feature(array_split_array)]` to the crate attributes to enable


error[E0277]: can't compare `&[{integer}; 0]` with `[_; 0]`
  |
  |
8 |    assert_eq!(left, []);
  |    ^^^^^^^^^^^^^^^^^^^^^ no implementation for `&[{integer}; 0] == [_; 0]`
  |
  = help: the trait `PartialEq<[_; 0]>` is not implemented for `&[{integer}; 0]`


error[E0107]: this associated function takes 0 const arguments but 1 const argument was supplied
     |
     |
13   |     let (left, right) = v.split_at::<2>();
     |                           ^^^^^^^^----- help: remove these generics
     |                           expected 0 const arguments
     |
     |
note: associated function defined here, with 0 const parameters
     |
     |
1504 |     pub fn split_at(&self, mid: usize) -> (&[T], &[T]) {

error[E0061]: this function takes 1 argument but 0 arguments were supplied
    --> src/array/mod.rs:556:27
     |
     |
13   |     let (left, right) = v.split_at::<2>();
     |                           |
     |                           expected 1 argument
     |
note: associated function defined here
note: associated function defined here
    --> /checkout/library/core/src/slice/mod.rs:1504:12
     |
1504 |     pub fn split_at(&self, mid: usize) -> (&[T], &[T]) {


error[E0107]: this associated function takes 0 const arguments but 1 const argument was supplied
     |
     |
19   |     let (left, right) = v.split_at::<6>();
     |                           ^^^^^^^^----- help: remove these generics
     |                           expected 0 const arguments
     |
     |
note: associated function defined here, with 0 const parameters
     |
     |
1504 |     pub fn split_at(&self, mid: usize) -> (&[T], &[T]) {

error[E0061]: this function takes 1 argument but 0 arguments were supplied
    --> src/array/mod.rs:562:27
     |
     |
19   |     let (left, right) = v.split_at::<6>();
     |                           |
     |                           expected 1 argument
     |
note: associated function defined here
note: associated function defined here
    --> /checkout/library/core/src/slice/mod.rs:1504:12
     |
1504 |     pub fn split_at(&self, mid: usize) -> (&[T], &[T]) {

error: aborting due to 6 previous errors

Some errors have detailed explanations: E0061, E0107, E0277, E0658.
Some errors have detailed explanations: E0061, E0107, E0277, E0658.
For more information about an error, try `rustc --explain E0061`.
Couldn't compile the test.
---- src/array/mod.rs - array::[T; N]::split_array_mut (line 591) stdout ----
error[E0658]: use of unstable library feature 'slice_split_array': new API
  |
  |
5 | let (left, right) = v.split_array_mut::<2>();
  |
  = note: see issue #74674 <https://github.com/rust-lang/rust/issues/74674> for more information
  = note: see issue #74674 <https://github.com/rust-lang/rust/issues/74674> for more information
  = help: add `#![feature(slice_split_array)]` to the crate attributes to enable

error[E0277]: can't compare `&mut [{integer}; 2]` with `[{integer}; 2]`
  |
  |
6 | assert_eq!(left, [1, 0]);
  | ^^^^^^^^^^^^^^^^^^^^^^^^^ no implementation for `&mut [{integer}; 2] == [{integer}; 2]`
  |
  = help: the trait `PartialEq<[{integer}; 2]>` is not implemented for `&mut [{integer}; 2]`

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0277, E0658.
Some errors have detailed explanations: E0277, E0658.
For more information about an error, try `rustc --explain E0277`.
Couldn't compile the test.
---- src/slice/mod.rs - slice::[T]::split_array (line 1648) stdout ----
error[E0658]: use of unstable library feature 'slice_split_array': new API
  |
  |
7 |    let (left, right) = v.split_array::<0>();
  |
  = note: see issue #74674 <https://github.com/rust-lang/rust/issues/74674> for more information
  = note: see issue #74674 <https://github.com/rust-lang/rust/issues/74674> for more information
  = help: add `#![feature(slice_split_array)]` to the crate attributes to enable

error[E0277]: can't compare `&[{integer}; 0]` with `[_; 0]`
  |
  |
8 |    assert_eq!(left, []);
  |    ^^^^^^^^^^^^^^^^^^^^^ no implementation for `&[{integer}; 0] == [_; 0]`
  |
  = help: the trait `PartialEq<[_; 0]>` is not implemented for `&[{integer}; 0]`


error[E0107]: this associated function takes 0 const arguments but 1 const argument was supplied
     |
     |
13   |     let (left, right) = v.split_at::<2>();
     |                           ^^^^^^^^----- help: remove these generics
     |                           expected 0 const arguments
     |
     |
note: associated function defined here, with 0 const parameters
     |
     |
1504 |     pub fn split_at(&self, mid: usize) -> (&[T], &[T]) {

error[E0061]: this function takes 1 argument but 0 arguments were supplied
    --> src/slice/mod.rs:1658:27
     |
     |
13   |     let (left, right) = v.split_at::<2>();
     |                           |
     |                           expected 1 argument
     |
note: associated function defined here
note: associated function defined here
    --> /checkout/library/core/src/slice/mod.rs:1504:12
     |
1504 |     pub fn split_at(&self, mid: usize) -> (&[T], &[T]) {


error[E0107]: this associated function takes 0 const arguments but 1 const argument was supplied
     |
     |
19   |     let (left, right) = v.split_at::<6>();
     |                           ^^^^^^^^----- help: remove these generics
     |                           expected 0 const arguments
     |
     |
note: associated function defined here, with 0 const parameters
     |
     |
1504 |     pub fn split_at(&self, mid: usize) -> (&[T], &[T]) {

error[E0061]: this function takes 1 argument but 0 arguments were supplied
    --> src/slice/mod.rs:1664:27
     |
     |
19   |     let (left, right) = v.split_at::<6>();
     |                           |
     |                           expected 1 argument
     |
note: associated function defined here
note: associated function defined here
    --> /checkout/library/core/src/slice/mod.rs:1504:12
     |
1504 |     pub fn split_at(&self, mid: usize) -> (&[T], &[T]) {

error: aborting due to 6 previous errors

Some errors have detailed explanations: E0061, E0107, E0277, E0658.
Some errors have detailed explanations: E0061, E0107, E0277, E0658.
For more information about an error, try `rustc --explain E0061`.
Couldn't compile the test.
---- src/slice/mod.rs - slice::[T]::split_array_mut (line 1689) stdout ----
error[E0658]: use of unstable library feature 'slice_split_array': new API
  |
  |
5 | let (left, right) = v.split_array_mut::<2>();
  |
  = note: see issue #74674 <https://github.com/rust-lang/rust/issues/74674> for more information
  = note: see issue #74674 <https://github.com/rust-lang/rust/issues/74674> for more information
  = help: add `#![feature(slice_split_array)]` to the crate attributes to enable

error[E0277]: can't compare `&mut [{integer}; 2]` with `[{integer}; 2]`
  |
  |
6 | assert_eq!(left, [1, 0]);
  | ^^^^^^^^^^^^^^^^^^^^^^^^^ no implementation for `&mut [{integer}; 2] == [{integer}; 2]`
  |
  = help: the trait `PartialEq<[{integer}; 2]>` is not implemented for `&mut [{integer}; 2]`

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0277, E0658.
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:19:55
