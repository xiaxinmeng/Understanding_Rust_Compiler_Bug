plain
.................................................................................................... 9300/11514
.................................................................................................... 9400/11514
.................................................................i......i........................... 9500/11514
.................................................................................................... 9600/11514
....iiiiiii..iiiiii.i............................................................................... 9700/11514
.................................................................................................... 9900/11514
.................................................................................................... 10000/11514
.................................................................................................... 10100/11514
.................................................................................................... 10200/11514
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii

Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
 finished in 0.077 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.43s

 finished in 2.510 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
...................................................i................................................ 400/2948
.................................................................................................... 500/2948
........................i..i.................iiii................................................... 600/2948
.................................................................................................... 700/2948
.......................................................FFF.F.F.F.F.F......F......................... 800/2948
.........................................F..F..F..F.FF.FF............F.............................. 900/2948
.........................F...F....F..FF.F..F..F.........F........................................... 1000/2948
....................FFFFF..F.F.F.............F...................................................... 1100/2948
.F.F.....FF.F.FF.F.........F..........................................................F...F...F.FFF. 1200/2948
.FF..........F...........................................................FF.FF..F......F.F....F..... 1300/2948
.F.....................................................F....FFFFFFF.............F................... 1400/2948
.......................................F.FFFFF..FF...................F.............................. 1500/2948
..................F..FFFFF.FF.....................F................................................. 1600/2948
.FFFFFFF.F................F........................................................................F 1700/2948
.FFF.FF.FF..........F............................................................................... 1800/2948
.................................................................................................... 2000/2948
.................................................................................................... 2100/2948
.................................................................................................... 2200/2948
.................................................................................................... 2300/2948
---
---- src/num/mod.rs - num::i128::fetch_add (line 124) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:126:16
  |
5 | assert_eq!(foo.fetch_add(10), 0);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::i128::fetch_and (line 122) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:124:16
  |
5 | assert_eq!(foo.fetch_and(0b110011), 0b101101);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::i128::fetch_max (line 122) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:124:16
  |
5 | assert_eq!(foo.fetch_max(42), 23);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::i128::fetch_min (line 122) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:124:16
  |
5 | assert_eq!(foo.fetch_min(42), 23);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:126:16
  |
7 | assert_eq!(foo.fetch_min(22), 23);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::i128::fetch_sub (line 124) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:126:16
  |
5 | assert_eq!(foo.fetch_sub(10), 20);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::i128::fetch_nand (line 122) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:124:16
  |
5 | assert_eq!(foo.fetch_nand(0x31), 0x13);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::i128::fetch_or (line 122) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:124:16
  |
5 | assert_eq!(foo.fetch_or(0b110011), 0b101101);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::i128::fetch_xor (line 122) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:124:16
  |
5 | assert_eq!(foo.fetch_xor(0b110011), 0b101101);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::i128::fetch_update (line 122) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:124:14
  |
5 | assert_eq!(x.fetch_update(|_| None), 7);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable


error[E0308]: mismatched types
 --> src/num/mod.rs:124:31
  |
5 | assert_eq!(x.fetch_update(|_| None), 7);
  |                               ^^^^ expected `i128`, found enum `Option`
  = note: expected type `i128`
             found enum `Option<_>`

error[E0658]: use of unstable library feature 'int_fetch_ops'
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:125:14
  |
6 | assert_eq!(x.fetch_update(|x| Some(x + 1)), 7);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable


error[E0308]: mismatched types
 --> src/num/mod.rs:125:31
  |
6 | assert_eq!(x.fetch_update(|x| Some(x + 1)), 7);
  |                               ^^^^^^^^^^^ expected `i128`, found enum `Option`
  = note: expected type `i128`
             found enum `Option<i128>`

error[E0658]: use of unstable library feature 'int_fetch_ops'
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:126:14
  |
7 | assert_eq!(x.fetch_update(|x| Some(x + 1)), 8);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable


error[E0308]: mismatched types
 --> src/num/mod.rs:126:31
  |
7 | assert_eq!(x.fetch_update(|x| Some(x + 1)), 8);
  |                               ^^^^^^^^^^^ expected `i128`, found enum `Option`
  = note: expected type `i128`
             found enum `Option<i128>`

