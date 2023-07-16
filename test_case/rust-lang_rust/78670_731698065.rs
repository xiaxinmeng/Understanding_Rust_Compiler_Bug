
warning: spurious network error (2 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/quote/0.3.15/download`, got 504
warning: spurious network error (1 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/quote/0.3.15/download`, got 504
error: failed to download from `https://crates.io/api/v1/crates/quote/0.3.15/download`

Caused by:
  failed to get 200 response from `https://crates.io/api/v1/crates/quote/0.3.15/download`, got 504
thread 'main' panicked at 'tests failed for https://github.com/BurntSushi/xsv', src\tools\cargotest\main.rs:72:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
