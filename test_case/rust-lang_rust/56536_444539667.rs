plain
travis_time:end:13f715f3:start=1544024759698311935,finish=1544024761069629778,duration=1371317843
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:18:03]    Compiling libc v0.0.0 (/checkout/src/rustc/libc_shim)
[00:18:03]    Compiling alloc v0.0.0 (/checkout/src/liballoc)
[00:18:04]    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[00:18:08]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:18:10] error: use of deprecated item 'error::Error::cause': replaced by Error::source, which can support downcasting
[00:18:10]     |
[00:18:10]     |
[00:18:10] 537 |         Error::cause(&**self)
[00:18:10]     |
[00:18:10]     = note: `-D deprecated` implied by `-D warnings`
[00:18:10] 
[00:18:10] 
[00:18:12] error: use of deprecated item 'error::Error::cause': replaced by Error::source, which can support downcasting
[00:18:12]    --> src/libstd/io/error.rs:562:44
[00:18:12]     |
[00:18:12] 562 |             Repr::Custom(ref c) => c.error.cause(),
[00:18:12] 
[00:18:13] error: aborting due to 2 previous errors
[00:18:13] 
[00:18:13] error: Could not compile `std`.
[00:18:13] error: Could not compile `std`.
[00:18:13] 
[00:18:13] To learn more, run the command again with --verbose.
[00:18:13] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:18:13] expected success, got: exit code: 101
[00:18:13] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:18:13] Build completed unsuccessfully in 0:14:49
[00:18:13] make: *** [all] Error 1
[00:18:13] Makefile:28: recipe for target 'all' failed
1467436 ./obj
1467396 ./obj/build
1165712 ./src
813580 ./obj/build/x86_64-unknown-linux-gnu
---
187088 ./obj/build/cache/2018-12-05
159656 ./obj/build/bootstrap/debug/incremental
153292 ./src/tools/clang
149800 ./.git
143564 ./obj/build/bootstrap/debug/incremental/bootstrap-gg4wocctfx8q
143560 ./obj/build/bootstrap/debug/incremental/bootstrap-gg4wocctfx8q/s-f7b78p2if9-z7c6ad-1jpjqomavnfay
134544 ./.git/modules/src
115356 ./src/llvm/test/CodeGen
111160 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
107416 ./src/tools/lldb
---
travis_time:end:1e8fe18e:start=1544025864147140979,finish=1544025864153696500,duration=6555521
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0edd22c4
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:004457dc
travis_time:start:004457dc
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:05c20430
$ dmesg | grep -i kill