error: aborting due to 6 previous errors
---
---- src/num/mod.rs - num::i16::fetch_add (line 103) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:105:16
  |
5 | assert_eq!(foo.fetch_add(10), 0);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::i16::fetch_and (line 101) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:103:16
  |
5 | assert_eq!(foo.fetch_and(0b110011), 0b101101);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::i16::fetch_max (line 101) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:103:16
  |
5 | assert_eq!(foo.fetch_max(42), 23);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::i16::fetch_or (line 101) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:103:16
  |
5 | assert_eq!(foo.fetch_or(0b110011), 0b101101);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::i16::fetch_min (line 101) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:103:16
  |
5 | assert_eq!(foo.fetch_min(42), 23);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:105:16
  |
7 | assert_eq!(foo.fetch_min(22), 23);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::i16::fetch_nand (line 101) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:103:16
  |
5 | assert_eq!(foo.fetch_nand(0x31), 0x13);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::i16::fetch_sub (line 103) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:105:16
  |
5 | assert_eq!(foo.fetch_sub(10), 20);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::i16::fetch_xor (line 101) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:103:16
  |
5 | assert_eq!(foo.fetch_xor(0b110011), 0b101101);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::i16::fetch_update (line 101) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:103:14
  |
5 | assert_eq!(x.fetch_update(|_| None), 7);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable


error[E0308]: mismatched types
 --> src/num/mod.rs:103:31
  |
5 | assert_eq!(x.fetch_update(|_| None), 7);
  |                               ^^^^ expected `i16`, found enum `Option`
  = note: expected type `i16`
             found enum `Option<_>`

error[E0658]: use of unstable library feature 'int_fetch_ops'
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:104:14
  |
6 | assert_eq!(x.fetch_update(|x| Some(x + 1)), 7);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable


error[E0308]: mismatched types
 --> src/num/mod.rs:104:31
  |
6 | assert_eq!(x.fetch_update(|x| Some(x + 1)), 7);
  |                               ^^^^^^^^^^^ expected `i16`, found enum `Option`
  = note: expected type `i16`
             found enum `Option<i16>`

error[E0658]: use of unstable library feature 'int_fetch_ops'
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:105:14
  |
7 | assert_eq!(x.fetch_update(|x| Some(x + 1)), 8);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable


error[E0308]: mismatched types
 --> src/num/mod.rs:105:31
  |
7 | assert_eq!(x.fetch_update(|x| Some(x + 1)), 8);
  |                               ^^^^^^^^^^^ expected `i16`, found enum `Option`
  = note: expected type `i16`
             found enum `Option<i16>`

error: aborting due to 6 previous errors
---
---- src/num/mod.rs - num::i32::fetch_add (line 109) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:111:16
  |
5 | assert_eq!(foo.fetch_add(10), 0);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::i32::fetch_and (line 107) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:109:16
  |
5 | assert_eq!(foo.fetch_and(0b110011), 0b101101);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::i32::fetch_max (line 107) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:109:16
  |
5 | assert_eq!(foo.fetch_max(42), 23);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::i32::fetch_min (line 107) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:109:16
  |
5 | assert_eq!(foo.fetch_min(42), 23);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:111:16
  |
7 | assert_eq!(foo.fetch_min(22), 23);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::i32::fetch_or (line 107) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:109:16
  |
5 | assert_eq!(foo.fetch_or(0b110011), 0b101101);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::i32::fetch_nand (line 107) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:109:16
  |
5 | assert_eq!(foo.fetch_nand(0x31), 0x13);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::i32::fetch_sub (line 109) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:111:16
  |
5 | assert_eq!(foo.fetch_sub(10), 20);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::i32::fetch_xor (line 107) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:109:16
  |
5 | assert_eq!(foo.fetch_xor(0b110011), 0b101101);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::i32::fetch_update (line 107) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:109:14
  |
5 | assert_eq!(x.fetch_update(|_| None), 7);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable


error[E0308]: mismatched types
 --> src/num/mod.rs:109:31
  |
