plain
.................................................................................................... 9000/11196
.................................................................................................... 9100/11196
......................................................................................i......i...... 9200/11196
.................................................................................................... 9300/11196
.........................iiiiii..iiiiii.i........................................................... 9400/11196
.................................................................................................... 9600/11196
.................................................................................................... 9700/11196
.................................................................................................... 9800/11196
.................................................................................................... 9900/11196
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 27 tests
iiiiiiiiiiiiiiiiiiiiiiiiiii

 finished in 0.066 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii...........iiii.........i....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.31s

 finished in 2.376 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---

   Doc-tests alloc

running 528 tests
FFFFFFFFFFFFFF.FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF 100/528
FFFFFFFFFFFFFFFFFFFFFFFFFFFFFF.FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF 200/528
FFFFFFFFFFFFFFFFFFFFFFiFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF.FFFFFFFFFFFFFFFFFFFF 300/528
F..FFFFFFFFFFFFFFFFFFFF.FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF.FFFF.FFFFFFFFFFFFFFFFFFFFFFF 400/528
FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF.FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF 500/528
failures:

---- src/boxed.rs - boxed (line 12) stdout ----
error: unnecessary trailing semicolon
error: unnecessary trailing semicolon
 --> src/boxed.rs:15:2
  |
6 | }; _doctest_main_src_boxed_rs_12_0() }
  |  ^ help: remove this semicolon
note: the lint level is defined here
 --> src/boxed.rs:11:9
  |
2 | #![deny(warnings)]
2 | #![deny(warnings)]
  |         ^^^^^^^^
  = note: `#[deny(redundant_semicolons)]` implied by `#[deny(warnings)]`
error: aborting due to previous error

Couldn't compile the test.
---- src/boxed.rs - boxed (line 97) stdout ----
---- src/boxed.rs - boxed (line 97) stdout ----
error: unnecessary trailing semicolon
  --> src/boxed.rs:110:2
   |
16 | }; _doctest_main_src_boxed_rs_97_0() }
   |  ^ help: remove this semicolon
note: the lint level is defined here
  --> src/boxed.rs:96:9
   |
2  | #![deny(warnings)]
2  | #![deny(warnings)]
   |         ^^^^^^^^
   = note: `#[deny(redundant_semicolons)]` implied by `#[deny(warnings)]`
error: aborting due to previous error

Couldn't compile the test.
---- src/boxed.rs - boxed (line 19) stdout ----
---- src/boxed.rs - boxed (line 19) stdout ----
error: unnecessary trailing semicolon
 --> src/boxed.rs:22:2
  |
6 | }; _doctest_main_src_boxed_rs_19_0() }
  |  ^ help: remove this semicolon
note: the lint level is defined here
 --> src/boxed.rs:18:9
  |
2 | #![deny(warnings)]
2 | #![deny(warnings)]
  |         ^^^^^^^^
  = note: `#[deny(redundant_semicolons)]` implied by `#[deny(warnings)]`
error: aborting due to previous error

Couldn't compile the test.
---- src/borrow.rs - borrow::ToOwned::to_owned (line 46) stdout ----
---- src/borrow.rs - borrow::ToOwned::to_owned (line 46) stdout ----
error: unnecessary trailing semicolon
 --> src/borrow.rs:52:2
  |
9 | }; _doctest_main_src_borrow_rs_46_0() }
  |  ^ help: remove this semicolon
note: the lint level is defined here
 --> src/borrow.rs:45:9
  |
2 | #![deny(warnings)]
2 | #![deny(warnings)]
  |         ^^^^^^^^
  = note: `#[deny(redundant_semicolons)]` implied by `#[deny(warnings)]`
error: aborting due to previous error

Couldn't compile the test.
---- src/alloc.rs - alloc::alloc (line 70) stdout ----
---- src/alloc.rs - alloc::alloc (line 70) stdout ----
error: unnecessary trailing semicolon
  --> src/alloc.rs:82:2
   |
15 | }; _doctest_main_src_alloc_rs_70_0() }
   |  ^ help: remove this semicolon
note: the lint level is defined here
  --> src/alloc.rs:69:9
   |
2  | #![deny(warnings)]
2  | #![deny(warnings)]
   |         ^^^^^^^^
   = note: `#[deny(redundant_semicolons)]` implied by `#[deny(warnings)]`
