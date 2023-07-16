
[00:06:17]  Downloading unicode-segmentation v1.2.0
[00:07:17] warning: spurious network error (2 tries remaining): [28] Timeout was reached (Operation too slow. Less than 10 bytes/sec transferred the last 30 seconds)
[00:07:47] warning: spurious network error (1 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/unicode-segmentation/1.2.0/download`, got 503
[00:08:17] error: unable to get packages from source
[00:08:17] 
[00:08:17] Caused by:
[00:08:17]   failed to get 200 response from `https://crates.io/api/v1/crates/unicode-segmentation/1.2.0/download`, got 503
[00:08:17] thread 'main' panicked at 'command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "metadata" "--format-version" "1" "--manifest-path" "/checkout/src/libstd/Cargo.toml"
[00:08:17] expected success, got: exit code: 101', /checkout/src/build_helper/lib.rs:137:8
[00:08:17] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:08:17] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build nonexistent/path/to/trigger/cargo/metadata
[00:08:17] Build completed unsuccessfully in 0:03:33
[00:08:17] Makefile:77: recipe for target 'prepare' failed
[00:08:17] make: *** [prepare] Error 1
[00:08:17] Command failed. Attempt 3/5:
[00:08:17]     Finished dev [unoptimized] target(s) in 0.0 secs
[00:08:18] warning: Package `toml v0.4.5` does not have feature `serde`. It has a required dependency with that name, but only optional dependencies can be used as features. This is currently a warning to ease the transition, but it will become an error in the future.
[00:08:18]  Downloading clap v2.26.0
[00:08:18]  Downloading memchr v0.1.11
[00:08:18]  Downloading debug_unreachable v0.1.1
[00:08:49]  Downloading mac v0.1.1
[00:08:49]  Downloading bitflags v0.9.1
[00:09:49] warning: spurious network error (2 tries remaining): [28] Timeout was reached (Operation too slow. Less than 10 bytes/sec transferred the last 30 seconds)
[00:09:49]  Downloading strsim v0.6.0
[00:10:50] warning: spurious network error (2 tries remaining): [28] Timeout was reached (Operation too slow. Less than 10 bytes/sec transferred the last 30 seconds)
[00:10:50]  Downloading unicode-xid v0.0.3
[00:11:20]  Downloading kuchiki v0.5.1
[00:12:21] warning: spurious network error (2 tries remaining): [28] Timeout was reached (Operation too slow. Less than 10 bytes/sec transferred the last 30 seconds)
[00:12:51] warning: spurious network error (1 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/kuchiki/0.5.1/download`, got 503
[00:13:21] error: unable to get packages from source
[00:13:21] 
[00:13:21] Caused by:
[00:13:21]   [28] Timeout was reached (Operation too slow. Less than 10 bytes/sec transferred the last 30 seconds)
[00:13:21] thread 'main' panicked at 'command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "metadata" "--format-version" "1" "--manifest-path" "/checkout/src/libstd/Cargo.toml"
[00:13:21] expected success, got: exit code: 101', /checkout/src/build_helper/lib.rs:137:8
[00:13:21] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:13:21] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build nonexistent/path/to/trigger/cargo/metadata
[00:13:21] Build completed unsuccessfully in 0:05:03
[00:13:21] make: *** [prepare] Error 1
[00:13:21] Makefile:77: recipe for target 'prepare' failed
[00:13:21] Command failed. Attempt 4/5:
[00:13:21]     Finished dev [unoptimized] target(s) in 0.0 secs
[00:13:21] warning: Package `toml v0.4.5` does not have feature `serde`. It has a required dependency with that name, but only optional dependencies can be used as features. This is currently a warning to ease the transition, but it will become an error in the future.
[00:13:21]  Downloading pest v0.3.3
[00:13:22]  Downloading unicode-bidi v0.3.4
[00:14:22] warning: spurious network error (2 tries remaining): [28] Timeout was reached (Operation too slow. Less than 10 bytes/sec transferred the last 30 seconds)
[00:14:52] warning: spurious network error (1 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/unicode-bidi/0.3.4/download`, got 503
[00:15:22] error: unable to get packages from source
[00:15:22] 
[00:15:22] Caused by:
[00:15:22]   failed to get 200 response from `https://crates.io/api/v1/crates/unicode-bidi/0.3.4/download`, got 503
[00:15:22] thread 'main' panicked at 'command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "metadata" "--format-version" "1" "--manifest-path" "/checkout/src/libstd/Cargo.toml"
[00:15:22] expected success, got: exit code: 101', /checkout/src/build_helper/lib.rs:137:8
[00:15:22] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:15:22] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build nonexistent/path/to/trigger/cargo/metadata
[00:15:22] Build completed unsuccessfully in 0:02:01
[00:15:22] make: *** [prepare] Error 1
[00:15:22] Makefile:77: recipe for target 'prepare' failed
[00:15:22] Command failed. Attempt 5/5:
[00:15:23]     Finished dev [unoptimized] target(s) in 0.0 secs
[00:15:23] warning: Package `toml v0.4.5` does not have feature `serde`. It has a required dependency with that name, but only optional dependencies can be used as features. This is currently a warning to ease the transition, but it will become an error in the future.
[00:15:23]  Downloading handlebars v0.26.2
[00:15:53] warning: spurious network error (2 tries remaining): [28] Timeout was reached (Operation too slow. Less than 10 bytes/sec transferred the last 30 seconds)
[00:16:23] warning: spurious network error (1 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/handlebars/0.26.2/download`, got 503
[00:16:53] error: unable to get packages from source
[00:16:53] 
[00:16:53] Caused by:
[00:16:53]   failed to get 200 response from `https://crates.io/api/v1/crates/handlebars/0.26.2/download`, got 503
[00:16:53] thread 'main' panicked at 'command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "metadata" "--format-version" "1" "--manifest-path" "/checkout/src/libstd/Cargo.toml"
[00:16:53] expected success, got: exit code: 101', /checkout/src/build_helper/lib.rs:137:8
[00:16:53] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:16:53] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build nonexistent/path/to/trigger/cargo/metadata
[00:16:53] Build completed unsuccessfully in 0:01:30
[00:16:53] Makefile:77: recipe for target 'prepare' failed
[00:16:53] make: *** [prepare] Error 1
[00:16:53] The command has failed after 5 attempts.
