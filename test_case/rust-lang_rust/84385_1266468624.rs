plain
[2022-10-04T03:48:15Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-10-04T03:48:15Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-10-04T03:48:15Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpsePb4x#hyper@0.14.18" "--features=client,http1,http2,server,stream" "--" "--wrap-rustc-with" "Eprintln"
Finished benchmark hyper-0.14.18 (3/7)
collector error: Failed to profile 'hyper-0.14.18' with Eprintln, recorded: expected success, got exit status: 101

stderr=   Compiling http-body v0.4.4
   Compiling tracing-attributes v0.1.18
   Compiling tracing-attributes v0.1.18
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "http_body", "--edition=2018", "/cargo/registry/src/github.com-1ecc6299db9ec823/http-body-0.4.4/src/lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "lib", "--emit=dep-info,metadata,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "-C", "metadata=b3012d31e53829d4", "-C", "extra-filename=-b3012d31e53829d4", "--out-dir", "/tmp/.tmpsePb4x/target/debug/deps", "-C", "linker=clang", "-L", "dependency=/tmp/.tmpsePb4x/target/debug/deps", "--extern", "bytes=/tmp/.tmpsePb4x/target/debug/deps/libbytes-5e8424a0f7103369.rmeta", "--extern", "http=/tmp/.tmpsePb4x/target/debug/deps/libhttp-4da2cfc5497c929c.rmeta", "--extern", "pin_project_lite=/tmp/.tmpsePb4x/target/debug/deps/libpin_project_lite-d4223cfaa04037d7.rmeta", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
warning: build failed, waiting for other jobs to finish...
warning: build failed, waiting for other jobs to finish...
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "tokio_macros", "--edition=2018", "/cargo/registry/src/github.com-1ecc6299db9ec823/tokio-macros-1.7.0/src/lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "proc-macro", "--emit=dep-info,link", "-C", "prefer-dynamic", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "-C", "metadata=02fe0df9f514a20a", "-C", "extra-filename=-02fe0df9f514a20a", "--out-dir", "/tmp/.tmpsePb4x/target/debug/deps", "-C", "linker=clang", "-L", "dependency=/tmp/.tmpsePb4x/target/debug/deps", "--extern", "proc_macro2=/tmp/.tmpsePb4x/target/debug/deps/libproc_macro2-d937f1f82ffb437d.rlib", "--extern", "quote=/tmp/.tmpsePb4x/target/debug/deps/libquote-67e27136f08bcb92.rlib", "--extern", "syn=/tmp/.tmpsePb4x/target/debug/deps/libsyn-e7f320076a693462.rlib", "--extern", "proc_macro", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
error: could not compile `tokio-macros`
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "tracing_attributes", "--edition=2018", "/cargo/registry/src/github.com-1ecc6299db9ec823/tracing-attributes-0.1.18/src/lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "proc-macro", "--emit=dep-info,link", "-C", "prefer-dynamic", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "-C", "metadata=5c84e2ca77b89e67", "-C", "extra-filename=-5c84e2ca77b89e67", "--out-dir", "/tmp/.tmpsePb4x/target/debug/deps", "-C", "linker=clang", "-L", "dependency=/tmp/.tmpsePb4x/target/debug/deps", "--extern", "proc_macro2=/tmp/.tmpsePb4x/target/debug/deps/libproc_macro2-d937f1f82ffb437d.rlib", "--extern", "quote=/tmp/.tmpsePb4x/target/debug/deps/libquote-67e27136f08bcb92.rlib", "--extern", "syn=/tmp/.tmpsePb4x/target/debug/deps/libsyn-e7f320076a693462.rlib", "--extern", "proc_macro", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
error: could not compile `tracing-attributes`

 stdout=

Executing benchmark regex-1.5.5 (4/7)
