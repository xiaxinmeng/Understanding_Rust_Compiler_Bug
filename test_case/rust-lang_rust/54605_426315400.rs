plain
[00:45:16] .................................................................................................... 500/4319
[00:45:20] .......................i............................................................................ 600/4319
[00:45:25] .................................................................................................... 700/4319
[00:45:29] ................................i...........i....................................................... 800/4319
[00:45:32] .......................F...........................iiiii..........F................................. 900/4319
[00:45:36] .................................................................................................... 1000/4319
[00:45:38] ..........................................................F......................................... 1100/4319
[00:45:40] .................F.................................................................................. 1200/4319
[00:45:43] .......................................F............................................................ 1300/4319
[00:45:45] ......................................F............................................................. 1400/4319
[00:45:48] .....................F.....................i..........................................F.F........... 1500/4319
[00:45:52] ..........i......................................................................................... 1600/4319
[00:45:55] .F...FFF..F.F.F...FFF.......F.................F..........................................F.......... 1700/4319
[00:45:57] ...........................................F........................................................ 1800/4319
[00:46:00] ..i................................F................................................................ 1900/4319
[00:46:03] .................................................................................................... 2000/4319
[00:46:06] ............................................................................F....................... 2100/4319
[00:46:09] ............................................F........F............................................F. 2200/4319
[00:46:12] .................F......................F........................................................... 2300/4319
[00:46:18] .................................................................................................... 2500/4319
[00:46:21] .................................................................................................... 2600/4319
[00:46:25] .................................................................................................... 2700/4319
[00:46:27] .i.................................................................................................. 2800/4319
[00:46:27] .i.................................................................................................. 2800/4319
[00:46:30] .............................................................i.i..ii................................ 2900/4319
[00:46:34] .................................................................................................... 3000/4319
[00:46:37] ...............................................................................................i.... 3100/4319
[00:46:40] .................................................................................................... 3200/4319
[00:46:43] ....................FFF..FF............................F............................................ 3300/4319
[00:46:46] .................................................................................................... 3400/4319
[00:46:49] ......................................................................F............................. 3500/4319
[00:46:52] ....F................................i...............................F.FFFF....F.................... 3600/4319
[00:46:57] .................................F........F........F.........F.....F................................ 3700/4319
[00:47:00] .............................................................................F...................... 3800/4319
[00:47:04] .....................................F.............................................................. 3900/4319
[00:47:10] .................................................................................................... 4100/4319
[00:47:10] .................................................................................................... 4100/4319
[00:47:13] ...........................................................FF......................................F 4200/4319
[00:47:15] .F.FFFFF..F...............................................i......................................... 4300/4319
[00:47:16] failures:
[00:47:16] 
[00:47:16] ---- [ui] ui/did_you_mean/issue-38054-do-not-show-unresolved-names.rs stdout ----
[00:47:16] 
[00:47:16] 
[00:47:16] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:47:16] status: exit code: 101
[00:47:16] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/did_you_mean/issue-38054-do-not-show-unresolved-names.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-38054-do-not-show-unresolved-names/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-38054-do-not-show-unresolved-names/auxiliary" "-A" "unused"
[00:47:16] ------------------------------------------
[00:47:16] 
[00:47:16] ------------------------------------------
[00:47:16] stderr:
[00:47:16] stderr:
[00:47:16] ------------------------------------------
[00:47:16] thread 'main' panicked at 'librustc_resolve/lib.rs:4613: parent module is reset for binding', librustc/util/bug.rs:47:26
[00:47:16] 
[00:47:16] error: internal compiler error: unexpected panic
[00:47:16] 
[00:47:16] note: the compiler unexpectedly panicked. this is a bug.
[00:47:16] note: the compiler unexpectedly panicked. this is a bug.
[00:47:16] 
[00:47:16] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:47:16] 
[00:47:16] note: rustc 1.31.0-dev running on x86_64-unknown-linux-gnu
[00:47:16] 
[00:47:16] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[00:47:16] 
[00:47:16] ------------------------------------------
[00:47:16] 
[00:47:16] thread '[ui] ui/did_you_mean/issue-38054-do-not-show-unresolved-names.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3267:9
[00:47:16] thread '[ui] ui/did_you_mean/issue-38054-do-not-show-unresolved-names.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3267:9
[00:47:16] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:47:16] 
[00:47:16] ---- [ui] ui/dollar-crate/dollar-crate-is-keyword-2.rs stdout ----
[00:47:16] 
[00:47:16] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:47:16] status: exit code: 101
[00:47:16] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dollar-crate/dollar-crate-is-keyword-2.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dollar-crate/dollar-crate-is-keyword-2/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dollar-crate/dollar-crate-is-keyword-2/auxiliary" "-A" "unused"
[00:47:16] ------------------------------------------
[00:47:16] 
[00:47:16] ------------------------------------------
[00:47:16] stderr:
[00:47:16] stderr:
[00:47:16] ------------------------------------------
[00:47:16] thread 'main' panicked at 'librustc_resolve/lib.rs:4613: parent module is reset for binding', librustc/util/bug.rs:47:26
[00:47:16] 
[00:47:16] error: internal compiler error: unexpected panic
[00:47:16] 
[00:47:16] note: the compiler unexpectedly panicked. this is a bug.
[00:47:16] note: the compiler unexpectedly panicked. this is a bug.
[00:47:16] 
[00:47:16] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:47:16] 
[00:47:16] note: rustc 1.31.0-dev running on x86_64-unknown-linux-gnu
[00:47:16] 
[00:47:16] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[00:47:16] 
[00:47:16] ------------------------------------------
[00:47:16] 
[00:47:16] thread '[ui] ui/dollar-crate/dollar-crate-is-keyword-2.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3267:9
[00:47:16] thread '[ui] ui/dollar-crate/dollar-crate-is-keyword-2.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3267:9
[00:47:16] 
[00:47:16] ---- [ui] ui/error-codes/E0253.rs stdout ----
[00:47:16] 
[00:47:16] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:47:16] status: exit code: 101
[00:47:16] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0253.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0253/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0253/auxiliary" "-A" "unused"
[00:47:16] ------------------------------------------
[00:47:16] 
[00:47:16] ------------------------------------------
[00:47:16] stderr:
[00:47:16] stderr:
[00:47:16] ------------------------------------------
[00:47:16] {"message":"`do_something` is not directly importable","code":{"code":"E0253","explanation":"\nAttempt was made to import an unimportable value. This can happen when trying\nto import a method from a trait.\n\nErroneous code example:\n\n