plain
travis_time:end:1433df90:start=1550401329510912510,finish=1550401330308322789,duration=797410279
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:01:53] warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
[00:01:53] package:   /checkout/src/tools/miri/Cargo.toml
[00:01:53] workspace: /checkout/Cargo.toml
[00:01:53]     Updating crates.io index
[00:02:04] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:04] Build completed unsuccessfully in 0:00:24
[00:02:04] make: *** [prepare] Error 1
[00:02:04] Makefile:70: recipe for target 'prepare' failed
[00:02:05] Command failed. Attempt 2/5:
[00:02:05] Command failed. Attempt 2/5:
[00:02:05] warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
[00:02:05] package:   /checkout/src/tools/miri/Cargo.toml
[00:02:05] workspace: /checkout/Cargo.toml
[00:02:06] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:06] Build completed unsuccessfully in 0:00:00
[00:02:06] make: *** [prepare] Error 1
[00:02:06] Makefile:70: recipe for target 'prepare' failed
[00:02:08] Command failed. Attempt 3/5:
[00:02:08] Command failed. Attempt 3/5:
[00:02:08] warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
[00:02:08] package:   /checkout/src/tools/miri/Cargo.toml
[00:02:08] workspace: /checkout/Cargo.toml
[00:02:08] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:08] Build completed unsuccessfully in 0:00:00
[00:02:08] make: *** [prepare] Error 1
[00:02:08] Makefile:70: recipe for target 'prepare' failed
[00:02:11] Command failed. Attempt 4/5:
[00:02:11] Command failed. Attempt 4/5:
[00:02:11] warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
[00:02:11] package:   /checkout/src/tools/miri/Cargo.toml
[00:02:11] workspace: /checkout/Cargo.toml
[00:02:11] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:11] Build completed unsuccessfully in 0:00:00
[00:02:11] Makefile:70: recipe for target 'prepare' failed
[00:02:11] make: *** [prepare] Error 1
[00:02:15] Command failed. Attempt 5/5:
[00:02:15] Command failed. Attempt 5/5:
[00:02:16] warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
[00:02:16] package:   /checkout/src/tools/miri/Cargo.toml
[00:02:16] workspace: /checkout/Cargo.toml
[00:02:16] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:16] Build completed unsuccessfully in 0:00:00
[00:02:16] Makefile:70: recipe for target 'prepare' failed
[00:02:16] The command has failed after 5 attempts.
[00:02:16] make: *** [prepare] Error 1
---
travis_time:end:0d0f990e:start=1550401479448708678,finish=1550401479454649930,duration=5941252
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:01fb9285
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:24152a38
travis_time:start:24152a38
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1ea1e9dd
$ dmesg | grep -i kill