error: aborting due to previous error

Couldn't compile the test.
---- src/borrow.rs - borrow::Cow::into_owned (line 288) stdout ----
---- src/borrow.rs - borrow::Cow::into_owned (line 288) stdout ----
error: unnecessary trailing semicolon
  --> src/borrow.rs:298:2
   |
13 | }; _doctest_main_src_borrow_rs_288_0() }
   |  ^ help: remove this semicolon
note: the lint level is defined here
  --> src/borrow.rs:287:9
   |
2  | #![deny(warnings)]
2  | #![deny(warnings)]
   |         ^^^^^^^^
   = note: `#[deny(redundant_semicolons)]` implied by `#[deny(warnings)]`
error: aborting due to previous error

Couldn't compile the test.
---- src/alloc.rs - alloc::alloc_zeroed (line 140) stdout ----
---- src/alloc.rs - alloc::alloc_zeroed (line 140) stdout ----
error: unnecessary trailing semicolon
  --> src/alloc.rs:151:2
   |
14 | }; _doctest_main_src_alloc_rs_140_0() }
   |  ^ help: remove this semicolon
note: the lint level is defined here
  --> src/alloc.rs:139:9
   |
2  | #![deny(warnings)]
2  | #![deny(warnings)]
   |         ^^^^^^^^
   = note: `#[deny(redundant_semicolons)]` implied by `#[deny(warnings)]`
error: aborting due to previous error

Couldn't compile the test.
---- src/borrow.rs - borrow::Cow::to_mut (line 254) stdout ----
---- src/borrow.rs - borrow::Cow::to_mut (line 254) stdout ----
error: unnecessary trailing semicolon
  --> src/borrow.rs:264:2
   |
13 | }; _doctest_main_src_borrow_rs_254_0() }
   |  ^ help: remove this semicolon
note: the lint level is defined here
  --> src/borrow.rs:253:9
   |
2  | #![deny(warnings)]
2  | #![deny(warnings)]
   |         ^^^^^^^^
   = note: `#[deny(redundant_semicolons)]` implied by `#[deny(warnings)]`
error: aborting due to previous error

Couldn't compile the test.
---- src/borrow.rs - borrow::Cow::is_owned (line 232) stdout ----
---- src/borrow.rs - borrow::Cow::is_owned (line 232) stdout ----
error: unnecessary trailing semicolon
  --> src/borrow.rs:241:2
   |
12 | }; _doctest_main_src_borrow_rs_232_0() }
   |  ^ help: remove this semicolon
note: the lint level is defined here
  --> src/borrow.rs:231:9
   |
2  | #![deny(warnings)]
2  | #![deny(warnings)]
   |         ^^^^^^^^
   = note: `#[deny(redundant_semicolons)]` implied by `#[deny(warnings)]`
error: aborting due to previous error

Couldn't compile the test.
---- src/borrow.rs - borrow::Cow::into_owned (line 302) stdout ----
---- src/borrow.rs - borrow::Cow::into_owned (line 302) stdout ----
error: unnecessary trailing semicolon
  --> src/borrow.rs:312:2
   |
13 | }; _doctest_main_src_borrow_rs_302_0() }
   |  ^ help: remove this semicolon
note: the lint level is defined here
  --> src/borrow.rs:301:9
   |
2  | #![deny(warnings)]
2  | #![deny(warnings)]
   |         ^^^^^^^^
   = note: `#[deny(redundant_semicolons)]` implied by `#[deny(warnings)]`
error: aborting due to previous error

Couldn't compile the test.
---- src/borrow.rs - borrow::ToOwned::clone_into (line 65) stdout ----
---- src/borrow.rs - borrow::ToOwned::clone_into (line 65) stdout ----
error: unnecessary trailing semicolon
  --> src/borrow.rs:72:2
   |
10 | }; _doctest_main_src_borrow_rs_65_0() }
   |  ^ help: remove this semicolon
note: the lint level is defined here
  --> src/borrow.rs:64:9
   |
2  | #![deny(warnings)]
2  | #![deny(warnings)]
   |         ^^^^^^^^
   = note: `#[deny(redundant_semicolons)]` implied by `#[deny(warnings)]`
error: aborting due to previous error

