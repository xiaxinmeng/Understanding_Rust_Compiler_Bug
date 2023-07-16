plain
.................................................................................................... 2800/3739
................................................................................................F... 2900/3739
.......................................................iii.......................................... 3000/3739
.................................................................................................... 3100/3739
.........F.F........................................................................................ 3200/3739
......................................................................i............................. 3400/3739
....................i................i.....................i.....................i.................. 3500/3739
...i.....................i................................i.....................i................... 3600/3739
..i.....................i.....................i..................................................... 3700/3739
..i.....................i.....................i..................................................... 3700/3739
.......................................
failures:

---- src/option.rs - option::Option<T>::is_some_with (line 558) stdout ----
error[E0658]: use of unstable library feature 'is_some_with'
 --> src/option.rs:560:14
  |
5 | assert_eq!(x.is_some_with(|x| x > 1), true);
  |
  = note: see issue #93050 <https://github.com/rust-lang/rust/issues/93050> for more information
  = note: see issue #93050 <https://github.com/rust-lang/rust/issues/93050> for more information
  = help: add `#![feature(is_some_with)]` to the crate attributes to enable
error[E0308]: mismatched types
 --> src/option.rs:560:35
  |
  |
5 | assert_eq!(x.is_some_with(|x| x > 1), true);
  |                                   |
  |                                   |
  |                                   expected `&u32`, found integer
  |                                   help: consider borrowing here: `&1`
error[E0658]: use of unstable library feature 'is_some_with'
 --> src/option.rs:563:14
  |
  |
8 | assert_eq!(x.is_some_with(|x| x > 1), false);
  |
  = note: see issue #93050 <https://github.com/rust-lang/rust/issues/93050> for more information
  = note: see issue #93050 <https://github.com/rust-lang/rust/issues/93050> for more information
  = help: add `#![feature(is_some_with)]` to the crate attributes to enable
error[E0308]: mismatched types
 --> src/option.rs:563:35
  |
  |
8 | assert_eq!(x.is_some_with(|x| x > 1), false);
  |                                   |
  |                                   |
  |                                   expected `&u32`, found integer
  |                                   help: consider borrowing here: `&1`
error[E0658]: use of unstable library feature 'is_some_with'
  --> src/option.rs:566:14
   |
   |
11 | assert_eq!(x.is_some_with(|x| x > 1), false);
   |
   = note: see issue #93050 <https://github.com/rust-lang/rust/issues/93050> for more information
   = note: see issue #93050 <https://github.com/rust-lang/rust/issues/93050> for more information
   = help: add `#![feature(is_some_with)]` to the crate attributes to enable
error[E0308]: mismatched types
  --> src/option.rs:566:35
   |
   |
11 | assert_eq!(x.is_some_with(|x| x > 1), false);
   |                                   |
   |                                   |
   |                                   expected `&u32`, found integer
   |                                   help: consider borrowing here: `&1`
error: aborting due to 6 previous errors

Some errors have detailed explanations: E0308, E0658.
For more information about an error, try `rustc --explain E0308`.
For more information about an error, try `rustc --explain E0308`.
Couldn't compile the test.
---- src/result.rs - result::Result<T,E>::is_ok_with (line 549) stdout ----
error[E0658]: use of unstable library feature 'is_some_with'
 --> src/result.rs:551:14
  |
5 | assert_eq!(x.is_ok_with(|x| x > 1), true);
  |
  = note: see issue #93050 <https://github.com/rust-lang/rust/issues/93050> for more information
  = note: see issue #93050 <https://github.com/rust-lang/rust/issues/93050> for more information
  = help: add `#![feature(is_some_with)]` to the crate attributes to enable
error[E0308]: mismatched types
 --> src/result.rs:551:33
  |
  |
5 | assert_eq!(x.is_ok_with(|x| x > 1), true);
  |                                 |
  |                                 |
  |                                 expected `&u32`, found integer
  |                                 help: consider borrowing here: `&1`
