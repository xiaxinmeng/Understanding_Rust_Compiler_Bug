plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
........................................................................................ 1760/3953
........................................................................................ 1848/3953
........................................................................................ 1936/3953
........................................................................................ 2024/3953
......................................................................................F. 2112/3953
....FF....F.......F.F......F..F.......F.....F....FF.......F.......F.F...F........F....F. 2200/3953
.F....F.......F....F.F.....F............................................................ 2288/3953
........................................................................................ 2464/3953
........................................................................................ 2552/3953
........................................................................................ 2640/3953
........................................................................................ 2728/3953
---
.i.......................i.......................i.......................i.............. 3872/3953
.................................................................................
failures:

---- src/num/nonzero.rs - num::nonzero::NonZero<i128>::checked_neg (line 1181) stdout ----
error[E0658]: use of unstable library feature 'generic_nonzero'
  |
  |
8 | let pos_five = NonZero::<i128>::new(5)?;
  |
  = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
  = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
  = help: add `#![feature(generic_nonzero)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'generic_nonzero'
 --> src/num/nonzero.rs:1188:16
  |
  |
9 | let neg_five = NonZero::<i128>::new(-5)?;
  |
  = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
  = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
  = help: add `#![feature(generic_nonzero)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'generic_nonzero'
  --> src/num/nonzero.rs:1189:11
   |
   |
10 | let min = NonZero::<i128>::new(i128::MIN)?;
   |
   = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
   = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
   = help: add `#![feature(generic_nonzero)]` to the crate attributes to enable
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/nonzero.rs - num::nonzero::NonZero<i128>::overflowing_neg (line 1184) stdout ----
error[E0658]: use of unstable library feature 'generic_nonzero'
  |
  |
8 | let pos_five = NonZero::<i128>::new(5)?;
  |
  = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
  = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
  = help: add `#![feature(generic_nonzero)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'generic_nonzero'
 --> src/num/nonzero.rs:1191:16
  |
  |
9 | let neg_five = NonZero::<i128>::new(-5)?;
  |
  = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
  = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
  = help: add `#![feature(generic_nonzero)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'generic_nonzero'
  --> src/num/nonzero.rs:1192:11
   |
   |
10 | let min = NonZero::<i128>::new(i128::MIN)?;
   |
   = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
   = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
   = help: add `#![feature(generic_nonzero)]` to the crate attributes to enable
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/nonzero.rs - num::nonzero::NonZero<i128>::saturating_neg (line 1182) stdout ----
error[E0658]: use of unstable library feature 'generic_nonzero'
  |
  |
8 | let pos_five = NonZero::<i128>::new(5)?;
  |
  = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
  = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
  = help: add `#![feature(generic_nonzero)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'generic_nonzero'
 --> src/num/nonzero.rs:1189:16
  |
  |
9 | let neg_five = NonZero::<i128>::new(-5)?;
  |
  = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
  = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
  = help: add `#![feature(generic_nonzero)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'generic_nonzero'
  --> src/num/nonzero.rs:1190:11
   |
   |
10 | let min = NonZero::<i128>::new(i128::MIN)?;
   |
   = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
   = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
   = help: add `#![feature(generic_nonzero)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'generic_nonzero'
  --> src/num/nonzero.rs:1191:20
   |
   |
11 | let min_plus_one = NonZero::<i128>::new(i128::MIN + 1)?;
   |
   = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
   = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
   = help: add `#![feature(generic_nonzero)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'generic_nonzero'
  --> src/num/nonzero.rs:1192:11
   |
   |
12 | let max = NonZero::<i128>::new(i128::MAX)?;
   |
   = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
   = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
   = help: add `#![feature(generic_nonzero)]` to the crate attributes to enable
error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/nonzero.rs - num::nonzero::NonZero<i128>::wrapping_neg (line 1185) stdout ----
error[E0658]: use of unstable library feature 'generic_nonzero'
  |
  |
8 | let pos_five = NonZero::<i128>::new(5)?;
  |
  = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
  = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
  = help: add `#![feature(generic_nonzero)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'generic_nonzero'
 --> src/num/nonzero.rs:1192:16
  |
  |
9 | let neg_five = NonZero::<i128>::new(-5)?;
  |
  = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
  = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
  = help: add `#![feature(generic_nonzero)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'generic_nonzero'
  --> src/num/nonzero.rs:1193:11
   |
   |
10 | let min = NonZero::<i128>::new(i128::MIN)?;
   |
   = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
   = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
   = help: add `#![feature(generic_nonzero)]` to the crate attributes to enable
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/nonzero.rs - num::nonzero::NonZero<i16>::checked_neg (line 1181) stdout ----
error[E0658]: use of unstable library feature 'generic_nonzero'
  |
  |
