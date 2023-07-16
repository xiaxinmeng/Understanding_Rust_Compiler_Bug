plain
[00:01:25]  Downloading serde_derive v1.0.40
[00:01:25] error: unable to get packages from source
[00:01:25] 
[00:01:25] Caused by:
[00:01:25]   failed to get 200 response from `https://crates.io/api/v1/crates/serde_derive/1.0.40/download`, got 404
[00:01:25] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:25] Build completed unsuccessfully in 0:00:37
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
[00:01:26] Makefile:81: recipe for target 'prepare' failed
[00:01:26] make: *** [prepare] Error 1
[00:01:26]  Downloading toml v0.4.6
[00:01:26] error: unable to get packages from source
[00:01:26] 
[00:01:26] Caused by:
[00:01:26] Caused by:
[00:01:26]   failed to get 200 response from `https://crates.io/api/v1/crates/toml/0.4.6/download`, got 404
[00:01:26] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:26] Build completed unsuccessfully in 0:00:00
[00:01:26] Makefile:81: recipe for target 'prepare' failed
[00:01:26] make: *** [prepare] Error 1
[00:01:27]  Downloading toml v0.4.6
[00:01:27] error: unable to get packages from source
[00:01:27] 
[00:01:27] Caused by:
[00:01:27] Caused by:
[00:01:27]   failed to get 200 response from `https://crates.io/api/v1/crates/toml/0.4.6/download`, got 404
[00:01:27] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:27] Build completed unsuccessfully in 0:00:00
[00:01:27] Makefile:81: recipe for target 'prepare' failed
[00:01:27] make: *** [prepare] Error 1
[00:01:27]  Downloading serde_derive v1.0.40
[00:01:27] error: unable to get packages from source
[00:01:27] 
[00:01:27] Caused by:
[00:01:27] Caused by:
[00:01:27]   failed to get 200 response from `https://crates.io/api/v1/crates/serde_derive/1.0.40/download`, got 404
[00:01:27] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:27] Build completed unsuccessfully in 0:00:00
[00:01:27] Makefile:81: recipe for target 'prepare' failed
[00:01:27] make: *** [prepare] Error 1
[00:01:27] The command has failed after 5 attempts.

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:06a943fd
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
