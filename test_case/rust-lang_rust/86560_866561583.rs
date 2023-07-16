plain
[RUSTC-TIMING] cargo_fmt test:false 2.641
[RUSTC-TIMING] git_rustfmt test:false 3.680
[RUSTC-TIMING] rustfmt test:false 3.864
    Finished release [optimized] target(s) in 40.22s
thread 'main' panicked at 'tools should not compile multiple copies of the same crate', src/bootstrap/tool.rs:199:13
duplicate artifacts found when compiling a tool, this typically means that something was recompiled because a transitive dependency has different features activated than in a previous build:


the following dependencies are duplicated although they have the same features enabled:
the following dependencies have different features:
  bstr 0.2.13 (registry+https://github.com/rust-lang/crates.io-index)
    `rustfmt` additionally enabled features {} at "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-freebsd/release/deps/libbstr-4c0d5521d98df29e.rlib"
    `cargo` additionally enabled features {"regex-automata", "default", "lazy_static", "unicode"} at "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-freebsd/release/deps/libbstr-8212bfe6d684f840.rlib"

to fix this you will probably want to edit the local src/tools/rustc-workspace-hack/Cargo.toml crate, as that will update the dependency graph to ensure that these crates all share the same feature set
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-freebsd --target x86_64-unknown-freebsd
