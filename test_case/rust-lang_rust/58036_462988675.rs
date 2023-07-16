plain
travis_time:end:0ac0fcac:start=1550014145217149720,finish=1550014148723401717,duration=3506251997
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:07:16]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:07:17] error[E0061]: this function takes 1 parameter but 0 parameters were supplied
[00:07:17]    --> src/libsyntax_ext/format.rs:712:12
[00:07:17]     |
[00:07:17] 712 |     if !sp.allows_unstable()
[00:07:17] 
[00:07:17] error: aborting due to previous error
[00:07:17] 
[00:07:17] For more information about this error, try `rustc --explain E0061`.
[00:07:17] For more information about this error, try `rustc --explain E0061`.
[00:07:17] error: Could not compile `syntax_ext`.
[00:07:17] warning: build failed, waiting for other jobs to finish...
[00:12:20] error: build failed
[00:12:20] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:12:20] expected success, got: exit code: 101
[00:12:20] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:12:20] Build completed unsuccessfully in 0:08:11
[00:12:20] Makefile:18: recipe for target 'all' failed
[00:12:20] make: *** [all] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:18d4c265
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Feb 12 23:41:39 UTC 2019
---
travis_time:end:18addd8c:start=1550014900488680391,finish=1550014900493376621,duration=4696230
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:01784a18
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1298ca6e
travis_time:start:1298ca6e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers
