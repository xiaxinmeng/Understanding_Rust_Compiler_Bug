plain
[00:44:17] ..........i..............................................................................i..........
[00:44:21] ....................................................................................................
[00:44:27] ....................................................................................................
[00:44:33] ....................................................................................................
[00:44:38] ......................i.................iiiiiiiii...................................................
[00:44:38] test result: ok. 1486 passed; 0 failed; 14 ignored; 0 measured; 0 filtered out
[00:44:38] 
[00:44:38] travis_fold:start:test_ui_nll
travis_time:start:test_ui_nll
---
[00:45:27] ..........i..............................................................................i..........
[00:45:32] ....................................................................................................
[00:45:37] ....................................................................................................
[00:45:42] ....................................................................................................
[00:45:47] ......................i.................iiiiiiiii...................................................
[00:45:47] test result: ok. 1486 passed; 0 failed; 14 ignored; 0 measured; 0 filtered out
[00:45:47] 
[00:45:47]  finished in 69.082
[00:45:47] travis_fold:end:test_ui_nll
---
[00:45:47] 
[00:45:47] running 3022 tests
[00:46:01] ....................................................................................................
[00:46:15] .............................................i......................................................
[00:46:31] ........................................................................F...........................
[00:46:45] ..........................................................................F....................F....
[00:47:17] ....................................................................................................
[00:47:30] ....................................................................................................
[00:47:42] ....................................................................................................
[00:48:00] ....................................................................................................
---
[00:51:49] ....................................................................................................
[00:51:51] ........test [run-pass] run-pass/mir_heavy_promoted.rs has been running for over 60 seconds
[00:52:04] ............................................................................................
[00:52:40] ......................ii...........................................................i....ii..........
[00:53:02] ..........................................i.ii............F.........................................
[00:53:31] ..................................................iiiiiii...........................................
[00:54:05] ....................................................................................................
[00:54:25] ....................................................................................................
[00:55:22] ......................
[00:55:22] failures:
[00:55:22] failures:
[00:55:22] 
[00:55:22] ---- [run-pass] run-pass/check-static-recursion-foreign.rs stdout ----
[00:55:22] 
[00:55:22] error: compilation failed!
[00:55:22] status: exit code: 101
[00:55:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/check-static-recursion-foreign.rs" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/check-static-recursion-foreign/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/check-static-recursion-foreign/auxiliary"
[00:55:22] ------------------------------------------
[00:55:22] 
[00:55:22] ------------------------------------------
[00:55:22] stderr:
[00:55:22] stderr:
[00:55:22] ------------------------------------------
[00:55:22] warning: static item is never used: `B`
[00:55:22]   --> /checkout/src/test/run-pass/check-static-recursion-foreign.rs:31:1
[00:55:22]    |
[00:55:22] 31 | static B: &'static c_int = unsafe { &test_static };
[00:55:22]    |
[00:55:22]    = note: #[warn(dead_code)] on by default
[00:55:22] 
[00:55:22] 
[00:55:22] error: internal compiler error: librustc_typeck/check/mod.rs:829: can't type-check body of DefId(0/1:9 ~ check_static_recursion_foreign[317d]::[0]::test_static[0])
[00:55:22]    |
[00:55:22]    |
[00:55:22] 28 |     static test_static: c_int;
[00:55:22] 
[00:55:22] thread 'main' panicked at 'Box<Any>', librustc_errors/lib.rs:499:9
[00:55:22] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:55:22] error: aborting due to previous error
[00:55:22] error: aborting due to previous error
[00:55:22] 
[00:55:22] 
[00:55:22] note: the compiler unexpectedly panicked. this is a bug.
[00:55:22] 
[00:55:22] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:55:22] note: rustc 1.28.0-dev running on x86_64-unknown-linux-gnu
[00:55:22] 
[00:55:22] 
[00:55:22] note: compiler flags: -Z unstable-options -C prefer-dynamic -C rpath
[00:55:22] 
[00:55:22] ------------------------------------------
[00:55:22] 
[00:55:22] thread '[run-pass] run-pass/check-static-recursion-foreign.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3096:9
[00:55:22] thread '[run-pass] run-pass/check-static-recursion-foreign.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3096:9
[00:55:22] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:55:22] 
[00:55:22] ---- [run-pass] run-pass/const-cast.rs stdout ----
[00:55:22] 
[00:55:22] error: compilation failed!
[00:55:22] status: exit code: 101
[00:55:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/const-cast.rs" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/const-cast/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/const-cast/auxiliary"
[00:55:22] ------------------------------------------
[00:55:22] 
[00:55:22] ------------------------------------------
[00:55:22] stderr:
[00:55:22] stderr:
[00:55:22] ------------------------------------------
[00:55:22] warning: constant `x` should have an upper case name such as `X`
[00:55:22]    |
[00:55:22]    |
[00:55:22] 18 | const x: extern "C" fn() = foo;
[00:55:22]    |
[00:55:22]    = note: #[warn(non_upper_case_globals)] on by default
[00:55:22] 
[00:55:22] 
[00:55:22] warning: static variable `y` should have an upper case name such as `Y`
[00:55:22]    |
[00:55:22]    |
[00:55:22] 19 | static y: TestStruct = TestStruct { x: x as *const u8 };
[00:55:22] 
[00:55:22] 
[00:55:22] error: this static likely exhibits undefined behavior. The rules on what exactly is undefined behavior aren't clear, so this check might be ovezealous.
[00:55:22]    |
[00:55:22]    |
[00:55:22] 19 | static y: TestStruct = TestStruct { x: x as *const u8 };
[00:55:22]    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ tried to dereference a function pointer
[00:55:22]    = note: #[deny(const_err)] on by default
[00:55:22] 
[00:55:22] error: aborting due to previous error
[00:55:22] 
---
[00:55:22] ---- [run-pass] run-pass/const-err.rs stdout ----
[00:55:22] 
[00:55:22] error: compilation failed!
[00:55:22] status: exit code: 101
[00:55:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/const-err.rs" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/const-err/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/const-err/auxiliary"
[00:55:22] ------------------------------------------
[00:55:22] 
[00:55:22] ------------------------------------------
[00:55:22] stderr:
[00:55:22] stderr:
[00:55:22] ------------------------------------------
[00:55:22] warning: constant item is never used: `X`
[00:55:22]   --> /checkout/src/test/run-pass/const-err.rs:15:1
[00:55:22]    |
[00:55:22] 15 | const X: *const u8 = b"" as _;
[00:55:22]    |
[00:55:22]    = note: #[warn(dead_code)] on by default
[00:55:22] 
[00:55:22] warning: constant item is never used: `Y`
[00:55:22] warning: constant item is never used: `Y`
[00:55:22]   --> /checkout/src/test/run-pass/const-err.rs:16:1
[00:55:22]    |
[00:55:22] 16 | const Y: bool = 'A' == 'B';
[00:55:22] 
[00:55:22] warning: constant item is never used: `Z`
[00:55:22]   --> /checkout/src/test/run-pass/const-err.rs:17:1
[00:55:22]    |
[00:55:22]    |
[00:55:22] 17 | const Z: char = 'A';
[00:55:22] 
[00:55:22] warning: constant item is never used: `W`
[00:55:22]   --> /checkout/src/test/run-pass/const-err.rs:18:1
[00:55:22]    |
[00:55:22]    |
[00:55:22] 18 | const W: bool = Z <= 'B';
[00:55:22] 
[00:55:22] 
[00:55:22] error: this constant likely exhibits undefined behavior. The rules on what exactly is undefined behavior aren't clear, so this check might be ovezealous.
[00:55:22]    |
[00:55:22]    |
[00:55:22] 15 | const X: *const u8 = b"" as _;
[00:55:22]    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ memory access at offset 1, outside bounds of allocation 0 which has size 0
[00:55:22] note: lint level defined here
[00:55:22]   --> /checkout/src/test/run-pass/const-err.rs:13:9
[00:55:22]    |
[00:55:22] 13 | #![deny(const_err)]
---
[00:55:22] ---- [run-pass] run-pass/static-recursive.rs stdout ----
[00:55:22] 
[00:55:22] error: compilation failed!
[00:55:22] status: exit code: 101
[00:55:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/static-recursive.rs" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/static-recursive/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/static-recursive/auxiliary"
[00:55:22] ------------------------------------------
[00:55:22] 
[00:55:22] ------------------------------------------
[00:55:22] stderr:
[00:55:22] stderr:
[00:55:22] ------------------------------------------
[00:55:22] error: this static likely exhibits undefined behavior. The rules on what exactly is undefined behavior aren't clear, so this check might be ovezealous.
[00:55:22]    |
[00:55:22]    |
[00:55:22] 11 | static mut S: *const u8 = unsafe { &S as *const *const u8 as *const u8 };
[00:55:22]    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ a raw memory access tried to access part of a pointer value as raw bytes
[00:55:22]    = note: #[deny(const_err)] on by default
[00:55:22] 
[00:55:22] error: aborting due to previous error
[00:55:22] 
---
[00:55:22]     [run-pass] run-pass/check-static-recursion-foreign.rs
[00:55:22]     [run-pass] run-pass/const-cast.rs
[00:55:22]  :22] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:55:22] Build completed unsuccessfully in 0:13:19
[00:55:22] Makefile:58: recipe for target 'check' failed
[00:55:22] make: *** [check] Error 1
travis_time:end:08f1e9e4:start=1528372103868045071,finish=1528372104412368901,duration=544323830

The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
