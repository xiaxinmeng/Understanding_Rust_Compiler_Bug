plain
travis_time:start:test_incremental
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:06:00] 
[01:06:00] running 89 tests
[01:06:23] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[01:06:23] ..........................F.............F................................................
[01:06:23] 
[01:06:23] ---- [incremental] incremental/hashes/if_expressions.rs stdout ----
[01:06:23] 
[01:06:23] 
[01:06:23] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:06:23] status: exit code: 101
[01:06:23] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/if_expressions.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/objhighlight_end":8},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: `MirOptimized(add_else_branch)` should be clean but is not\n  --> /checkout/src/test/incremental/hashes/if_expressions.rs:110:1\n   |\nLL | / pub fn add_else_branch(x: bool) -> u32 {\nLL | |     let mut ret = 1;\nLL | |\nLL | |     if x {\n...  |\nLL | |     ret\nLL | | }\n   | |_^\n\n"}
[01:06:23] {"message":"`MirValidated(add_else_branch)` should be clean but is not","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/hashes/if_expressions.rs","byte_start":2526,"byte_end":2646,"line_start":110,"line_end":119,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"pub fn add_else_branch(x: bool) -> u32 {","highlight_start":1,"highlight_end":41},{"text":"    let mut ret = 1;","highlight_start":1,"highlight_end":21},{"text":"","highlight_start":1,"highlight_end":1},{"text":"    if x {","highlight_start":1,"highlight_end":11},{"text":"        ret = 2;","highlight_start":1,"highlight_end":17},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"    }","highlight_start":1,"highlight_end":6},{"text":"","highlight_start":1,"highlight_end":1},{"text":"    ret","highlight_start":1,"highlight_end":8},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: `MirValidated(add_else_branch)` should be clean but is not\n  --> /checkout/src/test/incremental/hashes/if_exemental/hashes/if_expressions.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[01:06:23] 
[01:06:23] ---- [incremental] incremental/hashes/trait_defs.rs stdout ----
[01:06:23] 
[01:06:23] 
[01:06:23] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:06:23] status: exit code: 101
[01:06:23] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/trait_defs.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/trait_defs/trait_defs.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/trait_defs/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/trait_defs/auxiliary"
[01:06:23] ------------------------------------------
[01:06:23] 
[01:06:23] ------------------------------------------
[01:06:23] stderr:
[01:06:23] stderr:
[01:06:23] ------------------------------------------
[01:06:23] {"message":"`Hir(TraitChangeModeSelfOwnToMut::method)` should be clean but is not","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/hashes/trait_defs.rs","byte_start":7313,"byte_end":7335,"line_start":277,"line_end":277,"column_start":5,"column_end":27,"is_primary":true,"text":[{"text":"    fn method(mut self) {}","highlight_start":5,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: `Hir(TraitChangeModeSelfOwnToMut::method)` should be clean but is not\n  --> /checkout/src/test/incremental/hashes/trait_defs.rs:277:5\n   |\nLL |     fn method(mut self) {}\n   |     ^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:06:23] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:06:23] ------------------------------------------
[01:06:23] 
[01:06:23] thread '[incremental] incremental/hashes/trait_defs.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[01:06:23] 
---
[01:06:23] test result: FAILED. 87 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
[01:06:23] 
[01:06:23] 
[01:06:23] 
[01:06:23] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:06:23] 
[01:06:23] 
[01:06:23] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:06:23] Build completed unsuccessfully in 0:17:50
[01:06:23] Build completed unsuccessfully in 0:17:50
[01:06:23] make: *** [check] Error 1
[01:06:23] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:192cc01e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0b5384fd:start=1528859612727395794,finish=1528859612733696161,duration=6300367
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0a5be788
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0d2c6fa2
$ dmesg | grep -i kill
