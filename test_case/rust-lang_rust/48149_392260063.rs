plain
[00:43:35] ....................................................................................................
[00:43:39] ....................................................................................................
[00:43:42] ....................................................................................................
[00:43:45] ....................................................................................................
[00:43:49] ........................................................F...........................................
[00:43:58] ....................................................................................................
[00:44:03] .................................................................................i..................
[00:44:03] .................................................................................i..................
[00:44:08] ........................................F..F..............i.........................................
[00:44:17] ....................................................................................................
[00:44:24] .....................................................................................i..............
[00:44:24] .....................................................................................i..............
[00:44:26] ...iiiiiiiii...................................................
[00:44:26] 
[00:44:26] ---- [ui] ui/impl-trait/region-escape-via-bound-contravariant-closure.rs stdout ----
[00:44:26] 
[00:44:26] 
[00:44:26] error: test compilation failed although it shouldn't!
[00:44:26] status: exit code: 101
[00:44:26] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/region-escape-via-bound-contravariant-closure.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/region-escape-via-bound-contravariant-closure/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/region-escape-via-bound-contravariant-closure/auxiliary" "-A" "unused"
[00:44:26] ------------------------------------------
[00:44:26] 
[00:44:26] ------------------------------------------
[00:44:26] stderr:
[00:44:26] stderr:
[00:44:26] ------------------------------------------
[00:44:26] {"message":"free region `'x` does not outlive free region `'static`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/impl-trait/region-escape-via-bound-contravariant-closure.rs","byte_start":942,"byte_end":951,"line_start":27,"line_end":27,"column_start":5,"column_end":14,"is_primary":true,"text":[{"text":"    move || x","highlight_start":5,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: free region `'x` does not outlive free region `'static`\n  --> /checkout/src/test/ui/impl-trait/region-escape-via-bound-contravariant-closure.rs:27:5\n   |\nLL |     move || x\n   |     ^^^^^^^^^\n\n"}
[00:44:26] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:44:26] ------------------------------------------
[00:44:26] 
[00:44:26] thread '[ui] ui/impl-trait/region-escape-via-bound-contravariant-closure.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3053:9
[00:44:26] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:44:26] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:44:26] 
[00:44:26] ---- [ui] ui/nll/ty-outlives/impl-trait-captures.rs stdout ----
[00:44:26] diff of stderr:
[00:44:26] 
[00:44:26] - warning: not reporting region error due to nll
[00:44:26] -   --> $DIR/impl-trait-capturenu/test/ui/nll/ty-outlives/impl-trait-captures/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/impl-trait-captures/auxiliary" "-A" "unused"
[00:44:26] ------------------------------------------
[00:44:26] 
[00:44:26] ------------------------------------------
[00:44:26] stderr:
[00:44:26] stderr:
[00:44:26] ------------------------------------------
[00:44:26] {"message":"librustc/ty/subst.rs:425: Region parameter out of range when substituting in region 'a (root type=None) (index=1)","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/ui/nll/ty-outlives/impl-trait-captures.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Copyright 2016 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: internal compiler error: librustc/ty/subst.rs:425: Region parameter out of range when substituting in region 'a (root type=None) (index=1)\n\n"}
[00:44:26] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:44:26] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:44:26] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:44:26] 
[00:44:26] note: the compiler unexpectedly panicked. this is a bug.
[00:44:26] 
[00:44:26] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:44:26] note: rustc 1.28.0-dev running on x86_64-unknown-linux-gnu
[00:44:26] 
[00:44:26] 
[00:44:26] note: compiler flags: -Z ui-testing -Z unstable-options -Z borrowck=mir -Z verbose -C prefer-dynamic -C rpath
[00:44:26] 
[00:44:26] ------------------------------------------
[00:44:26] 
[00:44:26] thread '[ui] ui/nll/ty-outlives/impl-trait-captures.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3053:9
[00:44:26] thread '[ui] ui/nll/ty-outlives/impl-trait-captures.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3053:9
[00:44:26] 
[00:44:26] ---- [ui] ui/nll/ty-outlives/impl-trait-outlives.rs stdout ----
[00:44:26] diff of stderr:
[00:44:26] 
[00:44:26] - warning: not reporting region error due to nll
[00:44:26] -    |
[00:44:26] -    |
[00:44:26] - LL | fn no_region<'a, T>(x: Box<T>) -> impl Debug + 'a
[00:44:26] -    |                                   ^^^^^^^^^^^^^^^
[00:44:26] + error: internal compiler error: librustc/ty/subst.rs:425: Region parameter out of range when substituting in region 'a (root type=None) (index=1)
[00:44:26] 6 
[00:44:26] - warning: not reporting region error due to nll
[00:44:26] -    |
[00:44:26] -    |
[00:44:26] - LL | fn wrong_region<'a, 'b, T>(x: Box<T>) -> impl Debug + 'a
[00:44:26] + error: aborting due to previous error
[00:44:26] 12 
[00:44:26] 12 
[00:44:26] - error[E0309]: the parameter type `T` may not live long enough
[00:44:2ppreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:44:26] note: rustc 1.28.0-dev running on x86_64-unknown-linux-gnu
[00:44:26] 
[00:44:26] 
[00:44:26] note: compiler flags: -Z ui-testing -Z unstable-options -Z borrowck=mir -Z verbose -C prefer-dynamic -C rpath
[00:44:26] 
[00:44:26] ------------------------------------------
[00:44:26] 
[00:44:26] thread '[ui] ui/nll/ty-outlives/impl-trait-outlives.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3053:9
---
[00:44:26] 
[00:44:26] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:44:26] 
[00:44:26] 
[00:44:26] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:44:26] 
[00:44:26] 
[00:44:26] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:44:26] Build completed unsuccessfully in 0:02:29
[00:44:26] Build completed unsuccessfully in 0:02:29
[00:44:26] make: *** [check] Error 1
[00:44:26] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:292e6bc4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
