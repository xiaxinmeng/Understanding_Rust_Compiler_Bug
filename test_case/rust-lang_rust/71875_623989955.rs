
duplicate artifacts found when compiling a tool, this typically means that something was recompiled because a transitive dependency has different features activated than in a previous build:

the following dependencies are duplicated although they have the same features enabled:
the following dependencies have different features:
  syn 1.0.11 (registry+https://github.com/rust-lang/crates.io-index)
    `rls` additionally enabled features {"fold"} at "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/arm-unknown-linux-gnueabihf/release/deps/libsyn-d311e1ffc0871272.rlib"
    `rustfmt` additionally enabled features {} at "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/arm-unknown-linux-gnueabihf/release/deps/libsyn-5e1327192627d80b.rlib"

to fix this you will probably want to edit the local src/tools/rustc-workspace-hack/Cargo.toml crate, as that will update the dependency graph to ensure that these crates all share the same feature set
thread 'main' panicked at 'tools should not compile multiple copies of the same crate', src/bootstrap/tool.rs:195:13
