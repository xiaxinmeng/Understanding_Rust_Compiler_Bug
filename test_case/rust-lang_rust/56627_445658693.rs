plain
travis_time:end:2def26c8:start=1544413239665410606,finish=1544413240922384251,duration=1256973645
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:01:15] extracting /checkout/obj/build/cache/2018-10-30/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:15] error: failed to resolve patches for `https://github.com/rust-lang/cargo`
[00:01:15] 
[00:01:15] Caused by:
[00:01:15]   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:01:15] Build completed unsuccessfully in 0:00:11
[00:01:15] make: *** [prepare] Error 1
[00:01:15] Makefile:81: recipe for target 'prepare' failed
[00:01:16] Command failed. Attempt 2/5:
[00:01:16] Command failed. Attempt 2/5:
[00:01:16] error: failed to resolve patches for `https://github.com/rust-lang/cargo`
[00:01:16] 
[00:01:16] Caused by:
[00:01:16]   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:01:16] Build completed unsuccessfully in 0:00:00
[00:01:16] make: *** [prepare] Error 1
[00:01:16] Makefile:81: recipe for target 'prepare' failed
[00:01:18] Command failed. Attempt 3/5:
[00:01:18] Command failed. Attempt 3/5:
[00:01:18] error: failed to resolve patches for `https://github.com/rust-lang/cargo`
[00:01:18] 
[00:01:18] Caused by:
[00:01:18]   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:01:18] Build completed unsuccessfully in 0:00:00
[00:01:18] make: *** [prepare] Error 1
[00:01:18] Makefile:81: recipe for target 'prepare' failed
[00:01:21] Command failed. Attempt 4/5:
[00:01:21] Command failed. Attempt 4/5:
[00:01:21] error: failed to resolve patches for `https://github.com/rust-lang/cargo`
[00:01:21] 
[00:01:21] Caused by:
[00:01:21]   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:01:21] Build completed unsuccessfully in 0:00:00
[00:01:21] Makefile:81: recipe for target 'prepare' failed
[00:01:21] make: *** [prepare] Error 1
[00:01:25] Command failed. Attempt 5/5:
[00:01:25] Command failed. Attempt 5/5:
[00:01:25] error: failed to resolve patches for `https://github.com/rust-lang/cargo`
[00:01:25] 
[00:01:25] Caused by:
[00:01:25]   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:01:25] Build completed unsuccessfully in 0:00:00
[00:01:25] make: *** [prepare] Error 1
[00:01:25] Makefile:81: recipe for target 'prepare' failed
[00:01:25] The command has failed after 5 attempts.
---
travis_time:end:013b7482:start=1544413336085605458,finish=1544413336090547796,duration=4942338
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:184b33b0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1d352e78
travis_time:start:1d352e78
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0b13446a
$ dmesg | grep -i kill
