plain
[2022-10-07T21:05:39Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpHnPikB#hyper@0.14.18" "--features=client,http1,http2,server,stream" "--" "--skip-this-rustc"
Running hyper-0.14.18: Debug + [Full]
[2022-10-07T21:06:00Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-10-07T21:06:00Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-10-07T21:06:00Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpMyniiW#hyper@0.14.18" "--features=client,http1,http2,server,stream" "--" "--wrap-rustc-with" "Eprintln"
Finished benchmark hyper-0.14.18 (3/7)
collector error: Failed to profile 'hyper-0.14.18' with Eprintln, recorded: expected success, got exit status: 101

stderr=   Compiling tokio-util v0.6.9
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "tokio_util", "--edition=2018", "/cargo/registry/src/github.com-1ecc6299db9ec823/tokio-util-0.6.9/src/lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "lib", "--emit=dep-info,metadata,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "--cfg", "feature=\"codec\"", "--cfg", "feature=\"default\"", "-C", "metadata=7b7b879e83c6c87f", "-C", "extra-filename=-7b7b879e83c6c87f", "--out-dir", "/tmp/.tmpMyniiW/target/debug/deps", "-C", "linker=clang", "-L", "dependency=/tmp/.tmpMyniiW/target/debug/deps", "--extern", "bytes=/tmp/.tmpMyniiW/target/debug/deps/libbytes-5e8424a0f7103369.rmeta", "--extern", "futures_core=/tmp/.tmpMyniiW/target/debug/deps/libfutures_core-2cccdbab4f5fe80d.rmeta", "--extern", "futures_sink=/tmp/.tmpMyniiW/target/debug/deps/libfutures_sink-5a29710ac8e9aa10.rmeta", "--extern", "log=/tmp/.tmpMyniiW/target/debug/deps/liblog-41607b4997ff699e.rmeta", "--extern", "pin_project_lite=/tmp/.tmpMyniiW/target/debug/deps/libpin_project_lite-d4223cfaa04037d7.rmeta", "--extern", "tokio=/tmp/.tmpMyniiW/target/debug/deps/libtokio-a385249852872367.rmeta", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
error: could not compile `tokio-util`

 stdout=

Executing benchmark regex-1.5.5 (4/7)
---
Executing benchmark syn-1.0.89 (7/7)
Preparing syn-1.0.89
[2022-10-07T21:07:15Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-10-07T21:07:15Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-10-07T21:07:15Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmplDjQAj#syn@1.0.89" "--" "--skip-this-rustc"
[2022-10-07T21:07:15Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpLtNaUT#syn@1.0.89" "--release" "--" "--skip-this-rustc"
[2022-10-07T21:07:18Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-10-07T21:07:18Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-10-07T21:07:18Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpZjKIif#syn@1.0.89" "--" "--wrap-rustc-with" "Eprintln"
Running syn-1.0.89: Opt + [Full]