8 | let pos_five = NonZero::<i16>::new(5)?;
  |
  = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
  = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
  = help: add `#![feature(generic_nonzero)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'generic_nonzero'
 --> src/num/nonzero.rs:1188:16
  |
  |
9 | let neg_five = NonZero::<i16>::new(-5)?;
  |
  = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
  = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
  = help: add `#![feature(generic_nonzero)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'generic_nonzero'
  --> src/num/nonzero.rs:1189:11
   |
   |
10 | let min = NonZero::<i16>::new(i16::MIN)?;
   |
   = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
   = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
   = help: add `#![feature(generic_nonzero)]` to the crate attributes to enable
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/nonzero.rs - num::nonzero::NonZero<i16>::overflowing_neg (line 1184) stdout ----
error[E0658]: use of unstable library feature 'generic_nonzero'
  |
  |
8 | let pos_five = NonZero::<i16>::new(5)?;
  |
  = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
  = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
  = help: add `#![feature(generic_nonzero)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'generic_nonzero'
 --> src/num/nonzero.rs:1191:16
  |
  |
9 | let neg_five = NonZero::<i16>::new(-5)?;
  |
  = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
  = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
  = help: add `#![feature(generic_nonzero)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'generic_nonzero'
  --> src/num/nonzero.rs:1192:11
   |
   |
10 | let min = NonZero::<i16>::new(i16::MIN)?;
   |
   = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
   = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
   = help: add `#![feature(generic_nonzero)]` to the crate attributes to enable
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/nonzero.rs - num::nonzero::NonZero<i16>::saturating_neg (line 1182) stdout ----
error[E0658]: use of unstable library feature 'generic_nonzero'
  |
  |
8 | let pos_five = NonZero::<i16>::new(5)?;
  |
  = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
  = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
  = help: add `#![feature(generic_nonzero)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'generic_nonzero'
 --> src/num/nonzero.rs:1189:16
  |
  |
9 | let neg_five = NonZero::<i16>::new(-5)?;
  |
  = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
  = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
  = help: add `#![feature(generic_nonzero)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'generic_nonzero'
  --> src/num/nonzero.rs:1190:11
   |
   |
10 | let min = NonZero::<i16>::new(i16::MIN)?;
   |
   = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
   = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
   = help: add `#![feature(generic_nonzero)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'generic_nonzero'
  --> src/num/nonzero.rs:1191:20
   |
   |
11 | let min_plus_one = NonZero::<i16>::new(i16::MIN + 1)?;
   |
   = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
   = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
   = help: add `#![feature(generic_nonzero)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'generic_nonzero'
  --> src/num/nonzero.rs:1192:11
   |
   |
12 | let max = NonZero::<i16>::new(i16::MAX)?;
   |
   = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
   = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
   = help: add `#![feature(generic_nonzero)]` to the crate attributes to enable
error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/nonzero.rs - num::nonzero::NonZero<i16>::wrapping_neg (line 1185) stdout ----
error[E0658]: use of unstable library feature 'generic_nonzero'
  |
  |
8 | let pos_five = NonZero::<i16>::new(5)?;
  |
  = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
  = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
  = help: add `#![feature(generic_nonzero)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'generic_nonzero'
 --> src/num/nonzero.rs:1192:16
  |
  |
9 | let neg_five = NonZero::<i16>::new(-5)?;
  |
  = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
  = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
  = help: add `#![feature(generic_nonzero)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'generic_nonzero'
  --> src/num/nonzero.rs:1193:11
   |
   |
10 | let min = NonZero::<i16>::new(i16::MIN)?;
   |
   = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
   = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
   = help: add `#![feature(generic_nonzero)]` to the crate attributes to enable
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/nonzero.rs - num::nonzero::NonZero<i32>::checked_neg (line 1181) stdout ----
error[E0658]: use of unstable library feature 'generic_nonzero'
  |
  |
8 | let pos_five = NonZero::<i32>::new(5)?;
  |
  = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
  = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
  = help: add `#![feature(generic_nonzero)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'generic_nonzero'
 --> src/num/nonzero.rs:1188:16
  |
  |
9 | let neg_five = NonZero::<i32>::new(-5)?;
  |
  = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
  = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
  = help: add `#![feature(generic_nonzero)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'generic_nonzero'
  --> src/num/nonzero.rs:1189:11
   |
   |
10 | let min = NonZero::<i32>::new(i32::MIN)?;
   |
   = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
   = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
   = help: add `#![feature(generic_nonzero)]` to the crate attributes to enable
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/nonzero.rs - num::nonzero::NonZero<i32>::overflowing_neg (line 1184) stdout ----
error[E0658]: use of unstable library feature 'generic_nonzero'
  |
  |
