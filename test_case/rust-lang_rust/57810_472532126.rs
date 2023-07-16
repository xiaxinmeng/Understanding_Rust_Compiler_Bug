plain
travis_time:end:061acc80:start=1552498929937044462,finish=1552498931054450927,duration=1117406465
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:01:57] extracting /checkout/obj/build/cache/2019-02-27/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:57] error: failed to resolve patches for `https://github.com/rust-lang/cargo`
[00:01:57] 
[00:01:57] Caused by:
[00:01:57]   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:01:57] Build completed unsuccessfully in 0:00:13
[00:01:57] Makefile:70: recipe for target 'prepare' failed
[00:01:57] make: *** [prepare] Error 1
[00:01:58] Command failed. Attempt 2/5:
[00:01:58] Command failed. Attempt 2/5:
[00:01:59] error: failed to resolve patches for `https://github.com/rust-lang/cargo`
[00:01:59] 
[00:01:59] Caused by:
[00:01:59]   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:01:59] Build completed unsuccessfully in 0:00:00
[00:01:59] make: *** [prepare] Error 1
[00:01:59] Makefile:70: recipe for target 'prepare' failed
[00:02:01] Command failed. Attempt 3/5:
[00:02:01] Command failed. Attempt 3/5:
[00:02:01] error: failed to resolve patches for `https://github.com/rust-lang/cargo`
[00:02:01] 
[00:02:01] Caused by:
[00:02:01]   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:02:01] Build completed unsuccessfully in 0:00:00
[00:02:01] make: *** [prepare] Error 1
[00:02:01] Makefile:70: recipe for target 'prepare' failed
[00:02:04] Command failed. Attempt 4/5:
[00:02:04] Command failed. Attempt 4/5:
[00:02:04] error: failed to resolve patches for `https://github.com/rust-lang/cargo`
[00:02:04] 
[00:02:04] Caused by:
[00:02:04]   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:02:04] Build completed unsuccessfully in 0:00:00
[00:02:04] make: *** [prepare] Error 1
[00:02:04] Makefile:70: recipe for target 'prepare' failed
[00:02:08] Command failed. Attempt 5/5:
[00:02:08] Command failed. Attempt 5/5:
[00:02:08] error: failed to resolve patches for `https://github.com/rust-lang/cargo`
[00:02:08] 
[00:02:08] Caused by:
[00:02:08]   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:02:08] Build completed unsuccessfully in 0:00:00
[00:02:08] make: *** [prepare] Error 1
[00:02:08] Makefile:70: recipe for target 'prepare' failed
[00:02:08] The command has failed after 5 attempts.
---
travis_time:end:003e6e58:start=1552499073907935567,finish=1552499073915666809,duration=7731242
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1fdacd74
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:134130d4
travis_time:start:134130d4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:03f7998d
$ dmesg | grep -i kill