5 | assert_eq!(x.fetch_update(|_| None), 7);
  |                               ^^^^ expected `i32`, found enum `Option`
  = note: expected type `i32`
             found enum `Option<_>`

error[E0658]: use of unstable library feature 'int_fetch_ops'
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:110:14
  |
6 | assert_eq!(x.fetch_update(|x| Some(x + 1)), 7);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable


error[E0308]: mismatched types
 --> src/num/mod.rs:110:31
  |
6 | assert_eq!(x.fetch_update(|x| Some(x + 1)), 7);
  |                               ^^^^^^^^^^^ expected `i32`, found enum `Option`
  = note: expected type `i32`
             found enum `Option<i32>`

error[E0658]: use of unstable library feature 'int_fetch_ops'
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:111:14
  |
7 | assert_eq!(x.fetch_update(|x| Some(x + 1)), 8);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable


error[E0308]: mismatched types
 --> src/num/mod.rs:111:31
  |
7 | assert_eq!(x.fetch_update(|x| Some(x + 1)), 8);
  |                               ^^^^^^^^^^^ expected `i32`, found enum `Option`
  = note: expected type `i32`
             found enum `Option<i32>`

error: aborting due to 6 previous errors
---
---- src/num/mod.rs - num::i64::fetch_add (line 116) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:118:16
  |
5 | assert_eq!(foo.fetch_add(10), 0);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::i64::fetch_min (line 114) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:116:16
  |
5 | assert_eq!(foo.fetch_min(42), 23);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:118:16
  |
7 | assert_eq!(foo.fetch_min(22), 23);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::i64::fetch_max (line 114) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:116:16
  |
5 | assert_eq!(foo.fetch_max(42), 23);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::i64::fetch_and (line 114) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:116:16
  |
5 | assert_eq!(foo.fetch_and(0b110011), 0b101101);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::i64::fetch_nand (line 114) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:116:16
  |
5 | assert_eq!(foo.fetch_nand(0x31), 0x13);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::i64::fetch_or (line 114) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:116:16
  |
5 | assert_eq!(foo.fetch_or(0b110011), 0b101101);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::i64::fetch_sub (line 116) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:118:16
  |
5 | assert_eq!(foo.fetch_sub(10), 20);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::i64::fetch_xor (line 114) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:116:16
  |
5 | assert_eq!(foo.fetch_xor(0b110011), 0b101101);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::i64::fetch_update (line 114) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:116:14
  |
5 | assert_eq!(x.fetch_update(|_| None), 7);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable


error[E0308]: mismatched types
 --> src/num/mod.rs:116:31
  |
5 | assert_eq!(x.fetch_update(|_| None), 7);
  |                               ^^^^ expected `i64`, found enum `Option`
  = note: expected type `i64`
             found enum `Option<_>`

error[E0658]: use of unstable library feature 'int_fetch_ops'
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:117:14
  |
6 | assert_eq!(x.fetch_update(|x| Some(x + 1)), 7);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable


error[E0308]: mismatched types
 --> src/num/mod.rs:117:31
  |
6 | assert_eq!(x.fetch_update(|x| Some(x + 1)), 7);
  |                               ^^^^^^^^^^^ expected `i64`, found enum `Option`
  = note: expected type `i64`
             found enum `Option<i64>`

error[E0658]: use of unstable library feature 'int_fetch_ops'
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:118:14
  |
7 | assert_eq!(x.fetch_update(|x| Some(x + 1)), 8);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable


error[E0308]: mismatched types
 --> src/num/mod.rs:118:31
  |
7 | assert_eq!(x.fetch_update(|x| Some(x + 1)), 8);
  |                               ^^^^^^^^^^^ expected `i64`, found enum `Option`
  = note: expected type `i64`
             found enum `Option<i64>`

error: aborting due to 6 previous errors
---
---- src/num/mod.rs - num::i8::fetch_add (line 97) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:99:16
  |
5 | assert_eq!(foo.fetch_add(10), 0);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::i8::fetch_and (line 95) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:97:16
  |
5 | assert_eq!(foo.fetch_and(0b110011), 0b101101);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::i8::fetch_max (line 95) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:97:16
  |
5 | assert_eq!(foo.fetch_max(42), 23);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::i8::fetch_min (line 95) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:97:16
  |
