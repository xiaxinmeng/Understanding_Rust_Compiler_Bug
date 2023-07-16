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

 finished in 0.068 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.32s

 finished in 2.396 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
...................................................i................................................ 400/2948
.................................................................................................... 500/2948
........................i..i.................iiii................................................... 600/2948
.................................................................................................... 700/2948
.......................................................F.F..F.FFF..F.F.F............................ 800/2948
.......................................F...FF......FF.F..F.FF....................................... 900/2948
..........................FF.F....F...F.F..FF..F.................................................... 1000/2948
..............FF..F..FF.FF....FF...................................................................F 1100/2948
.....F..F.FF.FF.FF.....................................................................FF....F..FFF. 1200/2948
FF..F.....................................................................FFF.F.FF.FF.F............. 1300/2948
..........................................................FF.FFFFFFF................................ 1400/2948
.......................................FF.FFF.FF.FF................................................. 1500/2948
...................FFF...FF.FFFF.................................................................... 1600/2948
F..FF.FFFFF..F....................................................................................FF 1700/2948
.FF.F.FF.FF......................................................................................... 1800/2948
.................................................................................................... 2000/2948
.................................................................................................... 2100/2948
.................................................................................................... 2200/2948
.................................................................................................... 2300/2948
---
i.....................i.....................i.....................i................................. 2900/2948
................................................
failures:

---- src/num/mod.rs - num::i128::fetch_add (line 124) stdout ----
error[E0596]: cannot borrow `foo` as mutable, as it is not declared as mutable
  |
5 | let foo: i128 = 0;
5 | let foo: i128 = 0;
  |     --- help: consider changing this to be mutable: `mut foo`
6 | assert_eq!(foo.fetch_add(10), 0);
  |            ^^^ cannot borrow as mutable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0596`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/mod.rs - num::i128::fetch_and (line 122) stdout ----
error[E0596]: cannot borrow `foo` as mutable, as it is not declared as mutable
  |
  |
5 | let foo: i128 = 0b101101;
  |     --- help: consider changing this to be mutable: `mut foo`
6 | assert_eq!(foo.fetch_and(0b110011), 0b101101);
  |            ^^^ cannot borrow as mutable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0596`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/mod.rs - num::i128::fetch_max (line 122) stdout ----
error[E0596]: cannot borrow `foo` as mutable, as it is not declared as mutable
  |
5 | let foo: i128 = 23;
5 | let foo: i128 = 23;
  |     --- help: consider changing this to be mutable: `mut foo`
6 | assert_eq!(foo.fetch_max(42), 23);
  |            ^^^ cannot borrow as mutable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0596`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/mod.rs - num::i128::fetch_min (line 122) stdout ----
error[E0596]: cannot borrow `foo` as mutable, as it is not declared as mutable
  |
5 | let foo: i128 = 23;
5 | let foo: i128 = 23;
  |     --- help: consider changing this to be mutable: `mut foo`
6 | assert_eq!(foo.fetch_min(42), 23);
  |            ^^^ cannot borrow as mutable

error[E0596]: cannot borrow `foo` as mutable, as it is not declared as mutable
  |
5 | let foo: i128 = 23;
5 | let foo: i128 = 23;
  |     --- help: consider changing this to be mutable: `mut foo`
...
8 | assert_eq!(foo.fetch_min(22), 23);
  |            ^^^ cannot borrow as mutable
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0596`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/mod.rs - num::i128::fetch_or (line 122) stdout ----
error[E0596]: cannot borrow `foo` as mutable, as it is not declared as mutable
  |
  |
5 | let foo: i128 = 0b101101;
  |     --- help: consider changing this to be mutable: `mut foo`
6 | assert_eq!(foo.fetch_or(0b110011), 0b101101);
  |            ^^^ cannot borrow as mutable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0596`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/mod.rs - num::i128::fetch_sub (line 124) stdout ----
error[E0596]: cannot borrow `foo` as mutable, as it is not declared as mutable
  |
5 | let foo: i128 = 20;
5 | let foo: i128 = 20;
  |     --- help: consider changing this to be mutable: `mut foo`
6 | assert_eq!(foo.fetch_sub(10), 20);
  |            ^^^ cannot borrow as mutable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0596`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/mod.rs - num::i128::fetch_nand (line 122) stdout ----
error[E0596]: cannot borrow `foo` as mutable, as it is not declared as mutable
  |
  |
5 | let foo: i128 = 0x13;
  |     --- help: consider changing this to be mutable: `mut foo`
