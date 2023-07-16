plain
travis_time:end:1c0b6090:start=1559669689852908094,finish=1559669785790925895,duration=95938017801
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:04:55] 
[01:04:55] running 144 tests
[01:04:58] i..iii.....iii..iiii.....i......................i..i.................i.....i..........ii.i..i..i.ii. 100/144
[01:05:00] test result: ok. 114 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[01:05:00] 
[01:05:00]  finished in 4.628
[01:05:00] travis_fold:end:test_codegen
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:05:02] 
[01:05:02] running 9 tests
[01:05:02] iiiiiiiii
[01:05:02] 
[01:05:02]  finished in 0.147
[01:05:02] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:05:18] 
[01:05:18] running 122 tests
[01:05:44] .iiiii...i.....i..i...i..i.i.i..i.ii.Fi.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:05:49] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:05:49] .i.i......iii.i.....ii
[01:05:49] failures:
[01:05:49] 
[01:05:49] 
[01:05:49] ---- [debuginfo-gdb+lldb] debuginfo/generator-locals.rs stdout ----
[01:05:49] NOTE: compiletest thinks it is using GDB without native rust support
[01:05:49] error: compilation failed!
[01:05:49] status: exit code: 101
[01:05:49] status: exit code: 101
[01:05:49] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/debuginfo/generator-locals.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/generator-locals/a" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-g" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/generator-locals/auxiliary.gdb"
[01:05:49] ------------------------------------------
[01:05:49] 
[01:05:49] ------------------------------------------
[01:05:49] stderr:
[01:05:49] stderr:
[01:05:49] ------------------------------------------
[01:05:49] thread 'rustc' panicked at 'assertion failed: `(left != right)`
[01:05:49]   left: `_1`,
[01:05:49]  right: `_1`', src/librustc_mir/transform/generator.rs:99:9
[01:05:49] 
[01:05:49] error: internal compiler error: unexpected panic
[01:05:49] 
[01:05:49] note: the compiler unexpectedly panicked. this is a bug.
[01:05:49] note: the compiler unexpectedly panicked. this is a bug.
[01:05:49] 
[01:05:49] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:05:49] 
[01:05:49] note: rustc 1.37.0-dev running on x86_64-unknown-linux-gnu
[01:05:49] 
[01:05:49] note: compiler flags: -Z threads=1 -Z unstable-options -C prefer-dynamic -C rpath
[01:05:49] 
[01:05:49] ------------------------------------------
[01:05:49] 
[01:05:49] 
---
[01:05:49] test result: FAILED. 82 passed; 1 failed; 39 ignored; 0 measured; 0 filtered out
[01:05:49] 
[01:05:49] 
[01:05:49] 
[01:05:49] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/debuginfo" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "debuginfo-gdb+lldb" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FilTue, 04 Jun 2019 18:42:25 GMT
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:2264ba10
---
travis_time:end:0af43a91:start=1559673745995971198,finish=1559673746002010947,duration=6039749
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:34b6a4c3
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout
