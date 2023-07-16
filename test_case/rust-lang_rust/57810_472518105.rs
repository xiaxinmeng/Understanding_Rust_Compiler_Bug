plain
travis_time:end:01fa5a31:start=1552496900855406532,finish=1552496901778871706,duration=923465174
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:02:03] extracting /checkout/obj/build/cache/2019-02-27/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:02:03] error: failed to resolve patches for `https://github.com/rust-lang/cargo`
[00:02:03] 
[00:02:03] Caused by:
[00:02:03]   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:02:03] Build completed unsuccessfully in 0:00:18
[00:02:03] make: *** [prepare] Error 1
[00:02:03] Makefile:70: recipe for target 'prepare' failed
[00:02:04] Command failed. Attempt 2/5:
[00:02:04] Command failed. Attempt 2/5:
[00:02:04] error: failed to resolve patches for `https://github.com/rust-lang/cargo`
[00:02:04] 
[00:02:04] Caused by:
[00:02:04]   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:02:04] Build completed unsuccessfully in 0:00:00
[00:02:04] make: *** [prepare] Error 1
[00:02:04] Makefile:70: recipe for target 'prepare' failed
[00:02:06] Command failed. Attempt 3/5:
[00:02:06] Command failed. Attempt 3/5:
[00:02:06] error: failed to resolve patches for `https://github.com/rust-lang/cargo`
[00:02:06] 
[00:02:06] Caused by:
[00:02:06]   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:02:06] Build completed unsuccessfully in 0:00:00
[00:02:06] make: *** [prepare] Error 1
[00:02:06] Makefile:70: recipe for target 'prepare' failed
[00:02:09] Command failed. Attempt 4/5:
[00:02:09] Command failed. Attempt 4/5:
[00:02:09] error: failed to resolve patches for `https://github.com/rust-lang/cargo`
[00:02:09] 
[00:02:09] Caused by:
[00:02:09]   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:02:09] Build completed unsuccessfully in 0:00:00
[00:02:09] Makefile:70: recipe for target 'prepare' failed
[00:02:09] make: *** [prepare] Error 1
[00:02:13] Command failed. Attempt 5/5:
[00:02:13] Command failed. Attempt 5/5:
[00:02:13] error: failed to resolve patches for `https://github.com/rust-lang/cargo`
[00:02:13] 
[00:02:13] Caused by:
[00:02:13]   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:02:13] Build completed unsuccessfully in 0:00:00
[00:02:13] Makefile:70: recipe for target 'prepare' failed
[00:02:13] make: *** [prepare] Error 1
[00:02:13] The command has failed after 5 attempts.
---
travis_time:end:1118ad4d:start=1552497050027251442,finish=1552497050032367272,duration=5115830
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:07af1972
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:076d415e
travis_time:start:076d415e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:04ee9f90
$ dmesg | grep -i kill
