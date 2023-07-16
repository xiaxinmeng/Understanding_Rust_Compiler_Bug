
the following dependencies are duplicated although they have the same features enabled:
the following dependencies have different features:
  byteorder 1.3.4 (registry+https://github.com/rust-lang/crates.io-index)
    `rustfmt` additionally enabled features {} at "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libbyteorder-c5e2ae63a4128905.rlib"
    `rls` additionally enabled features {"default", "std"} at "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libbyteorder-286667693f43f34e.rlib"

to fix this you will probably want to edit the local src/tools/rustc-workspace-hack/Cargo.toml crate, as that will update the dependency graph to ensure that these crates all share the same feature set
