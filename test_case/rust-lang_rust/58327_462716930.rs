plain
travis_time:end:04b1a4f1:start=1549965418228328595,finish=1549965538396790302,duration=120168461707
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:02:09] .................................................................................................... 100/5386
[01:02:13] .................................................................................................... 200/5386
[01:02:16] .................................................................................................... 300/5386
[01:02:19] .................................................................................................... 400/5386
[01:02:23] ...............................................................F.................................... 500/5386
[01:02:31] .................................................................................................... 700/5386
[01:02:36] .................................................................................................... 800/5386
[01:02:42] ....................................................................................i............... 900/5386
[01:02:46] i................................................................................................... 1000/5386
---
[01:05:34] .................................................................................................... 5300/5386
[01:05:37] .........................i............................................................
[01:05:37] failures:
[01:05:37] 
[01:05:37] ---- [ui] ui/c_ffi_const2.rs stdout ----
[01:05:37] 
[01:05:37] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:05:37] status: exit code: 101
[01:05:37] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/c_ffi_const2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c_ffi_const2/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c_ffi_const2/auxiliary" "-A" "unused"
[01:05:37] ------------------------------------------
[01:05:37] 
[01:05:37] ------------------------------------------
[01:05:37] stderr:
[01:05:37] stderr:
[01:05:37] ------------------------------------------
[01:05:37] {"message":"`#[c_ffi_const]` function cannot be`#[c_ffi_pure]`","code":{"code":"E0726","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/c_ffi_const2.rs","byte_start":77,"byte_end":90,"line_start":5,"line_end":5,"column_start":5,"column_end":18,"is_primary":true,"text":[{"text":"    #[c_ffi_pure] //~ ERROR `#[c_ffi_const]` function cannot be`#[c_ffi_pure]` [E0726]","highlight_start":5,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0726]: `#[c_ffi_const]` function cannot be`#[c_ffi_pure]`\n  --> /checkout/src/test/ui/c_ffi_const2.rs:5:5\n   |\nLL |     #[c_ffi_pure] //~ ERROR `#[c_ffi_const]` function cannot be`#[c_ffi_pure]` [E0726]\n   |     ^^^^^^^^^^^^^\n\n"}
[01:05:37] Attributes 'readnone and readonly' are incompatible!
[01:05:37] void ()* @baz
[01:05:37] LLVM ERROR: Broken module found, compilation aborted!
[01:05:37] ------------------------------------------
[01:05:37] 
[01:05:37] 
[01:05:37] thread '[ui] ui/c_ffi_const2.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:05:37] 
[01:05:37] 
[01:05:37] failures:
[01:05:37] failures:
[01:05:37]     [ui] ui/c_ffi_const2.rs
[01:05:37] test result: FAILED. 5362 passed; 1 failed; 23 ignored; 0 measured; 0 filtered out
[01:05:37] 
[01:05:37] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:05:37] 
[01:05:37] 
[01:05:37] 
[01:05:37] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:05:37] 
[01:05:37] 
[01:05:37] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:05:37] Build completed unsuccessfully in 0:04:32
[01:05:37] Build completed unsuccessfully in 0:04:32
[01:05:37] make: *** [check] Error 1
[01:05:37] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1de05e22
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Feb 12 11:04:45 UTC 2019
