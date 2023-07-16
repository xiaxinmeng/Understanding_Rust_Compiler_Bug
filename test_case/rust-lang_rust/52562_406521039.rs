plain
###################################                                       49.5%
######################################################################## 100.0%
[00:01:25] extracting /checkout/obj/build/cache/2018-07-13/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:25]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:01:48] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:48] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:48] make: *** [prepare] Error 1
[00:01:48] Makefile:81: recipe for target 'prepare' failed
[00:01:49] Command failed. Attempt 2/5:
[00:01:49]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:01:49]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:01:49] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:49] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:49] make: *** [prepare] Error 1
[00:01:49] Makefile:81: recipe for target 'prepare' failed
[00:01:51] Command failed. Attempt 3/5:
[00:01:51]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:01:51]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:01:52] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:52] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:52] Makefile:81: recipe for target 'prepare' failed
[00:01:52] make: *** [prepare] Error 1
[00:01:55] Command failed. Attempt 4/5:
[00:01:55]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:01:55]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:01:56] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:56] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:56] Makefile:81: recipe for target 'prepare' failed
[00:01:56] make: *** [prepare] Error 1
[00:02:00] Command failed. Attempt 5/5:
[00:02:00]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:00]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:00] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:00] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:00] make: *** [prepare] Error 1
[00:02:00] Makefile:81: recipe for target 'prepare' failed
[00:02:00] The command has failed after 5 attempts.
travis_time:end:057d05f3:start=1532073589498275237,finish=1532073716333599863,duration=126835324626
---
travis_time:end:0374be99:start=1532073716721943213,finish=1532073716734478164,duration=12534951
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:031bca2b
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:09441980
travis_time:start:09441980
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1621fe06
$ dmesg | grep -i kill
