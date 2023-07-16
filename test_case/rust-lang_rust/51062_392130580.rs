plain
###########################################################               82.4%
######################################################################## 100.0%
[00:01:13] extracting /checkout/obj/build/cache/2018-05-10/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:13]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:01:31] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:31] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:31] Build completed unsuccessfully in 0:00:33
[00:01:31] make: *** [prepare] Error 1
[00:01:31] Makefile:81: recipe for target 'prepare' failed
[00:01:32]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:01:32]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:01:33] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:33] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:33] Build completed unsuccessfully in 0:00:00
[00:01:33] make: *** [prepare] Error 1
[00:01:33] Makefile:81: recipe for target 'prepare' failed
[00:01:35]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:01:35]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:01:36] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:36] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:36] Build completed unsuccessfully in 0:00:00
[00:01:36] Makefile:81: recipe for target 'prepare' failed
[00:01:36] make: *** [prepare] Error 1
[00:01:39]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:01:39]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:01:40] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:40] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:40] Build completed unsuccessfully in 0:00:01
[00:01:40] Makefile:81: recipe for target 'prepare' failed
[00:01:40] make: *** [prepare] Error 1
[00:01:44]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:01:44]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:01:45] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:45] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:45] Build completed unsuccessfully in 0:00:00
[00:01:45] Makefile:81: recipe for target 'prepare' failed
[00:01:45] make: *** [prepare] Error 1
[00:01:45] The command has failed after 5 attempts.

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:01c2a7a3
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
