
 Downloading crates ...
warning: spurious network error (2 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/colored/2.0.0/download`, got 502
warning: spurious network error (1 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/colored/2.0.0/download`, got 502
thread 'main' panicked at 'packages downloaded: failed to download from `https://crates.io/api/v1/crates/colored/2.0.0/download`

Caused by:
    failed to get 200 response from `https://crates.io/api/v1/crates/colored/2.0.0/download`, got 502', src/tools/cargo/src/cargo/ops/cargo_compile.rs:1054:14
