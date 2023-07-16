plain
travis_time:end:1cf21d6c:start=1552939069397571770,finish=1552939154303446712,duration=84905874942
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:34:16] 
[01:34:16] running 99 tests
[01:34:31] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:34:31] .....F.............................................................................................
[01:34:31] 
[01:34:31] ---- [incremental] incremental/change_name_of_static_in_fn.rs stdout ----
[01:34:31] 
[01:34:31] 
[01:34:31] error in revision `rpass2`: compilation failed!
[01:34:31] status: exit code: 101
[01:34:31] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_name_of_static_in_fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_name_of_static_in_fn/change_name_of_static_in_fn.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_name_of_static_in_fn/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_name_of_static_in_fn/auxiliary"
[01:34:31] ------------------------------------------
[01:34:31] 
[01:34:31] ------------------------------------------
[01:34:31] stderr:
[01:34:31] stderr:
[01:34:31] ------------------------------------------
[01:34:31] {"message":"src/librustc/dep_graph/graph.rs:722: try_mark_previous_green() - Forcing the DepNode should have set its color","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: src/librustc/dep_graph/graph.rs:722: try_mark_previous_green() - Forcing the DepNode should have set its color\n\n"}
[01:34:31] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:34:31] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:34:31] 
[01:34:31] 
[01:34:31] note: the compiler unrget" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:34:31] 
[01:34:31] 
[01:34:31] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:34:31] Build completed unsuccessfully in 0:11:26
[01:34:31] Build completed unsuccessfully in 0:11:26
[01:34:31] make: *** [check] Error 1
[01:34:31] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0d158e9a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Mar 18 21:33:55 UTC 2019
---
146108 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release
143324 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps
143268 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps
141928 ./obj/build/bootstrap/debug/incremental/bootstrap-5f86g9tk67ex
141924 ./obj/build/bootstrap/debug/incremental/bootstrap-5f86g9tk67ex/s-fagygcsaee-ipvmbd-3d9d4wenbbd1z
123628 ./src/llvm-project/llvm/test/CodeGen
108532 ./src/llvm-project/lldb
97584 ./src/llvm-project/clang/test
97216 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/release/deps
