plain
travis_time:end:04ef53ec:start=1561381100243087267,finish=1561381190736450864,duration=90493363597
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
#########################################################                 79.5%
######################################################################## 100.0%
[00:01:14] extracting /checkout/obj/build/cache/2019-05-23/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:14]     Updating crates.io index
[00:01:43] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:43] Build completed unsuccessfully in 0:00:46
[00:01:43] make: *** [prepare] Error 1
[00:01:43] Makefile:69: recipe for target 'prepare' failed
[00:01:44] Command failed. Attempt 2/5:
[00:01:44] Command failed. Attempt 2/5:
[00:01:44]     Updating crates.io index
[00:01:44] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:44] Build completed unsuccessfully in 0:00:00
[00:01:44] make: *** [prepare] Error 1
[00:01:44] Makefile:69: recipe for target 'prepare' failed
[00:01:46] Command failed. Attempt 3/5:
[00:01:46] Command failed. Attempt 3/5:
[00:01:46]     Updating crates.io index
[00:01:47] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:47] Build completed unsuccessfully in 0:00:00
[00:01:47] Makefile:69: recipe for target 'prepare' failed
[00:01:47] make: *** [prepare] Error 1
[00:01:50] Command failed. Attempt 4/5:
[00:01:50] Command failed. Attempt 4/5:
[00:01:50]     Updating crates.io index
[00:01:50] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:50] Build completed unsuccessfully in 0:00:00
[00:01:50] make: *** [prepare] Error 1
[00:01:50] Makefile:69: recipe for target 'prepare' failed
[00:01:54] Command failed. Attempt 5/5:
[00:01:54] Command failed. Attempt 5/5:
[00:01:54]     Updating crates.io index
[00:01:55] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:55] Build completed unsuccessfully in 0:00:00
[00:01:55] make: *** [prepare] Error 1
[00:01:55] Makefile:69: recipe for target 'prepare' failed
[00:01:55] The command has failed after 5 attempts.
---
travis_time:end:00b53aff:start=1561381319153985322,finish=1561381319159402554,duration=5417232
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:01e40ee0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:04981a94
travis_time:start:04981a94
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0a81c644
$ dmesg | grep -i kill
