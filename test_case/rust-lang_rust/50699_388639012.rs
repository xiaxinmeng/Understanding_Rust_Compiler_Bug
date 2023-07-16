plain
###################################################                       72.0%
######################################################################## 100.0%
[00:01:15] extracting /checkout/obj/build/cache/2018-04-24/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:19]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:01:38] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:38] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:38] Build completed unsuccessfully in 0:00:42
[00:01:38] Makefile:81: recipe for target 'prepare' failed
[00:01:38] make: *** [prepare] Error 1
[00:01:39] Command failed. Attempt 2/5:
[00:01:39] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:39] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:39] Build completed unsuccessfully in 0:00:00
[00:01:39] make: *** [prepare] Error 1
[00:01:39] Makefile:81: recipe for target 'prepare' failed
[00:01:41] Command failed. Attempt 3/5:
[00:01:41] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:41] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:41] Build completed unsuccessfully in 0:00:00
[00:01:41] make: *** [prepare] Error 1
[00:01:41] Makefile:81: recipe for target 'prepare' failed
[00:01:44] Command failed. Attempt 4/5:
[00:01:45] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:45] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:45] Build completed unsuccessfully in 0:00:00
[00:01:45] make: *** [prepare] Error 1
[00:01:45] Makefile:81: recipe for target 'prepare' failed
[00:01:49] Command failed. Attempt 5/5:
[00:01:49] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:49] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:49] Build completed unsuccessfully in 0:00:00
[00:01:49] make: *** [prepare] Error 1
[00:01:49] Makefile:81: recipe for target 'prepare' failed
[00:01:49] The command has failed after 5 attempts.

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:0a282d7a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
