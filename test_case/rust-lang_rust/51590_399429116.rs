plain
######################################################################## 100.0%
[00:01:06] extracting /checkout/obj/build/cache/2018-05-10/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:06]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:01:25]     Updating git repository `https://github.com/rust-lang-nursery/rustfmt`
[00:01:27] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:27] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:27] Build completed unsuccessfully in 0:00:37
[00:01:27] Makefile:81: recipe for target 'prepare' failed
[00:01:27] make: *** [prepare] Error 1
[00:01:28] Command failed. Attempt 2/5:
[00:01:29] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:29] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:29] Build completed unsuccessfully in 0:00:00
[00:01:29] Makefile:81: recipe for target 'prepare' failed
[00:01:29] make: *** [prepare] Error 1
[00:01:31] Command failed. Attempt 3/5:
[00:01:31] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:31] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:31] Build completed unsuccessfully in 0:00:00
[00:01:31] Makefile:81: recipe for target 'prepare' failed
[00:01:31] make: *** [prepare] Error 1
[00:01:34] Command failed. Attempt 4/5:
[00:01:34] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:34] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:34] Build completed unsuccessfully in 0:00:00
[00:01:34] Makefile:81: recipe for target 'prepare' failed
[00:01:34] make: *** [prepare] Error 1
[00:01:38] Command failed. Attempt 5/5:
[00:01:38] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:38] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:38] Build completed unsuccessfully in 0:00:00
[00:01:38] Makefile:81: recipe for target 'prepare' failed
[00:01:38] make: *** [prepare] Error 1
[00:01:38] The command has failed after 5 attempts.

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:050bcab9
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0d422055:start=1529671229709549546,finish=1529671229715767977,duration=6218431
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0045a001
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:09a8c03d
$ dmesg | grep -i kill
