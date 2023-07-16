
$ rustup target add aarch64-apple-ios
info: component 'rust-std' for target 'aarch64-apple-ios' is up to date
$ cargo lipo --release
error: failed to run `rustc` to learn about target-specific information

Caused by:
  process didn't exit successfully: `rustc - --crate-name ___ --print=file-names --crate-type bin --crate-type proc-macro --crate-type rlib --crate-type staticlib --target aarch64-apple-ios` (exit code: 101)
--- stderr
error: Error loading target specification: Could not find specification for target "aarch64-apple-ios"
  |
  = help: Use `--print target-list` for a list of built-in targets


Cargo exited unsuccessfully: exit code: 101
$ cargo build --release --target aarch64-apple-ios
error: failed to run `rustc` to learn about target-specific information

Caused by:
  process didn't exit successfully: `rustc - --crate-name ___ --print=file-names --crate-type bin --crate-type proc-macro --crate-type rlib --crate-type staticlib --target aarch64-apple-ios` (exit code: 101)
--- stderr
error: Error loading target specification: Could not find specification for target "aarch64-apple-ios"
  |
  = help: Use `--print target-list` for a list of built-in targets
