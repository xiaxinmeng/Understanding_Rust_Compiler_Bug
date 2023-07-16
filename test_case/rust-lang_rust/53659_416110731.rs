plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:108b9853
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
############                                                              17.7%
######################################################################## 100.0%
[00:04:00] extracting /checkout/obj/build/cache/2018-08-01/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:04:02]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:04:27] error: the lock file needs to be updated but --locked was passed to prevent this
[00:04:27] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:04:27] make: *** [prepare] Error 1
[00:04:28] Command failed. Attempt 2/5:
[00:04:28] Command failed. Attempt 2/5:
[00:04:30] error: the lock file needs to be updated but --locked was passed to prevent this
[00:04:30] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:04:30] make: *** [prepare] Error 1
[00:04:32] Command failed. Attempt 3/5:
[00:04:32] Command failed. Attempt 3/5:
[00:04:33] error: the lock file needs to be updated but --locked was passed to prevent this
[00:04:33] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:04:33] make: *** [prepare] Error 1
[00:04:36] Command failed. Attempt 4/5:
[00:04:36] Command failed. Attempt 4/5:
[00:04:36] error: the lock file needs to be updated but --locked was passed to prevent this
[00:04:36] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:04:36] make: *** [prepare] Error 1
[00:04:40] Command failed. Attempt 5/5:
[00:04:40] Command failed. Attempt 5/5:
[00:04:40] error: the lock file needs to be updated but --locked was passed to prevent this
[00:04:40] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:04:40] make: *** [prepare] Error 1
[00:04:40] The command has failed after 5 attempts.
travis_time:end:26e94942:start=1535343640974327068,finish=1535343939862089430,duration=298887762362

---
travis_time:end:3274f410:start=1535343940307294801,finish=1535343940315297174,duration=8002373
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0236b348
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:274d92c3
travis_time:start:274d92c3
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0b5e2c99
$ dmesg | grep -i kill
