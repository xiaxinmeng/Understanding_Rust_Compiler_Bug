plain
[00:46:58]    Compiling std v0.0.0 (file:///checkout/src/libstd)
[00:47:16] warning: `[T]` cannot be resolved, ignoring it...
[00:47:16]   --> libcore/ops/unsize.rs:89:9
[00:47:16]    |
[00:47:16] 89 | /// - &[T] is CoerceSized<&[T; N]> for any N
[00:47:16]    |
[00:47:16]    = note: #[warn(intra_doc_link_resolution_failure)] on by default
[00:47:16]    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[00:47:16] 
[00:47:16] 
[00:47:16] warning: `[T]` cannot be resolved, ignoring it...
[00:47:16]   --> libcore/ops/unsize.rs:89:9
[00:47:16]    |
[00:47:16] 89 | /// - &[T] is CoerceSized<&[T; N]> for any N
[00:47:16]    |
[00:47:16]    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[00:47:16] 
[00:47:37]     Checking compiler_builtins v0.0.0 (file:///checkout/src/rustc/compiler_builtins_shim)
---
[00:47:40]     Checking panic_abort v0.0.0 (file:///checkout/src/libpanic_abort)
[00:47:42] warning: `[T]` cannot be resolved, ignoring it...
[00:47:42]   --> /checkout/src/libcore/ops/unsize.rs:89:9
[00:47:42]    |
[00:47:42] 89 | /// - &[T] is CoerceSized<&[T; N]> for any N
[00:47:42]    |
[00:47:42]    = note: #[warn(intra_doc_link_resolution_failure)] on by default
[00:47:42]    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[00:47:42] 
---
[00:47:51] 
[00:47:51] warning: `[T]` cannot be resolved, ignoring it...
[00:47:51]   --> /checkout/src/libcore/ops/unsize.rs:89:9
[00:47:51]    |
[00:47:51] 89 | /// - &[T] is CoerceSized<&[T; N]> for any N
[00:47:51]    |
[00:47:51]    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[00:47:51] 
[00:47:51] warning: `[Weak::upgrade]` cannot be resolved, ignoring it...
---
[00:57:48] ....................................................................................................
[00:57:51] .......................................................i............................................
[00:57:54] ....................................................................................................
[00:57:57] ....................................................................................................
[00:58:00] ..iiiiiiiii.........................................................................................
[00:58:06] ....................................................................................................
[00:58:10] .....................................................................................i..............
[00:58:12] ....................................................................................................
[00:58:16] ........................................i.i..ii.....................................................
---
[01:03:36] .......................................................................................i............
[01:03:39] ....................................................................................................
[01:03:43] ....................................................................................................
[01:03:45] ....................................................................................................
[01:03:48] ..i.ii.ii.ii.............................i..........................................................
[01:03:48] test result: ok. 6714 passed; 0 failed; 88 ignored; 0 measured; 0 filtered out
[01:03:48] 
[01:03:48]  finished in 421.992
[01:03:48] travis_fold:end:test_ui_nll
---
travis_time:start:test_run-pass
Check compiletest suite=run-pass mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:03:48] 
[01:03:48] running 551 tests
[01:04:03] ........F.................................i.........................................................
[01:04:30] ....................................................................................................
[01:04:40] ....................................................................................................
[01:04:58] ..................i.................................................................................
[01:05:05] ...................................................
[01:05:05] ...................................................
[01:05:05] failures:
[01:05:05] 
[01:05:05] ---- [run-pass] run-pass/arbitrary_self_types_stdlib_pointers.rs stdout ----
[01:05:05] 
[01:05:05] error: compilation failed!
[01:05:05] status: exit code: 1
[01:05:05] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/arbitrary_self_types_stdlib_pointers.rs" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/arbitrary_self_types_stdlib_pointers/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/arbitrary_self_types_stdlib_pointers/auxiliary"
[01:05:05] ------------------------------------------
[01:05:05] 
[01:05:05] ------------------------------------------
[01:05:05] stderr:
[01:05:05] stderr:
[01:05:05] ------------------------------------------
[01:05:05] error[E0282]: type annotations needed
[01:05:05]   --> /checkout/src/test/run-pass/arbitrary_self_types_stdlib_pointers.rs:54:19
[01:05:05]    |
[01:05:05] 54 |     let pin_box = Box::new(4i64).into() as Pin<Box<dyn Trait>>;
[01:05:05]    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot infer type for `T`
[01:05:05]    = note: type must be known at this point
[01:05:05] 
[01:05:05] error: aborting due to previous error
[01:05:05] 
[01:05:05] 
[01:05:05] For more information about this error, try `rustc --explain E0282`.
[01:05:05] 
[01:05:05] ------------------------------------------
[01:05:05] 
[01:05:05] thread '[run-pass] run-pass/arbitrary_self_types_stdlib_pointers.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3205:9
[01:05:05] 
[01:05:05] 
[01:05:05] failures:
[01:05:05]     [run-pass] run-pass/arbitrary_self_types_stdlib_pointers.rs
[01:05:05]     [run-pass] run-pass/arbitrary_self_types_stdlib_pointers.rs
[01:05:05] 
[01:05:05] test result: FAILED. 548 passed; 1 failed; 2 ignored; 0 measured; 0 filtered out
[01:05:05] 
[01:05:05] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:496:22
[01:05:05] 
[01:05:05] 
[01:05:05] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/check

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2270ea6e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
