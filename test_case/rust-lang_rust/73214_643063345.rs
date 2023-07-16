
  Downloaded itertools v0.8.0
  Downloaded vte v0.3.3
warning: spurious network error (2 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/precomputed-hash/0.1.1/download`, got 502
warning: spurious network error (1 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/precomputed-hash/0.1.1/download`, got 502
error: failed to download from `https://crates.io/api/v1/crates/precomputed-hash/0.1.1/download`

Caused by:
  failed to get 200 response from `https://crates.io/api/v1/crates/precomputed-hash/0.1.1/download`, got 502
thread 'main' panicked at 'command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "metadata" "--format-version" "1" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml"
expected success, got: exit code: 101', src/build_helper/lib.rs:131:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
