plain
[00:01:28]  Downloading toml v0.4.6
[00:01:28] error: unable to get packages from source
[00:01:28] 
[00:01:28] Caused by:
[00:01:28]   failed to get 200 response from `https://crates.io/api/v1/crates/toml/0.4.6/download`, got 404
[00:01:28] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:28] Build completed unsuccessfully in 0:00:38
[00:01:28] make: *** [prepare] Error 1
[00:01:28] Makefile:81: recipe for target 'prepare' failed
[00:01:29]  Downloading libc v0.2.40
[00:01:29] error: unable to get packages from source
[00:01:29] 
[00:01:29] Caused by:
[00:01:29] Caused by:
[00:01:29]   failed to get 200 response from `https://crates.io/api/v1/crates/libc/0.2.40/download`, got 404
[00:01:29] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:29] Build completed unsuccessfully in 0:00:00
[00:01:29] make: *** [prepare] Error 1
[00:01:29] Makefile:81: recipe for target 'prepare' failed
[00:01:29]  Downloading lazy_static v0.2.11
[00:01:30] error: unable to get packages from source
[00:01:30] 
[00:01:30] Caused by:
[00:01:30] Caused by:
[00:01:30]   failed to get 200 response from `https://crates.io/api/v1/crates/lazy_static/0.2.11/download`, got 404
[00:01:30] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:30] Build completed unsuccessfully in 0:00:00
[00:01:30] Makefile:81: recipe for target 'prepare' failed
[00:01:30] make: *** [prepare] Error 1
[00:01:30]  Downloading serde v1.0.40
[00:01:30] error: unable to get packages from source
[00:01:30] 
[00:01:30] Caused by:
[00:01:30] Caused by:
[00:01:30]   failed to get 200 response from `https://crates.io/api/v1/crates/serde/1.0.40/download`, got 404
[00:01:30] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:30] Build completed unsuccessfully in 0:00:00
[00:01:30] make: *** [prepare] Error 1
[00:01:30] Makefile:81: recipe for target 'prepare' failed
[00:01:31]  Downloading cc v1.0.10
[00:01:31] error: unable to get packages from source
[00:01:31] 
[00:01:31] Caused by:
[00:01:31] Caused by:
[00:01:31]   failed to get 200 response from `https://crates.io/api/v1/crates/cc/1.0.10/download`, got 404
[00:01:31] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:31] Build completed unsuccessfully in 0:00:00
[00:01:31] Makefile:81: recipe for target 'prepare' failed
[00:01:31] make: *** [prepare] Error 1
[00:01:31] The command has failed after 5 attempts.

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:00e0ba8c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