6 | assert_eq!(foo.fetch_nand(0x31), 0x13);
  |            ^^^ cannot borrow as mutable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0596`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/mod.rs - num::i128::fetch_update (line 122) stdout ----
error[E0596]: cannot borrow `x` as mutable, as it is not declared as mutable
  |
5 | let x: i128 = 7;
5 | let x: i128 = 7;
  |     - help: consider changing this to be mutable: `mut x`
6 | assert_eq!(x.fetch_update(|x| x + 1), 7);
  |            ^ cannot borrow as mutable

error[E0596]: cannot borrow `x` as mutable, as it is not declared as mutable
  |
5 | let x: i128 = 7;
5 | let x: i128 = 7;
  |     - help: consider changing this to be mutable: `mut x`
6 | assert_eq!(x.fetch_update(|x| x + 1), 7);
7 | assert_eq!(x.fetch_update(|x| x + 1), 8);
  |            ^ cannot borrow as mutable
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0596`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/mod.rs - num::i128::fetch_xor (line 122) stdout ----
error[E0596]: cannot borrow `foo` as mutable, as it is not declared as mutable
  |
  |
5 | let foo: i128 = 0b101101;
  |     --- help: consider changing this to be mutable: `mut foo`
6 | assert_eq!(foo.fetch_xor(0b110011), 0b101101);
  |            ^^^ cannot borrow as mutable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0596`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/mod.rs - num::i16::fetch_add (line 103) stdout ----
error[E0596]: cannot borrow `foo` as mutable, as it is not declared as mutable
  |
5 | let foo: i16 = 0;
5 | let foo: i16 = 0;
  |     --- help: consider changing this to be mutable: `mut foo`
6 | assert_eq!(foo.fetch_add(10), 0);
  |            ^^^ cannot borrow as mutable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0596`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/mod.rs - num::i16::fetch_and (line 101) stdout ----
error[E0596]: cannot borrow `foo` as mutable, as it is not declared as mutable
  |
  |
5 | let foo: i16 = 0b101101;
  |     --- help: consider changing this to be mutable: `mut foo`
6 | assert_eq!(foo.fetch_and(0b110011), 0b101101);
  |            ^^^ cannot borrow as mutable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0596`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/mod.rs - num::i16::fetch_max (line 101) stdout ----
error[E0596]: cannot borrow `foo` as mutable, as it is not declared as mutable
  |
5 | let foo: i16 = 23;
5 | let foo: i16 = 23;
  |     --- help: consider changing this to be mutable: `mut foo`
6 | assert_eq!(foo.fetch_max(42), 23);
  |            ^^^ cannot borrow as mutable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0596`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/mod.rs - num::i16::fetch_min (line 101) stdout ----
error[E0596]: cannot borrow `foo` as mutable, as it is not declared as mutable
  |
5 | let foo: i16 = 23;
5 | let foo: i16 = 23;
  |     --- help: consider changing this to be mutable: `mut foo`
6 | assert_eq!(foo.fetch_min(42), 23);
  |            ^^^ cannot borrow as mutable

error[E0596]: cannot borrow `foo` as mutable, as it is not declared as mutable
  |
5 | let foo: i16 = 23;
5 | let foo: i16 = 23;
  |     --- help: consider changing this to be mutable: `mut foo`
...
8 | assert_eq!(foo.fetch_min(22), 23);
  |            ^^^ cannot borrow as mutable
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0596`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/mod.rs - num::i16::fetch_nand (line 101) stdout ----
error[E0596]: cannot borrow `foo` as mutable, as it is not declared as mutable
  |
  |
5 | let foo: i16 = 0x13;
  |     --- help: consider changing this to be mutable: `mut foo`
6 | assert_eq!(foo.fetch_nand(0x31), 0x13);
  |            ^^^ cannot borrow as mutable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0596`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/mod.rs - num::i16::fetch_or (line 101) stdout ----
error[E0596]: cannot borrow `foo` as mutable, as it is not declared as mutable
  |
  |
5 | let foo: i16 = 0b101101;
  |     --- help: consider changing this to be mutable: `mut foo`
6 | assert_eq!(foo.fetch_or(0b110011), 0b101101);
  |            ^^^ cannot borrow as mutable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0596`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/mod.rs - num::i16::fetch_sub (line 103) stdout ----
error[E0596]: cannot borrow `foo` as mutable, as it is not declared as mutable
  |
5 | let foo: i16 = 20;
5 | let foo: i16 = 20;
  |     --- help: consider changing this to be mutable: `mut foo`
6 | assert_eq!(foo.fetch_sub(10), 20);
  |            ^^^ cannot borrow as mutable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0596`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/mod.rs - num::i16::fetch_xor (line 101) stdout ----
