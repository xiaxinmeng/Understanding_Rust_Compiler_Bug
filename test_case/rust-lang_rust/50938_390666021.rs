plain
[00:44:38] ....................................................................................................
[00:44:43] ....................................................................................................
[00:44:48] ....................................................................................................
[00:44:53] .....................................................................i..............................
[00:44:57] ..........F...................................i.....................................................
[00:45:07] ....................................................................................................
[00:45:14] ...................................................................i................................
[00:45:16] .............................................
[00:45:16] failures:
[00:45:16] failures:
[00:45:16] 
[00:45:16] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:45:16] ---- [ui] ui/nll/generator-distinct-lifetime.rs stdout ----
[00:45:16] 
[00:45:16] error: test compilation failed although it shouldn't!
[00:45:16] status: exit code: 101
[00:45:16] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/generator-distinct-lifetime.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/generator-distinct-lifetime/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/generator-distinct-lifetime/auxiliary" "-A" "unused"
[00:45:16] ------------------------------------------
[00:45:16] 
[00:45:16] ------------------------------------------
[00:45:16] stderr:
[00:45:16] stderr:
[00:45:16] ------------------------------------------
[00:45:16] {"message":"librustc_mir/transform/generator.rs:494: Broken MIR: generator contains type std::ops::GeneratorState<(), ()> in MIR, but typeck only knows about for<'r, 's, 't0> {&'r mut u32, u32, &'s mut u32, (), &'t0 mut u32}","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/ui/nll/generator-distinct-lifetime.rs","byte_start":835,"byte_end":965,"line_start":22,"line_end":30,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    move || {","highlight_start":5,"highlight_end":14},{"text":"        let s = &mut *x;","highlight_start":1,"highlight_end":25},{"text":"        yield;","highlight_start":1,"highlight_end":15},{"text":"        *s += 1;","highlight_start":1,"highlight_end":17},{"text":"","highlight_start":1,"highlight_end":1},{"text":"        let t = &mut *x;","highlight_start":1,"highlight_end":25},{"text":"        yield;","highlight_start":1,"highlight_end":15},{"text":"        *t += 1;","highlight_start":1,"highlight_end":17},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error: internal compiler error: librustc_mir/transform/generator.rs:494: Broken MIR: generator contains type std::ops::GeneratorState<(), ()> in MIR, but typeck only knows about for<'r, 's, 't0> {&'r mut u32, u32, &'s mut u32, (), &'t0 mut u32}\n  --> /checkout/src/test/ui/nll/generator-distinct-lifetime.rs:22:5\n   |\nLL | /     move || {\nLL | |         let s = &mut *x;\nLL | |         yield;\nLL | |         *s += 1;\n...  |\nLL | |         *t += 1;\nLL | |     };\n   | |_____^\n\n"}
[00:45:16] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:45:16] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:45:16] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:45:16] 
[00:45:16] note: the compiler unexpectedly panicked. this is a bug.
[00:45:16] 
[00:45:16] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:45:16] note: rustc 1.28.0-dev running on x86_64-unknown-linux-gnu
[00:45:16] 
[00:45:16] 
[00:45:16] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[00:45:16] 
[00:45:16] ------------------------------------------
[00:45:16] 
[00:45:16] thread '[ui] ui/nll/generator-distinct-lifetime.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3042:9
---
[00:45:16] test result: FAILED. 1437 passed; 1 failed; 7 ignored; 0 measured; 0 filtered out
[00:45:16] 
[00:45:16] 
[00:45:16] 
[00:45:16] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0eb197a8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