Couldn't compile the test.
---- src/boxed.rs - boxed (line 26) stdout ----
---- src/boxed.rs - boxed (line 26) stdout ----
error: unnecessary trailing semicolon
  --> src/boxed.rs:35:2
   |
12 | }; _doctest_main_src_boxed_rs_26_0() }
   |  ^ help: remove this semicolon
note: the lint level is defined here
  --> src/boxed.rs:25:9
   |
2  | #![deny(warnings)]
2  | #![deny(warnings)]
   |         ^^^^^^^^
   = note: `#[deny(redundant_semicolons)]` implied by `#[deny(warnings)]`
error: aborting due to previous error

Couldn't compile the test.
---- src/borrow.rs - borrow::Cow::is_borrowed (line 209) stdout ----
---- src/borrow.rs - borrow::Cow::is_borrowed (line 209) stdout ----
error: unnecessary trailing semicolon
  --> src/borrow.rs:218:2
   |
12 | }; _doctest_main_src_borrow_rs_209_0() }
   |  ^ help: remove this semicolon
note: the lint level is defined here
  --> src/borrow.rs:208:9
   |
2  | #![deny(warnings)]
2  | #![deny(warnings)]
   |         ^^^^^^^^
   = note: `#[deny(redundant_semicolons)]` implied by `#[deny(warnings)]`
error: aborting due to previous error

Couldn't compile the test.
---- src/borrow.rs - borrow::Cow (line 108) stdout ----
---- src/borrow.rs - borrow::Cow (line 108) stdout ----
error: unnecessary trailing semicolon
  --> src/borrow.rs:134:2
   |
29 | }; _doctest_main_src_borrow_rs_108_0() }
   |  ^ help: remove this semicolon
note: the lint level is defined here
  --> src/borrow.rs:107:9
   |
2  | #![deny(warnings)]
2  | #![deny(warnings)]
   |         ^^^^^^^^
   = note: `#[deny(redundant_semicolons)]` implied by `#[deny(warnings)]`
error: aborting due to previous error

Couldn't compile the test.
---- src/boxed.rs - boxed::Box<T, A>::from_raw_in (line 599) stdout ----
---- src/boxed.rs - boxed::Box<T, A>::from_raw_in (line 599) stdout ----
error: unnecessary trailing semicolon
  --> src/boxed.rs:607:2
   |
11 | }; _doctest_main_src_boxed_rs_599_0() }
   |  ^ help: remove this semicolon
note: the lint level is defined here
  --> src/boxed.rs:598:9
   |
2  | #![deny(warnings)]
2  | #![deny(warnings)]
   |         ^^^^^^^^
   = note: `#[deny(redundant_semicolons)]` implied by `#[deny(warnings)]`
error: aborting due to previous error

Couldn't compile the test.
---- src/boxed.rs - boxed::Box<Any, A>::downcast (line 1158) stdout ----
---- src/boxed.rs - boxed::Box<Any, A>::downcast (line 1158) stdout ----
error: unnecessary trailing semicolon
  --> src/boxed.rs:1170:2
   |
15 | }; _doctest_main_src_boxed_rs_1158_0() }
   |  ^ help: remove this semicolon
note: the lint level is defined here
  --> src/boxed.rs:1157:9
   |
2  | #![deny(warnings)]
2  | #![deny(warnings)]
   |         ^^^^^^^^
   = note: `#[deny(redundant_semicolons)]` implied by `#[deny(warnings)]`
error: aborting due to previous error

Couldn't compile the test.
---- src/boxed.rs - boxed::Box<Any + Send, A>::downcast (line 1190) stdout ----
---- src/boxed.rs - boxed::Box<Any + Send, A>::downcast (line 1190) stdout ----
error: unnecessary trailing semicolon
  --> src/boxed.rs:1202:2
   |
15 | }; _doctest_main_src_boxed_rs_1190_0() }
   |  ^ help: remove this semicolon
note: the lint level is defined here
  --> src/boxed.rs:1189:9
   |
2  | #![deny(warnings)]
2  | #![deny(warnings)]
   |         ^^^^^^^^
   = note: `#[deny(redundant_semicolons)]` implied by `#[deny(warnings)]`
error: aborting due to previous error

