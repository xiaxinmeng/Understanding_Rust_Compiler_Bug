plain
travis_time:end:213d55f1:start=1548950613012460478,finish=1548950614004998827,duration=992538349
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:02:03] 
##############################################                            64.6%
######################################################################## 100.0%
[00:02:03] extracting /checkout/obj/build/cache/2019-01-18/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:02:03] error: failed to read `/par/rayon-tlv/Cargo.toml`
[00:02:03] Caused by:
[00:02:03]   No such file or directory (os error 2)
[00:02:03] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:03] Build completed unsuccessfully in 0:00:13
[00:02:03] Build completed unsuccessfully in 0:00:13
[00:02:03] make: *** [prepare] Error 1
[00:02:03] Makefile:70: recipe for target 'prepare' failed
[00:02:04] Command failed. Attempt 2/5:
[00:02:04] error: failed to read `/par/rayon-tlv/Cargo.toml`
[00:02:04] Caused by:
[00:02:04]   No such file or directory (os error 2)
[00:02:04] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:04] Build completed unsuccessfully in 0:00:00
[00:02:04] Build completed unsuccessfully in 0:00:00
[00:02:04] Makefile:70: recipe for target 'prepare' failed
[00:02:04] make: *** [prepare] Error 1
[00:02:06] Command failed. Attempt 3/5:
[00:02:06] error: failed to read `/par/rayon-tlv/Cargo.toml`
[00:02:06] Caused by:
[00:02:06]   No such file or directory (os error 2)
[00:02:06] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:06] Build completed unsuccessfully in 0:00:00
[00:02:06] Build completed unsuccessfully in 0:00:00
[00:02:06] Makefile:70: recipe for target 'prepare' failed
[00:02:06] make: *** [prepare] Error 1
[00:02:09] Command failed. Attempt 4/5:
[00:02:10] error: failed to read `/par/rayon-tlv/Cargo.toml`
[00:02:10] Caused by:
[00:02:10]   No such file or directory (os error 2)
[00:02:10] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:10] Build completed unsuccessfully in 0:00:00
[00:02:10] Build completed unsuccessfully in 0:00:00
[00:02:10] make: *** [prepare] Error 1
[00:02:10] Makefile:70: recipe for target 'prepare' failed
[00:02:14] Command failed. Attempt 5/5:
[00:02:14] error: failed to read `/par/rayon-tlv/Cargo.toml`
[00:02:14] Caused by:
[00:02:14]   No such file or directory (os error 2)
[00:02:14] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:14] Build completed unsuccessfully in 0:00:00
---
travis_time:end:26b7ab04:start=1548950762198461327,finish=1548950762203164341,duration=4703014
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:13727568
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1723865a
travis_time:start:1723865a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:06c07b54
$ dmesg | grep -i kill
