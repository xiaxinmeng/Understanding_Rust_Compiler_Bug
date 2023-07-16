plain
[00:03:07] Caused by:
[00:03:07]   failed to get 200 response from `https://crates.io/api/v1/crates/time/0.1.39/download`, got 500
[00:03:07] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:03:07] Build completed unsuccessfully in 0:02:16
[00:03:07] Makefile:81: recipe for target 'prepare' failed
[00:03:07] make: *** [prepare] Error 1
[00:03:08]  Downloading num_cpus v1.8.0
[00:03:08]  Downloading cc v1.0.15
[00:03:25] warning: spurious network error (2 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/cc/1.0.15/download`, got 500
[00:03:44] warning: spurious network error (1 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/cc/1.0.15/download`, got 500
[00:03:44] warning: spurious network error (1 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/cc/1.0.15/download`, got 500
[00:04:11] error: unable to get packages from source
[00:04:11] 
[00:04:11] Caused by:
[00:04:11]   failed to get 200 response from `https://crates.io/api/v1/crates/cc/1.0.15/download`, got 500
[00:04:11] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:04:11] Build completed unsuccessfully in 0:01:03
[00:04:11] Makefile:81: recipe for target 'prepare' failed
[00:04:11] make: *** [prepare] Error 1
[00:04:13]  Downloading serde v1.0.40
[00:04:38] warning: spurious network error (2 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/serde/1.0.40/download`, got 500
[00:04:38]  Downloading cc v1.0.15
[00:04:38]  Downloading time v0.1.39
---
[00:06:54]  Downloading quote v0.5.1
[00:06:54]  Downloading unicode-xid v0.1.0
[00:07:11] warning: spurious network error (2 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/unicode-xid/0.1.0/download`, got 500
[00:07:11]  Downloading itoa v0.4.1
[00:07:26] warning: spurious network error (2 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/itoa/0.4.1/download`, got 500
[00:08:15] warning: spurious network error (2 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/num-traits/0.2.2/download`, got 500
[00:08:15]  Downloading dtoa v0.4.2
[00:08:15]  Downloading fixedbitset v0.1.9
[00:08:30] warning: spurious network error (2 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/fixedbitset/0.1.9/download`, got 500
---
[00:09:55]  Downloading winapi v0.3.4
[00:10:19]  Downloading diff v0.1.11
[00:10:35] warning: spurious network error (2 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/diff/0.1.11/download`, got 500
[00:10:45]  Downloading miow v0.3.1
[00:10:59] warning: spurious network error (2 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/miow/0.3.1/download`, got 500
[00:10:59]  Downloading quote v0.3.15
[00:10:59]  Downloading syn v0.11.11
[00:11:25]  Downloading num-traits v0.1.43
[00:11:25]  Downloading bitflags v1.0.1
---
[00:15:57] warning: spurious network error (2 tries remaining): [28] Timeout was reached (Operation too slow. Less than 10 bytes/sec transferred the last 30 seconds)
[00:15:57]  Downloading failure_derive v0.1.1
[00:16:19] warning: spurious network error (2 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/failure_derive/0.1.1/download`, got 500
[00:16:49] warning: spurious network error (1 tries remaining): [28] Timeout was reached (Operation too slow. Less than 10 bytes/sec transferred the last 30 seconds)
[00:17:19] thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Error { description: "Timeout was reached", code: 28, extra: Some("Operation too slow. Less than 10 bytes/sec transferred the last 30 seconds") }
[00:17:19] unable to get packages from source', libcore/result.rs:945:5
[00:17:19] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:17:19] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:17:19] thread 'main' panicked at 'command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "metadata" "--format-version" "1" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml"
[00:17:19] expected success, got: exit code: 101', build_helper/lib.rs:122:9
[00:17:19] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:17:19] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build nonexistent/path/to/trigger/cargo/metadata
[00:17:19] Build completed unsuccessfully in 0:13:06
[00:17:19] Makefile:81: recipe for target 'prepare' failed
[00:17:19] make: *** [prepare] Error 1
[00:17:23]     Finished dev [unoptimized] target(s) in 0.26s
[00:17:23]  Downloading failure v0.1.1
[00:17:23]  Downloading datafrog v0.1.0
[00:17:53] warning: spurious network error (2 tries remaining): [28] Timeout was reached (Operation too slow. Less than 10 bytes/sec transferred the last 30 seconds)
[00:17:53] warning: spurious network error (2 tries remaining): [28] Timeout was reached (Operation too slow. Less than 10 bytes/sec transferred the last 30 seconds)
[00:18:23] warning: spurious network error (1 tries remaining): [28] Timeout was reached (Operation too slow. Less than 10 bytes/sec transferred the last 30 seconds)
[00:18:54] thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Error { description: "Timeout was reached", code: 28, extra: Some("Operation too slow. Less than 10 bytes/sec transferred the last 30 seconds") }
[00:18:54] unable to get packages from source', libcore/result.rs:945:5
[00:18:54] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:18:54] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:18:54] thread 'main' panicked at 'command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "metadata" "--format-version" "1" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml"
[00:18:54] expected success, got: exit code: 101', build_helper/lib.rs:122:9
[00:18:54] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:18:54] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build nonexistent/path/to/trigger/cargo/metadata
[00:18:54] make: *** [prepare] Error 1
[00:18:54] make: *** [prepare] Error 1
[00:18:54] Makefile:81: recipe for target 'prepare' failed
[00:18:58]     Finished dev [unoptimized] target(s) in 0.27s
[00:18:58]  Downloading rustc-ap-rustc_data_structures v149.0.0
[00:18:59]  Downloading atty v0.2.8
[00:18:59]  Downloading rustc-ap-serialize v149.0.0
---
[00:21:50]  Downloading fuchsia-zircon v0.3.3
[00:22:08]  Downloading foreign-types v0.3.2
[00:22:33]  Downloading thread_local v0.3.5
[00:22:33]  Downloading globset v0.3.0
[00:22:53] warning: spurious network error (2 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/globset/0.3.0/download`, got 500
[00:23:06] warning: spurious network error (1 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/globset/0.3.0/download`, got 500
[00:23:24] thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: HttpNot200 { code: 500, url: "https://crates.io/api/v1/crates/globset/0.3.0/download" }
[00:23:24] unable to get packages from source', libcore/result.rs:945:5
[00:23:24] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:23:24] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:23:24] thread 'main' panicked at 'command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "metadata" "--format-version" "1" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml"
[00:23:24] expected success, got: exit code: 101', build_helper/lib.rs:122:9
[00:23:24] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:23:24] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build nonexistent/path/to/trigger/cargo/metadata
[00:23:24] make: *** [prepare] Error 1
[00:23:24] make: *** [prepare] Error 1
[00:23:24] Makefile:81: recipe for target 'prepare' failed
[00:23:24] The command has failed after 5 attempts.

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:0087038c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Jun 24 09:30:53 UTC 2018
185384 ./obj/build/cache/2018-05-10
149124 ./src/llvm-emscripten/test
147692 ./.git/modules
147688 ./.git/modules/src
138744 ./obj/build/bootstrap/debug/incremental
124172 ./obj/build/bootstrap/debug/incremental/bootstrap-1r3bppl29tbrj
124168 ./obj/build/bootstrap/debug/incremental/bootstrap-1r3bppl29tbrj/s-f2a30xe62k-12af0mc-183djf9a4im2w
89808 ./src/llvm/test/CodeGen
72832 ./.git/modules/src/tools
68788 ./src/llvm/lib
65420 ./src/llvm-emscripten/test/CodeGen
