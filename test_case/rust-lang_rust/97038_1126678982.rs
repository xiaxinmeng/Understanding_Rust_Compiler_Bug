plain
[2022-05-14T08:51:52Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp5wrwfm#clap:3.1.6" "--release" "--" "--skip-this-rustc"
Running clap-3.1.6: Debug + [Full]
[2022-05-14T08:51:55Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-14T08:51:55Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-14T08:51:55Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpLbypZT#clap:3.1.6" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-14T08:52:01Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-14T08:52:01Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-14T08:52:01Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp51fUnx#clap:3.1.6" "--release" "--" "--wrap-rustc-with" "Eprintln"
5 normal benchmarks remaining
---
4 normal benchmarks remaining
Preparing regex-1.5.5
[2022-05-14T08:53:19Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-14T08:53:19Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-14T08:53:19Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp3rGODm#regex:1.5.5" "--release" "--" "--skip-this-rustc"
[2022-05-14T08:53:19Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKdfs2g#regex:1.5.5" "--" "--skip-this-rustc"
[2022-05-14T08:53:34Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-14T08:53:34Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-14T08:53:34Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp5DFp4o#regex:1.5.5" "--" "--wrap-rustc-with" "Eprintln"
Running regex-1.5.5: Opt + [Full]
Running regex-1.5.5: Opt + [Full]
[2022-05-14T08:53:38Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-14T08:53:38Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-14T08:53:38Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpmyeiEY#regex:1.5.5" "--release" "--" "--wrap-rustc-with" "Eprintln"
Preparing ripgrep-13.0.0
[2022-05-14T08:53:59Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-14T08:53:59Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-14T08:53:59Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-14T08:53:59Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzzRGeJ#ripgrep:13.0.0" "--" "--skip-this-rustc"
[2022-05-14T08:53:59Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpyFsbHn#ripgrep:13.0.0" "--release" "--" "--skip-this-rustc"
[2022-05-14T08:54:35Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-14T08:54:36Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-14T08:54:36Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpAHB5P0#ripgrep:13.0.0" "--" "--wrap-rustc-with" "Eprintln"
Running ripgrep-13.0.0: Opt + [Full]
---
[2022-05-14T08:55:17Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpS1Ste0#serde:1.0.136" "--" "--wrap-rustc-with" "Eprintln"
Running serde-1.0.136: Opt + [Full]
[2022-05-14T08:55:22Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-14T08:55:22Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-14T08:55:22Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCYgeVT#serde:1.0.136" "--release" "--" "--wrap-rustc-with" "Eprintln"
Preparing syn-1.0.89
[2022-05-14T08:55:27Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-14T08:55:27Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-14T08:55:27Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpK1St62#syn:1.0.89" "--release" "--" "--skip-this-rustc"
---
[2022-05-14T08:55:32Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpSt0J5N#syn:1.0.89" "--" "--wrap-rustc-with" "Eprintln"
Running syn-1.0.89: Opt + [Full]
[2022-05-14T08:55:36Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-14T08:55:36Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-14T08:55:36Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpGlBfdu#syn:1.0.89" "--release" "--" "--wrap-rustc-with" "Eprintln"
+ /rustroot/bin/llvm-profdata merge -o /tmp/llvm-pgo.profdata ./build/x86_64-unknown-linux-gnu/llvm/build/profiles
+ rm -r ./build/x86_64-unknown-linux-gnu/llvm ./build/x86_64-unknown-linux-gnu/lld
+ python3 ../x.py build --target=x86_64-unknown-linux-gnu --host=x86_64-unknown-linux-gnu --stage 2 library/std --rust-profile-generate=/tmp/rustc-pgo --libstd-profile-generate=/tmp/libstd-pgo
warning: sqlite3 not available in python, skipping build directory lock
---
   Compiling profiler_builtins v0.0.0 (/checkout/library/profiler_builtins)
[RUSTC-TIMING] build_script_build test:false 0.335
[RUSTC-TIMING] build_script_build test:false 0.363
[RUSTC-TIMING] build_script_build test:false 0.613
error: `profiler_builtins` crate (required by compiler options) is not compatible with crate attribute `#![no_core]`
error[E0463]: can't find crate for `profiler_builtins`
  |
  = note: the compiler may have been built without the profiler runtime

