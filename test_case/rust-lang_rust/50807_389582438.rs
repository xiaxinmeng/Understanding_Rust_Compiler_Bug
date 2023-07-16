plain
#######################################################################   99.8%
######################################################################## 100.0%
[00:06:47] extracting /checkout/obj/build/cache/2018-04-24/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:06:47]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:07:08] error: the lock file needs to be updated but --locked was passed to prevent this
[00:07:08] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:07:08] Build completed unsuccessfully in 0:01:23
[00:07:08] Makefile:81: recipe for target 'prepare' failed
[00:07:08] make: *** [prepare] Error 1
[00:07:09] Command failed. Attempt 2/5:
[00:07:09] error: the lock file needs to be updated but --locked was passed to prevent this
[00:07:09] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:07:09] Build completed unsuccessfully in 0:00:00
[00:07:09] Makefile:81: recipe for target 'prepare' failed
[00:07:09] make: *** [prepare] Error 1
[00:07:11] Command failed. Attempt 3/5:
[00:07:11] error: the lock file needs to be updated but --locked was passed to prevent this
[00:07:11] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:07:11] Build completed unsuccessfully in 0:00:00
[00:07:11] Makefile:81: recipe for target 'prepare' failed
[00:07:11] make: *** [prepare] Error 1
[00:07:14] Command failed. Attempt 4/5:
[00:07:14] error: the lock file needs to be updated but --locked was passed to prevent this
[00:07:14] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:07:14] Build completed unsuccessfully in 0:00:00
[00:07:14] make: *** [prepare] Error 1
[00:07:14] Makefile:81: recipe for target 'prepare' failed
[00:07:18] Command failed. Attempt 5/5:
[00:07:19] error: the lock file needs to be updated but --locked was passed to prevent this
[00:07:19] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:07:19] Build completed unsuccessfully in 0:00:00
[00:07:19] Makefile:81: recipe for target 'prepare' failed
[00:07:19] make: *** [prepare] Error 1
[00:07:19] The command has failed after 5 attempts.

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:12e446fc
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
