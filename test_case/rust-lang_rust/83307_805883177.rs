
warning: spurious network error (1 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/utf8parse/0.1.1/download`, got 503
warning: spurious network error (1 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/libz-sys/1.1.2/download`, got 503
warning: spurious network error (1 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/foreign-types/0.3.2/download`, got 503
error: failed to download from `https://crates.io/api/v1/crates/home/0.5.3/download`

Caused by:
  failed to get 200 response from `https://crates.io/api/v1/crates/home/0.5.3/download`, got 503
command did not execute successfully: "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0/bin/cargo" "build" "--target" "aarch64-apple-darwin" "-Zdual-proc-macros" "-Zbinary-dep-depinfo" "-j" "3" "--release" "--locked" "--color" "always" "--manifest-path" "/Users/runner/work/rust/rust/src/tools/cargo/Cargo.toml" "--features" "rustc-workspace-hack/all-static" "--message-format" "json-render-diagnostics"
expected success, got: exit code: 101
failed to run: /Users/runner/work/rust/rust/build/bootstrap/debug/bootstrap dist --stage 2
Build completed unsuccessfully in 1:58:00
== clock drift check ==
  local time: Wed Mar 24 09:03:12 UTC 2021
  network time: Wed, 24 Mar 2021 07:05:38 GMT
== end clock drift check ==
Error: Process completed with exit code 1.