Couldn't compile the test.
---- src/boxed.rs - boxed::Box<T, A>::into_raw_with_allocator (line 697) stdout ----
---- src/boxed.rs - boxed::Box<T, A>::into_raw_with_allocator (line 697) stdout ----
error: unnecessary trailing semicolon
  --> src/boxed.rs:705:2
   |
11 | }; _doctest_main_src_boxed_rs_697_0() }
   |  ^ help: remove this semicolon
note: the lint level is defined here
  --> src/boxed.rs:696:9
   |
2  | #![deny(warnings)]
2  | #![deny(warnings)]
   |         ^^^^^^^^
   = note: `#[deny(redundant_semicolons)]` implied by `#[deny(warnings)]`
error: aborting due to previous error

Couldn't compile the test.
---- src/boxed.rs - boxed::Box<T, A>::into_raw (line 652) stdout ----
---- src/boxed.rs - boxed::Box<T, A>::into_raw (line 652) stdout ----
error: unnecessary trailing semicolon
 --> src/boxed.rs:656:2
  |
7 | }; _doctest_main_src_boxed_rs_652_0() }
  |  ^ help: remove this semicolon
note: the lint level is defined here
 --> src/boxed.rs:651:9
  |
2 | #![deny(warnings)]
2 | #![deny(warnings)]
  |         ^^^^^^^^
  = note: `#[deny(redundant_semicolons)]` implied by `#[deny(warnings)]`
error: aborting due to previous error

Couldn't compile the test.
---- src/boxed.rs - boxed::Box<T, A>::into_raw (line 659) stdout ----
---- src/boxed.rs - boxed::Box<T, A>::into_raw (line 659) stdout ----
error: unnecessary trailing semicolon
  --> src/boxed.rs:669:2
   |
13 | }; _doctest_main_src_boxed_rs_659_0() }
   |  ^ help: remove this semicolon
note: the lint level is defined here
  --> src/boxed.rs:658:9
   |
2  | #![deny(warnings)]
2  | #![deny(warnings)]
   |         ^^^^^^^^
   = note: `#[deny(redundant_semicolons)]` implied by `#[deny(warnings)]`
error: aborting due to previous error

Couldn't compile the test.
---- src/borrow.rs - borrow::Cow (line 138) stdout ----
---- src/borrow.rs - borrow::Cow (line 138) stdout ----
error: unnecessary trailing semicolon
  --> src/borrow.rs:169:2
   |
34 | }; _doctest_main_src_borrow_rs_138_0() }
   |  ^ help: remove this semicolon
note: the lint level is defined here
  --> src/borrow.rs:137:9
   |
2  | #![deny(warnings)]
2  | #![deny(warnings)]
   |         ^^^^^^^^
   = note: `#[deny(redundant_semicolons)]` implied by `#[deny(warnings)]`
error: aborting due to previous error

Couldn't compile the test.
---- src/boxed.rs - boxed::Box<T, A>::from_raw_in (line 609) stdout ----
---- src/boxed.rs - boxed::Box<T, A>::from_raw_in (line 609) stdout ----
error: unnecessary trailing semicolon
  --> src/boxed.rs:623:2
   |
17 | }; _doctest_main_src_boxed_rs_609_0().unwrap() }
   |  ^ help: remove this semicolon
note: the lint level is defined here
  --> src/boxed.rs:608:9
   |
2  | #![deny(warnings)]
2  | #![deny(warnings)]
   |         ^^^^^^^^
   = note: `#[deny(redundant_semicolons)]` implied by `#[deny(warnings)]`
error: aborting due to previous error

Couldn't compile the test.
---- src/boxed.rs - boxed::Box<T, A>::clone_from (line 875) stdout ----
---- src/boxed.rs - boxed::Box<T, A>::clone_from (line 875) stdout ----
error: unnecessary trailing semicolon
  --> src/boxed.rs:887:2
   |
15 | }; _doctest_main_src_boxed_rs_875_0() }
   |  ^ help: remove this semicolon
note: the lint level is defined here
  --> src/boxed.rs:874:9
   |
2  | #![deny(warnings)]
2  | #![deny(warnings)]
   |         ^^^^^^^^
   = note: `#[deny(redundant_semicolons)]` implied by `#[deny(warnings)]`
error: aborting due to previous error

