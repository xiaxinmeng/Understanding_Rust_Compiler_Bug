plain
[00:01:31]  Downloading time v0.1.39
[00:01:31] error: unable to get packages from source
[00:01:31] 
[00:01:31] Caused by:
[00:01:31]   failed to get 200 response from `https://crates.io/api/v1/crates/time/0.1.39/download`, got 404
[00:01:31] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:31] Build completed unsuccessfully in 0:00:44
[00:01:31] make: *** [prepare] Error 1
[00:01:31] Makefile:81: recipe for target 'prepare' failed
[00:01:32]  Downloading cmake v0.1.30
[00:01:32] warning: spurious network error (2 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/cmake/0.1.30/download`, got 500
[00:01:32] warning: spurious network error (1 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/cmake/0.1.30/download`, got 500
[00:01:32] error: unable to get packages from source
[00:01:32] error: unable to get packages from source
[00:01:32] 
[00:01:32] Caused by:
[00:01:32]   failed to get 200 response from `https://crates.io/api/v1/crates/cmake/0.1.30/download`, got 500
[00:01:32] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:32] Build completed unsuccessfully in 0:00:00
[00:01:32] Makefile:81: recipe for target 'prepare' failed
[00:01:32] make: *** [prepare] Error 1
[00:01:32]  Downloading time v0.1.39
[00:01:32] error: unable to get packages from source
[00:01:32] 
[00:01:32] Caused by:
[00:01:32] Caused by:
[00:01:32]   failed to get 200 response from `https://crates.io/api/v1/crates/time/0.1.39/download`, got 404
[00:01:32] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:32] Build completed unsuccessfully in 0:00:00
[00:01:32] make: *** [prepare] Error 1
[00:01:32] Makefile:81: recipe for target 'prepare' failed
[00:01:33]  Downloading toml v0.4.6
[00:01:33] error: unable to get packages from source
[00:01:33] 
[00:01:33] Caused by:
[00:01:33] Caused by:
[00:01:33]   failed to get 200 response from `https://crates.io/api/v1/crates/toml/0.4.6/download`, got 404
[00:01:33] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:33] Build completed unsuccessfully in 0:00:00
[00:01:33] Makefile:81: recipe for target 'prepare' failed
[00:01:33] make: *** [prepare] Error 1
[00:01:33]  Downloading cmake v0.1.30
[00:01:34] warning: spurious network error (2 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/cmake/0.1.30/download`, got 500
[00:01:34] warning: spurious network error (1 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/cmake/0.1.30/download`, got 500
[00:01:34] error: unable to get packages from source
[00:01:34] error: unable to get packages from source
[00:01:34] 
[00:01:34] Caused by:
[00:01:34]   failed to get 200 response from `https://crates.io/api/v1/crates/cmake/0.1.30/download`, got 500
[00:01:34] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:34] Build completed unsuccessfully in 0:00:00
[00:01:34] Makefile:81: recipe for target 'prepare' failed
[00:01:34] make: *** [prepare] Error 1
[00:01:34] The command has failed after 5 attempts.

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:0ef11de2
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
