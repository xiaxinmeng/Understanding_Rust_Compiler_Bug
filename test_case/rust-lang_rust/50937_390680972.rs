plain
################################################################          89.8%
######################################################################## 100.0%
[00:01:19] extracting /checkout/obj/build/cache/2018-05-10/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:19]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:01:37] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:37] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:37] Build completed unsuccessfully in 0:00:36
[00:01:37] make: *** [prepare] Error 1
[00:01:37] Makefile:81: recipe for target 'prepare' failed
[00:01:38]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:01:38]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:01:39] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:39] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:39] Build completed unsuccessfully in 0:00:00
[00:01:39] make: *** [prepare] Error 1
[00:01:39] Makefile:81: recipe for target 'prepare' failed
[00:01:41]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:01:41]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:01:41] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:41] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:41] Build completed unsuccessfully in 0:00:00
[00:01:41] make: *** [prepare] Error 1
[00:01:41] Makefile:81: recipe for target 'prepare' failed
[00:01:45]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:01:45]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:01:46] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:46] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:46] Build completed unsuccessfully in 0:00:01
[00:01:46] make: *** [prepare] Error 1
[00:01:46] Makefile:81: recipe for target 'prepare' failed
[00:01:50]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:01:50]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:01:53] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:53] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:53] Build completed unsuccessfully in 0:00:03
[00:01:53] make: *** [prepare] Error 1
[00:01:53] Makefile:81: recipe for target 'prepare' failed
[00:01:53] The command has failed after 5 attempts.

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:06656e88
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