Couldn't compile the test.
---- src/boxed.rs - boxed::Box<T, A>::into_raw_with_allocator (line 708) stdout ----
---- src/boxed.rs - boxed::Box<T, A>::into_raw_with_allocator (line 708) stdout ----
error: unnecessary trailing semicolon
  --> src/boxed.rs:721:2
   |
16 | }; _doctest_main_src_boxed_rs_708_0() }
   |  ^ help: remove this semicolon
note: the lint level is defined here
  --> src/boxed.rs:707:9
   |
2  | #![deny(warnings)]
2  | #![deny(warnings)]
   |         ^^^^^^^^
   = note: `#[deny(redundant_semicolons)]` implied by `#[deny(warnings)]`
error: aborting due to previous error

Couldn't compile the test.
---- src/boxed.rs - boxed::Box<T, A>::new_in (line 253) stdout ----
---- src/boxed.rs - boxed::Box<T, A>::new_in (line 253) stdout ----
error: unnecessary trailing semicolon
 --> src/boxed.rs:259:2
  |
9 | }; _doctest_main_src_boxed_rs_253_0() }
  |  ^ help: remove this semicolon
note: the lint level is defined here
 --> src/boxed.rs:252:9
  |
2 | #![deny(warnings)]
2 | #![deny(warnings)]
  |         ^^^^^^^^
  = note: `#[deny(redundant_semicolons)]` implied by `#[deny(warnings)]`
error: aborting due to previous error

Couldn't compile the test.
---- src/boxed.rs - boxed::Box<T, A>::leak (line 779) stdout ----
---- src/boxed.rs - boxed::Box<T, A>::leak (line 779) stdout ----
error: unnecessary trailing semicolon
 --> src/boxed.rs:784:2
  |
8 | }; _doctest_main_src_boxed_rs_779_0() }
  |  ^ help: remove this semicolon
note: the lint level is defined here
 --> src/boxed.rs:778:9
  |
2 | #![deny(warnings)]
2 | #![deny(warnings)]
  |         ^^^^^^^^
  = note: `#[deny(redundant_semicolons)]` implied by `#[deny(warnings)]`
error: aborting due to previous error

Couldn't compile the test.
---- src/boxed.rs - boxed::Box<T>::new (line 180) stdout ----
---- src/boxed.rs - boxed::Box<T>::new (line 180) stdout ----
error: unnecessary trailing semicolon
 --> src/boxed.rs:182:2
  |
5 | }; _doctest_main_src_boxed_rs_180_0() }
  |  ^ help: remove this semicolon
note: the lint level is defined here
 --> src/boxed.rs:179:9
  |
2 | #![deny(warnings)]
2 | #![deny(warnings)]
  |         ^^^^^^^^
  = note: `#[deny(redundant_semicolons)]` implied by `#[deny(warnings)]`
error: aborting due to previous error

Couldn't compile the test.
---- src/boxed.rs - boxed::Box<T>::from_raw (line 551) stdout ----
---- src/boxed.rs - boxed::Box<T>::from_raw (line 551) stdout ----
error: unnecessary trailing semicolon
 --> src/boxed.rs:555:2
  |
7 | }; _doctest_main_src_boxed_rs_551_0() }
  |  ^ help: remove this semicolon
note: the lint level is defined here
 --> src/boxed.rs:550:9
  |
2 | #![deny(warnings)]
2 | #![deny(warnings)]
  |         ^^^^^^^^
  = note: `#[deny(redundant_semicolons)]` implied by `#[deny(warnings)]`
error: aborting due to previous error

Couldn't compile the test.
---- src/boxed.rs - boxed::Box<T>::from_raw (line 557) stdout ----
---- src/boxed.rs - boxed::Box<T>::from_raw (line 557) stdout ----
error: unnecessary trailing semicolon
  --> src/boxed.rs:568:2
   |
14 | }; _doctest_main_src_boxed_rs_557_0() }
   |  ^ help: remove this semicolon
note: the lint level is defined here
  --> src/boxed.rs:556:9
   |
2  | #![deny(warnings)]
2  | #![deny(warnings)]
   |         ^^^^^^^^
---
test result: FAILED. 9 passed; 518 failed; 1 ignored; 0 measured; 0 filtered out; finished in 2.60s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "alloc" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:21:06
