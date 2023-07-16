plain
travis_time:end:0de4ad07:start=1546874520184216074,finish=1546874590757267183,duration=70573051109
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:03:28]    Compiling libc v0.2.46
[00:03:28]    Compiling core v0.0.0 (/checkout/src/libcore)
[00:03:28]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:03:29]    Compiling unwind v0.0.0 (/checkout/src/libunwind)
[00:03:34] error[E0658]: unadjusted ABI is an implementation detail and perma-unstable
[00:03:34]   --> src/libcore/../stdsimd/coresimd/x86/rdrand.rs:6:1
[00:03:34]    |
[00:03:34] 6  | / extern "unadjusted" {
[00:03:34] 7  | |     #[link_name = "llvm.x86.rdrand.16"]
[00:03:34] 8  | |     fn x86_rdrand16_step() -> (u16, i32);
[00:03:34] 9  | |     #[link_name = "llvm.x86.rdrand.32"]
[00:03:34] ...  |
[00:03:34] 14 | |     fn x86_rdseed32_step() -> (u32, i32);
[00:03:34]    | |_^
[00:03:34]    |
[00:03:34]    |
[00:03:34]    = help: add #![feature(abi_unadjusted)] to the crate attributes to enable
[00:03:34] 
[00:03:34] error[E0658]: unadjusted ABI is an implementation detail and perma-unstable
[00:03:34]   --> src/libcore/../stdsimd/coresimd/x86_64/rdrand.rs:6:1
[00:03:34]    |
[00:03:34] 6  | / extern "unadjusted" {
[00:03:34] 7  | |     #[link_name = "llvm.x86.rdrand.64"]
[00:03:34] 8  | |     fn x86_rdrand64_step() -> (u64, i32);
[00:03:34] 9  | |     #[link_name = "llvm.x86.rdseed.64"]
[00:03:34] 10 | |     fn x86_rdseed64_step() -> (u64, i32);
[00:03:34]    | |_^
[00:03:34]    |
[00:03:34]    |
[00:03:34]    = help: add #![feature(abi_unadjusted)] to the crate attributes to enable
[00:03:34] error: aborting due to 2 previous errors
[00:03:34] 
[00:03:34] For more information about this error, try `rustc --explain E0658`.
[00:03:34] error: Could not compile `core`.
[00:03:34] error: Could not compile `core`.
[00:03:34] warning: build failed, waiting for other jobs to finish...
[00:03:34] error: build failed
[00:03:34] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:03:34] expected success, got: exit code: 101
[00:03:34] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:03:34] Build completed unsuccessfully in 0:00:07
[00:03:34] Makefile:18: recipe for target 'all' failed
[00:03:34] make: *** [all] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:35634450
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Jan  7 15:26:53 UTC 2019
---
travis_time:end:03d9ec54:start=1546874814386356520,finish=1546874814390649897,duration=4293377
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:01af5c62
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:07d003b5
travis_time:start:07d003b5
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:03e5a148
$ dmesg | grep -i kill
