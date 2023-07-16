plain
.................................................................................................... 1800/3719
.................................................................................................... 1900/3719
.................................................................................................... 2000/3719
.................................................................................................... 2100/3719
..............................................................F..F........................FF........ 2200/3719
...............FF......................FF......................F.....F...................FF......... 2300/3719
..............FF......................F.F..................FF.....................FF................ 2400/3719
...F...F.................FF......................................................................... 2500/3719
.................................................................................................... 2700/3719
.................................................................................................... 2800/3719
.................................................................................................... 2900/3719
.....................................iii............................................................ 3000/3719
---
---- src/num/saturating.rs - num::saturating::Saturating<i128> (line 601) stdout ----
error[E0658]: use of unstable library feature 'saturating_int_impl'
 --> src/num/saturating.rs:605:12
  |
7 | assert_eq!(Saturating(2i128), Saturating(5i128) / 2);
  |
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = help: add `#![feature(saturating_int_impl)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'saturating_int_impl'
 --> src/num/saturating.rs:605:31
  |
  |
7 | assert_eq!(Saturating(2i128), Saturating(5i128) / 2);
  |
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = help: add `#![feature(saturating_int_impl)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'saturating_int_impl'
 --> src/num/saturating.rs:606:12
  |
  |
8 | assert_eq!(Saturating(i128::MAX), Saturating(i128::MAX) / 1);
  |
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = help: add `#![feature(saturating_int_impl)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'saturating_int_impl'
 --> src/num/saturating.rs:606:35
  |
  |
8 | assert_eq!(Saturating(i128::MAX), Saturating(i128::MAX) / 1);
  |
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = help: add `#![feature(saturating_int_impl)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'saturating_int_impl'
 --> src/num/saturating.rs:607:12
  |
  |
9 | assert_eq!(Saturating(i128::MIN), Saturating(i128::MIN) / 1);
  |
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = help: add `#![feature(saturating_int_impl)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'saturating_int_impl'
 --> src/num/saturating.rs:607:35
  |
  |
9 | assert_eq!(Saturating(i128::MIN), Saturating(i128::MIN) / 1);
  |
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = help: add `#![feature(saturating_int_impl)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'saturating_int_impl'
 --> src/num/saturating.rs:603:5
  |
5 | use std::num::Saturating;
5 | use std::num::Saturating;
  |     ^^^^^^^^^^^^^^^^^^^^
  |
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = help: add `#![feature(saturating_int_impl)]` to the crate attributes to enable
error: aborting due to 7 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/saturating.rs - num::saturating::Saturating<i128> (line 610) stdout ----
error[E0658]: use of unstable library feature 'saturating_int_impl'
 --> src/num/saturating.rs:614:9
  |
7 | let _ = Saturating(0i128) / 0;
  |
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = help: add `#![feature(saturating_int_impl)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'saturating_int_impl'
 --> src/num/saturating.rs:612:5
  |
5 | use std::num::Saturating;
5 | use std::num::Saturating;
  |     ^^^^^^^^^^^^^^^^^^^^
  |
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = help: add `#![feature(saturating_int_impl)]` to the crate attributes to enable
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/saturating.rs - num::saturating::Saturating<i16> (line 601) stdout ----
error[E0658]: use of unstable library feature 'saturating_int_impl'
 --> src/num/saturating.rs:605:12
  |
7 | assert_eq!(Saturating(2i16), Saturating(5i16) / 2);
  |
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = help: add `#![feature(saturating_int_impl)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'saturating_int_impl'
 --> src/num/saturating.rs:605:30
  |
  |
7 | assert_eq!(Saturating(2i16), Saturating(5i16) / 2);
  |
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = help: add `#![feature(saturating_int_impl)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'saturating_int_impl'
 --> src/num/saturating.rs:606:12
  |
  |
8 | assert_eq!(Saturating(i16::MAX), Saturating(i16::MAX) / 1);
  |
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = help: add `#![feature(saturating_int_impl)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'saturating_int_impl'
 --> src/num/saturating.rs:606:34
  |
  |
8 | assert_eq!(Saturating(i16::MAX), Saturating(i16::MAX) / 1);
  |
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = help: add `#![feature(saturating_int_impl)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'saturating_int_impl'
 --> src/num/saturating.rs:607:12
  |
  |
9 | assert_eq!(Saturating(i16::MIN), Saturating(i16::MIN) / 1);
  |
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = help: add `#![feature(saturating_int_impl)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'saturating_int_impl'
 --> src/num/saturating.rs:607:34
  |
  |
9 | assert_eq!(Saturating(i16::MIN), Saturating(i16::MIN) / 1);
  |
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = help: add `#![feature(saturating_int_impl)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'saturating_int_impl'
 --> src/num/saturating.rs:603:5
  |
