plain
[00:01:23]  Downloading serde_json v1.0.15
[00:01:23] error: unable to get packages from source
[00:01:23] 
[00:01:23] Caused by:
[00:01:23]   failed to get 200 response from `https://crates.io/api/v1/crates/serde_json/1.0.15/download`, got 404
[00:01:23] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:23] Build completed unsuccessfully in 0:00:36
[00:01:23] make: *** [prepare] Error 1
[00:01:23] Makefile:81: recipe for target 'prepare' failed
[00:01:24]  Downloading num_cpus v1.8.0
[00:01:24] error: unable to get packages from source
[00:01:24] 
[00:01:24] Caused by:
[00:01:24] Caused by:
[00:01:24]   failed to get 200 response from `https://crates.io/api/v1/crates/num_cpus/1.8.0/download`, got 404
[00:01:24] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:24] Build completed unsuccessfully in 0:00:00
[00:01:24] make: *** [prepare] Error 1
[00:01:24] Makefile:81: recipe for target 'prepare' failed
[00:01:25]  Downloading toml v0.4.6
[00:01:25] error: unable to get packages from source
[00:01:25] 
[00:01:25] Caused by:
[00:01:25] Caused by:
[00:01:25]   failed to get 200 response from `https://crates.io/api/v1/crates/toml/0.4.6/download`, got 404
[00:01:25] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:25] Build completed unsuccessfully in 0:00:00
[00:01:25] Makefile:81: recipe for target 'prepare' failed
[00:01:25] make: *** [prepare] Error 1
[00:01:25]  Downloading cc v1.0.10
[00:01:25] error: unable to get packages from source
[00:01:25] 
[00:01:25] Caused by:
[00:01:25] Caused by:
[00:01:25]   failed to get 200 response from `https://crates.io/api/v1/crates/cc/1.0.10/download`, got 404
[00:01:25] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:25] Build completed unsuccessfully in 0:00:00
[00:01:25] Makefile:81: recipe for target 'prepare' failed
[00:01:25] make: *** [prepare] Error 1
[00:01:26]  Downloading filetime v0.1.15
[00:01:26] warning: spurious network error (2 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/filetime/0.1.15/download`, got 500
[00:01:26] warning: spurious network error (1 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/filetime/0.1.15/download`, got 500
[00:01:26] error: unable to get packages from source
[00:01:26] error: unable to get packages from source
[00:01:26] 
[00:01:26] Caused by:
[00:01:26]   failed to get 200 response from `https://crates.io/api/v1/crates/filetime/0.1.15/download`, got 500
[00:01:26] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:26] Build completed unsuccessfully in 0:00:00
[00:01:26] make: *** [prepare] Error 1
[00:01:26] Makefile:81: recipe for target 'prepare' failed
[00:01:26] The command has failed after 5 attempts.

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:213d22b8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
