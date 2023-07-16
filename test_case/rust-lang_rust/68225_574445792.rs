plain
2020-01-15T00:54:44.5946188Z [RUSTC-TIMING] rustfmt test:false 12.009
2020-01-15T00:54:47.7791839Z [RUSTC-TIMING] rustfmt_format_diff test:false 3.923
2020-01-15T00:54:51.8097372Z [RUSTC-TIMING] git_rustfmt test:false 7.209
2020-01-15T00:54:51.8125294Z     Finished release [optimized] target(s) in 8m 21s
2020-01-15T00:54:51.8327559Z duplicate artifacts found when compiling a tool, this typically means that something was recompiled because a transitive dependency has different features activated than in a previous build:
2020-01-15T00:54:51.8329838Z the following dependencies are duplicated although they have the same features enabled:
2020-01-15T00:54:51.8330071Z the following dependencies have different features:
2020-01-15T00:54:51.8330071Z the following dependencies have different features:
2020-01-15T00:54:51.8330640Z   syn 1.0.11 (registry+https://github.com/rust-lang/crates.io-index)
2020-01-15T00:54:51.8332040Z     `rustfmt` additionally enabled features {"full"} at "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/powerpc-unknown-linux-gnu/release/deps/libsyn-c46f31f7c0d3689b.rlib"
2020-01-15T00:54:51.8332651Z     `cargo` additionally enabled features {} at "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/powerpc-unknown-linux-gnu/release/deps/libsyn-e2851189709bbdb1.rlib"
2020-01-15T00:54:51.8332942Z 
2020-01-15T00:54:51.8333730Z to fix this you will probably want to edit the local src/tools/rustc-workspace-hack/Cargo.toml crate, as that will update the dependency graph to ensure that these crates all share the same feature set
2020-01-15T00:54:51.8334292Z thread 'main' panicked at 'tools should not compile multiple copies of the same crate', src/bootstrap/tool.rs:195:13
2020-01-15T00:54:51.8372166Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host powerpc-unknown-linux-gnu --target powerpc-unknown-linux-gnu
2020-01-15T00:54:51.8372532Z Build completed unsuccessfully in 1:19:46
2020-01-15T00:54:51.8413280Z == clock drift check ==
2020-01-15T00:54:51.8428130Z   local time: Wed Jan 15 00:54:51 UTC 2020
2020-01-15T00:54:51.8428130Z   local time: Wed Jan 15 00:54:51 UTC 2020
2020-01-15T00:54:52.0327230Z   network time: Wed, 15 Jan 2020 00:54:52 GMT
2020-01-15T00:54:52.0329446Z == end clock drift check ==
2020-01-15T00:54:53.2112117Z 
2020-01-15T00:54:53.2194637Z ##[error]Bash exited with code '1'.
2020-01-15T00:54:53.2230608Z ##[section]Starting: Checkout
2020-01-15T00:54:53.2232283Z ==============================================================================
2020-01-15T00:54:53.2232369Z Task         : Get sources
2020-01-15T00:54:53.2232430Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
