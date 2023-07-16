plain
[RUSTC-TIMING] cargo_fmt test:false 2.659
[RUSTC-TIMING] git_rustfmt test:false 5.592
[RUSTC-TIMING] rustfmt test:false 5.628
    Finished release [optimized] target(s) in 36.81s
duplicate artifacts found when compiling a tool, this typically means that something was recompiled because a transitive dependency has different features activated than in a previous build:
the following dependencies have different features:
the following dependencies have different features:
  syn 1.0.91 (registry+https://github.com/rust-lang/crates.io-index)
    `rustfmt` additionally enabled features {"visit"} at "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/powerpc64-unknown-linux-gnu/release/deps/libsyn-44cb9feed67d615b.rlib"
    `cargo` additionally enabled features {} at "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/powerpc64-unknown-linux-gnu/release/deps/libsyn-f8400fc61d7292f5.rlib"

to fix this you will probably want to edit the local src/tools/rustc-workspace-hack/Cargo.toml crate, as that will update the dependency graph to ensure that these crates all share the same feature set
thread 'main' panicked at 'tools should not compile multiple copies of the same crate', tool.rs:201:13
Build completed unsuccessfully in 0:20:37
