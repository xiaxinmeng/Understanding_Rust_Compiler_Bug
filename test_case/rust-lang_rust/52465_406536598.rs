
[00:03:13]  Downloading openssl v0.10.10
[00:03:44] warning: spurious network error (2 tries remaining): [28] Timeout was reached (Operation too slow. Less than 10 bytes/sec transferred the last 30 seconds)
[00:04:14] warning: spurious network error (1 tries remaining): [28] Timeout was reached (Operation too slow. Less than 10 bytes/sec transferred the last 30 seconds)
[00:04:44] error: unable to get packages from source
[00:04:44] 
[00:04:44] Caused by:
[00:04:44]   [28] Timeout was reached (Operation too slow. Less than 10 bytes/sec transferred the last 30 seconds)
[00:04:44] thread 'main' panicked at 'command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "metadata" "--format-version" "1" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml"
[00:04:44] expected success, got: exit code: 101', build_helper/lib.rs:125:9
[00:04:44] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:04:44] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
[00:04:44] Build completed unsuccessfully in 0:03:17