5 | assert_eq!(foo.fetch_min(42), 23);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:99:16
  |
7 | assert_eq!(foo.fetch_min(22), 23);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::i8::fetch_sub (line 97) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:99:16
  |
5 | assert_eq!(foo.fetch_sub(10), 20);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::i8::fetch_nand (line 95) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:97:16
  |
5 | assert_eq!(foo.fetch_nand(0x31), 0x13);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::i8::fetch_xor (line 95) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:97:16
  |
5 | assert_eq!(foo.fetch_xor(0b110011), 0b101101);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::i8::fetch_or (line 95) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:97:16
  |
5 | assert_eq!(foo.fetch_or(0b110011), 0b101101);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::i8::fetch_update (line 95) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:97:14
  |
5 | assert_eq!(x.fetch_update(|_| None), 7);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable


error[E0308]: mismatched types
 --> src/num/mod.rs:97:31
  |
5 | assert_eq!(x.fetch_update(|_| None), 7);
  |                               ^^^^ expected `i8`, found enum `Option`
  = note: expected type `i8`
             found enum `Option<_>`

error[E0658]: use of unstable library feature 'int_fetch_ops'
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:98:14
  |
6 | assert_eq!(x.fetch_update(|x| Some(x + 1)), 7);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable


error[E0308]: mismatched types
 --> src/num/mod.rs:98:31
  |
6 | assert_eq!(x.fetch_update(|x| Some(x + 1)), 7);
  |                               ^^^^^^^^^^^ expected `i8`, found enum `Option`
  = note: expected type `i8`
             found enum `Option<i8>`

error[E0658]: use of unstable library feature 'int_fetch_ops'
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:99:14
  |
7 | assert_eq!(x.fetch_update(|x| Some(x + 1)), 8);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable


error[E0308]: mismatched types
 --> src/num/mod.rs:99:31
  |
7 | assert_eq!(x.fetch_update(|x| Some(x + 1)), 8);
  |                               ^^^^^^^^^^^ expected `i8`, found enum `Option`
  = note: expected type `i8`
             found enum `Option<i8>`

error: aborting due to 6 previous errors
---
---- src/num/mod.rs - num::isize::fetch_add (line 154) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:156:16
  |
5 | assert_eq!(foo.fetch_add(10), 0);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::isize::fetch_and (line 152) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:154:16
  |
5 | assert_eq!(foo.fetch_and(0b110011), 0b101101);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::isize::fetch_max (line 152) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:154:16
  |
5 | assert_eq!(foo.fetch_max(42), 23);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::isize::fetch_or (line 152) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:154:16
  |
5 | assert_eq!(foo.fetch_or(0b110011), 0b101101);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::isize::fetch_sub (line 154) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:156:16
  |
5 | assert_eq!(foo.fetch_sub(10), 20);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::isize::fetch_min (line 152) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:154:16
  |
5 | assert_eq!(foo.fetch_min(42), 23);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:156:16
  |
7 | assert_eq!(foo.fetch_min(22), 23);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::isize::fetch_nand (line 152) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:154:16
  |
5 | assert_eq!(foo.fetch_nand(0x31), 0x13);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::isize::fetch_xor (line 152) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:154:16
  |
5 | assert_eq!(foo.fetch_xor(0b110011), 0b101101);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::isize::fetch_update (line 152) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:154:14
  |
5 | assert_eq!(x.fetch_update(|_| None), 7);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable


error[E0308]: mismatched types
 --> src/num/mod.rs:154:31
  |
5 | assert_eq!(x.fetch_update(|_| None), 7);
  |                               ^^^^ expected `isize`, found enum `Option`
  = note: expected type `isize`
             found enum `Option<_>`

error[E0658]: use of unstable library feature 'int_fetch_ops'
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:155:14
  |
6 | assert_eq!(x.fetch_update(|x| Some(x + 1)), 7);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable


error[E0308]: mismatched types
 --> src/num/mod.rs:155:31
  |
6 | assert_eq!(x.fetch_update(|x| Some(x + 1)), 7);
  |                               ^^^^^^^^^^^ expected `isize`, found enum `Option`
  = note: expected type `isize`
             found enum `Option<isize>`

