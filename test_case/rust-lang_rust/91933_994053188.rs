`
  Downloaded libgit2-sys v0.10.0
warning: spurious network error (2 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/regex/1.3.4/download`, got 502
warning: spurious network error (1 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/regex/1.3.4/download`, got 502
error: failed to download from `https://crates.io/api/v1/crates/regex/1.3.4/download`

Caused by:
  failed to get 200 response from `https://crates.io/api/v1/crates/regex/1.3.4/download`, got 502
thread 'main' panicked at 'tests failed for https://github.com/XAMPPRocky/tokei', src\tools\cargotest\main.rs:101:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


command did not execute successfully: "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-tools-bin\\cargotest.exe" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2-tools-bin\\cargo.exe" "D:\\a\\rust\\rust\\build\\ct"
expected success, got: exit code: 101



