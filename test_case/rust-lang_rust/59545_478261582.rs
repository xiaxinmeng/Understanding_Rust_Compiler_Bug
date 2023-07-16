plain
travis_time:end:13bcb01a:start=1553963113302374990,finish=1553963115650547509,duration=2348172519
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:01:47] 
######################################################################## 100.0%
[00:01:48] extracting /checkout/obj/build/cache/2019-03-20/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:48]     Updating crates.io index
[00:02:02] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:02] Build completed unsuccessfully in 0:00:27
[00:02:02] Makefile:69: recipe for target 'prepare' failed
[00:02:02] make: *** [prepare] Error 1
[00:02:03] Command failed. Attempt 2/5:
[00:02:03] Command failed. Attempt 2/5:
[00:02:04] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:04] Build completed unsuccessfully in 0:00:00
[00:02:04] make: *** [prepare] Error 1
[00:02:04] Makefile:69: recipe for target 'prepare' failed
[00:02:06] Command failed. Attempt 3/5:
[00:02:06] Command failed. Attempt 3/5:
[00:02:06] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:06] Build completed unsuccessfully in 0:00:00
[00:02:06] Makefile:69: recipe for target 'prepare' failed
[00:02:06] make: *** [prepare] Error 1
[00:02:09] Command failed. Attempt 4/5:
[00:02:09] Command failed. Attempt 4/5:
[00:02:10] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:10] Build completed unsuccessfully in 0:00:00
[00:02:10] make: *** [prepare] Error 1
[00:02:10] Makefile:69: recipe for target 'prepare' failed
[00:02:14] Command failed. Attempt 5/5:
[00:02:14] Command failed. Attempt 5/5:
[00:02:14] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:14] Build completed unsuccessfully in 0:00:00
[00:02:14] Makefile:69: recipe for target 'prepare' failed
[00:02:14] make: *** [prepare] Error 1
[00:02:14] The command has failed after 5 attempts.
---
travis_time:end:2f7e1dac:start=1553963263560597429,finish=1553963263567736470,duration=7139041
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:105e7584
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:00471a38
travis_time:start:00471a38
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:02900c3a
$ dmesg | grep -i kill
