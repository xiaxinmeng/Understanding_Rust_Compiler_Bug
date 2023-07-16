plain
[00:01:15] extracting /checkout/obj/build/cache/2018-05-10/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:15] error: failed to parse manifest at `/checkout/src/tools/clippy/Cargo.toml`
[00:01:15] 
[00:01:15] Caused by:
[00:01:15]   the cargo feature `edition` requires a nightly version of Cargo, but this is the `beta` channel
[00:01:15] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:15] Build completed unsuccessfully in 0:00:22
[00:01:15] Makefile:81: recipe for target 'prepare' failed
[00:01:15] make: *** [prepare] Error 1
[00:01:16] error: failed to parse manifest at `/checkout/src/tools/clippy/Cargo.toml`
[00:01:16] 
[00:01:16] Caused by:
[00:01:16] Caused by:
[00:01:16]   the cargo feature `edition` requires a nightly version of Cargo, but this is the `beta` channel
[00:01:16] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:16] Build completed unsuccessfully in 0:00:00
[00:01:16] Makefile:81: recipe for target 'prepare' failed
[00:01:16] make: *** [prepare] Error 1
[00:01:18] error: failed to parse manifest at `/checkout/src/tools/clippy/Cargo.toml`
[00:01:18] 
[00:01:18] Caused by:
[00:01:18] Caused by:
[00:01:18]   the cargo feature `edition` requires a nightly version of Cargo, but this is the `beta` channel
[00:01:18] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:18] make: *** [prepare] Error 1
[00:01:18] make: *** [prepare] Error 1
[00:01:18] Makefile:81: recipe for target 'prepare' failed
[00:01:21] error: failed to parse manifest at `/checkout/src/tools/clippy/Cargo.toml`
[00:01:21] 
[00:01:21] Caused by:
[00:01:21] Caused by:
[00:01:21]   the cargo feature `edition` requires a nightly version of Cargo, but this is the `beta` channel
[00:01:21] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:21] make: *** [prepare] Error 1
[00:01:21] make: *** [prepare] Error 1
[00:01:21] Makefile:81: recipe for target 'prepare' failed
[00:01:25] error: failed to parse manifest at `/checkout/src/tools/clippy/Cargo.toml`
[00:01:25] 
[00:01:25] Caused by:
[00:01:25] Caused by:
[00:01:25]   the cargo feature `edition` requires a nightly version of Cargo, but this is the `beta` channel
[00:01:25] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:25] make: *** [prepare] Error 1
[00:01:25] make: *** [prepare] Error 1
[00:01:25] Makefile:81: recipe for target 'prepare' failed
[00:01:25] The command has failed after 5 attempts.

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:07d480e0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:08a34418:start=1528996875077938085,finish=1528996875084662358,duration=6724273
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:068bc958
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:16bbd18c
$ dmesg | grep -i kill
