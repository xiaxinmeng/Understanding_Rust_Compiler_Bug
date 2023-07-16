plain
travis_time:end:2674194c:start=1548586675562018706,finish=1548586676523200066,duration=961181360
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:01:58] 
######################################################################## 100.0%
[00:01:58] extracting /checkout/obj/build/cache/2019-01-04/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:58]     Updating crates.io index
[00:02:09] error: checksum for `libc v0.2.48` changed between lock files
[00:02:09] 
[00:02:09] this could be indicative of a few possible errors:
[00:02:09]     * the lock file is corrupt
[00:02:09]     * the lock file is corrupt
[00:02:09]     * a replacement source in use (e.g. a mirror) returned a different checksum
[00:02:09]     * the source itself may be corrupt in one way or another
[00:02:09] 
[00:02:09] unable to verify that `libc v0.2.48` is the same as when the lockfile was generated
[00:02:09] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:09] Build completed unsuccessfully in 0:00:23
[00:02:09] Makefile:70: recipe for target 'prepare' failed
[00:02:09] make: *** [prepare] Error 1
[00:02:09] make: *** [prepare] Error 1
[00:02:10] Command failed. Attempt 2/5:
[00:02:10] error: checksum for `libc v0.2.48` changed between lock files
[00:02:10] 
[00:02:10] this could be indicative of a few possible errors:
[00:02:10]     * the lock file is corrupt
[00:02:10]     * the lock file is corrupt
[00:02:10]     * a replacement source in use (e.g. a mirror) returned a different checksum
[00:02:10]     * the source itself may be corrupt in one way or another
[00:02:10] 
[00:02:10] unable to verify that `libc v0.2.48` is the same as when the lockfile was generated
[00:02:10] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:10] Build completed unsuccessfully in 0:00:00
[00:02:10] Makefile:70: recipe for target 'prepare' failed
[00:02:10] make: *** [prepare] Error 1
[00:02:10] make: *** [prepare] Error 1
[00:02:12] Command failed. Attempt 3/5:
[00:02:13] error: checksum for `libc v0.2.48` changed between lock files
[00:02:13] 
[00:02:13] this could be indicative of a few possible errors:
[00:02:13]     * the lock file is corrupt
[00:02:13]     * the lock file is corrupt
[00:02:13]     * a replacement source in use (e.g. a mirror) returned a different checksum
[00:02:13]     * the source itself may be corrupt in one way or another
[00:02:13] 
[00:02:13] unable to verify that `libc v0.2.48` is the same as when the lockfile was generated
[00:02:13] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:13] Build completed unsuccessfully in 0:00:00
[00:02:13] Makefile:70: recipe for target 'prepare' failed
[00:02:13] make: *** [prepare] Error 1
[00:02:13] make: *** [prepare] Error 1
[00:02:16] Command failed. Attempt 4/5:
[00:02:16] error: checksum for `libc v0.2.48` changed between lock files
[00:02:16] 
[00:02:16] this could be indicative of a few possible errors:
[00:02:16]     * the lock file is corrupt
[00:02:16]     * the lock file is corrupt
[00:02:16]     * a replacement source in use (e.g. a mirror) returned a different checksum
[00:02:16]     * the source itself may be corrupt in one way or another
[00:02:16] 
[00:02:16] unable to verify that `libc v0.2.48` is the same as when the lockfile was generated
[00:02:16] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:16] Build completed unsuccessfully in 0:00:00
[00:02:16] make: *** [prepare] Error 1
[00:02:16] Makefile:70: recipe for target 'prepare' failed
[00:02:16] Makefile:70: recipe for target 'prepare' failed
[00:02:20] Command failed. Attempt 5/5:
[00:02:20] error: checksum for `libc v0.2.48` changed between lock files
[00:02:20] 
[00:02:20] this could be indicative of a few possible errors:
[00:02:20]     * the lock file is corrupt
[00:02:20]     * the lock file is corrupt
[00:02:20]     * a replacement source in use (e.g. a mirror) returned a different checksum
[00:02:20]     * the source itself may be corrupt in one way or another
[00:02:20] 
[00:02:20] unable to verify that `libc v0.2.48` is the same as when the lockfile was generated
[00:02:20] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:20] Build completed unsuccessfully in 0:00:00
[00:02:20] Makefile:70: recipe for target 'prepare' failed
[00:02:20] make: *** [prepare] Error 1
---
travis_time:end:035a7a55:start=1548586832385194847,finish=1548586832390012699,duration=4817852
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0046f3ef
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:004c6b4d
travis_time:start:004c6b4d
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00af7e05
$ dmesg | grep -i kill
