plain
[00:01:36] warning: spurious network error (1 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/getopts/0.2.17/download`, got 503
[00:01:36] error: unable to get packages from source
[00:01:36] 
[00:01:36] Caused by:
[00:01:36]   failed to get 200 response from `https://crates.io/api/v1/crates/getopts/0.2.17/download`, got 503
[00:01:36] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:36] Build completed unsuccessfully in 0:00:47
[00:01:36] make: *** [prepare] Error 1
[00:01:36] Makefile:81: recipe for target 'prepare' failed
[00:01:36]  Downloading serde_json v1.0.15
[00:01:36] warning: spurious network error (2 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/serde_json/1.0.15/download`, got 503
[00:01:37] warning: spurious network error (1 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/serde_json/1.0.15/download`, got 503
[00:01:37] error: unable to get packages from source
[00:01:37] error: unable to get packages from source
[00:01:37] 
[00:01:37] Caused by:
[00:01:37]   failed to get 200 response from `https://crates.io/api/v1/crates/serde_json/1.0.15/download`, got 503
[00:01:37] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:37] Build completed unsuccessfully in 0:00:00
[00:01:37] Makefile:81: recipe for target 'prepare' failed
[00:01:37] make: *** [prepare] Error 1
[00:01:37]  Downloading num_cpus v1.8.0
[00:01:37]  Downloading num_cpus v1.8.0
[00:01:37] warning: spurious network error (2 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/num_cpus/1.8.0/download`, got 503
[00:01:37] warning: spurious network error (1 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/num_cpus/1.8.0/download`, got 503
[00:01:38] error: unable to get packages from source
[00:01:38] Caused by:
[00:01:38] Caused by:
[00:01:38]   failed to get 200 response from `https://crates.io/api/v1/crates/num_cpus/1.8.0/download`, got 503
[00:01:38] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:38] Build completed unsuccessfully in 0:00:00
[00:01:38] Makefile:81: recipe for target 'prepare' failed
[00:01:38] make: *** [prepare] Error 1
[00:01:38]  Downloading serde_derive v1.0.40
[00:01:38] warning: spurious network error (2 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/serde_derive/1.0.40/download`, got 503
[00:01:38] warning: spurious network error (1 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/serde_derive/1.0.40/download`, got 503
[00:01:38] error: unable to get packages from source
[00:01:38] error: unable to get packages from source
[00:01:38] 
[00:01:38] Caused by:
[00:01:38]   failed to get 200 response from `https://crates.io/api/v1/crates/serde_derive/1.0.40/download`, got 503
[00:01:38] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:38] Build completed unsuccessfully in 0:00:00
[00:01:38] make: *** [prepare] Error 1
[00:01:38] Makefile:81: recipe for target 'prepare' failed
[00:01:39]  Downloading lazy_static v0.2.11
[00:01:39] warning: spurious network error (2 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/lazy_static/0.2.11/download`, got 503
[00:01:39] warning: spurious network error (1 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/lazy_static/0.2.11/download`, got 503
[00:01:39] error: unable to get packages from source
[00:01:39] error: unable to get packages from source
[00:01:39] 
[00:01:39] Caused by:
[00:01:39]   failed to get 200 response from `https://crates.io/api/v1/crates/lazy_static/0.2.11/download`, got 503
[00:01:39] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:39] Build completed unsuccessfully in 0:00:00
[00:01:39] make: *** [prepare] Error 1
[00:01:39] Makefile:81: recipe for target 'prepare' failed
[00:01:39] The command has failed after 5 attempts.

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:2b0b4cb4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_fold:start:after_failure.4
travis_time:start:145e86e8
$ dmesg | grep -i kill
[   10.436234] init: failsafe main process (1094) killed by TERM signal
[   41.766948] init: plymouth-upstart-bridge main process (510) killed by TERM signal
travis_fold:end:after_failure.4

Done. Your build exited with 1.
