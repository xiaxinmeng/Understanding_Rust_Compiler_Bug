plain
travis_time:end:00038e3c:start=1543768431821801159,finish=1543768491192500693,duration=59370699534
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:06:41]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:07:59] error: unused variable: `ty`
[00:07:59]    --> src/librustc/middle/stability.rs:761:21
[00:07:59]     |
[00:07:59] 761 |                 let ty = self.tcx.type_of(def_id);
[00:07:59]     |                     ^^ help: consider using `_ty` instead
[00:07:59]     = note: `-D unused-variables` implied by `-D warnings`
[00:07:59] 
[00:07:59] error: aborting due to previous error
[00:07:59] 
[00:07:59] 
[00:07:59] error: Could not compile `rustc`.
[00:07:59] 
[00:07:59] To learn more, run the command again with --verbose.
[00:07:59] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:07:59] expected success, got: exit code: 101
[00:07:59] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:07:59] Build completed unsuccessfully in 0:04:45
[00:07:59] make: *** [all] Error 1
[00:07:59] Makefile:28: recipe for target 'all' failed
47684 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release
46352 ./src/llvm-emscripten/test/MC
45752 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps
44272 ./src/libcompiler_builtins
---
travis_time:end:1e2aac1c:start=1543768980348625398,finish=1543768980354680610,duration=6055212
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0136f2c9
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:089fb5ca
travis_time:start:089fb5ca
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:027e490b
$ dmesg | grep -i kill
