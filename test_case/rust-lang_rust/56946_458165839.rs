plain
travis_time:end:033a3257:start=1548687577477865491,finish=1548687578611090201,duration=1133224710
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:03:02] downloading https://static.rust-lang.org/dist/2019-01-18/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:03:02] 
######################################################################## 100.0%
[00:03:03] extracting /checkout/obj/build/cache/2019-01-18/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:03:03] error: failed to read `/par/rayon-tlv/Cargo.toml`
[00:03:03] Caused by:
[00:03:03]   No such file or directory (os error 2)
[00:03:03] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:03:03] Build completed unsuccessfully in 0:00:20
[00:03:03] Build completed unsuccessfully in 0:00:20
[00:03:03] make: *** [prepare] Error 1
[00:03:03] Makefile:70: recipe for target 'prepare' failed
[00:03:04] Command failed. Attempt 2/5:
[00:03:04] error: failed to read `/par/rayon-tlv/Cargo.toml`
[00:03:04] Caused by:
[00:03:04]   No such file or directory (os error 2)
[00:03:04] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:03:04] Build completed unsuccessfully in 0:00:00
[00:03:04] Build completed unsuccessfully in 0:00:00
[00:03:04] Makefile:70: recipe for target 'prepare' failed
[00:03:04] make: *** [prepare] Error 1
[00:03:06] Command failed. Attempt 3/5:
[00:03:06] error: failed to read `/par/rayon-tlv/Cargo.toml`
[00:03:06] Caused by:
[00:03:06]   No such file or directory (os error 2)
[00:03:06] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:03:06] Build completed unsuccessfully in 0:00:00
[00:03:06] Build completed unsuccessfully in 0:00:00
[00:03:06] make: *** [prepare] Error 1
[00:03:06] Makefile:70: recipe for target 'prepare' failed
[00:03:09] Command failed. Attempt 4/5:
[00:03:09] error: failed to read `/par/rayon-tlv/Cargo.toml`
[00:03:09] Caused by:
[00:03:09]   No such file or directory (os error 2)
[00:03:09] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:03:09] Build completed unsuccessfully in 0:00:00
[00:03:09] Build completed unsuccessfully in 0:00:00
[00:03:09] make: *** [prepare] Error 1
[00:03:09] Makefile:70: recipe for target 'prepare' failed
[00:03:13] Command failed. Attempt 5/5:
[00:03:13] error: failed to read `/par/rayon-tlv/Cargo.toml`
[00:03:13] Caused by:
[00:03:13]   No such file or directory (os error 2)
[00:03:13] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:03:13] Build completed unsuccessfully in 0:00:00
---
travis_time:end:115607eb:start=1548687786851108165,finish=1548687786856004355,duration=4896190
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0efa67b0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0d089c00
travis_time:start:0d089c00
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:085dc3f4
$ dmesg | grep -i kill
