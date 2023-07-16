plain
##################################################                        69.8%
######################################################################## 100.0%
[00:01:58] extracting /checkout/obj/build/cache/2018-08-01/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:02:00]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:23] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:23] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:23] Makefile:81: recipe for target 'prepare' failed
[00:02:23] make: *** [prepare] Error 1
[00:02:24] Command failed. Attempt 2/5:
[00:02:24]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:24]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:25] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:25] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:25] Makefile:81: recipe for target 'prepare' failed
[00:02:25] make: *** [prepare] Error 1
[00:02:27] Command failed. Attempt 3/5:
[00:02:27]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:27]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:27] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:27] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:27] Makefile:81: recipe for target 'prepare' failed
[00:02:27] make: *** [prepare] Error 1
[00:02:30] Command failed. Attempt 4/5:
[00:02:30]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:30]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:31] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:31] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:31] Makefile:81: recipe for target 'prepare' failed
[00:02:31] make: *** [prepare] Error 1
[00:02:35] Command failed. Attempt 5/5:
[00:02:35]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:35]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:35] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:35] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:35] Makefile:81: recipe for target 'prepare' failed
[00:02:35] make: *** [prepare] Error 1
[00:02:35] The command has failed after 5 attempts.
travis_time:end:0dcfbeea:start=1535064729664454389,finish=1535064891877610796,duration=162213156407
---
travis_time:end:006157ad:start=1535064892370594622,finish=1535064892383636310,duration=13041688
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:05f3b9cd
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:039d02b7
travis_time:start:039d02b7
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:01edb894
$ dmesg | grep -i kill
