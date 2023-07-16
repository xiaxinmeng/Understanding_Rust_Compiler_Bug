plain
travis_time:end:2925f054:start=1548037272419408147,finish=1548037343674922585,duration=71255514438
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu
---
[00:30:33]    Compiling rustc_llvm v0.0.0 (/checkout/src/librustc_llvm)
[00:30:34] error: failed to run custom build command for `rustc_llvm v0.0.0 (/checkout/src/librustc_llvm)`
[00:30:34] process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-codegen/release/build/rustc_llvm-32ba2d0adcc10f37/build-script-build` (exit code: 101)
[00:30:34] --- stdout
[00:30:34] cargo:rerun-if-changed=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/llvm-config
[00:30:34] cargo:rerun-if-env-changed=LLVM_CONFIG
[00:30:34] --- stderr
[00:30:34] --- stderr
[00:30:34] Some("/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-codegen/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib")
[00:30:34] "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/llvm-config"
[00:30:34] /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/llvm-config: relocation error: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/llvm-config: symbol _ZN4llvm6Triple9normalizeB5cxx11ENS_9StringRefE version LLVM_8 not defined in file libLLVM-8svn.so with link time reference
[00:30:34] thread 'main' panicked at 'command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/llvm-config" "--version"
[00:30:34] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:30:34] 
[00:30:34] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/librustc_codegen_llvm/Cargo.toml" "--features" "" "--message-format" "json"
[00:30:34] expected success, got: exit code: 101
---
travis_time:end:005b73a8:start=1548039189310355193,finish=1548039189314571869,duration=4216676
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:11eee28c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1f47cce0
travis_time:start:1f47cce0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:14be83e8
$ dmesg | grep -i kill
