plain
[2022-11-14T10:54:24Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-11-14T10:54:24Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-11-14T10:54:24Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCUBCYt#hyper@0.14.18" "--features=client,http1,http2,server,stream" "--" "--wrap-rustc-with" "Eprintln"
Finished benchmark hyper-0.14.18 (3/7)
collector error: Failed to profile 'hyper-0.14.18' with Eprintln, recorded: expected success, got exit status: 101

stderr=   Compiling tokio v1.16.1
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "tokio", "--edition=2018", "/cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.16.1/src/lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "lib", "--emit=dep-info,metadata,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "--cfg", "feature=\"bytes\"", "--cfg", "feature=\"default\"", "--cfg", "feature=\"fs\"", "--cfg", "feature=\"io-std\"", "--cfg", "feature=\"io-util\"", "--cfg", "feature=\"macros\"", "--cfg", "feature=\"memchr\"", "--cfg", "feature=\"num_cpus\"", "--cfg", "feature=\"rt\"", "--cfg", "feature=\"rt-multi-thread\"", "--cfg", "feature=\"sync\"", "--cfg", "feature=\"test-util\"", "--cfg", "feature=\"time\"", "--cfg", "feature=\"tokio-macros\"", "-C", "metadata=a385249852872367", "-C", "extra-filename=-a385249852872367", "--out-dir", "/tmp/.tmpCUBCYt/target/debug/deps", "-C", "linker=clang", "-L", "dependency=/tmp/.tmpCUBCYt/target/debug/deps", "--extern", "bytes=/tmp/.tmpCUBCYt/target/debug/deps/libbytes-5e8424a0f7103369.rmeta", "--extern", "memchr=/tmp/.tmpCUBCYt/target/debug/deps/libmemchr-e6bbee201eb478c9.rmeta", "--extern", "num_cpus=/tmp/.tmpCUBCYt/target/debug/deps/libnum_cpus-266fedb888314087.rmeta", "--extern", "pin_project_lite=/tmp/.tmpCUBCYt/target/debug/deps/libpin_project_lite-d4223cfaa04037d7.rmeta", "--extern", "tokio_macros=/tmp/.tmpCUBCYt/target/debug/deps/libtokio_macros-02fe0df9f514a20a.so", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
error: could not compile `tokio`

 stdout=

Executing benchmark regex-1.5.5 (4/7)