5 | use std::num::Saturating;
5 | use std::num::Saturating;
  |     ^^^^^^^^^^^^^^^^^^^^
  |
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = help: add `#![feature(saturating_int_impl)]` to the crate attributes to enable
error: aborting due to 7 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/saturating.rs - num::saturating::Saturating<i16> (line 610) stdout ----
error[E0658]: use of unstable library feature 'saturating_int_impl'
 --> src/num/saturating.rs:614:9
  |
7 | let _ = Saturating(0i16) / 0;
  |
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = help: add `#![feature(saturating_int_impl)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'saturating_int_impl'
 --> src/num/saturating.rs:612:5
  |
5 | use std::num::Saturating;
5 | use std::num::Saturating;
  |     ^^^^^^^^^^^^^^^^^^^^
  |
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = help: add `#![feature(saturating_int_impl)]` to the crate attributes to enable
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/saturating.rs - num::saturating::Saturating<i32> (line 601) stdout ----
error[E0658]: use of unstable library feature 'saturating_int_impl'
 --> src/num/saturating.rs:605:12
  |
7 | assert_eq!(Saturating(2i32), Saturating(5i32) / 2);
  |
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = help: add `#![feature(saturating_int_impl)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'saturating_int_impl'
 --> src/num/saturating.rs:605:30
  |
  |
7 | assert_eq!(Saturating(2i32), Saturating(5i32) / 2);
  |
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = help: add `#![feature(saturating_int_impl)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'saturating_int_impl'
 --> src/num/saturating.rs:606:12
  |
  |
8 | assert_eq!(Saturating(i32::MAX), Saturating(i32::MAX) / 1);
  |
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = help: add `#![feature(saturating_int_impl)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'saturating_int_impl'
 --> src/num/saturating.rs:606:34
  |
  |
8 | assert_eq!(Saturating(i32::MAX), Saturating(i32::MAX) / 1);
  |
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = help: add `#![feature(saturating_int_impl)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'saturating_int_impl'
 --> src/num/saturating.rs:607:12
  |
  |
9 | assert_eq!(Saturating(i32::MIN), Saturating(i32::MIN) / 1);
  |
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = help: add `#![feature(saturating_int_impl)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'saturating_int_impl'
 --> src/num/saturating.rs:607:34
  |
  |
9 | assert_eq!(Saturating(i32::MIN), Saturating(i32::MIN) / 1);
  |
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = help: add `#![feature(saturating_int_impl)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'saturating_int_impl'
 --> src/num/saturating.rs:603:5
  |
5 | use std::num::Saturating;
5 | use std::num::Saturating;
  |     ^^^^^^^^^^^^^^^^^^^^
  |
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = help: add `#![feature(saturating_int_impl)]` to the crate attributes to enable
error: aborting due to 7 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/saturating.rs - num::saturating::Saturating<i32> (line 610) stdout ----
error[E0658]: use of unstable library feature 'saturating_int_impl'
 --> src/num/saturating.rs:614:9
  |
7 | let _ = Saturating(0i32) / 0;
  |
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = help: add `#![feature(saturating_int_impl)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'saturating_int_impl'
 --> src/num/saturating.rs:612:5
  |
5 | use std::num::Saturating;
5 | use std::num::Saturating;
  |     ^^^^^^^^^^^^^^^^^^^^
  |
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = help: add `#![feature(saturating_int_impl)]` to the crate attributes to enable
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/saturating.rs - num::saturating::Saturating<i64> (line 601) stdout ----
error[E0658]: use of unstable library feature 'saturating_int_impl'
 --> src/num/saturating.rs:605:12
  |
7 | assert_eq!(Saturating(2i64), Saturating(5i64) / 2);
  |
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = help: add `#![feature(saturating_int_impl)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'saturating_int_impl'
 --> src/num/saturating.rs:605:30
  |
  |
7 | assert_eq!(Saturating(2i64), Saturating(5i64) / 2);
  |
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = help: add `#![feature(saturating_int_impl)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'saturating_int_impl'
 --> src/num/saturating.rs:606:12
  |
  |
8 | assert_eq!(Saturating(i64::MAX), Saturating(i64::MAX) / 1);
  |
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = help: add `#![feature(saturating_int_impl)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'saturating_int_impl'
 --> src/num/saturating.rs:606:34
  |
  |
8 | assert_eq!(Saturating(i64::MAX), Saturating(i64::MAX) / 1);
  |
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = help: add `#![feature(saturating_int_impl)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'saturating_int_impl'
 --> src/num/saturating.rs:607:12
  |
  |
9 | assert_eq!(Saturating(i64::MIN), Saturating(i64::MIN) / 1);
  |
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = help: add `#![feature(saturating_int_impl)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'saturating_int_impl'
 --> src/num/saturating.rs:607:34
  |
  |
9 | assert_eq!(Saturating(i64::MIN), Saturating(i64::MIN) / 1);
  |
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = help: add `#![feature(saturating_int_impl)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'saturating_int_impl'
 --> src/num/saturating.rs:603:5
  |
