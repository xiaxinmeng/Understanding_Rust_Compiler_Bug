plain
######################################################################## 100.0%
[00:01:07] extracting /checkout/obj/build/cache/2018-04-24/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:10]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:01:29]     Updating git repository `https://github.com/Zoxc/rayon.git`
[00:01:42] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:42] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:42] Build completed unsuccessfully in 0:00:57
[00:01:42] make: *** [prepare] Error 1
[00:01:42] Makefile:81: recipe for target 'prepare' failed
[00:01:43]     Updating git repository `https://github.com/Zoxc/rayon.git`
[00:01:43]     Updating git repository `https://github.com/Zoxc/rayon.git`
[00:01:45] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:45] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:45] Build completed unsuccessfully in 0:00:02
[00:01:45] make: *** [prepare] Error 1
[00:01:45] Makefile:81: recipe for target 'prepare' failed
[00:01:45]     Updating git repository `https://github.com/Zoxc/rayon.git`
[00:01:45]     Updating git repository `https://github.com/Zoxc/rayon.git`
[00:01:46] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:46] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:46] Build completed unsuccessfully in 0:00:01
[00:01:46] make: *** [prepare] Error 1
[00:01:46] Makefile:81: recipe for target 'prepare' failed
[00:01:46]     Updating git repository `https://github.com/Zoxc/rayon.git`
[00:01:46]     Updating git repository `https://github.com/Zoxc/rayon.git`
[00:01:48] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:48] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:48] Build completed unsuccessfully in 0:00:01
[00:01:48] Makefile:81: recipe for target 'prepare' failed
[00:01:48] make: *** [prepare] Error 1
[00:01:48]     Updating git repository `https://github.com/Zoxc/rayon.git`
[00:01:48]     Updating git repository `https://github.com/Zoxc/rayon.git`
[00:01:49] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:49] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:49] Build completed unsuccessfully in 0:00:01
[00:01:49] make: *** [prepare] Error 1
[00:01:49] Makefile:81: recipe for target 'prepare' failed
[00:01:49] The command has failed after 5 attempts.

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:025c4a8d
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
