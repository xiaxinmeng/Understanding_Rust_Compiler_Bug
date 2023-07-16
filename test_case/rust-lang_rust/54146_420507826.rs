plain
[00:05:02] make: *** [prepare] Error 1
[00:05:05] Command failed. Attempt 4/5:
[00:05:05]     Finished dev [unoptimized] target(s) in 0.27s
[00:05:06]  Downloading winapi v0.2.8
[00:05:16] warning: spurious network error (2 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/winapi/0.2.8/download`, got 500
[00:05:26] warning: spurious network error (1 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/winapi/0.2.8/download`, got 500
[00:05:36] 
[00:05:36] Caused by:
[00:05:36] Caused by:
[00:05:36]   failed to get 200 response from `https://crates.io/api/v1/crates/winapi/0.2.8/download`, got 500
[00:05:36] expected success, got: exit code: 101', build_helper/lib.rs:123:9
[00:05:36] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:05:36] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build nonexistent/path/to/trigger/cargo/metadata
[00:05:36] Build completed unsuccessfully in 0:00:30
