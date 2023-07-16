plain
##########################                                                36.2%
######################################################################## 100.0%
[00:01:59] extracting /checkout/obj/build/cache/2018-08-01/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:59]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:22] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:22] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:22] Makefile:81: recipe for target 'prepare' failed
[00:02:22] make: *** [prepare] Error 1
[00:02:23] Command failed. Attempt 2/5:
[00:02:23]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:23]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:23] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:23] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:23] Makefile:81: recipe for target 'prepare' failed
[00:02:23] make: *** [prepare] Error 1
[00:02:25] Command failed. Attempt 3/5:
[00:02:25]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:25]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:26] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:26] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:26] Makefile:81: recipe for target 'prepare' failed
[00:02:26] make: *** [prepare] Error 1
[00:02:29] Command failed. Attempt 4/5:
[00:02:29]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:29]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:29] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:29] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:29] make: *** [prepare] Error 1
[00:02:29] Makefile:81: recipe for target 'prepare' failed
[00:02:33] Command failed. Attempt 5/5:
[00:02:33]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:33]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:34] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:34] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:34] Makefile:81: recipe for target 'prepare' failed
[00:02:34] make: *** [prepare] Error 1
[00:02:34] The command has failed after 5 attempts.
travis_time:end:000f2b98:start=1535757820279338279,finish=1535757990449408057,duration=170170069778
---
travis_time:end:05a6fff6:start=1535757990866516336,finish=1535757990874825456,duration=8309120
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1ecd35de
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1a566b38
travis_time:start:1a566b38
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0272bba5
$ dmesg | grep -i kill
