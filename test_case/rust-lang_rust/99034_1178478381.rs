plain
[RUSTC-TIMING] regex test:false 3.876
[RUSTC-TIMING] miri test:false 26.890
[RUSTC-TIMING] miri test:false 1.406
    Finished release [optimized] target(s) in 30.87s
duplicate artifacts found when compiling a tool, this typically means that something was recompiled because a transitive dependency has different features activated than in a previous build:

the following dependencies are duplicated although they have the same features enabled:
the following dependencies have different features:
  regex-syntax 0.6.25 (registry+https://github.com/rust-lang/crates.io-index)
    `miri` additionally enabled features {} at "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-freebsd/release/deps/libregex_syntax-f2ed343ec9a15a58.rlib"
    `cargo` additionally enabled features {"unicode-perl", "unicode-age", "unicode-gencat", "default", "unicode", "unicode-script", "unicode-bool", "unicode-case", "unicode-segment"} at "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-freebsd/release/deps/libregex_syntax-92eccfb941431f39.rlib"
  regex 1.5.5 (registry+https://github.com/rust-lang/crates.io-index)
    `miri` additionally enabled features {} at "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-freebsd/release/deps/libregex-88d84b6be8559626.rlib"
    `cargo` additionally enabled features {"unicode-case", "default", "unicode", "unicode-perl", "unicode-segment", "unicode-gencat", "unicode-script", "unicode-age", "unicode-bool"} at "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-freebsd/release/deps/libregex-9d57205a49b60b1e.rlib"

to fix this you will probably want to edit the local src/tools/rustc-workspace-hack/Cargo.toml crate, as that will update the dependency graph to ensure that these crates all share the same feature set
thread 'main' panicked at 'tools should not compile multiple copies of the same crate', tool.rs:197:13
Build completed unsuccessfully in 0:21:55
