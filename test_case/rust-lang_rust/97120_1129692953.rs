plain
[2022-05-18T07:21:40Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpoErs3L#clap:3.1.6" "--release" "--" "--skip-this-rustc"
Running clap-3.1.6: Debug + [Full]
[2022-05-18T07:21:43Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-18T07:21:43Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-18T07:21:43Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQKGqvN#clap:3.1.6" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-18T07:21:48Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-18T07:21:48Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-18T07:21:48Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJFP0Up#clap:3.1.6" "--release" "--" "--wrap-rustc-with" "Eprintln"
5 normal benchmarks remaining
5 normal benchmarks remaining
Preparing hyper-0.14.18
[2022-05-18T07:22:03Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-18T07:22:03Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-18T07:22:03Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOmOxSb#hyper:0.14.18" "--release" "--features" "client,http1,http2,server,stream" "--" "--skip-this-rustc"
[2022-05-18T07:22:03Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpmnuoiI#hyper:0.14.18" "--features" "client,http1,http2,server,stream" "--" "--skip-this-rustc"
[2022-05-18T07:22:37Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-18T07:22:37Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-18T07:22:37Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp53O56R#hyper:0.14.18" "--features" "client,http1,http2,server,stream" "--" "--wrap-rustc-with" "Eprintln"
Running hyper-0.14.18: Opt + [Full]
Running hyper-0.14.18: Opt + [Full]
[2022-05-18T07:22:42Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-18T07:22:42Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-18T07:22:42Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpRvhlvB#hyper:0.14.18" "--release" "--features" "client,http1,http2,server,stream" "--" "--wrap-rustc-with" "Eprintln"
4 normal benchmarks remaining
Preparing regex-1.5.5
[2022-05-18T07:22:55Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-18T07:22:55Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-18T07:22:55Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpapSKlE#regex:1.5.5" "--release" "--" "--skip-this-rustc"
[2022-05-18T07:22:55Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpMS75ZA#regex:1.5.5" "--" "--skip-this-rustc"
[2022-05-18T07:23:07Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-18T07:23:07Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-18T07:23:07Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp4RKdRa#regex:1.5.5" "--" "--wrap-rustc-with" "Eprintln"
Running regex-1.5.5: Opt + [Full]
Running regex-1.5.5: Opt + [Full]
[2022-05-18T07:23:11Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-18T07:23:11Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-18T07:23:11Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpR75uWy#regex:1.5.5" "--release" "--" "--wrap-rustc-with" "Eprintln"
3 normal benchmarks remaining
Preparing ripgrep-13.0.0
[2022-05-18T07:23:27Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-18T07:23:27Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-18T07:23:27Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpGrvY2c#ripgrep:13.0.0" "--release" "--" "--skip-this-rustc"
[2022-05-18T07:23:27Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpTatRGd#ripgrep:13.0.0" "--" "--skip-this-rustc"
[2022-05-18T07:24:03Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-18T07:24:04Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-18T07:24:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOAvpVS#ripgrep:13.0.0" "--" "--wrap-rustc-with" "Eprintln"
Running ripgrep-13.0.0: Opt + [Full]
Running ripgrep-13.0.0: Opt + [Full]
[2022-05-18T07:24:10Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-18T07:24:10Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-18T07:24:10Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp8iCszk#ripgrep:13.0.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
2 normal benchmarks remaining
Preparing serde-1.0.136
[2022-05-18T07:24:33Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-18T07:24:33Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-18T07:24:33Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwuPtOU#serde:1.0.136" "--release" "--" "--skip-this-rustc"
[2022-05-18T07:24:33Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpIMtNBw#serde:1.0.136" "--" "--skip-this-rustc"
[2022-05-18T07:24:34Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-18T07:24:34Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-18T07:24:34Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp9EzZCR#serde:1.0.136" "--" "--wrap-rustc-with" "Eprintln"
Running serde-1.0.136: Opt + [Full]
Running serde-1.0.136: Opt + [Full]
[2022-05-18T07:24:38Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-18T07:24:38Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-18T07:24:38Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpqKJnp4#serde:1.0.136" "--release" "--" "--wrap-rustc-with" "Eprintln"
1 normal benchmark remaining
Preparing syn-1.0.89
[2022-05-18T07:24:43Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-18T07:24:43Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-18T07:24:43Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDdNBx8#syn:1.0.89" "--" "--skip-this-rustc"
[2022-05-18T07:24:43Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpVXptHR#syn:1.0.89" "--release" "--" "--skip-this-rustc"
[2022-05-18T07:24:47Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-18T07:24:47Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-18T07:24:47Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpIqcKIq#syn:1.0.89" "--" "--wrap-rustc-with" "Eprintln"
Running syn-1.0.89: Opt + [Full]
Running syn-1.0.89: Opt + [Full]
[2022-05-18T07:24:51Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-18T07:24:51Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-18T07:24:51Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKWmtJS#syn:1.0.89" "--release" "--" "--wrap-rustc-with" "Eprintln"
+ /rustroot/bin/llvm-profdata merge -o /tmp/llvm-pgo.profdata ./build/x86_64-unknown-linux-gnu/llvm/build/profiles
+ rm -r ./build/x86_64-unknown-linux-gnu/llvm ./build/x86_64-unknown-linux-gnu/lld
+ python3 ../x.py build --target=x86_64-unknown-linux-gnu --host=x86_64-unknown-linux-gnu --stage 2 library/std --rust-profile-generate=/tmp/rustc-pgo
warning: sqlite3 not available in python, skipping build directory lock
---
Copying stage2 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[TIMING] compile::StdLink { compiler: Compiler { stage: 1, host: x86_64-unknown-linux-gnu }, target_compiler: Compiler { stage: 2, host: x86_64-unknown-linux-gnu }, target: x86_64-unknown-linux-gnu } -- 0.001
[TIMING] compile::Std { target: x86_64-unknown-linux-gnu, compiler: Compiler { stage: 2, host: x86_64-unknown-linux-gnu } } -- 0.000
Build completed successfully in 0:12:21
+ gather_profiles Check,Debug,Opt All externs,ctfe-stress-4,cargo-0.60.0,token-stream-stress,match-stress,tuple-stress,diesel-1.4.8,bitmaps-3.1.0
+ RUSTC_BOOTSTRAP=1
+ ./build/x86_64-unknown-linux-gnu/stage2/bin/rustc --edition=2021 --crate-type=lib ../library/core/src/lib.rs
+ RUSTC_BOOTSTRAP=1
+ ./build/x86_64-unknown-linux-gnu/stage2/bin/rustc --edition=2021 --crate-type=lib -Copt-level=3 ../library/core/src/lib.rs
+ ./build/x86_64-unknown-linux-gnu/stage2/bin/rustc --edition=2021 --crate-type=lib -Copt-level=3 ../library/core/src/lib.rs
+ cd rustc-perf
+ RUST_LOG=collector=debug
+ RUSTC=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc
+ RUSTC_BOOTSTRAP=1
+ /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo run -p collector --bin collector -- profile_local eprintln /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc --id Test --profiles Check,Debug,Opt --cargo /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo --scenarios All --include externs,ctfe-stress-4,cargo-0.60.0,token-stream-stress,match-stress,tuple-stress,diesel-1.4.8,bitmaps-3.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 0.15s
     Running `target/debug/collector profile_local eprintln /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc --id Test --profiles Check,Debug,Opt --cargo /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo --scenarios All --include externs,ctfe-stress-4,cargo-0.60.0,token-stream-stress,match-stress,tuple-stress,diesel-1.4.8,bitmaps-3.1.0`
[2022-05-18T07:38:50Z DEBUG collector] benchmark README.md - ignored
[2022-05-18T07:38:50Z DEBUG collector] benchmark html5ever-0.26.0 - not named by --include argument, skipping
[2022-05-18T07:38:50Z DEBUG collector] benchmark piston-image - not named by --include argument, skipping
[2022-05-18T07:38:50Z DEBUG collector] benchmark many-assoc-items - not named by --include argument, skipping
---
[2022-05-18T07:38:50Z DEBUG collector] benchmark tokio-webpush-simple - not named by --include argument, skipping
[2022-05-18T07:38:50Z DEBUG collector] benchmark regex-1.5.5 - not named by --include argument, skipping
[2022-05-18T07:38:50Z DEBUG collector] benchmark serde-1.0.136 - not named by --include argument, skipping
[2022-05-18T07:38:50Z DEBUG collector] benchmark `cargo-0.60.0`- registered
collector error: Warning: one or more invalid --include entries: {"bitmaps-3.1.0"}
