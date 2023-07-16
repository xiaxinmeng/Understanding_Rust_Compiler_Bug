plain
[2022-12-17T05:29:27Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-12-17T05:29:27Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-12-17T05:29:27Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpUk4al7#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln"
Finished benchmark cargo-0.60.0 (1/7)
collector error: Failed to profile 'cargo-0.60.0' with Eprintln, recorded: expected success, got exit status: 101

stderr=   Compiling git2 v0.13.25
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "git2", "--edition=2018", "/cargo/registry/src/github.com-1ecc6299db9ec823/git2-0.13.25/src/lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "lib", "--emit=dep-info,metadata,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "--cfg", "feature=\"default\"", "--cfg", "feature=\"https\"", "--cfg", "feature=\"openssl-probe\"", "--cfg", "feature=\"openssl-sys\"", "--cfg", "feature=\"ssh\"", "--cfg", "feature=\"ssh_key_from_memory\"", "-C", "metadata=d015ea996c6b9259", "-C", "extra-filename=-d015ea996c6b9259", "--out-dir", "/tmp/.tmpUk4al7/target/debug/deps", "-C", "linker=clang", "-L", "dependency=/tmp/.tmpUk4al7/target/debug/deps", "--extern", "bitflags=/tmp/.tmpUk4al7/target/debug/deps/libbitflags-05dfd5b5d1225bed.rmeta", "--extern", "libc=/tmp/.tmpUk4al7/target/debug/deps/liblibc-f248f2e22eb0916d.rmeta", "--extern", "libgit2_sys=/tmp/.tmpUk4al7/target/debug/deps/liblibgit2_sys-5f1a238f4aad01e6.rmeta", "--extern", "log=/tmp/.tmpUk4al7/target/debug/deps/liblog-41607b4997ff699e.rmeta", "--extern", "openssl_probe=/tmp/.tmpUk4al7/target/debug/deps/libopenssl_probe-9fb528d4b515d117.rmeta", "--extern", "openssl_sys=/tmp/.tmpUk4al7/target/debug/deps/libopenssl_sys-acef1491a9c830ea.rmeta", "--extern", "url=/tmp/.tmpUk4al7/target/debug/deps/liburl-d12fc40f8e40722d.rmeta", "--cap-lints", "allow", "-L", "native=/tmp/.tmpUk4al7/target/debug/build/libgit2-sys-da3c6ff9ee6b1457/out/build", "-L", "native=/tmp/.tmpUk4al7/target/debug/build/libssh2-sys-3d6a254962dd01bf/out/build", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
error: could not compile `git2`

 stdout=

Executing benchmark clap-3.1.6 (2/7)
