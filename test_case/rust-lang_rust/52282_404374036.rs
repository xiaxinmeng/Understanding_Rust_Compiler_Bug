plain
#####################################################                     74.2%
######################################################################## 100.0%
[00:01:15] extracting /checkout/obj/build/cache/2018-06-30/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:16]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:01:37] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:37] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:37] Makefile:81: recipe for target 'prepare' failed
[00:01:37] make: *** [prepare] Error 1
[00:01:38] Command failed. Attempt 2/5:
[00:01:38] Command failed. Attempt 2/5:
[00:01:38] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:38] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:38] Makefile:81: recipe for target 'prepare' failed
[00:01:38] make: *** [prepare] Error 1
[00:01:40] Command failed. Attempt 3/5:
[00:01:40] Command failed. Attempt 3/5:
[00:01:40] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:40] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:40] make: *** [prepare] Error 1
[00:01:40] Makefile:81: recipe for target 'prepare' failed
[00:01:43] Command failed. Attempt 4/5:
[00:01:43] Command failed. Attempt 4/5:
[00:01:44] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:44] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:44] make: *** [prepare] Error 1
[00:01:44] Makefile:81: recipe for target 'prepare' failed
[00:01:48] Command failed. Attempt 5/5:
[00:01:48] Command failed. Attempt 5/5:
[00:01:48] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:48] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:48] make: *** [prepare] Error 1
[00:01:48] Makefile:81: recipe for target 'prepare' failed
[00:01:48] The command has failed after 5 attempts.
travis_time:end:11ba8b8a:start=1531364168316566935,finish=1531364287682116994,duration=119365550059
---
travis_time:end:055ffe84:start=1531364287945856104,finish=1531364287953333709,duration=7477605
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0e35cf5c
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1bc7a1c6
$ dmesg | grep -i kill
