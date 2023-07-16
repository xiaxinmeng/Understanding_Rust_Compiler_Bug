plain
travis_time:end:03d9977c:start=1553937250163891566,finish=1553937252389211871,duration=2225320305
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:25:55] 
[01:25:55] running 9 tests
[01:25:55] iiiiiiiii
[01:25:55] 
[01:25:55]  finished in 0.161
[01:25:55] travis_fold:end:test_assembly


[01:25:55] travis_time:end:test_assembly:start=1553942417331841154,finish=1553942417493061403,duration=161220249

[01:25:55] travis_fold:start:test_incremental
travis_time:start:test_incremental
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:25:55] 
[01:25:55] running 99 tests
[01:26:11] ............................F......................................................................
[01:26:11] 
[01:26:11] ---- [incremental] incremental/hashes/if_expressions.rs stdout ----
[01:26:11] 
[01:26:11] 
[01:26:11] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:26:11] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:516:22
[01:26:11] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:516:22
[01:26:11] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/if_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/if_expressions/if_expressions.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/if_expressions/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/if_expressions/auxiliary"
[01:26:11] ------------------------------------------
[01:26:11] 
[01:26:11] ------------------------------------------
[01:26:11] stderr:
[01:26:11] stderr:
[01:26:11] ------------------------------------------
[01:26:11] {"message":"`mir_built(add_else_branch)` should be clean but is not","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/hashes/if_expressions.rs","byte_start":2052,"byte_end":2172,"line_start":99,"line_end":108,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"pub fn add_else_branch(x: bool) -> u32 {","highlight_start":1,"highlight_end":41},{"text":"    let mut ret = 1;","highlight_start":1,"highlight_end":21},{"text":"","highlight_start":1,"highlight_end":1},{"text":"    if x {","highlight_start":1,"highlight_end":11},{"text":"        ret = 2;","highlight_start":1,"highlight_end":17},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"    }","highlight_start":1,"highlight_end":6},{"text":"","highlight_start":1,"highlight_end":1},{"text":"    ret","highlight_start":1,"highlight_end":8},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: `mir_built(add_else_branch)` should be clean but is not\n  --> /checkout/src/test/incremental/hashes/if_expressions.rs:99:1\n   |\nLL | / pub fn add_else_branch(x: bool) -> u32 {\nLL | |     let mut ret = 1;\nLL | |\nLL | |     if x {\n...  |\nLL | |     ret\nLL | | }\n   | |_^\n\n"}
[01:26:11] {"message":"`optimized_mir(add_else_branch)` should be clean but is not","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/hashes/if_expressions.rs","byte_start":2052,"byte_end":2172,"line_start":99,"line_end":108,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"pub fn add_else_branch(x: bool) -> u32 {","highlight_start":1,"highlight_end":41},{"text":"    let mut ret = 1;","highlight_start":1,"highlight_end":21},{"text":"","highlight_start":1,"highlight_end":1},{"text":"    if x {","highlight_start":1,"highlight_end":11},{"text":"        ret = 2;","highlight_start":1,"highlight_end":17},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"    }","highlight_start":1,"highlight_end":6},{"text":"","highlight_start":1,"highlight_end":1},{"text":"    ret","highlight_start":1,"highlight_end":8},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: `optimized_mir(add_else_branch)` should be clean but is not\n  --> /checkout/src/test/incremental/hashes/if_expressions.rs:99:1\n   |\nLL | / pub fn add_else_branch(x: bool) -> u32 {\nLL | |     let mut ret = 1;\nLL | |\nLL | |     if x {\n...  |\nLL | |     ret\nLL | | }\n   | |_^\n\n"}
[01:26:11] 
[01:26:11] ------------------------------------------
[01:26:11] 
[01:26:11] thread '[incremental] incremental/hashes/if_expressions.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3422:9
---
[01:26:11] test result: FAILED. 98 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:26:11] 
[01:26:11] 
[01:26:11] 
[01:26:11] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:26:11] 
[01:26:11] 
[01:26:11] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:26:11] Build completed unsuccessfully in 0:12:34
[01:26:11] Build completed unsuccessfully in 0:12:34
[01:26:11] Makefile:48: recipe for target 'check' failed
[01:26:11] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1f8ace54
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Mar 30 10:40:34 UTC 2019