error[E0596]: cannot borrow `foo` as mutable, as it is not declared as mutable
  |
  |
5 | let foo: i16 = 0b101101;
  |     --- help: consider changing this to be mutable: `mut foo`
6 | assert_eq!(foo.fetch_xor(0b110011), 0b101101);
  |            ^^^ cannot borrow as mutable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0596`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/mod.rs - num::i16::fetch_update (line 101) stdout ----
error[E0596]: cannot borrow `x` as mutable, as it is not declared as mutable
  |
5 | let x: i16 = 7;
5 | let x: i16 = 7;
  |     - help: consider changing this to be mutable: `mut x`
6 | assert_eq!(x.fetch_update(|x| x + 1), 7);
  |            ^ cannot borrow as mutable

error[E0596]: cannot borrow `x` as mutable, as it is not declared as mutable
  |
5 | let x: i16 = 7;
5 | let x: i16 = 7;
  |     - help: consider changing this to be mutable: `mut x`
6 | assert_eq!(x.fetch_update(|x| x + 1), 7);
7 | assert_eq!(x.fetch_update(|x| x + 1), 8);
  |            ^ cannot borrow as mutable
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0596`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/mod.rs - num::i32::fetch_and (line 107) stdout ----
error[E0596]: cannot borrow `foo` as mutable, as it is not declared as mutable
  |
  |
5 | let foo: i32 = 0b101101;
  |     --- help: consider changing this to be mutable: `mut foo`
6 | assert_eq!(foo.fetch_and(0b110011), 0b101101);
  |            ^^^ cannot borrow as mutable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0596`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/mod.rs - num::i32::fetch_add (line 109) stdout ----
error[E0596]: cannot borrow `foo` as mutable, as it is not declared as mutable
  |
5 | let foo: i32 = 0;
5 | let foo: i32 = 0;
  |     --- help: consider changing this to be mutable: `mut foo`
6 | assert_eq!(foo.fetch_add(10), 0);
  |            ^^^ cannot borrow as mutable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0596`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/mod.rs - num::i32::fetch_max (line 107) stdout ----
error[E0596]: cannot borrow `foo` as mutable, as it is not declared as mutable
  |
5 | let foo: i32 = 23;
5 | let foo: i32 = 23;
  |     --- help: consider changing this to be mutable: `mut foo`
6 | assert_eq!(foo.fetch_max(42), 23);
  |            ^^^ cannot borrow as mutable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0596`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/mod.rs - num::i32::fetch_min (line 107) stdout ----
error[E0596]: cannot borrow `foo` as mutable, as it is not declared as mutable
  |
5 | let foo: i32 = 23;
5 | let foo: i32 = 23;
  |     --- help: consider changing this to be mutable: `mut foo`
6 | assert_eq!(foo.fetch_min(42), 23);
  |            ^^^ cannot borrow as mutable

error[E0596]: cannot borrow `foo` as mutable, as it is not declared as mutable
  |
5 | let foo: i32 = 23;
5 | let foo: i32 = 23;
  |     --- help: consider changing this to be mutable: `mut foo`
...
8 | assert_eq!(foo.fetch_min(22), 23);
  |            ^^^ cannot borrow as mutable
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0596`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/mod.rs - num::i32::fetch_nand (line 107) stdout ----
error[E0596]: cannot borrow `foo` as mutable, as it is not declared as mutable
  |
  |
5 | let foo: i32 = 0x13;
  |     --- help: consider changing this to be mutable: `mut foo`
6 | assert_eq!(foo.fetch_nand(0x31), 0x13);
  |            ^^^ cannot borrow as mutable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0596`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/mod.rs - num::i32::fetch_or (line 107) stdout ----
error[E0596]: cannot borrow `foo` as mutable, as it is not declared as mutable
  |
  |
5 | let foo: i32 = 0b101101;
  |     --- help: consider changing this to be mutable: `mut foo`
6 | assert_eq!(foo.fetch_or(0b110011), 0b101101);
  |            ^^^ cannot borrow as mutable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0596`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/mod.rs - num::i32::fetch_sub (line 109) stdout ----
error[E0596]: cannot borrow `foo` as mutable, as it is not declared as mutable
  |
5 | let foo: i32 = 20;
5 | let foo: i32 = 20;
  |     --- help: consider changing this to be mutable: `mut foo`
