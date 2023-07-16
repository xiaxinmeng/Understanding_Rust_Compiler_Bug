plain
travis_time:end:1c25d732:start=1548093176753043609,finish=1548093177632981698,duration=879938089
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:00:00] Submodule 'src/doc/edition-guide' (https://github.com/rust-lang-nursery/edition-guide) registered for path 'src/doc/edition-guide'
[00:00:00] Submodule 'src/doc/nomicon' (https://github.com/rust-lang-nursery/nomicon.git) registered for path 'src/doc/nomicon'
[00:00:00] Submodule 'src/doc/reference' (https://github.com/rust-lang-nursery/reference.git) registered for path 'src/doc/reference'
[00:00:00] Submodule 'src/doc/rustc-guide' (https://github.com/rust-lang/rustc-guide.git) registered for path 'src/doc/rustc-guide'
[00:00:00] Submodule 'src/stdsimd' (https://github.com/gnzlbg/stdsimd.git) registered for path 'src/stdsimd'
[00:00:00] Submodule 'src/tools/clippy' (https://github.com/rust-lang-nursery/rust-clippy.git) registered for path 'src/tools/clippy'
[00:00:00] Submodule 'src/tools/miri' (https://github.com/solson/miri.git) registered for path 'src/tools/miri'
[00:00:00] Submodule 'src/tools/rls' (https://github.com/rust-lang-nursery/rls.git) registered for path 'src/tools/rls'
[00:00:00] Submodule 'src/rust-installer' (https://github.com/rust-lang/rust-installer.git) registered for path 'src/tools/rust-installer'
---
##############################################################            86.2%
######################################################################## 100.0%
[00:01:56] extracting /checkout/obj/build/cache/2019-01-04/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:56]     Updating crates.io index
[00:02:06]     Updating git repository `https://github.com/gnzlbg/stdsimd`
[00:02:07] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:07] Build completed unsuccessfully in 0:00:26
[00:02:07] Makefile:71: recipe for target 'prepare' failed
[00:02:07] make: *** [prepare] Error 1
[00:02:08] Command failed. Attempt 2/5:
[00:02:08] Command failed. Attempt 2/5:
[00:02:08]     Updating git repository `https://github.com/gnzlbg/stdsimd`
[00:02:09] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:09] Build completed unsuccessfully in 0:00:00
[00:02:09] Makefile:71: recipe for target 'prepare' failed
[00:02:09] make: *** [prepare] Error 1
[00:02:11] Command failed. Attempt 3/5:
[00:02:11] Command failed. Attempt 3/5:
[00:02:11]     Updating git repository `https://github.com/gnzlbg/stdsimd`
[00:02:11] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:11] Build completed unsuccessfully in 0:00:00
[00:02:11] Makefile:71: recipe for target 'prepare' failed
[00:02:11] make: *** [prepare] Error 1
[00:02:14] Command failed. Attempt 4/5:
[00:02:14] Command failed. Attempt 4/5:
[00:02:14]     Updating git repository `https://github.com/gnzlbg/stdsimd`
[00:02:15] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:15] Build completed unsuccessfully in 0:00:00
[00:02:15] Makefile:71: recipe for target 'prepare' failed
[00:02:15] make: *** [prepare] Error 1
[00:02:19] Command failed. Attempt 5/5:
[00:02:19] Command failed. Attempt 5/5:
[00:02:19]     Updating git repository `https://github.com/gnzlbg/stdsimd`
[00:02:19] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:19] Build completed unsuccessfully in 0:00:00
[00:02:19] Makefile:71: recipe for target 'prepare' failed
[00:02:19] make: *** [prepare] Error 1
[00:02:19] The command has failed after 5 attempts.
---
travis_time:end:0974384a:start=1548093331876735984,finish=1548093331883289736,duration=6553752
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1af515ec
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0538f7da
travis_time:start:0538f7da
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:120c3480
$ dmesg | grep -i kill