5 | use std::num::Saturating;
5 | use std::num::Saturating;
  |     ^^^^^^^^^^^^^^^^^^^^
  |
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = help: add `#![feature(saturating_int_impl)]` to the crate attributes to enable
error: aborting due to 7 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/saturating.rs - num::saturating::Saturating<i64> (line 610) stdout ----
error[E0658]: use of unstable library feature 'saturating_int_impl'
 --> src/num/saturating.rs:614:9
  |
7 | let _ = Saturating(0i64) / 0;
  |
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = help: add `#![feature(saturating_int_impl)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'saturating_int_impl'
 --> src/num/saturating.rs:612:5
  |
5 | use std::num::Saturating;
5 | use std::num::Saturating;
  |     ^^^^^^^^^^^^^^^^^^^^
  |
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = help: add `#![feature(saturating_int_impl)]` to the crate attributes to enable
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/saturating.rs - num::saturating::Saturating<i8> (line 601) stdout ----
error[E0658]: use of unstable library feature 'saturating_int_impl'
 --> src/num/saturating.rs:605:12
  |
7 | assert_eq!(Saturating(2i8), Saturating(5i8) / 2);
  |
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = help: add `#![feature(saturating_int_impl)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'saturating_int_impl'
 --> src/num/saturating.rs:605:29
  |
  |
7 | assert_eq!(Saturating(2i8), Saturating(5i8) / 2);
  |
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = help: add `#![feature(saturating_int_impl)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'saturating_int_impl'
 --> src/num/saturating.rs:606:12
  |
  |
8 | assert_eq!(Saturating(i8::MAX), Saturating(i8::MAX) / 1);
  |
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = help: add `#![feature(saturating_int_impl)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'saturating_int_impl'
 --> src/num/saturating.rs:606:33
  |
  |
8 | assert_eq!(Saturating(i8::MAX), Saturating(i8::MAX) / 1);
  |
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = help: add `#![feature(saturating_int_impl)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'saturating_int_impl'
 --> src/num/saturating.rs:607:12
  |
  |
9 | assert_eq!(Saturating(i8::MIN), Saturating(i8::MIN) / 1);
  |
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = help: add `#![feature(saturating_int_impl)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'saturating_int_impl'
 --> src/num/saturating.rs:607:33
  |
  |
9 | assert_eq!(Saturating(i8::MIN), Saturating(i8::MIN) / 1);
  |
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = help: add `#![feature(saturating_int_impl)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'saturating_int_impl'
 --> src/num/saturating.rs:603:5
  |
5 | use std::num::Saturating;
5 | use std::num::Saturating;
  |     ^^^^^^^^^^^^^^^^^^^^
  |
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = help: add `#![feature(saturating_int_impl)]` to the crate attributes to enable
error: aborting due to 7 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/saturating.rs - num::saturating::Saturating<i8> (line 610) stdout ----
error[E0658]: use of unstable library feature 'saturating_int_impl'
 --> src/num/saturating.rs:614:9
  |
7 | let _ = Saturating(0i8) / 0;
  |
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = help: add `#![feature(saturating_int_impl)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'saturating_int_impl'
 --> src/num/saturating.rs:612:5
  |
5 | use std::num::Saturating;
5 | use std::num::Saturating;
  |     ^^^^^^^^^^^^^^^^^^^^
  |
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = help: add `#![feature(saturating_int_impl)]` to the crate attributes to enable
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/saturating.rs - num::saturating::Saturating<isize> (line 601) stdout ----
error[E0658]: use of unstable library feature 'saturating_int_impl'
 --> src/num/saturating.rs:605:12
  |
7 | assert_eq!(Saturating(2isize), Saturating(5isize) / 2);
  |
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = help: add `#![feature(saturating_int_impl)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'saturating_int_impl'
 --> src/num/saturating.rs:605:32
  |
  |
7 | assert_eq!(Saturating(2isize), Saturating(5isize) / 2);
  |
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = help: add `#![feature(saturating_int_impl)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'saturating_int_impl'
 --> src/num/saturating.rs:606:12
  |
  |
8 | assert_eq!(Saturating(isize::MAX), Saturating(isize::MAX) / 1);
  |
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = help: add `#![feature(saturating_int_impl)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'saturating_int_impl'
 --> src/num/saturating.rs:606:36
  |
  |
8 | assert_eq!(Saturating(isize::MAX), Saturating(isize::MAX) / 1);
  |
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = help: add `#![feature(saturating_int_impl)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'saturating_int_impl'
 --> src/num/saturating.rs:607:12
  |
  |
9 | assert_eq!(Saturating(isize::MIN), Saturating(isize::MIN) / 1);
  |
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = note: see issue #87920 <https://github.com/rust-lang/rust/issues/87920> for more information
  = help: add `#![feature(saturating_int_impl)]` to the crate attributes to enable
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


Build completed unsuccessfully in 0:22:14
