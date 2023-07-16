plain
travis_time:end:2ad8b489:start=1560191022840849948,finish=1560191023618423792,duration=777573844
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
#########################################################                 80.4%
######################################################################## 100.0%
[00:02:00] extracting /checkout/obj/build/cache/2019-05-23/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:02:00]     Updating crates.io index
[00:02:36] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:36] Build completed unsuccessfully in 0:00:48
[00:02:36] make: *** [prepare] Error 1
[00:02:36] Makefile:69: recipe for target 'prepare' failed
[00:02:37] Command failed. Attempt 2/5:
[00:02:37] Command failed. Attempt 2/5:
[00:02:37]     Updating crates.io index
[00:02:37] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:37] Build completed unsuccessfully in 0:00:00
[00:02:37] make: *** [prepare] Error 1
[00:02:37] Makefile:69: recipe for target 'prepare' failed
[00:02:39] Command failed. Attempt 3/5:
[00:02:39] Command failed. Attempt 3/5:
[00:02:39]     Updating crates.io index
[00:02:40] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:40] Build completed unsuccessfully in 0:00:00
[00:02:40] make: *** [prepare] Error 1
[00:02:40] Makefile:69: recipe for target 'prepare' failed
[00:02:43] Command failed. Attempt 4/5:
[00:02:43] Command failed. Attempt 4/5:
[00:02:43]     Updating crates.io index
[00:02:43] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:43] Build completed unsuccessfully in 0:00:00
[00:02:43] make: *** [prepare] Error 1
[00:02:43] Makefile:69: recipe for target 'prepare' failed
[00:02:47] Command failed. Attempt 5/5:
[00:02:47] Command failed. Attempt 5/5:
[00:02:48]     Updating crates.io index
[00:02:48] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:48] Build completed unsuccessfully in 0:00:00
[00:02:48] make: *** [prepare] Error 1
[00:02:48] Makefile:69: recipe for target 'prepare' failed
[00:02:48] The command has failed after 5 attempts.
---
travis_time:end:03ffc403:start=1560191204021648209,finish=1560191204026707608,duration=5059399
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:07e40618
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2c35b8c5
travis_time:start:2c35b8c5
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1b8d21b6
$ dmesg | grep -i kill
