plain
travis_time:end:06f5f82d:start=1540907242926221584,finish=1540907243999233473,duration=1073011889
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
---
[00:01:45] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:01:45] error: failed to resolve patches for `https://github.com/rust-lang-nursery/rust-clippy`
[00:01:45] 
[00:01:45] Caused by:
[00:01:45]   failed to load source for a dependency on `rustc_tools_util`
[00:01:45] Caused by:
[00:01:45]   Unable to update /checkout/src/tools/clippy/rustc_tools_util
[00:01:45] 
[00:01:45] Caused by:
---
[00:01:46] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:01:46] error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
[00:01:46] 
[00:01:46] Caused by:
[00:01:46]   patch for `rustfmt-nightly` in `https://github.com/rust-lang/crates.io-index` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:01:46] Build completed unsuccessfully in 0:00:00
[00:01:46] Makefile:81: recipe for target 'prepare' failed
[00:01:46] make: *** [prepare] Error 1
[00:01:48] Command failed. Attempt 3/5:
[00:01:48] Command failed. Attempt 3/5:
[00:01:48] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:01:48] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:01:48] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:01:48] error: failed to resolve patches for `https://github.com/rust-lang-nursery/rust-clippy`
[00:01:48] 
[00:01:48] Caused by:
[00:01:48]   failed to load source for a dependency on `rustc_tools_util`
[00:01:48] Caused by:
[00:01:48]   Unable to update /checkout/src/tools/clippy/rustc_tools_util
[00:01:48] 
[00:01:48] Caused by:
---
[00:01:51] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:01:52] error: failed to resolve patches for `https://github.com/rust-lang-nursery/rust-clippy`
[00:01:52] 
[00:01:52] Caused by:
[00:01:52]   failed to load source for a dependency on `rustc_tools_util`
[00:01:52] Caused by:
[00:01:52]   Unable to update /checkout/src/tools/clippy/rustc_tools_util
[00:01:52] 
[00:01:52] Caused by:
---
[00:01:56] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:01:56] error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
[00:01:56] 
[00:01:56] Caused by:
[00:01:56]   patch for `rustfmt-nightly` in `https://github.com/rust-lang/crates.io-index` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:01:56] Build completed unsuccessfully in 0:00:00
[00:01:56] make: *** [prepare] Error 1
[00:01:56] Makefile:81: recipe for target 'prepare' failed
[00:01:56] The command has failed after 5 attempts.
---
travis_time:end:01a35425:start=1540907371132445435,finish=1540907371140159660,duration=7714225
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0671282a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:10f1f190
travis_time:start:10f1f190
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:211c5590
$ dmesg | grep -i kill
