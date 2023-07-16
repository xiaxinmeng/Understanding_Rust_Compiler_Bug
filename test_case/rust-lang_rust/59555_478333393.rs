
[01:21:53] the following dependencies have different features:
[01:21:53]   rand 0.6.1 (registry+https://github.com/rust-lang/crates.io-index)
[01:21:53]     `miri` additionally enabled features {} at "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2-tools/x86_64-apple-darwin/release/deps/librand-bb9ec2d01a61cf1d.rlib"
[01:21:53]     `cargo` additionally enabled features {"i128_support"} at "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2-tools/x86_64-apple-darwin/release/deps/librand-ff2bb5dd6d03d558.rlib"
[01:21:53] 
[01:21:53] to fix this you will probably want to edit the local src/tools/rustc-workspace-hack/Cargo.toml crate, as that will update the dependency graph to ensure that these crates all share the same feature set
[01:21:53] thread 'main' panicked at 'tools should not compile multiple copies of the same crate', src/bootstrap/tool.rs:170:13