error[E0658]: use of unstable library feature 'int_fetch_ops'
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:156:14
  |
7 | assert_eq!(x.fetch_update(|x| Some(x + 1)), 8);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable


error[E0308]: mismatched types
 --> src/num/mod.rs:156:31
  |
7 | assert_eq!(x.fetch_update(|x| Some(x + 1)), 8);
  |                               ^^^^^^^^^^^ expected `isize`, found enum `Option`
  = note: expected type `isize`
             found enum `Option<isize>`

error: aborting due to 6 previous errors
---
---- src/num/mod.rs - num::u128::fetch_and (line 693) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:695:16
  |
5 | assert_eq!(foo.fetch_and(0b110011), 0b101101);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::u128::fetch_add (line 695) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:697:16
  |
5 | assert_eq!(foo.fetch_add(10), 0);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::u128::fetch_min (line 693) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:695:16
  |
5 | assert_eq!(foo.fetch_min(42), 23);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:697:16
  |
7 | assert_eq!(foo.fetch_min(22), 23);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::u128::fetch_max (line 693) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:695:16
  |
5 | assert_eq!(foo.fetch_max(42), 23);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::u128::fetch_nand (line 693) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:695:16
  |
5 | assert_eq!(foo.fetch_nand(0x31), 0x13);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::u128::fetch_or (line 693) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:695:16
  |
5 | assert_eq!(foo.fetch_or(0b110011), 0b101101);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::u128::fetch_sub (line 695) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:697:16
  |
5 | assert_eq!(foo.fetch_sub(10), 20);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::u128::fetch_xor (line 693) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:695:16
  |
5 | assert_eq!(foo.fetch_xor(0b110011), 0b101101);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::u128::fetch_update (line 693) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:695:14
  |
5 | assert_eq!(x.fetch_update(|_| None), 7);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable


error[E0308]: mismatched types
 --> src/num/mod.rs:695:31
  |
5 | assert_eq!(x.fetch_update(|_| None), 7);
  |                               ^^^^ expected `u128`, found enum `Option`
  = note: expected type `u128`
             found enum `Option<_>`

error[E0658]: use of unstable library feature 'int_fetch_ops'
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:696:14
  |
6 | assert_eq!(x.fetch_update(|x| Some(x + 1)), 7);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable


error[E0308]: mismatched types
 --> src/num/mod.rs:696:31
  |
6 | assert_eq!(x.fetch_update(|x| Some(x + 1)), 7);
  |                               ^^^^^^^^^^^ expected `u128`, found enum `Option`
  = note: expected type `u128`
             found enum `Option<u128>`

error[E0658]: use of unstable library feature 'int_fetch_ops'
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:697:14
  |
7 | assert_eq!(x.fetch_update(|x| Some(x + 1)), 8);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable


error[E0308]: mismatched types
 --> src/num/mod.rs:697:31
  |
7 | assert_eq!(x.fetch_update(|x| Some(x + 1)), 8);
  |                               ^^^^^^^^^^^ expected `u128`, found enum `Option`
  = note: expected type `u128`
             found enum `Option<u128>`

error: aborting due to 6 previous errors
---
---- src/num/mod.rs - num::u16::fetch_add (line 674) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:676:16
  |
5 | assert_eq!(foo.fetch_add(10), 0);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::u16::fetch_max (line 672) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:674:16
  |
5 | assert_eq!(foo.fetch_max(42), 23);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::u16::fetch_and (line 672) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:674:16
  |
5 | assert_eq!(foo.fetch_and(0b110011), 0b101101);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::u16::fetch_or (line 672) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:674:16
  |
5 | assert_eq!(foo.fetch_or(0b110011), 0b101101);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::u16::fetch_sub (line 674) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:676:16
  |
5 | assert_eq!(foo.fetch_sub(10), 20);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::u16::fetch_min (line 672) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:674:16
  |
5 | assert_eq!(foo.fetch_min(42), 23);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:676:16
  |
7 | assert_eq!(foo.fetch_min(22), 23);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::u16::fetch_xor (line 672) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:674:16
  |
