plain
[00:47:39] ....................................................................................................
[00:47:44] ....................................................................................................
[00:47:48] ....................................................................................................
[00:47:54] .............................................................................................i......
[00:48:00] ..................................F...................................i.............................
[00:48:10] ....................................................................................................
[00:48:16] ...................................................................................................i
[00:48:16] ...................................................................................................i
[00:48:19] .................iiiiiiiii...................................................
[00:48:19] 
[00:48:19] ---- [ui] ui/nll/generator-distinct-lifetime.rs stdout ----
[00:48:19] 
[00:48:19] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:48:19] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:48:19] error: test compilation failed although it shouldn't!
[00:48:19] status: exit code: 101
[00:48:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/generator-distinct-lifetime.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/generator-distinct-lifetime/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/generator-distinct-lifetime/auxiliary" "-A" "unused"
[00:48:19] ------------------------------------------
[00:48:19] 
[00:48:19] ------------------------------------------
[00:48:19] stderr:
[00:48:19] stderr:
[00:48:19] ------------------------------------------
[00:48:19] {"message":"librustc_mir/transform/generator.rs:494: Broken MIR: generator contains type std::ops::GeneratorState<(), ()> in MIR, but typeck only knows about for<'r, 's, 't0> {&'r mut u32, u32, &'s mut u32, (), &'t0 mut u32}","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/ui/nll/generator-distinct-lifetime.rs","byte_start":835,"byte_end":965,"line_start":22,"line_end":30,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    move || {","highlight_start":5,"highlight_end":14},{"text":"        let s = &mut *x;","highlight_start":1,"highlight_end":25},{"text":"        yield;","highlight_start":1,"highlight_end":15},{"text":"        *s += 1;","highlight_start":1,"highlight_end":17},{"text":"","highlight_start":1,"highlight_end":1},{"text":"        let t = &mut *x;","highlight_start":1,"highlight_end":25},{"text":"        yield;","highlight_start":1,"highlight_end":15},{"text":"        *t += 1;","highlight_start":1,"highlight_end":17},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: internal compiler error: librustc_mir/transform/generator.rs:494: Broken MIR: generator contains type std::ops::GeneratorState<(), ()> in MIR, but typeck only knows about for<'r, 's, 't0> {&'r mut u32, u32, &'s mut u32, (), &'t0 mut u32}\n  --> /checkout/src/test/ui/nll/generator-distinct-lifetime.rs:22:5\n   |\nLL | /     move || {\nLL | |         let s = &mut *x;\nLL | |         yield;\nLL | |         *s += 1;\n...  |\nLL | |         *t += 1;\nLL | |     };\n   | |_____^\n\n"}
[00:48:19] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:48:19] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:48:19] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:48:19] 
[00:48:19] note: the compiler unexpectedly panicked. this is a bug.
[00:48:19] 
[00:48:19] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:48:19] note: rustc 1.28.0-dev running on x86_64-unknown-linux-gnu
[00:48:19] 
[00:48:19] 
[00:48:19] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[00:48:19] 
[00:48:19] ------------------------------------------
[00:48:19] 
[00:48:19] thread '[ui] ui/nll/generator-distinct-lifetime.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3053:9
---
[00:48:19] test result: FAILED. 1462 passed; 1 failed; 14 ignored; 0 measured; 0 filtered out
[00:48:19] 
[00:48:19] 
[00:48:19] 
[00:48:19] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:48:19] 
[00:48:19] 
[00:48:19] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:48:19] Build completed unsuccessfully in 0:02:40
[00:48:19] Build completed unsuccessfully in 0:02:40
[00:48:19] make: *** [check] Error 1
[00:48:19] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00b1c4dc
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
