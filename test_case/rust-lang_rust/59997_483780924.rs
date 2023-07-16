plain
travis_time:end:0d8963d8:start=1555436632363393173,finish=1555436914175675560,duration=281812282387
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:01:48] extracting /checkout/obj/build/cache/2019-03-20/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:49] error: failed to resolve patches for `https://github.com/rust-lang/cargo`
[00:01:49] 
[00:01:49] Caused by:
[00:01:49]   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:01:49] Build completed unsuccessfully in 0:00:34
[00:01:49] make: *** [prepare] Error 1
[00:01:49] Makefile:69: recipe for target 'prepare' failed
[00:01:50] Command failed. Attempt 2/5:
[00:01:50] Command failed. Attempt 2/5:
[00:01:50] error: failed to resolve patches for `https://github.com/rust-lang/cargo`
[00:01:50] 
[00:01:50] Caused by:
[00:01:50]   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:01:50] Build completed unsuccessfully in 0:00:00
[00:01:50] Makefile:69: recipe for target 'prepare' failed
[00:01:50] make: *** [prepare] Error 1
[00:01:52] Command failed. Attempt 3/5:
[00:01:52] Command failed. Attempt 3/5:
[00:01:52] error: failed to resolve patches for `https://github.com/rust-lang/cargo`
[00:01:52] 
[00:01:52] Caused by:
[00:01:52]   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:01:52] Build completed unsuccessfully in 0:00:00
[00:01:52] Makefile:69: recipe for target 'prepare' failed
[00:01:52] make: *** [prepare] Error 1
[00:01:55] Command failed. Attempt 4/5:
[00:01:55] Command failed. Attempt 4/5:
[00:01:55] error: failed to resolve patches for `https://github.com/rust-lang/cargo`
[00:01:55] 
[00:01:55] Caused by:
[00:01:55]   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:01:55] Build completed unsuccessfully in 0:00:00
[00:01:55] make: *** [prepare] Error 1
[00:01:55] Makefile:69: recipe for target 'prepare' failed
[00:01:59] Command failed. Attempt 5/5:
[00:01:59] Command failed. Attempt 5/5:
[00:01:59] error: failed to resolve patches for `https://github.com/rust-lang/cargo`
[00:01:59] 
[00:01:59] Caused by:
[00:01:59]   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:01:59] Build completed unsuccessfully in 0:00:00
[00:01:59] make: *** [prepare] Error 1
[00:01:59] Makefile:69: recipe for target 'prepare' failed
[00:01:59] The command has failed after 5 attempts.
---
travis_time:end:0736773c:start=1555437048102739529,finish=1555437048107644236,duration=4904707
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:01747384
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:00477570
travis_time:start:00477570
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:12337698
$ dmesg | grep -i kill
