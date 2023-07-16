plain
[00:01:39] warning: spurious network error (1 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/filetime/0.1.15/download`, got 500
[00:01:39] error: unable to get packages from source
[00:01:39] 
[00:01:39] Caused by:
[00:01:39]   failed to get 200 response from `https://crates.io/api/v1/crates/filetime/0.1.15/download`, got 500
[00:01:39] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:39] Build completed unsuccessfully in 0:00:51
[00:01:39] Makefile:81: recipe for target 'prepare' failed
[00:01:39] make: *** [prepare] Error 1
[00:01:39]  Downloading serde_json v1.0.15
[00:01:40] error: unable to get packages from source
[00:01:40] 
[00:01:40] Caused by:
[00:01:40] Caused by:
[00:01:40]   failed to get 200 response from `https://crates.io/api/v1/crates/serde_json/1.0.15/download`, got 404
[00:01:40] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:40] Build completed unsuccessfully in 0:00:00
[00:01:40] Makefile:81: recipe for target 'prepare' failed
[00:01:40] make: *** [prepare] Error 1
[00:01:40]  Downloading getopts v0.2.17
[00:01:40] error: unable to get packages from source
[00:01:40] 
[00:01:40] Caused by:
[00:01:40] Caused by:
[00:01:40]   failed to get 200 response from `https://crates.io/api/v1/crates/getopts/0.2.17/download`, got 404
[00:01:40] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:40] Build completed unsuccessfully in 0:00:00
[00:01:40] Makefile:81: recipe for target 'prepare' failed
[00:01:40] make: *** [prepare] Error 1
[00:01:40]  Downloading serde_json v1.0.15
[00:01:41] error: unable to get packages from source
[00:01:41] 
[00:01:41] Caused by:
[00:01:41] Caused by:
[00:01:41]   failed to get 200 response from `https://crates.io/api/v1/crates/serde_json/1.0.15/download`, got 404
[00:01:41] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:41] Build completed unsuccessfully in 0:00:00
[00:01:41] Makefile:81: recipe for target 'prepare' failed
[00:01:41] make: *** [prepare] Error 1
[00:01:41]  Downloading filetime v0.1.15
[00:01:41] warning: spurious network error (2 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/filetime/0.1.15/download`, got 500
[00:01:41] warning: spurious network error (1 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/filetime/0.1.15/download`, got 500
[00:01:41] error: unable to get packages from source
[00:01:41] error: unable to get packages from source
[00:01:41] 
[00:01:41] Caused by:
[00:01:41]   failed to get 200 response from `https://crates.io/api/v1/crates/filetime/0.1.15/download`, got 500
[00:01:41] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:41] Build completed unsuccessfully in 0:00:00
[00:01:41] make: *** [prepare] Error 1
[00:01:41] Makefile:81: recipe for target 'prepare' failed
[00:01:41] The command has failed after 5 attempts.

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:1914da01
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
