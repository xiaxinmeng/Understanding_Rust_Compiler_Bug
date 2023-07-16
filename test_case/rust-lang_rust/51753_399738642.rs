plain
[00:02:02]  Downloading cmake v0.1.30
[00:02:02]  Downloading getopts v0.2.17
[00:02:02]  Downloading serde_json v1.0.15
[00:02:03]  Downloading petgraph v0.4.12
[00:02:16] warning: spurious network error (2 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/petgraph/0.4.12/download`, got 500
[00:02:16]  Downloading time v0.1.39
[00:02:37]  Downloading lazy_static v0.2.11
[00:02:51] warning: spurious network error (2 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/lazy_static/0.2.11/download`, got 500
[00:02:51]  Downloading toml v0.4.6
---
[00:07:39]  Downloading rustc-ap-rustc_data_structures v164.0.0
[00:07:40]  Downloading termcolor v0.3.6
[00:07:40]  Downloading rustc-ap-serialize v164.0.0
[00:07:40]  Downloading atty v0.2.8
[00:07:58] warning: spurious network error (2 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/atty/0.2.8/download`, got 500
[00:07:59]  Downloading rustc-ap-syntax_pos v164.0.0
[00:08:13] warning: spurious network error (2 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/rustc-ap-syntax_pos/164.0.0/download`, got 500
[00:08:35]  Downloading mac v0.1.1
[00:08:35]  Downloading debug_unreachable v0.1.1
[00:08:35]  Downloading debug_unreachable v0.1.1
[00:08:35]  Downloading either v1.5.0
[00:08:56] warning: spurious network error (2 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/either/1.5.0/download`, got 500
[00:09:09] warning: spurious network error (2 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/rustc-rayon-core/0.1.1/download`, got 500
[00:09:09]  Downloading bitflags v1.0.1
[00:09:28] warning: spurious network error (2 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/bitflags/1.0.1/download`, got 500
[00:09:28]  Downloading fuchsia-zircon-sys v0.3.3
---
[00:10:10]  Downloading memoffset v0.2.1
[00:10:24] warning: spurious network error (2 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/memoffset/0.2.1/download`, got 500
[00:10:46]  Downloading remove_dir_all v0.5.1
[00:11:06]  Downloading winapi v0.3.4
[00:11:19] warning: spurious network error (2 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/winapi/0.3.4/download`, got 500
[00:11:48]  Downloading redox_syscall v0.1.37
[00:12:07] warning: spurious network error (2 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/redox_syscall/0.1.37/download`, got 500
[00:12:20] warning: spurious network error (1 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/redox_syscall/0.1.37/download`, got 500
[00:12:20]  Downloading rand v0.3.22
---
[00:13:51]  Downloading url v1.7.0
[00:14:12]  Downloading libgit2-sys v0.7.1
[00:14:13]  Downloading openssl-sys v0.9.28
[00:14:28]  Downloading rustc-serialize v0.3.24
[00:14:41] warning: spurious network error (2 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/rustc-serialize/0.3.24/download`, got 500
[00:14:55] warning: spurious network error (1 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/rustc-serialize/0.3.24/download`, got 500
[00:15:21] thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: HttpNot200 { code: 500, url: "https://crates.io/api/v1/crates/rustc-serialize/0.3.24/download" }
[00:15:21] unable to get packages from source', libcore/result.rs:945:5
[00:15:21] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:15:21] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:15:21] thread 'main' panicked at 'command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "metadata" "--format-version" "1" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml"
[00:15:21] expected success, got: exit code: 101', build_helper/lib.rs:122:9
[00:15:21] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:15:21] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build nonexistent/path/to/trigger/cargo/metadata
[00:15:21] make: *** [prepare] Error 1
[00:15:21] make: *** [prepare] Error 1
[00:15:21] Makefile:81: recipe for target 'prepare' failed
[00:15:22]     Finished dev [unoptimized] target(s) in 0.25s
[00:15:23]  Downloading rustc-ap-rustc_target v164.0.0
[00:15:23]  Downloading derive-new v0.5.4
[00:15:44]  Downloading isatty v0.1.8
[00:15:44]  Downloading isatty v0.1.8
[00:15:58] warning: spurious network error (2 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/isatty/0.1.8/download`, got 500
[00:15:58]  Downloading failure v0.1.1
[00:15:58]  Downloading itertools v0.7.8
[00:15:58]  Downloading env_logger v0.5.8
[00:15:58]  Downloading term v0.5.1
[00:15:58]  Downloading term v0.5.1
[00:15:58]  Downloading assert_cli v0.6.0
[00:15:58]  Downloading rustc-ap-syntax v164.0.0
[00:15:58]  Downloading unicode-segmentation v1.2.0
[00:15:58]  Downloading diff v0.1.11
[00:16:11] warning: spurious network error (2 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/diff/0.1.11/download`, got 500
[00:16:25] warning: spurious network error (1 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/diff/0.1.11/download`, got 500
[00:16:38] thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: HttpNot200 { code: 500, url: "https://crates.io/api/v1/crates/diff/0.1.11/download" }
[00:16:38] unable to get packages from source', libcore/result.rs:945:5
[00:16:38] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:16:38] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:16:39] thread 'main' panicked at 'command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "metadata" "--format-version" "1" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml"
[00:16:39] expected success, got: exit code: 101', build_helper/lib.rs:122:9
[00:16:39] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:16:39] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build nonexistent/path/to/trigger/cargo/metadata
[00:16:39] make: *** [prepare] Error 1
[00:16:39] make: *** [prepare] Error 1
[00:16:39] Makefile:81: recipe for target 'prepare' failed
[00:16:41]     Finished dev [unoptimized] target(s) in 0.27s
[00:16:41]  Downloading syntex_pos v0.52.0
[00:16:55] warning: spurious network error (2 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/syntex_pos/0.52.0/download`, got 500
[00:17:08] warning: spurious network error (1 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/syntex_pos/0.52.0/download`, got 500
---
[00:19:16]  Downloading vcpkg v0.2.3
[00:19:16]  Downloading backtrace v0.3.6
[00:19:16]  Downloading rls-span v0.4.0
[00:19:16]  Downloading racer v2.0.14
[00:19:30] warning: spurious network error (2 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/racer/2.0.14/download`, got 500
[00:19:43] warning: spurious network error (1 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/racer/2.0.14/download`, got 500
[00:19:43]  Downloading environment v0.1.1
[00:20:05]  Downloading difference v2.0.0
[00:20:05]  Downloading colored v1.6.0
[00:20:06]  Downloading failure_derive v0.1.1
---
[00:21:49] warning: spurious network error (2 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/rustc-ap-rustc_cratesio_shim/164.0.0/download`, got 500
[00:21:49]  Downloading owning_ref v0.3.3
[00:22:10]  Downloading crossbeam v0.3.2
[00:22:10]  Downloading walkdir v2.1.4
[00:22:23] warning: spurious network error (2 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/walkdir/2.1.4/download`, got 500
[00:22:37] warning: spurious network error (1 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/walkdir/2.1.4/download`, got 500
[00:22:58]  Downloading thread_local v0.3.5
[00:23:12] warning: spurious network error (2 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/thread_local/0.3.5/download`, got 500
[00:23:12]  Downloading memchr v2.0.1
[00:23:25] warning: spurious network error (2 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/memchr/2.0.1/download`, got 500
[00:23:25] warning: spurious network error (2 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/memchr/2.0.1/download`, got 500
[00:23:39] warning: spurious network error (1 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/memchr/2.0.1/download`, got 500
[00:23:52] thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: HttpNot200 { code: 500, url: "https://crates.io/api/v1/crates/memchr/2.0.1/download" }
[00:23:52] unable to get packages from source', libcore/result.rs:945:5
[00:23:52] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:23:52] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:23:52] thread 'main' panicked at 'command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "metadata" "--format-version" "1" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml"
[00:23:52] expected success, got: exit code: 101', build_helper/lib.rs:122:9
[00:23:52] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:23:52] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build nonexistent/path/to/trigger/cargo/metadata
[00:23:52] Build completed unsuccessfully in 0:07:11
[00:23:52] Makefile:81: recipe for target 'prepare' failed
[00:23:52] make: *** [prepare] Error 1
[00:23:55]     Finished dev [unoptimized] target(s) in 0.25s
[00:23:55]  Downloading unicode-xid v0.0.4
[00:23:56]  Downloading pulldown-cmark v0.1.2
[00:23:56]  Downloading matches v0.1.6
[00:23:56]  Downloading matches v0.1.6
[00:23:56]  Downloading if_chain v0.1.2
[00:23:56]  Downloading regex-syntax v0.6.0
[00:24:10] warning: spurious network error (2 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/regex-syntax/0.6.0/download`, got 500
[00:24:23] warning: spurious network error (1 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/regex-syntax/0.6.0/download`, got 500
[00:24:23]  Downloading quine-mc_cluskey v0.2.4
[00:24:23]  Downloading unicode-normalization v0.1.5
[00:24:37] warning: spurious network error (2 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/unicode-normalization/0.1.5/download`, got 500
[00:24:50] warning: spurious network error (1 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/unicode-normalization/0.1.5/download`, got 500
[00:25:03] thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: HttpNot200 { code: 500, url: "https://crates.io/api/v1/crates/unicode-normalization/0.1.5/download" }
[00:25:03] unable to get packages from source', libcore/result.rs:945:5
[00:25:03] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:25:03] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:25:03] thread 'main' panicked at 'command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "metadata" "--format-version" "1" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml"
[00:25:03] expected success, got: exit code: 101', build_helper/lib.rs:122:9
[00:25:03] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:25:03] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build nonexistent/path/to/trigger/cargo/metadata
[00:25:03] Build completed unsuccessfully in 0:01:08
[00:25:03] Makefile:81: recipe for target 'prepare' failed
[00:25:03] make: *** [prepare] Error 1
[00:25:08]     Finished dev [unoptimized] target(s) in 0.24s
[00:25:08]  Downloading rustfix v0.3.1
[00:25:08]  Downloading miow v0.3.1
[00:25:09]  Downloading diff v0.1.11
---
[00:26:55]  Downloading error-chain v0.11.0
[00:27:22]  Downloading crossbeam-deque v0.2.0
[00:27:22]  Downloading kernel32-sys v0.2.2
[00:27:22]  Downloading winapi v0.2.8
[00:27:35] warning: spurious network error (2 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/winapi/0.2.8/download`, got 500
[00:28:13] warning: spurious network error (2 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/curl/0.4.12/download`, got 500
[00:28:27] warning: spurious network error (1 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/curl/0.4.12/download`, got 500
[00:28:27]  Downloading lzma-sys v0.1.9
[00:28:48] warning: spurious network error (2 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/lzma-sys/0.1.9/download`, got 500
[00:28:48] warning: spurious network error (2 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/lzma-sys/0.1.9/download`, got 500
[00:28:48]  Downloading mdbook v0.1.7
[00:28:48]  Downloading clap v2.31.2
[00:29:02] warning: spurious network error (2 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/clap/2.31.2/download`, got 500
[00:29:02]  Downloading flate2 v1.0.1
[00:29:02]  Downloading tar v0.4.15
[00:29:17] warning: spurious network error (2 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/tar/0.4.15/download`, got 500
[00:29:30] warning: spurious network error (1 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/tar/0.4.15/download`, got 500
[00:29:44] warning: spurious network error (2 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/rayon/1.0.1/download`, got 500
[00:29:57] warning: spurious network error (1 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/rayon/1.0.1/download`, got 500
arget/X86
7304 ./src/llvm/test/CodeGen/ARM
---
travis_time:end:1f1788aa:start=1529828365641371952,finish=1529828365653118340,duration=11746388
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:04b0dfc0
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_
