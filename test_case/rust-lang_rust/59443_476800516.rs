plain
travis_time:end:00473d78:start=1553626662724647100,finish=1553626665183227254,duration=2458580154
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
############################################################              83.9%
######################################################################## 100.0%
[00:01:45] extracting /checkout/obj/build/cache/2019-03-20/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:45]     Updating crates.io index
[00:01:59] error: failed to select a version for the requirement `cfg-if = "^1.0"`
[00:01:59]   candidate versions found which didn't match: 0.1.7, 0.1.6, 0.1.5, ...
[00:01:59]   location searched: crates.io index
[00:01:59] required by package `core v0.0.0 (/checkout/src/libcore)`
[00:01:59] Build completed unsuccessfully in 0:00:28
[00:01:59] make: *** [prepare] Error 1
[00:01:59] Makefile:69: recipe for target 'prepare' failed
[00:02:00] Command failed. Attempt 2/5:
[00:02:00] Command failed. Attempt 2/5:
[00:02:00]     Updating crates.io index
[00:02:00] error: failed to select a version for the requirement `cfg-if = "^1.0"`
[00:02:00]   candidate versions found which didn't match: 0.1.7, 0.1.6, 0.1.5, ...
[00:02:00]   location searched: crates.io index
[00:02:00] required by package `core v0.0.0 (/checkout/src/libcore)`
[00:02:00] Build completed unsuccessfully in 0:00:00
[00:02:00] make: *** [prepare] Error 1
[00:02:00] Makefile:69: recipe for target 'prepare' failed
[00:02:02] Command failed. Attempt 3/5:
[00:02:02] Command failed. Attempt 3/5:
[00:02:02]     Updating crates.io index
[00:02:03] error: failed to select a version for the requirement `cfg-if = "^1.0"`
[00:02:03]   candidate versions found which didn't match: 0.1.7, 0.1.6, 0.1.5, ...
[00:02:03]   location searched: crates.io index
[00:02:03] required by package `core v0.0.0 (/checkout/src/libcore)`
[00:02:03] Build completed unsuccessfully in 0:00:00
[00:02:03] make: *** [prepare] Error 1
[00:02:03] Makefile:69: recipe for target 'prepare' failed
[00:02:06] Command failed. Attempt 4/5:
[00:02:06] Command failed. Attempt 4/5:
[00:02:06]     Updating crates.io index
[00:02:08] error: failed to select a version for the requirement `cfg-if = "^1.0"`
[00:02:08]   candidate versions found which didn't match: 0.1.7, 0.1.6, 0.1.5, ...
[00:02:08]   location searched: crates.io index
[00:02:08] required by package `core v0.0.0 (/checkout/src/libcore)`
[00:02:08] Build completed unsuccessfully in 0:00:01
[00:02:08] make: *** [prepare] Error 1
[00:02:08] Makefile:69: recipe for target 'prepare' failed
[00:02:12] Command failed. Attempt 5/5:
[00:02:12] Command failed. Attempt 5/5:
[00:02:12]     Updating crates.io index
[00:02:13] error: failed to select a version for the requirement `cfg-if = "^1.0"`
[00:02:13]   candidate versions found which didn't match: 0.1.7, 0.1.6, 0.1.5, ...
[00:02:13]   location searched: crates.io index
[00:02:13] required by package `core v0.0.0 (/checkout/src/libcore)`
[00:02:13] Build completed unsuccessfully in 0:00:01
[00:02:13] make: *** [prepare] Error 1
[00:02:13] Makefile:69: recipe for target 'prepare' failed
[00:02:13] The command has failed after 5 attempts.
---
travis_time:end:0fc28472:start=1553626811290366903,finish=1553626811301308339,duration=10941436
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0accb844
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0058a2a0
travis_time:start:0058a2a0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2a424300
$ dmesg | grep -i kill
