plain
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii

Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
 finished in 0.064 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i.....ii.........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.01s

 finished in 2.075 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
..................i.....................i........................................................... 2800/2822
......................
failures:

---- src/iter/traits/iterator.rs - iter::traits::iterator::Iterator::intersperse_with (line 629) stdout ----
error[E0277]: `NotClone` doesn't implement `Debug`
   |
   |
12 | assert_eq!(it.next(), Some(NotClone(0)));  // The first element from `v`.
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `NotClone` cannot be formatted using `{:?}`
   |
   = help: the trait `Debug` is not implemented for `NotClone`
   = note: add `#[derive(Debug)]` or manually implement `Debug`
   = note: required because of the requirements on the impl of `Debug` for `Option<NotClone>`
   = note: 1 redundant requirements hidden
   = note: required because of the requirements on the impl of `Debug` for `&Option<NotClone>`
   = note: required by `std::fmt::Debug::fmt`
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: `NotClone` doesn't implement `Debug`
   |
   |
12 | assert_eq!(it.next(), Some(NotClone(0)));  // The first element from `v`.
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `NotClone` cannot be formatted using `{:?}`
   |
   = help: the trait `Debug` is not implemented for `NotClone`
   = note: add `#[derive(Debug)]` or manually implement `Debug`
   = note: required because of the requirements on the impl of `Debug` for `Option<NotClone>`
   = note: 1 redundant requirements hidden
   = note: required because of the requirements on the impl of `Debug` for `&Option<NotClone>`
   = note: required by `std::fmt::Debug::fmt`
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: `NotClone` doesn't implement `Debug`
   |
   |
13 | assert_eq!(it.next(), Some(NotClone(99))); // The separator.
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `NotClone` cannot be formatted using `{:?}`
   |
   = help: the trait `Debug` is not implemented for `NotClone`
   = note: add `#[derive(Debug)]` or manually implement `Debug`
   = note: required because of the requirements on the impl of `Debug` for `Option<NotClone>`
   = note: 1 redundant requirements hidden
   = note: required because of the requirements on the impl of `Debug` for `&Option<NotClone>`
   = note: required by `std::fmt::Debug::fmt`
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: `NotClone` doesn't implement `Debug`
   |
   |
13 | assert_eq!(it.next(), Some(NotClone(99))); // The separator.
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `NotClone` cannot be formatted using `{:?}`
   |
   = help: the trait `Debug` is not implemented for `NotClone`
   = note: add `#[derive(Debug)]` or manually implement `Debug`
   = note: required because of the requirements on the impl of `Debug` for `Option<NotClone>`
   = note: 1 redundant requirements hidden
   = note: required because of the requirements on the impl of `Debug` for `&Option<NotClone>`
   = note: required by `std::fmt::Debug::fmt`
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: `NotClone` doesn't implement `Debug`
   |
   |
14 | assert_eq!(it.next(), Some(NotClone(1)));  // The next element from `v`.
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `NotClone` cannot be formatted using `{:?}`
   |
   = help: the trait `Debug` is not implemented for `NotClone`
   = note: add `#[derive(Debug)]` or manually implement `Debug`
   = note: required because of the requirements on the impl of `Debug` for `Option<NotClone>`
   = note: 1 redundant requirements hidden
   = note: required because of the requirements on the impl of `Debug` for `&Option<NotClone>`
   = note: required by `std::fmt::Debug::fmt`
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: `NotClone` doesn't implement `Debug`
   |
   |
14 | assert_eq!(it.next(), Some(NotClone(1)));  // The next element from `v`.
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `NotClone` cannot be formatted using `{:?}`
   |
   = help: the trait `Debug` is not implemented for `NotClone`
   = note: add `#[derive(Debug)]` or manually implement `Debug`
   = note: required because of the requirements on the impl of `Debug` for `Option<NotClone>`
   = note: 1 redundant requirements hidden
   = note: required because of the requirements on the impl of `Debug` for `&Option<NotClone>`
   = note: required by `std::fmt::Debug::fmt`
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: `NotClone` doesn't implement `Debug`
   |
   |
