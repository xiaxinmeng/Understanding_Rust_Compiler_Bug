plain
travis_time:end:005af703:start=1559506999528360217,finish=1559507000326258756,duration=797898539
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
############################################                              61.5%
######################################################################## 100.0%
[00:01:38] extracting /checkout/obj/build/cache/2019-05-23/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:38]     Updating crates.io index
[00:01:57] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:57] Build completed unsuccessfully in 0:00:32
[00:01:57] Makefile:69: recipe for target 'prepare' failed
[00:01:57] make: *** [prepare] Error 1
[00:01:58] Command failed. Attempt 2/5:
[00:01:58] Command failed. Attempt 2/5:
[00:01:58]     Updating crates.io index
[00:01:58] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:58] Build completed unsuccessfully in 0:00:00
[00:01:58] Makefile:69: recipe for target 'prepare' failed
[00:01:58] make: *** [prepare] Error 1
[00:02:00] Command failed. Attempt 3/5:
[00:02:00] Command failed. Attempt 3/5:
[00:02:01]     Updating crates.io index
[00:02:01] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:01] Build completed unsuccessfully in 0:00:00
[00:02:01] Makefile:69: recipe for target 'prepare' failed
[00:02:01] make: *** [prepare] Error 1
[00:02:04] Command failed. Attempt 4/5:
[00:02:04] Command failed. Attempt 4/5:
[00:02:04]     Updating crates.io index
[00:02:04] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:04] Build completed unsuccessfully in 0:00:00
[00:02:04] make: *** [prepare] Error 1
[00:02:04] Makefile:69: recipe for target 'prepare' failed
[00:02:08] Command failed. Attempt 5/5:
[00:02:08] Command failed. Attempt 5/5:
[00:02:09]     Updating crates.io index
[00:02:09] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:09] Build completed unsuccessfully in 0:00:00
[00:02:09] make: *** [prepare] Error 1
[00:02:09] Makefile:69: recipe for target 'prepare' failed
[00:02:09] The command has failed after 5 attempts.
---
travis_time:end:0d77300c:start=1559507141316852170,finish=1559507141323047443,duration=6195273
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:22c6ab7a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0adecd88
travis_time:start:0adecd88
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:05fa2ca0
$ dmesg | grep -i kill
