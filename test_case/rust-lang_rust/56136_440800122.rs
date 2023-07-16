`
Building stage2 tool cargo (x86_64-apple-darwin)
[01:07:24]  Downloading crates ...
[01:07:24] warning: spurious network error (2 tries remaining): [6] Couldn't resolve host name (Could not resolve host: crates.io)
[01:07:24] warning: spurious network error (1 tries remaining): [6] Couldn't resolve host name (Could not resolve host: crates.io)
[01:07:24] error: failed to download from `https://crates.io/api/v1/crates/openssl-src/111.0.1+1.1.1/download`
[01:07:24] Caused by:
[01:07:24]   [6] Couldn't resolve host name (Could not resolve host: crates.io)
[01:07:24] command did not execute successfully: "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage0/bin/cargo" "build" "--target" "x86_64-apple-darwin" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/Users/travis/build/rust-lang/rust/src/tools/cargo/Cargo.toml" "--features" "rustc-workspace-hack/all-static" "--message-format" "json"
[01:07:24] expected success, got: exit code: 101
[01:07:24] expected success, got: exit code: 101