15 | assert_eq!(it.next(), Some(NotClone(99))); // The separator.
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `NotClone` cannot be formatted using `{:?}`
   |
   = help: the trait `Debug` is not implemented for `NotClone`
   = note: add `#[derive(Debug)]` or manually implement `Debug`
   = note: required because of the requirements on the impl of `Debug` for `Option<NotClone>`
   = note: 1 redundant requirements hidden
   = note: required because of the requirements on the impl of `Debug` for `&Option<NotClone>`
   = note: required by `std::fmt::Debug::fmt`
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: `NotClone` doesn't implement `Debug`
   |
   |
15 | assert_eq!(it.next(), Some(NotClone(99))); // The separator.
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `NotClone` cannot be formatted using `{:?}`
   |
   = help: the trait `Debug` is not implemented for `NotClone`
   = note: add `#[derive(Debug)]` or manually implement `Debug`
   = note: required because of the requirements on the impl of `Debug` for `Option<NotClone>`
   = note: 1 redundant requirements hidden
   = note: required because of the requirements on the impl of `Debug` for `&Option<NotClone>`
   = note: required by `std::fmt::Debug::fmt`
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: `NotClone` doesn't implement `Debug`
   |
   |
16 | assert_eq!(it.next(), Some(NotClone(2)));  // The last element from from `v`.
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `NotClone` cannot be formatted using `{:?}`
   |
   = help: the trait `Debug` is not implemented for `NotClone`
   = note: add `#[derive(Debug)]` or manually implement `Debug`
   = note: required because of the requirements on the impl of `Debug` for `Option<NotClone>`
   = note: 1 redundant requirements hidden
   = note: required because of the requirements on the impl of `Debug` for `&Option<NotClone>`
   = note: required by `std::fmt::Debug::fmt`
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: `NotClone` doesn't implement `Debug`
   |
   |
16 | assert_eq!(it.next(), Some(NotClone(2)));  // The last element from from `v`.
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `NotClone` cannot be formatted using `{:?}`
   |
   = help: the trait `Debug` is not implemented for `NotClone`
   = note: add `#[derive(Debug)]` or manually implement `Debug`
   = note: required because of the requirements on the impl of `Debug` for `Option<NotClone>`
   = note: 1 redundant requirements hidden
   = note: required because of the requirements on the impl of `Debug` for `&Option<NotClone>`
   = note: required by `std::fmt::Debug::fmt`
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: `NotClone` doesn't implement `Debug`
   |
   |
17 | assert_eq!(it.next(), None);               // The iterator is finished.
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `NotClone` cannot be formatted using `{:?}`
   |
   = help: the trait `Debug` is not implemented for `NotClone`
   = note: add `#[derive(Debug)]` or manually implement `Debug`
   = note: required because of the requirements on the impl of `Debug` for `Option<NotClone>`
   = note: 1 redundant requirements hidden
   = note: required because of the requirements on the impl of `Debug` for `&Option<NotClone>`
   = note: required by `std::fmt::Debug::fmt`
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: `NotClone` doesn't implement `Debug`
   |
   |
17 | assert_eq!(it.next(), None);               // The iterator is finished.
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `NotClone` cannot be formatted using `{:?}`
   |
   = help: the trait `Debug` is not implemented for `NotClone`
   = note: add `#[derive(Debug)]` or manually implement `Debug`
   = note: required because of the requirements on the impl of `Debug` for `Option<NotClone>`
   = note: 1 redundant requirements hidden
   = note: required because of the requirements on the impl of `Debug` for `&Option<NotClone>`
   = note: required by `std::fmt::Debug::fmt`
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 12 previous errors

For more information about this error, try `rustc --explain E0277`.
Couldn't compile the test.
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:19:10
