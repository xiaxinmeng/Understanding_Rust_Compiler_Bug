plain
travis_time:end:033ca5ed:start=1561538054279229470,finish=1561538143349750510,duration=89070521040
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:01:13] extracting /checkout/obj/build/cache/2019-05-23/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:13] error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
[00:01:13] 
[00:01:13] Caused by:
[00:01:13]   patch for `rustfmt-nightly` in `https://github.com/rust-lang/crates.io-index` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:01:13] Build completed unsuccessfully in 0:00:13
[00:01:13] make: *** [prepare] Error 1
[00:01:13] Makefile:69: recipe for target 'prepare' failed
[00:01:14] Command failed. Attempt 2/5:
[00:01:14] Command failed. Attempt 2/5:
[00:01:15] error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
[00:01:15] 
[00:01:15] Caused by:
[00:01:15]   patch for `rustfmt-nightly` in `https://github.com/rust-lang/crates.io-index` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:01:15] Build completed unsuccessfully in 0:00:00
[00:01:15] Makefile:69: recipe for target 'prepare' failed
[00:01:15] make: *** [prepare] Error 1
[00:01:17] Command failed. Attempt 3/5:
[00:01:17] Command failed. Attempt 3/5:
[00:01:17] error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
[00:01:17] 
[00:01:17] Caused by:
[00:01:17]   patch for `rustfmt-nightly` in `https://github.com/rust-lang/crates.io-index` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:01:17] Build completed unsuccessfully in 0:00:00
[00:01:17] Makefile:69: recipe for target 'prepare' failed
[00:01:17] make: *** [prepare] Error 1
[00:01:20] Command failed. Attempt 4/5:
[00:01:20] Command failed. Attempt 4/5:
[00:01:20] error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
[00:01:20] 
[00:01:20] Caused by:
[00:01:20]   patch for `rustfmt-nightly` in `https://github.com/rust-lang/crates.io-index` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:01:20] Build completed unsuccessfully in 0:00:00
[00:01:20] make: *** [prepare] Error 1
[00:01:20] Makefile:69: recipe for target 'prepare' failed
[00:01:24] Command failed. Attempt 5/5:
[00:01:24] Command failed. Attempt 5/5:
[00:01:24] error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
[00:01:24] 
[00:01:24] Caused by:
[00:01:24]   patch for `rustfmt-nightly` in `https://github.com/rust-lang/crates.io-index` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:01:24] Build completed unsuccessfully in 0:00:00
[00:01:24] make: *** [prepare] Error 1
[00:01:24] Makefile:69: recipe for target 'prepare' failed
[00:01:24] The command has failed after 5 attempts.
---
travis_time:end:062027d4:start=1561538241204473205,finish=1561538241210097996,duration=5624791
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:21f813c4
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:03710121
travis_time:start:03710121
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:17e8b68f
$ dmesg | grep -i kill
