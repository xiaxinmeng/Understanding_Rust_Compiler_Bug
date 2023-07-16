plain
#################################################################         90.7%
######################################################################## 100.0%
[00:01:14] extracting /checkout/obj/build/cache/2018-05-10/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:14]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:01:32] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:32] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:32] Build completed unsuccessfully in 0:00:35
[00:01:32] Makefile:81: recipe for target 'prepare' failed
[00:01:32] make: *** [prepare] Error 1
[00:01:33] Command failed. Attempt 2/5:
[00:01:33] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:33] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:33] Build completed unsuccessfully in 0:00:00
[00:01:33] make: *** [prepare] Error 1
[00:01:33] Makefile:81: recipe for target 'prepare' failed
[00:01:35] Command failed. Attempt 3/5:
[00:01:35] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:35] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:35] Build completed unsuccessfully in 0:00:00
[00:01:35] Makefile:81: recipe for target 'prepare' failed
[00:01:35] make: *** [prepare] Error 1
[00:01:38] Command failed. Attempt 4/5:
[00:01:39] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:39] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:39] Build completed unsuccessfully in 0:00:00
[00:01:39] make: *** [prepare] Error 1
[00:01:39] Makefile:81: recipe for target 'prepare' failed
[00:01:43] Command failed. Attempt 5/5:
[00:01:43] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:43] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:43] Build completed unsuccessfully in 0:00:00
[00:01:43] make: *** [prepare] Error 1
[00:01:43] Makefile:81: recipe for target 'prepare' failed
[00:01:43] The command has failed after 5 attempts.

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:003825ac
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
