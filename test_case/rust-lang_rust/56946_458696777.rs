plain
travis_time:end:02bfb4c6:start=1548793653564940869,finish=1548793657726945160,duration=4162004291
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:06:40] downloading https://static.rust-lang.org/dist/2019-01-18/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:06:41] 
######################################################################## 100.0%
[00:06:41] extracting /checkout/obj/build/cache/2019-01-18/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:06:41] error: failed to read `/par/rayon-tlv/Cargo.toml`
[00:06:41] Caused by:
[00:06:41]   No such file or directory (os error 2)
[00:06:41] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:06:41] Build completed unsuccessfully in 0:01:01
[00:06:41] Build completed unsuccessfully in 0:01:01
[00:06:41] make: *** [prepare] Error 1
[00:06:41] Makefile:70: recipe for target 'prepare' failed
[00:06:42] Command failed. Attempt 2/5:
[00:06:42] error: failed to read `/par/rayon-tlv/Cargo.toml`
[00:06:42] Caused by:
[00:06:42]   No such file or directory (os error 2)
[00:06:42] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:06:42] Build completed unsuccessfully in 0:00:00
[00:06:42] Build completed unsuccessfully in 0:00:00
[00:06:42] make: *** [prepare] Error 1
[00:06:42] Makefile:70: recipe for target 'prepare' failed
[00:06:44] Command failed. Attempt 3/5:
[00:06:44] error: failed to read `/par/rayon-tlv/Cargo.toml`
[00:06:44] Caused by:
[00:06:44]   No such file or directory (os error 2)
[00:06:44] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:06:44] Build completed unsuccessfully in 0:00:00
[00:06:44] Build completed unsuccessfully in 0:00:00
[00:06:44] Makefile:70: recipe for target 'prepare' failed
[00:06:44] make: *** [prepare] Error 1
[00:06:47] Command failed. Attempt 4/5:
[00:06:47] error: failed to read `/par/rayon-tlv/Cargo.toml`
[00:06:47] Caused by:
[00:06:47]   No such file or directory (os error 2)
[00:06:47] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:06:47] Build completed unsuccessfully in 0:00:00
[00:06:47] Build completed unsuccessfully in 0:00:00
[00:06:47] Makefile:70: recipe for target 'prepare' failed
[00:06:47] make: *** [prepare] Error 1
[00:06:51] Command failed. Attempt 5/5:
[00:06:51] error: failed to read `/par/rayon-tlv/Cargo.toml`
[00:06:51] Caused by:
[00:06:51]   No such file or directory (os error 2)
[00:06:51] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:06:51] Build completed unsuccessfully in 0:00:00
---
travis_time:end:0703faa5:start=1548794081949525081,finish=1548794081956056200,duration=6531119
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1ecfc00f
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:151583d9
travis_time:start:151583d9
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:014fd860
$ dmesg | grep -i kill
