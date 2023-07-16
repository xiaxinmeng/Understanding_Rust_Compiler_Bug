plain
travis_time:end:0003e7c6:start=1554066203239478866,finish=1554066204236886755,duration=997407889
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:02:05] extracting /checkout/obj/build/cache/2019-03-20/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:02:05] error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
[00:02:05] 
[00:02:05] Caused by:
[00:02:05]   patch for `rustfmt-nightly` in `https://github.com/rust-lang/crates.io-index` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:02:05] Build completed unsuccessfully in 0:00:15
[00:02:05] make: *** [prepare] Error 1
[00:02:05] Makefile:69: recipe for target 'prepare' failed
[00:02:06] Command failed. Attempt 2/5:
[00:02:06] Command failed. Attempt 2/5:
[00:02:07] error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
[00:02:07] 
[00:02:07] Caused by:
[00:02:07]   patch for `rustfmt-nightly` in `https://github.com/rust-lang/crates.io-index` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:02:07] Build completed unsuccessfully in 0:00:00
[00:02:07] Makefile:69: recipe for target 'prepare' failed
[00:02:07] make: *** [prepare] Error 1
[00:02:09] Command failed. Attempt 3/5:
[00:02:09] Command failed. Attempt 3/5:
[00:02:09] error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
[00:02:09] 
[00:02:09] Caused by:
[00:02:09]   patch for `rustfmt-nightly` in `https://github.com/rust-lang/crates.io-index` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:02:09] Build completed unsuccessfully in 0:00:00
[00:02:09] Makefile:69: recipe for target 'prepare' failed
[00:02:09] make: *** [prepare] Error 1
[00:02:12] Command failed. Attempt 4/5:
[00:02:12] Command failed. Attempt 4/5:
[00:02:12] error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
[00:02:12] 
[00:02:12] Caused by:
[00:02:12]   patch for `rustfmt-nightly` in `https://github.com/rust-lang/crates.io-index` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:02:12] Build completed unsuccessfully in 0:00:00
[00:02:12] make: *** [prepare] Error 1
[00:02:12] Makefile:69: recipe for target 'prepare' failed
[00:02:16] Command failed. Attempt 5/5:
[00:02:16] Command failed. Attempt 5/5:
[00:02:16] error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
[00:02:16] 
[00:02:16] Caused by:
[00:02:16]   patch for `rustfmt-nightly` in `https://github.com/rust-lang/crates.io-index` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:02:16] Build completed unsuccessfully in 0:00:00
[00:02:16] make: *** [prepare] Error 1
[00:02:16] Makefile:69: recipe for target 'prepare' failed
[00:02:16] The command has failed after 5 attempts.
---
travis_time:end:281973b4:start=1554066355748232927,finish=1554066355755277710,duration=7044783
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0b0c9fbc
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:06830274
travis_time:start:06830274
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:041cda52
$ dmesg | grep -i kill
