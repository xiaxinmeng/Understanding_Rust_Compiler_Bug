plain
#####################################################################     97.2%
######################################################################## 100.0%
[00:01:26] extracting /checkout/obj/build/cache/2018-05-10/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:26]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:01:45] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:45] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:45] Build completed unsuccessfully in 0:00:41
[00:01:45] Makefile:81: recipe for target 'prepare' failed
[00:01:45] make: *** [prepare] Error 1
[00:01:46] Command failed. Attempt 2/5:
[00:01:47] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:47] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:47] Build completed unsuccessfully in 0:00:00
[00:01:47] Makefile:81: recipe for target 'prepare' failed
[00:01:47] make: *** [prepare] Error 1
[00:01:49] Command failed. Attempt 3/5:
[00:01:49] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:49] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:49] Build completed unsuccessfully in 0:00:00
[00:01:49] make: *** [prepare] Error 1
[00:01:49] Makefile:81: recipe for target 'prepare' failed
[00:01:52] Command failed. Attempt 4/5:
[00:01:52] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:52] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:52] Build completed unsuccessfully in 0:00:00
[00:01:52] Makefile:81: recipe for target 'prepare' failed
[00:01:52] make: *** [prepare] Error 1
[00:01:56] Command failed. Attempt 5/5:
[00:01:56] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:56] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:56] Build completed unsuccessfully in 0:00:00
[00:01:56] make: *** [prepare] Error 1
[00:01:56] Makefile:81: recipe for target 'prepare' failed
[00:01:56] The command has failed after 5 attempts.

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:01fbe831
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
