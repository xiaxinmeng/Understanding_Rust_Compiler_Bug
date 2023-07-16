plain
[00:50:10] .................................................................................................... 2200/4601
[00:50:15] ..................i................................................................................. 2300/4601
[00:50:18] .................................................................................................... 2400/4601
[00:50:22] .................................................................................................... 2500/4601
[00:50:25] ...............................iiiiiiiii............................................................ 2600/4601
[00:50:32] .................................................................................................... 2800/4601
[00:50:36] .................................................................................................... 2900/4601
[00:50:39] ......................................................i............................................. 3000/4601
[00:50:41] .................................................................................................... 3100/4601
---
[00:54:09] .................................................................................................... 1500/2869
[00:54:21] .........................................i.......................................................... 1600/2869
[00:54:35] .................................................................................................... 1700/2869
[00:54:47] .................................................................................................... 1800/2869
[00:54:57] ....................F..............................................i................................ 1900/2869
[00:55:30] .................................................................................................... 2100/2869
[00:55:37] ...................................................................................................i 2200/2869
[00:55:53] i.....................................................................i....i........................ 2300/2869
[00:56:07] .............i...................................................................................... 2400/2869
---
[00:57:14] 
[00:57:14] ------------------------------------------
[00:57:14] stderr:
[00:57:14] ------------------------------------------
[00:57:14] {"message":"librustc_mir/dataflow/move_paths/builder.rs:302: SetDiscriminant should not exist during borrowck","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/libcore/iter/iterator.rs","byte_start":56708,"byte_end":56731,"line_start":1682,"line_end":1682,"column_start":15,"column_end":38,"is_primary":true,"text":[{"text":"        }) == LoopState::Continue(())","highlight_start":15,"highlight_end":38}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: internal compiler error: librustc_mir/dataflow/move_paths/builder.rs:302: SetDiscriminant should not exist during borrowck\n  --> /checkout/src/libcore/iter/iterator.rs:1682:15\n   |\nLL |         }) == LoopState::Continue(())\n   |               ^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:57:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:57:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:57:14] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:57:14] note: the compiler unexpectedly panicked. this is a bug.
[00:57:14] 
[00:57:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:57:14] 
[00:57:14] 
[00:57:14] note: rustc 1.31.0-dev running on x86_64-unknown-linux-gnu
[00:57:14] 
[00:57:14] note: compiler flags: -Z ui-testing -Z unstable-options -Z mir-opt-level=3 -C prefer-dynamic -C rpath
[00:57:14] 
[00:57:14] ------------------------------------------
[00:57:14] 
[00:57:14] thread '[run-pass] run-pass/mir/mir-inlining/ice-issue-50411.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3277:9
---
[00:57:14] 
[00:57:14] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:501:22
[00:57:14] 
[00:57:14] 
[00:57:14] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath 

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:10fcb2fa
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