8 | let pos_five = NonZero::<i32>::new(5)?;
  |
  = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
  = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
  = help: add `#![feature(generic_nonzero)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'generic_nonzero'
error[E0658]: use of unstable library feature 'generic_nonzero'
error: doctest failed, to rerun pass `-p core --doc`
  |
  |
9 | let neg_five = NonZero::<i32>::new(-5)?;
  |
  = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
  = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
  = help: add `#![feature(generic_nonzero)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'generic_nonzero'
  --> src/num/nonzero.rs:1192:11
   |
   |
10 | let min = NonZero::<i32>::new(i32::MIN)?;
   |
   = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
   = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
   = help: add `#![feature(generic_nonzero)]` to the crate attributes to enable
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/nonzero.rs - num::nonzero::NonZero<i32>::saturating_neg (line 1182) stdout ----
error[E0658]: use of unstable library feature 'generic_nonzero'
  |
  |
8 | let pos_five = NonZero::<i32>::new(5)?;
  |
  = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
  = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
  = help: add `#![feature(generic_nonzero)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'generic_nonzero'
 --> src/num/nonzero.rs:1189:16
  |
  |
9 | let neg_five = NonZero::<i32>::new(-5)?;
  |
  = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
  = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
  = help: add `#![feature(generic_nonzero)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'generic_nonzero'
  --> src/num/nonzero.rs:1190:11
   |
   |
10 | let min = NonZero::<i32>::new(i32::MIN)?;
   |
   = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
   = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
   = help: add `#![feature(generic_nonzero)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'generic_nonzero'
  --> src/num/nonzero.rs:1191:20
   |
   |
11 | let min_plus_one = NonZero::<i32>::new(i32::MIN + 1)?;
   |
   = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
   = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
   = help: add `#![feature(generic_nonzero)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'generic_nonzero'
  --> src/num/nonzero.rs:1192:11
   |
   |
12 | let max = NonZero::<i32>::new(i32::MAX)?;
   |
   = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
   = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
   = help: add `#![feature(generic_nonzero)]` to the crate attributes to enable
error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/nonzero.rs - num::nonzero::NonZero<i32>::wrapping_neg (line 1185) stdout ----
error[E0658]: use of unstable library feature 'generic_nonzero'
  |
  |
8 | let pos_five = NonZero::<i32>::new(5)?;
  |
  = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
  = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
  = help: add `#![feature(generic_nonzero)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'generic_nonzero'
 --> src/num/nonzero.rs:1192:16
  |
  |
9 | let neg_five = NonZero::<i32>::new(-5)?;
  |
  = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
  = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
  = help: add `#![feature(generic_nonzero)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'generic_nonzero'
  --> src/num/nonzero.rs:1193:11
   |
   |
10 | let min = NonZero::<i32>::new(i32::MIN)?;
   |
   = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
   = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
   = help: add `#![feature(generic_nonzero)]` to the crate attributes to enable
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/nonzero.rs - num::nonzero::NonZero<i64>::checked_neg (line 1181) stdout ----
error[E0658]: use of unstable library feature 'generic_nonzero'
  |
  |
8 | let pos_five = NonZero::<i64>::new(5)?;
  |
  = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
  = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
  = help: add `#![feature(generic_nonzero)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'generic_nonzero'
 --> src/num/nonzero.rs:1188:16
  |
  |
9 | let neg_five = NonZero::<i64>::new(-5)?;
  |
  = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
  = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
  = help: add `#![feature(generic_nonzero)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'generic_nonzero'
  --> src/num/nonzero.rs:1189:11
   |
   |
10 | let min = NonZero::<i64>::new(i64::MIN)?;
   |
   = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
   = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
   = help: add `#![feature(generic_nonzero)]` to the crate attributes to enable
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/nonzero.rs - num::nonzero::NonZero<i64>::overflowing_neg (line 1184) stdout ----
error[E0658]: use of unstable library feature 'generic_nonzero'
  |
  |
8 | let pos_five = NonZero::<i64>::new(5)?;
  |
  = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
  = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
  = help: add `#![feature(generic_nonzero)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'generic_nonzero'
 --> src/num/nonzero.rs:1191:16
  |
  |
9 | let neg_five = NonZero::<i64>::new(-5)?;
  |
  = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
  = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
  = help: add `#![feature(generic_nonzero)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'generic_nonzero'
  --> src/num/nonzero.rs:1192:11
   |
   |
10 | let min = NonZero::<i64>::new(i64::MIN)?;
   |
   = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
