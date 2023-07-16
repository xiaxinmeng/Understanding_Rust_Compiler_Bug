plain
travis_time:end:090f38d7:start=1545256555563154078,finish=1545256556804093052,duration=1240938974
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:04:38]    Compiling env_logger v0.5.13
[00:04:39]    Compiling flate2 v1.0.6
[00:04:39]    Compiling backtrace v0.3.11
[00:04:41]    Compiling parking_lot_core v0.3.0
[00:04:41] error[E0658]: use of unstable library feature 'renamed_spin_loop' (see issue #55002)
[00:04:41]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/parking_lot_core-0.3.0/src/spinwait.rs:10:5
[00:04:41] 10 | use std::sync::atomic::spin_loop_hint;
[00:04:41]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:04:41]    |
[00:04:41]    = help: add #![feature(renamed_spin_loop)] to the crate attributes to enable
[00:04:41]    = help: add #![feature(renamed_spin_loop)] to the crate attributes to enable
[00:04:41] 
[00:04:41] error[E0658]: use of unstable library feature 'renamed_spin_loop' (see issue #55002)
[00:04:41]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/parking_lot_core-0.3.0/src/spinwait.rs:61:9
[00:04:41] 61 |         spin_loop_hint()
[00:04:41]    |         ^^^^^^^^^^^^^^
[00:04:41]    |
[00:04:41]    = help: add #![feature(renamed_spin_loop)] to the crate attributes to enable
---
[00:04:42] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:04:42] expected success, got: exit code: 101
[00:04:42] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:04:42] Build completed unsuccessfully in 0:01:49
[00:04:42] Makefile:28: recipe for target 'all' failed
[00:04:42] make: *** [all] Error 1
316816 ./src/llvm-emscripten
293692 ./src/tools
269844 ./src/llvm/test
252776 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib
252776 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib
210868 ./src/llvm-emscripten/test
198004 ./obj/build/bootstrap/debug/deps
187088 ./obj/build/cache
187084 ./obj/build/cache/2018-12-09
160388 ./obj/build/bootstrap/debug/incremental
153276 ./src/tools/clang
144288 ./obj/build/bootstrap/debug/incremental/bootstrap-2x7szixskz2uj
144284 ./obj/build/bootstrap/debug/incremental/bootstrap-2x7szixskz2uj/s-f7qx3wtlgb-cnie4x-3g9jcdhvsk79m
111160 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
107420 ./src/tools/lldb
95104 ./src/tools/clang/test
89964 ./src/llvm-emscripten/test/CodeGen
---
travis_time:end:06971ab7:start=1545256848362668406,finish=1545256848367511827,duration=4843421
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1b55f7b5
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1e3835c0
travis_time:start:1e3835c0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0f733e4a
$ dmesg | grep -i kill