5 | assert_eq!(foo.fetch_xor(0b110011), 0b101101);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::u16::fetch_nand (line 672) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:674:16
  |
5 | assert_eq!(foo.fetch_nand(0x31), 0x13);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::u16::fetch_update (line 672) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:674:14
  |
5 | assert_eq!(x.fetch_update(|_| None), 7);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable


error[E0308]: mismatched types
 --> src/num/mod.rs:674:31
  |
5 | assert_eq!(x.fetch_update(|_| None), 7);
  |                               ^^^^ expected `u16`, found enum `Option`
  = note: expected type `u16`
             found enum `Option<_>`

error[E0658]: use of unstable library feature 'int_fetch_ops'
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:675:14
  |
6 | assert_eq!(x.fetch_update(|x| Some(x + 1)), 7);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable


error[E0308]: mismatched types
 --> src/num/mod.rs:675:31
  |
6 | assert_eq!(x.fetch_update(|x| Some(x + 1)), 7);
  |                               ^^^^^^^^^^^ expected `u16`, found enum `Option`
  = note: expected type `u16`
             found enum `Option<u16>`

error[E0658]: use of unstable library feature 'int_fetch_ops'
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:676:14
  |
7 | assert_eq!(x.fetch_update(|x| Some(x + 1)), 8);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable


error[E0308]: mismatched types
 --> src/num/mod.rs:676:31
  |
7 | assert_eq!(x.fetch_update(|x| Some(x + 1)), 8);
  |                               ^^^^^^^^^^^ expected `u16`, found enum `Option`
  = note: expected type `u16`
             found enum `Option<u16>`

error: aborting due to 6 previous errors
---
---- src/num/mod.rs - num::u32::fetch_add (line 680) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:682:16
  |
5 | assert_eq!(foo.fetch_add(10), 0);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::u32::fetch_and (line 678) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:680:16
  |
5 | assert_eq!(foo.fetch_and(0b110011), 0b101101);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::u32::fetch_max (line 678) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:680:16
  |
5 | assert_eq!(foo.fetch_max(42), 23);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::u32::fetch_min (line 678) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:680:16
  |
5 | assert_eq!(foo.fetch_min(42), 23);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:682:16
  |
7 | assert_eq!(foo.fetch_min(22), 23);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::u32::fetch_sub (line 680) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:682:16
  |
5 | assert_eq!(foo.fetch_sub(10), 20);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::u32::fetch_xor (line 678) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:680:16
  |
5 | assert_eq!(foo.fetch_xor(0b110011), 0b101101);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::u32::fetch_or (line 678) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:680:16
  |
5 | assert_eq!(foo.fetch_or(0b110011), 0b101101);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::u32::fetch_nand (line 678) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:680:16
  |
5 | assert_eq!(foo.fetch_nand(0x31), 0x13);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::u32::fetch_update (line 678) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:680:14
  |
5 | assert_eq!(x.fetch_update(|_| None), 7);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable


error[E0308]: mismatched types
 --> src/num/mod.rs:680:31
  |
5 | assert_eq!(x.fetch_update(|_| None), 7);
  |                               ^^^^ expected `u32`, found enum `Option`
  = note: expected type `u32`
             found enum `Option<_>`

error[E0658]: use of unstable library feature 'int_fetch_ops'
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:681:14
  |
6 | assert_eq!(x.fetch_update(|x| Some(x + 1)), 7);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable


error[E0308]: mismatched types
 --> src/num/mod.rs:681:31
  |
6 | assert_eq!(x.fetch_update(|x| Some(x + 1)), 7);
  |                               ^^^^^^^^^^^ expected `u32`, found enum `Option`
  = note: expected type `u32`
             found enum `Option<u32>`

error[E0658]: use of unstable library feature 'int_fetch_ops'
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:682:14
  |
7 | assert_eq!(x.fetch_update(|x| Some(x + 1)), 8);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable


error[E0308]: mismatched types
 --> src/num/mod.rs:682:31
  |
7 | assert_eq!(x.fetch_update(|x| Some(x + 1)), 8);
  |                               ^^^^^^^^^^^ expected `u32`, found enum `Option`
  = note: expected type `u32`
             found enum `Option<u32>`

