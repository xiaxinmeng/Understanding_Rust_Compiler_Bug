plain
travis_time:end:0fd39488:start=1546322930638109442,finish=1546322931731305757,duration=1093196315
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:03:50]    Compiling rustc-demangle v0.1.10
[00:03:53] error[E0308]: mismatched types
[00:03:53]     --> src/liballoc/vec.rs:2359:53
[00:03:53]      |
[00:03:53] 2359 |                 arith_offset(self.ptr as *const i8, n) as *mut T
[00:03:53]      |                                                     ^ expected isize, found usize
[00:03:53] error[E0308]: mismatched types
[00:03:53]     --> src/liballoc/vec.rs:2361:33
[00:03:53]      |
[00:03:53] 2361 |                 self.ptr.offset(n)
---
[00:03:53] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:03:53] expected success, got: exit code: 101
[00:03:53] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:03:53] Build completed unsuccessfully in 0:00:38
[00:03:53] Makefile:18: recipe for target 'all' failed
[00:03:53] make: *** [all] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0d812998
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Jan  1 06:12:54 UTC 2019
---
travis_time:end:15d8797f:start=1546323174637536905,finish=1546323174644935435,duration=7398530
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:3645ef0a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:16ed8090
travis_time:start:16ed8090
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:19be9cd6
$ dmesg | grep -i kill
