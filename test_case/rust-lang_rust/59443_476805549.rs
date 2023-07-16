plain
travis_time:end:089adce6:start=1553627451320244568,finish=1553627453709124590,duration=2388880022
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:01:33] 
######################################################################## 100.0%
[00:01:33] extracting /checkout/obj/build/cache/2019-03-20/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:33]     Updating crates.io index
[00:01:47] error: failed to select a version for the requirement `cfg-if = "^1.0"`
[00:01:47]   candidate versions found which didn't match: 0.1.7, 0.1.6, 0.1.5, ...
[00:01:47]   location searched: crates.io index
[00:01:47] required by package `core v0.0.0 (/checkout/src/libcore)`
[00:01:47] Build completed unsuccessfully in 0:00:28
[00:01:47] make: *** [prepare] Error 1
[00:01:47] Makefile:69: recipe for target 'prepare' failed
[00:01:48] Command failed. Attempt 2/5:
[00:01:48] Command failed. Attempt 2/5:
[00:01:48]     Updating crates.io index
[00:01:48] error: failed to select a version for the requirement `cfg-if = "^1.0"`
[00:01:48]   candidate versions found which didn't match: 0.1.7, 0.1.6, 0.1.5, ...
[00:01:48]   location searched: crates.io index
[00:01:48] required by package `core v0.0.0 (/checkout/src/libcore)`
[00:01:48] Build completed unsuccessfully in 0:00:00
[00:01:48] Makefile:69: recipe for target 'prepare' failed
[00:01:48] make: *** [prepare] Error 1
[00:01:50] Command failed. Attempt 3/5:
[00:01:50] Command failed. Attempt 3/5:
[00:01:51]     Updating crates.io index
[00:01:51] error: failed to select a version for the requirement `cfg-if = "^1.0"`
[00:01:51]   candidate versions found which didn't match: 0.1.7, 0.1.6, 0.1.5, ...
[00:01:51]   location searched: crates.io index
[00:01:51] required by package `core v0.0.0 (/checkout/src/libcore)`
[00:01:51] Build completed unsuccessfully in 0:00:00
[00:01:51] make: *** [prepare] Error 1
[00:01:51] Makefile:69: recipe for target 'prepare' failed
[00:01:54] Command failed. Attempt 4/5:
[00:01:54] Command failed. Attempt 4/5:
[00:01:54]     Updating crates.io index
[00:01:54] error: failed to select a version for the requirement `cfg-if = "^1.0"`
[00:01:54]   candidate versions found which didn't match: 0.1.7, 0.1.6, 0.1.5, ...
[00:01:54]   location searched: crates.io index
[00:01:54] required by package `core v0.0.0 (/checkout/src/libcore)`
[00:01:54] Build completed unsuccessfully in 0:00:00
[00:01:54] Makefile:69: recipe for target 'prepare' failed
[00:01:54] make: *** [prepare] Error 1
[00:01:58] Command failed. Attempt 5/5:
[00:01:58] Command failed. Attempt 5/5:
[00:01:59]     Updating crates.io index
[00:01:59] error: failed to select a version for the requirement `cfg-if = "^1.0"`
[00:01:59]   candidate versions found which didn't match: 0.1.7, 0.1.6, 0.1.5, ...
[00:01:59]   location searched: crates.io index
[00:01:59] required by package `core v0.0.0 (/checkout/src/libcore)`
[00:01:59] Build completed unsuccessfully in 0:00:00
[00:01:59] Makefile:69: recipe for target 'prepare' failed
[00:01:59] make: *** [prepare] Error 1
[00:01:59] The command has failed after 5 attempts.
---
travis_time:end:163194c4:start=1553627586646013821,finish=1553627586652613202,duration=6599381
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:216fa77c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:02b6feba
travis_time:start:02b6feba
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:35472941
$ dmesg | grep -i kill