error[E0658]: use of unstable library feature 'is_some_with'
 --> src/result.rs:554:14
  |
  |
8 | assert_eq!(x.is_ok_with(|x| x > 1), false);
  |
  = note: see issue #93050 <https://github.com/rust-lang/rust/issues/93050> for more information
  = note: see issue #93050 <https://github.com/rust-lang/rust/issues/93050> for more information
  = help: add `#![feature(is_some_with)]` to the crate attributes to enable
error[E0308]: mismatched types
 --> src/result.rs:554:33
  |
  |
8 | assert_eq!(x.is_ok_with(|x| x > 1), false);
  |                                 |
  |                                 |
  |                                 expected `&u32`, found integer
  |                                 help: consider borrowing here: `&1`
error[E0658]: use of unstable library feature 'is_some_with'
  --> src/result.rs:557:14
   |
   |
11 | assert_eq!(x.is_ok_with(|x| x > 1), false);
   |
   = note: see issue #93050 <https://github.com/rust-lang/rust/issues/93050> for more information
   = note: see issue #93050 <https://github.com/rust-lang/rust/issues/93050> for more information
   = help: add `#![feature(is_some_with)]` to the crate attributes to enable
error[E0308]: mismatched types
  --> src/result.rs:557:33
   |
   |
11 | assert_eq!(x.is_ok_with(|x| x > 1), false);
   |                                 |
   |                                 |
   |                                 expected `&u32`, found integer
   |                                 help: consider borrowing here: `&1`
error: aborting due to 6 previous errors

Some errors have detailed explanations: E0308, E0658.
For more information about an error, try `rustc --explain E0308`.
For more information about an error, try `rustc --explain E0308`.
Couldn't compile the test.
---- src/result.rs - result::Result<T,E>::is_err_with (line 591) stdout ----
error[E0658]: use of unstable library feature 'is_some_with'
 --> src/result.rs:595:14
  |
7 | assert_eq!(x.is_err_with(|x| x.kind() == ErrorKind::NotFound), true);
  |
  = note: see issue #93050 <https://github.com/rust-lang/rust/issues/93050> for more information
  = note: see issue #93050 <https://github.com/rust-lang/rust/issues/93050> for more information
  = help: add `#![feature(is_some_with)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'is_some_with'
  --> src/result.rs:598:14
   |
   |
10 | assert_eq!(x.is_ok_with(|x| x.kind() == ErrorKind::NotFound), false);
   |
   = note: see issue #93050 <https://github.com/rust-lang/rust/issues/93050> for more information
   = note: see issue #93050 <https://github.com/rust-lang/rust/issues/93050> for more information
   = help: add `#![feature(is_some_with)]` to the crate attributes to enable

error[E0599]: no method named `kind` found for reference `&u32` in the current scope
   |
   |
10 | assert_eq!(x.is_ok_with(|x| x.kind() == ErrorKind::NotFound), false);
   |                               ^^^^ method not found in `&u32`
error[E0658]: use of unstable library feature 'is_some_with'
  --> src/result.rs:601:14
   |
   |
13 | assert_eq!(x.is_ok_with(|x| x.kind() == ErrorKind::NotFound), false);
   |
   = note: see issue #93050 <https://github.com/rust-lang/rust/issues/93050> for more information
   = note: see issue #93050 <https://github.com/rust-lang/rust/issues/93050> for more information
   = help: add `#![feature(is_some_with)]` to the crate attributes to enable

error[E0599]: no method named `kind` found for reference `&u32` in the current scope
   |
   |
13 | assert_eq!(x.is_ok_with(|x| x.kind() == ErrorKind::NotFound), false);
   |                               ^^^^ method not found in `&u32`
error: aborting due to 5 previous errors

Some errors have detailed explanations: E0599, E0658.
For more information about an error, try `rustc --explain E0599`.
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


Build completed unsuccessfully in 0:23:01