error: aborting due to 6 previous errors
---
---- src/num/mod.rs - num::u64::fetch_add (line 686) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:688:16
  |
5 | assert_eq!(foo.fetch_add(10), 0);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::u64::fetch_and (line 684) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:686:16
  |
5 | assert_eq!(foo.fetch_and(0b110011), 0b101101);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::u64::fetch_max (line 684) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:686:16
  |
5 | assert_eq!(foo.fetch_max(42), 23);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::u64::fetch_min (line 684) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:686:16
  |
5 | assert_eq!(foo.fetch_min(42), 23);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:688:16
  |
7 | assert_eq!(foo.fetch_min(22), 23);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::u64::fetch_nand (line 684) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:686:16
  |
5 | assert_eq!(foo.fetch_nand(0x31), 0x13);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::u64::fetch_or (line 684) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:686:16
  |
5 | assert_eq!(foo.fetch_or(0b110011), 0b101101);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::u64::fetch_sub (line 686) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:688:16
  |
5 | assert_eq!(foo.fetch_sub(10), 20);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::u64::fetch_xor (line 684) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:686:16
  |
5 | assert_eq!(foo.fetch_xor(0b110011), 0b101101);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::u64::fetch_update (line 684) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:686:14
  |
5 | assert_eq!(x.fetch_update(|_| None), 7);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable


error[E0308]: mismatched types
 --> src/num/mod.rs:686:31
  |
5 | assert_eq!(x.fetch_update(|_| None), 7);
  |                               ^^^^ expected `u64`, found enum `Option`
  = note: expected type `u64`
             found enum `Option<_>`

error[E0658]: use of unstable library feature 'int_fetch_ops'
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:687:14
  |
6 | assert_eq!(x.fetch_update(|x| Some(x + 1)), 7);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable


error[E0308]: mismatched types
 --> src/num/mod.rs:687:31
  |
6 | assert_eq!(x.fetch_update(|x| Some(x + 1)), 7);
  |                               ^^^^^^^^^^^ expected `u64`, found enum `Option`
  = note: expected type `u64`
             found enum `Option<u64>`

error[E0658]: use of unstable library feature 'int_fetch_ops'
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:688:14
  |
7 | assert_eq!(x.fetch_update(|x| Some(x + 1)), 8);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable


error[E0308]: mismatched types
 --> src/num/mod.rs:688:31
  |
7 | assert_eq!(x.fetch_update(|x| Some(x + 1)), 8);
  |                               ^^^^^^^^^^^ expected `u64`, found enum `Option`
  = note: expected type `u64`
             found enum `Option<u64>`

error: aborting due to 6 previous errors
---
---- src/num/mod.rs - num::u8::fetch_and (line 164) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:166:16
  |
5 | assert_eq!(foo.fetch_and(0b110011), 0b101101);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::u8::fetch_add (line 166) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:168:16
  |
5 | assert_eq!(foo.fetch_add(10), 0);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::u8::fetch_max (line 164) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:166:16
  |
5 | assert_eq!(foo.fetch_max(42), 23);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::u8::fetch_min (line 164) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:166:16
  |
5 | assert_eq!(foo.fetch_min(42), 23);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:168:16
  |
7 | assert_eq!(foo.fetch_min(22), 23);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::u8::fetch_or (line 164) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:166:16
  |
5 | assert_eq!(foo.fetch_or(0b110011), 0b101101);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::u8::fetch_sub (line 166) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:168:16
  |
5 | assert_eq!(foo.fetch_sub(10), 20);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::u8::fetch_nand (line 164) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:166:16
  |
5 | assert_eq!(foo.fetch_nand(0x31), 0x13);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::u8::fetch_xor (line 164) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:166:16
  |
5 | assert_eq!(foo.fetch_xor(0b110011), 0b101101);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::u8::fetch_update (line 164) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:166:14
  |
5 | assert_eq!(x.fetch_update(|_| None), 7);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable


error[E0308]: mismatched types
 --> src/num/mod.rs:166:31
  |
5 | assert_eq!(x.fetch_update(|_| None), 7);
  |                               ^^^^ expected `u8`, found enum `Option`
  = note: expected type `u8`
             found enum `Option<_>`

