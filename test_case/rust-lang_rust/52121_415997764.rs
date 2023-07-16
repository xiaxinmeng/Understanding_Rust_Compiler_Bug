plain
[00:52:41] ....................................................................................................
[00:52:44] ....................................................................................................
[00:52:47] ....i...............................................................................................
[00:52:50] ....................................................................................................
[00:52:53] .....................................................iiiiiiiii......................................
[00:52:59] ....................................................................................................
[00:53:03] ....................................................................................................
[00:53:06] ..................................i.................................................................
[00:53:10] ....................................................................................i.i..ii.........
---
travis_time:start:test_run-pass-fulldeps
Check compiletest suite=run-pass-fulldeps mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:01:36] 
[01:01:36] running 97 tests
[01:03:30] ......................................F.......................test [run-pass] run-pass-fulldeps/myriad-closures.rs has been running for over 60 seconds
[01:04:59] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[01:04:59] failures:
[01:04:59] 
[01:04:59] ---- [run-pass] run-pass-fulldeps/macros2.rs stdout ----
[01:04:59] 
[01:04:59] 
[01:04:59] error: auxiliary build of "/checkout/src/test/run-pass-fulldeps/auxiliary/macros2.rs" failed to compile: 
[01:04:59] status: exit code: 1
[01:04:59] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/auxiliary/macros2.rs" "--target=x86_64-unknown-linux-gnu" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/macros2/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/macros2/auxiliary"
[01:04:59] ------------------------------------------
[01:04:59] 
[01:04:59] ------------------------------------------
[01:04:59] stderr:
[01:04:59] stderr:
[01:04:59] ------------------------------------------
[01:04:59] warning: unused imports: `Span`, `quote`
[01:04:59]   --> /checkout/src/test/run-pass-fulldeps/auxiliary/macros2.rs:16:18
[01:04:59]    |
[01:04:59] 16 | use proc_macro::{quote, Span, TokenStream};
[01:04:59]    |                  ^^^^^  ^^^^
[01:04:59]    = note: #[warn(unused_imports)] on by default
[01:04:59] 
[01:04:59] 
[01:04:59] error[E0658]: use of unstable library feature 'proc_macro_quote' (see issue #38356)
[01:04:59]   --> /checkout/src/test/run-pass-fulldeps/auxiliary/macros2.rs:16:18
[01:04:59]    |
[01:04:59] 16 | use proc_macro::{quote, Span, TokenStream};
[01:04:59]    |
[01:04:59]    = help: add #![feature(proc_macro_quote)] to the crate attributes to enable
[01:04:59] 
[01:04:59] error: aborting due to previous error
