plain
####################################################                      73.4%
######################################################################## 100.0%
[00:01:01] extracting /checkout/obj/build/cache/2018-04-24/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:04]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:01:23] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:23] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:23] Build completed unsuccessfully in 0:00:38
[00:01:23] make: *** [prepare] Error 1
[00:01:23] Makefile:81: recipe for target 'prepare' failed
[00:01:23] Command failed. Attempt 2/5:
[00:01:24] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:24] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:24] Build completed unsuccessfully in 0:00:00
[00:01:24] make: *** [prepare] Error 1
[00:01:24] Makefile:81: recipe for target 'prepare' failed
[00:01:24] Command failed. Attempt 3/5:
[00:01:24] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:24] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:24] Build completed unsuccessfully in 0:00:00
[00:01:24] make: *** [prepare] Error 1
[00:01:24] Makefile:81: recipe for target 'prepare' failed
[00:01:24] Command failed. Attempt 4/5:
[00:01:24] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:24] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:24] Build completed unsuccessfully in 0:00:00
[00:01:24] make: *** [prepare] Error 1
[00:01:24] Makefile:81: recipe for target 'prepare' failed
[00:01:24] Command failed. Attempt 5/5:
[00:01:24] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:24] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:24] Build completed unsuccessfully in 0:00:00
[00:01:24] Makefile:81: recipe for target 'prepare' failed
[00:01:24] make: *** [prepare] Error 1
[00:01:24] The command has failed after 5 attempts.

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:00207c60
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