error[E0658]: use of unstable library feature 'int_fetch_ops'
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:167:14
  |
6 | assert_eq!(x.fetch_update(|x| Some(x + 1)), 7);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable


error[E0308]: mismatched types
 --> src/num/mod.rs:167:31
  |
6 | assert_eq!(x.fetch_update(|x| Some(x + 1)), 7);
  |                               ^^^^^^^^^^^ expected `u8`, found enum `Option`
  = note: expected type `u8`
             found enum `Option<u8>`

error[E0658]: use of unstable library feature 'int_fetch_ops'
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:168:14
  |
7 | assert_eq!(x.fetch_update(|x| Some(x + 1)), 8);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable


error[E0308]: mismatched types
 --> src/num/mod.rs:168:31
  |
7 | assert_eq!(x.fetch_update(|x| Some(x + 1)), 8);
  |                               ^^^^^^^^^^^ expected `u8`, found enum `Option`
  = note: expected type `u8`
             found enum `Option<u8>`

error: aborting due to 6 previous errors
---
---- src/num/mod.rs - num::usize::fetch_add (line 723) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:725:16
  |
5 | assert_eq!(foo.fetch_add(10), 0);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::usize::fetch_and (line 721) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:723:16
  |
5 | assert_eq!(foo.fetch_and(0b110011), 0b101101);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::usize::fetch_max (line 721) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:723:16
  |
5 | assert_eq!(foo.fetch_max(42), 23);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::usize::fetch_min (line 721) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:723:16
  |
5 | assert_eq!(foo.fetch_min(42), 23);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:725:16
  |
7 | assert_eq!(foo.fetch_min(22), 23);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::usize::fetch_nand (line 721) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:723:16
  |
5 | assert_eq!(foo.fetch_nand(0x31), 0x13);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::usize::fetch_or (line 721) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:723:16
  |
5 | assert_eq!(foo.fetch_or(0b110011), 0b101101);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::usize::fetch_sub (line 723) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:725:16
  |
5 | assert_eq!(foo.fetch_sub(10), 20);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::usize::fetch_xor (line 721) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:723:16
  |
5 | assert_eq!(foo.fetch_xor(0b110011), 0b101101);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable

---
---- src/num/mod.rs - num::usize::fetch_update (line 721) stdout ----
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:723:14
  |
5 | assert_eq!(x.fetch_update(|_| None), 7);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable


error[E0308]: mismatched types
 --> src/num/mod.rs:723:31
  |
5 | assert_eq!(x.fetch_update(|_| None), 7);
  |                               ^^^^ expected `usize`, found enum `Option`
  = note: expected type `usize`
             found enum `Option<_>`

error[E0658]: use of unstable library feature 'int_fetch_ops'
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:724:14
  |
6 | assert_eq!(x.fetch_update(|x| Some(x + 1)), 7);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable


error[E0308]: mismatched types
 --> src/num/mod.rs:724:31
  |
6 | assert_eq!(x.fetch_update(|x| Some(x + 1)), 7);
  |                               ^^^^^^^^^^^ expected `usize`, found enum `Option`
  = note: expected type `usize`
             found enum `Option<usize>`

error[E0658]: use of unstable library feature 'int_fetch_ops'
error[E0658]: use of unstable library feature 'int_fetch_ops'
 --> src/num/mod.rs:725:14
  |
7 | assert_eq!(x.fetch_update(|x| Some(x + 1)), 8);
  |
  = note: see issue #76904 <https://github.com/rust-lang/rust/issues/76904> for more information
  = help: add `#![feature(int_fetch_ops)]` to the crate attributes to enable


error[E0308]: mismatched types
 --> src/num/mod.rs:725:31
  |
7 | assert_eq!(x.fetch_update(|x| Some(x + 1)), 8);
  |                               ^^^^^^^^^^^ expected `usize`, found enum `Option`
  = note: expected type `usize`
             found enum `Option<usize>`

error: aborting due to 6 previous errors
---
test result: FAILED. 2813 passed; 108 failed; 27 ignored; 0 measured; 0 filtered out; finished in 45.36s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:21:08
