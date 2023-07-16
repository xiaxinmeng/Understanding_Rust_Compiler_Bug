plain
[00:01:24] extracting /checkout/obj/build/cache/2018-05-10/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:24] error: failed to parse manifest at `/checkout/src/tools/clippy/Cargo.toml`
[00:01:24] 
[00:01:24] Caused by:
[00:01:24]   the cargo feature `edition` requires a nightly version of Cargo, but this is the `beta` channel
[00:01:24] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:24] make: *** [prepare] Error 1
[00:01:24] make: *** [prepare] Error 1
[00:01:24] Makefile:81: recipe for target 'prepare' failed
[00:01:25] error: failed to parse manifest at `/checkout/src/tools/clippy/Cargo.toml`
[00:01:25] 
[00:01:25] Caused by:
[00:01:25] Caused by:
[00:01:25]   the cargo feature `edition` requires a nightly version of Cargo, but this is the `beta` channel
[00:01:25] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:25] make: *** [prepare] Error 1
[00:01:25] make: *** [prepare] Error 1
[00:01:25] Makefile:81: recipe for target 'prepare' failed
[00:01:28] error: failed to parse manifest at `/checkout/src/tools/clippy/Cargo.toml`
[00:01:28] 
[00:01:28] Caused by:
[00:01:28] Caused by:
[00:01:28]   the cargo feature `edition` requires a nightly version of Cargo, but this is the `beta` channel
[00:01:28] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:28] Build completed unsuccessfully in 0:00:00
[00:01:28] Makefile:81: recipe for target 'prepare' failed
[00:01:28] make: *** [prepare] Error 1
[00:01:31] error: failed to parse manifest at `/checkout/src/tools/clippy/Cargo.toml`
[00:01:31] 
[00:01:31] Caused by:
[00:01:31] Caused by:
[00:01:31]   the cargo feature `edition` requires a nightly version of Cargo, but this is the `beta` channel
[00:01:31] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:31] Build completed unsuccessfully in 0:00:00
[00:01:31] Makefile:81: recipe for target 'prepare' failed
[00:01:31] make: *** [prepare] Error 1
[00:01:35] error: failed to parse manifest at `/checkout/src/tools/clippy/Cargo.toml`
[00:01:35] 
[00:01:35] Caused by:
[00:01:35] Caused by:
[00:01:35]   the cargo feature `edition` requires a nightly version of Cargo, but this is the `beta` channel
[00:01:35] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:35] Build completed unsuccessfully in 0:00:00
[00:01:35] Makefile:81: recipe for target 'prepare' failed
[00:01:35] make: *** [prepare] Error 1
[00:01:35] The command has failed after 5 attempts.

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:0e6ea934
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