6 | assert_eq!(foo.fetch_sub(10), 20);
  |            ^^^ cannot borrow as mutable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0596`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/mod.rs - num::i32::fetch_update (line 107) stdout ----
error[E0596]: cannot borrow `x` as mutable, as it is not declared as mutable
  |
5 | let x: i32 = 7;
5 | let x: i32 = 7;
  |     - help: consider changing this to be mutable: `mut x`
6 | assert_eq!(x.fetch_update(|x| x + 1), 7);
  |            ^ cannot borrow as mutable

error[E0596]: cannot borrow `x` as mutable, as it is not declared as mutable
  |
5 | let x: i32 = 7;
5 | let x: i32 = 7;
  |     - help: consider changing this to be mutable: `mut x`
6 | assert_eq!(x.fetch_update(|x| x + 1), 7);
7 | assert_eq!(x.fetch_update(|x| x + 1), 8);
  |            ^ cannot borrow as mutable
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0596`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/mod.rs - num::i32::fetch_xor (line 107) stdout ----
error[E0596]: cannot borrow `foo` as mutable, as it is not declared as mutable
  |
  |
5 | let foo: i32 = 0b101101;
  |     --- help: consider changing this to be mutable: `mut foo`
6 | assert_eq!(foo.fetch_xor(0b110011), 0b101101);
  |            ^^^ cannot borrow as mutable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0596`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/mod.rs - num::i64::fetch_add (line 116) stdout ----
error[E0596]: cannot borrow `foo` as mutable, as it is not declared as mutable
  |
5 | let foo: i64 = 0;
5 | let foo: i64 = 0;
  |     --- help: consider changing this to be mutable: `mut foo`
6 | assert_eq!(foo.fetch_add(10), 0);
  |            ^^^ cannot borrow as mutable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0596`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/mod.rs - num::i64::fetch_and (line 114) stdout ----
error[E0596]: cannot borrow `foo` as mutable, as it is not declared as mutable
  |
  |
5 | let foo: i64 = 0b101101;
  |     --- help: consider changing this to be mutable: `mut foo`
6 | assert_eq!(foo.fetch_and(0b110011), 0b101101);
  |            ^^^ cannot borrow as mutable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0596`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/mod.rs - num::i64::fetch_max (line 114) stdout ----
error[E0596]: cannot borrow `foo` as mutable, as it is not declared as mutable
  |
5 | let foo: i64 = 23;
5 | let foo: i64 = 23;
  |     --- help: consider changing this to be mutable: `mut foo`
6 | assert_eq!(foo.fetch_max(42), 23);
  |            ^^^ cannot borrow as mutable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0596`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/mod.rs - num::i64::fetch_nand (line 114) stdout ----
error[E0596]: cannot borrow `foo` as mutable, as it is not declared as mutable
  |
  |
5 | let foo: i64 = 0x13;
  |     --- help: consider changing this to be mutable: `mut foo`
6 | assert_eq!(foo.fetch_nand(0x31), 0x13);
  |            ^^^ cannot borrow as mutable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0596`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/mod.rs - num::i64::fetch_min (line 114) stdout ----
error[E0596]: cannot borrow `foo` as mutable, as it is not declared as mutable
  |
5 | let foo: i64 = 23;
5 | let foo: i64 = 23;
  |     --- help: consider changing this to be mutable: `mut foo`
6 | assert_eq!(foo.fetch_min(42), 23);
  |            ^^^ cannot borrow as mutable

error[E0596]: cannot borrow `foo` as mutable, as it is not declared as mutable
  |
5 | let foo: i64 = 23;
5 | let foo: i64 = 23;
  |     --- help: consider changing this to be mutable: `mut foo`
...
8 | assert_eq!(foo.fetch_min(22), 23);
  |            ^^^ cannot borrow as mutable
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0596`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/mod.rs - num::i64::fetch_or (line 114) stdout ----
error[E0596]: cannot borrow `foo` as mutable, as it is not declared as mutable
  |
  |
5 | let foo: i64 = 0b101101;
  |     --- help: consider changing this to be mutable: `mut foo`
6 | assert_eq!(foo.fetch_or(0b110011), 0b101101);
  |            ^^^ cannot borrow as mutable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0596`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/mod.rs - num::i64::fetch_sub (line 116) stdout ----
error[E0596]: cannot borrow `foo` as mutable, as it is not declared as mutable
  |
5 | let foo: i64 = 20;
---
test result: FAILED. 2813 passed; 108 failed; 27 ignored; 0 measured; 0 filtered out; finished in 44.15s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:19:04
