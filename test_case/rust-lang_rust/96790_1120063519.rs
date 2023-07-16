plain
######################################################################## 100.0%
extracting /checkout/obj/build/cache/2022-04-05/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.gz
Building rustbuild
    Updating crates.io index
    Updating git repository `https://github.com/BusyJay/jemallocator`
    Updating git submodule `https://github.com/jemalloc/jemalloc`
---
   Compiling psm v0.1.16
   Compiling stacker v0.1.14
   Compiling rustc_llvm v0.0.0 (/checkout/compiler/rustc_llvm)
[RUSTC-TIMING] thread_local test:false 0.489
   Compiling jemalloc-sys v0.4.3+5.3.0-rc.0 (https://github.com/BusyJay/jemallocator?branch=update-to-jemalloc-5.3#8387cdb4)
[RUSTC-TIMING] build_script_build test:false 0.348
[RUSTC-TIMING] build_script_build test:false 0.396
[RUSTC-TIMING] log test:false 0.337
[RUSTC-TIMING] bitflags test:false 0.049
---
[RUSTC-TIMING] build_script_build test:false 0.417
   Compiling psm v0.1.16
   Compiling stacker v0.1.14
   Compiling rustc_llvm v0.0.0 (/checkout/compiler/rustc_llvm)
   Compiling jemalloc-sys v0.4.3+5.3.0-rc.0 (https://github.com/BusyJay/jemallocator?branch=update-to-jemalloc-5.3#8387cdb4)
   Compiling unic-emoji-char v0.9.0
[RUSTC-TIMING] build_script_build test:false 0.444
[RUSTC-TIMING] build_script_build test:false 0.524
[RUSTC-TIMING] log test:false 0.580
---
6 normal benchmarks remaining
Preparing clap-3.1.6
[2022-05-06T22:02:46Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-06T22:02:46Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-06T22:02:46Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpY07ZyT#clap:3.1.6" "--release" "--" "--skip-this-rustc"
[2022-05-06T22:02:46Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpETUctM#clap:3.1.6" "--" "--skip-this-rustc"
[2022-05-06T22:02:49Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-06T22:02:49Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-06T22:02:49Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpxgRQjo#clap:3.1.6" "--" "--wrap-rustc-with" "Eprintln"
Running clap-3.1.6: Opt + [Full]
Running clap-3.1.6: Opt + [Full]
[2022-05-06T22:02:54Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-06T22:02:54Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-06T22:02:54Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp7RsXme#clap:3.1.6" "--release" "--" "--wrap-rustc-with" "Eprintln"
5 normal benchmarks remaining
Preparing hyper-0.14.18
[2022-05-06T22:03:07Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-06T22:03:07Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-06T22:03:07Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpd5Olfu#hyper:0.14.18" "--features" "client,http1,http2,server,stream" "--" "--skip-this-rustc"
[2022-05-06T22:03:07Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpdzDEVM#hyper:0.14.18" "--release" "--features" "client,http1,http2,server,stream" "--" "--skip-this-rustc"
[2022-05-06T22:03:36Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-06T22:03:37Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-06T22:03:37Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpPvOmEP#hyper:0.14.18" "--features" "client,http1,http2,server,stream" "--" "--wrap-rustc-with" "Eprintln"
Running hyper-0.14.18: Opt + [Full]
---
[2022-05-06T22:03:52Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpHs9nlV#regex:1.5.5" "--" "--skip-this-rustc"
Running regex-1.5.5: Debug + [Full]
[2022-05-06T22:04:03Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-06T22:04:03Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-06T22:04:03Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpMCMGvT#regex:1.5.5" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-06T22:04:06Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-06T22:04:06Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-06T22:04:06Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmphIwzJH#regex:1.5.5" "--release" "--" "--wrap-rustc-with" "Eprintln"
3 normal benchmarks remaining
3 normal benchmarks remaining
Preparing ripgrep-13.0.0
[2022-05-06T22:04:20Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-06T22:04:20Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-06T22:04:20Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpGURZtc#ripgrep:13.0.0" "--" "--skip-this-rustc"
[2022-05-06T22:04:20Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpZvYi4s#ripgrep:13.0.0" "--release" "--" "--skip-this-rustc"
[2022-05-06T22:04:51Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-06T22:04:51Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-06T22:04:51Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-06T22:04:51Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpnLpZYD#ripgrep:13.0.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-06T22:04:56Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-06T22:04:57Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-06T22:04:57Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-06T22:04:57Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDZdTEH#ripgrep:13.0.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
Preparing serde-1.0.136
[2022-05-06T22:05:16Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-06T22:05:16Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-06T22:05:16Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpz23tk6#serde:1.0.136" "--" "--skip-this-rustc"
---
[2022-05-06T22:05:17Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDgSHG3#serde:1.0.136" "--" "--wrap-rustc-with" "Eprintln"
Running serde-1.0.136: Opt + [Full]
[2022-05-06T22:05:21Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-06T22:05:21Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-06T22:05:21Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpdkkNPp#serde:1.0.136" "--release" "--" "--wrap-rustc-with" "Eprintln"
Preparing syn-1.0.89
[2022-05-06T22:05:25Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-06T22:05:25Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-06T22:05:25Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKQkO08#syn:1.0.89" "--" "--skip-this-rustc"
---
[2022-05-06T22:05:29Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpACtK8Z#syn:1.0.89" "--" "--wrap-rustc-with" "Eprintln"
Running syn-1.0.89: Opt + [Full]
[2022-05-06T22:05:32Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-06T22:05:32Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-06T22:05:32Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKf4yOv#syn:1.0.89" "--release" "--" "--wrap-rustc-with" "Eprintln"
+ /rustroot/bin/llvm-profdata merge -o /tmp/llvm-pgo.profdata ./build/x86_64-unknown-linux-gnu/llvm/build/profiles
+ rm -r ./build/x86_64-unknown-linux-gnu/llvm ./build/x86_64-unknown-linux-gnu/lld
+ python3 ../x.py build --target=x86_64-unknown-linux-gnu --host=x86_64-unknown-linux-gnu --stage 2 library/std --rust-profile-generate=/tmp/rustc-pgo
warning: sqlite3 not available in python, skipping build directory lock
---
[RUSTC-TIMING] build_script_build test:false 0.311
   Compiling psm v0.1.16
   Compiling stacker v0.1.14
   Compiling rustc_llvm v0.0.0 (/checkout/compiler/rustc_llvm)
   Compiling jemalloc-sys v0.4.3+5.3.0-rc.0 (https://github.com/BusyJay/jemallocator?branch=update-to-jemalloc-5.3#8387cdb4)
[RUSTC-TIMING] build_script_build test:false 0.402
[RUSTC-TIMING] build_script_build test:false 0.445
[RUSTC-TIMING] bitflags test:false 0.062
[RUSTC-TIMING] build_script_build test:false 0.624
---
[2022-05-06T22:18:50Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-06T22:18:51Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-05-06T22:18:51Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpMoiYYZ#cargo:0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-06T22:19:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-05-06T22:19:12Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpMoiYYZ#cargo:0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpMoiYYZ/incremental-state"
[2022-05-06T22:19:37Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-05-06T22:19:37Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpMoiYYZ#cargo:0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpMoiYYZ/incremental-state"
[2022-05-06T22:19:42Z DEBUG collector::execute] applying println to "/tmp/.tmpMoiYYZ"
[2022-05-06T22:19:42Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-06T22:19:42Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-06T22:19:42Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpMoiYYZ#cargo:0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpMoiYYZ/incremental-state"
[2022-05-06T22:19:53Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-06T22:19:54Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-06T22:19:54Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp5RteT8#cargo:0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-06T22:20:44Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
---
[2022-05-06T22:25:53Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-06T22:25:53Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-06T22:25:53Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp1tGchD#ctfe-stress-4:0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-06T22:26:14Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-06T22:26:14Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp1tGchD#ctfe-stress-4:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp1tGchD/incremental-state"
[2022-05-06T22:26:38Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-05-06T22:26:38Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp1tGchD#ctfe-stress-4:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp1tGchD/incremental-state"
[2022-05-06T22:26:38Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-06T22:26:38Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-06T22:26:38Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpgrWVWo#ctfe-stress-4:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-06T22:26:58Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-06T22:26:58Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-06T22:26:58Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpgrWVWo#ctfe-stress-4:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpgrWVWo/incremental-state"
[2022-05-06T22:27:21Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-05-06T22:27:21Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpgrWVWo#ctfe-stress-4:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpgrWVWo/incremental-state"
Preparing externs
[2022-05-06T22:27:21Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-05-06T22:27:21Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-06T22:27:21Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2022-05-06T22:27:22Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-06T22:27:22Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-05-06T22:27:22Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzweqba#externs:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-06T22:27:23Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-05-06T22:27:23Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzweqba#externs:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpzweqba/incremental-state"
[2022-05-06T22:27:23Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-05-06T22:27:23Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzweqba#externs:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpzweqba/incremental-state"
[2022-05-06T22:27:24Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-06T22:27:24Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-06T22:27:24Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpyIaYl8#externs:0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-06T22:27:25Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
---
[2022-05-06T22:27:27Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-06T22:27:27Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-06T22:27:27Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpBTTziW#externs:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-06T22:27:27Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-06T22:27:27Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpBTTziW#externs:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpBTTziW/incremental-state"
[2022-05-06T22:27:28Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-05-06T22:27:28Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpBTTziW#externs:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpBTTziW/incremental-state"
Preparing match-stress
[2022-05-06T22:27:29Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-06T22:27:29Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-06T22:27:29Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
---
[2022-05-06T22:27:30Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-06T22:27:30Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-05-06T22:27:30Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpy0OM0N#match-stress:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-06T22:27:33Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-05-06T22:27:33Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpy0OM0N#match-stress:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpy0OM0N/incremental-state"
[2022-05-06T22:27:36Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-05-06T22:27:36Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpy0OM0N#match-stress:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpy0OM0N/incremental-state"
[2022-05-06T22:27:37Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-06T22:27:37Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-06T22:27:37Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmptxauiU#match-stress:0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-06T22:27:40Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-06T22:27:40Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-06T22:27:40Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmptxauiU#match-stress:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmptxauiU/incremental-state"
[2022-05-06T22:27:44Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-05-06T22:27:44Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmptxauiU#match-stress:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmptxauiU/incremental-state"
[2022-05-06T22:27:45Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-06T22:27:45Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-06T22:27:45Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpbzBGgq#match-stress:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-06T22:27:48Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-06T22:27:48Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-06T22:27:48Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpbzBGgq#match-stress:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpbzBGgq/incremental-state"
[2022-05-06T22:27:52Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-05-06T22:27:52Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpbzBGgq#match-stress:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpbzBGgq/incremental-state"
Preparing token-stream-stress
[2022-05-06T22:27:53Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-06T22:27:53Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-06T22:27:53Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
---
[2022-05-06T22:27:54Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-06T22:27:54Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-05-06T22:27:54Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpbQUvoZ#token-stream-stress:0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-06T22:27:54Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-05-06T22:27:54Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpbQUvoZ#token-stream-stress:0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpbQUvoZ/incremental-state"
[2022-05-06T22:27:54Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-05-06T22:27:54Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpbQUvoZ#token-stream-stress:0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpbQUvoZ/incremental-state"
[2022-05-06T22:27:55Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-06T22:27:55Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-06T22:27:55Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQ0QkJ5#token-stream-stress:0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-06T22:27:55Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-06T22:27:55Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-06T22:27:55Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQ0QkJ5#token-stream-stress:0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpQ0QkJ5/incremental-state"
[2022-05-06T22:27:55Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-05-06T22:27:55Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQ0QkJ5#token-stream-stress:0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpQ0QkJ5/incremental-state"
[2022-05-06T22:27:56Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-06T22:27:56Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-06T22:27:56Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjrVoxW#token-stream-stress:0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-06T22:27:56Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-06T22:27:56Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-06T22:27:56Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjrVoxW#token-stream-stress:0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpjrVoxW/incremental-state"
[2022-05-06T22:27:56Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-05-06T22:27:56Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjrVoxW#token-stream-stress:0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpjrVoxW/incremental-state"
Preparing tuple-stress
[2022-05-06T22:27:57Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-05-06T22:27:57Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-06T22:27:57Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2022-05-06T22:27:57Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-06T22:27:57Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-05-06T22:27:57Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpcmDx6E#tuple-stress:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-06T22:28:03Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-05-06T22:28:03Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpcmDx6E#tuple-stress:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpcmDx6E/incremental-state"
[2022-05-06T22:28:09Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-05-06T22:28:09Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpcmDx6E#tuple-stress:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpcmDx6E/incremental-state"
[2022-05-06T22:28:11Z DEBUG collector::execute] applying new row to "/tmp/.tmpcmDx6E"
[2022-05-06T22:28:11Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-05-06T22:28:11Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-05-06T22:28:11Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpcmDx6E#tuple-stress:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpcmDx6E/incremental-state"
[2022-05-06T22:28:17Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-06T22:28:17Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-06T22:28:17Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjKovk0#tuple-stress:0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-06T22:28:22Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-06T22:28:22Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-06T22:28:22Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjKovk0#tuple-stress:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpjKovk0/incremental-state"
[2022-05-06T22:28:29Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-05-06T22:28:29Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjKovk0#tuple-stress:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpjKovk0/incremental-state"
[2022-05-06T22:28:31Z DEBUG collector::execute] applying new row to "/tmp/.tmpjKovk0"
[2022-05-06T22:28:31Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-05-06T22:28:31Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-05-06T22:28:31Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjKovk0#tuple-stress:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpjKovk0/incremental-state"
[2022-05-06T22:28:37Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-06T22:28:37Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-06T22:28:37Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCoNHFD#tuple-stress:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-06T22:28:43Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-06T22:28:43Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-06T22:28:43Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCoNHFD#tuple-stress:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpCoNHFD/incremental-state"
[2022-05-06T22:28:49Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-05-06T22:28:49Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCoNHFD#tuple-stress:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpCoNHFD/incremental-state"
[2022-05-06T22:28:51Z DEBUG collector::execute] applying new row to "/tmp/.tmpCoNHFD"
[2022-05-06T22:28:51Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-05-06T22:28:51Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-05-06T22:28:51Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCoNHFD#tuple-stress:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpCoNHFD/incremental-state"
+ ./build/x86_64-unknown-linux-gnu/llvm/bin/llvm-profdata merge -o /tmp/rustc-pgo.profdata /tmp/rustc-pgo
+ rm -r ./build/x86_64-unknown-linux-gnu/llvm ./build/x86_64-unknown-linux-gnu/lld
+ python3 ../x.py dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu --include-default-paths build-manifest --rust-profile-use=/tmp/rustc-pgo.profdata --llvm-profile-use=/tmp/llvm-pgo.profdata
warning: sqlite3 not available in python, skipping build directory lock
---
   Compiling serde v1.0.125
   Compiling datafrog v2.0.1
[RUSTC-TIMING] scoped_tls test:false 0.088
   Compiling memchr v2.4.1
warning: rustc_graphviz.3e36d16d-cgu.1: no profile data available for function _RNvXs2_NtCsivMPc7fkJD3_4core7convertNtNtNtB7_5alloc6layout11LayoutErrorINtB5_4FromBy_E4fromCshYqNWwWVohl_14rustc_graphviz Hash = 742261418966908927
[RUSTC-TIMING] unicode_xid test:false 0.093
   Compiling tinyvec v0.3.4
[RUSTC-TIMING] self_cell test:false 0.088
   Compiling regex-syntax v0.6.25
   Compiling regex-syntax v0.6.25
warning: rustc_fs_util.b796ec74-cgu.3: no profile data available for function _RNvXs2_NtCsivMPc7fkJD3_4core7convertNtNtNtB7_5alloc6layout11LayoutErrorINtB5_4FromBy_E4fromCs2pWch2UMHZ0_13rustc_fs_util Hash = 742261418966908927
[RUSTC-TIMING] build_script_build test:false 0.429
   Compiling ansi_term v0.12.1
   Compiling ansi_term v0.12.1
warning: rustc_graphviz.3e36d16d-cgu.10: no profile data available for function _RNvMNtNtNtCsivMPc7fkJD3_4core4iter8adapters3revINtB2_3RevINtNtNtB8_3ops5range5RangejEE3newCshYqNWwWVohl_14rustc_graphviz Hash = 742261418966908927
[RUSTC-TIMING] ppv_lite86 test:false 0.469
   Compiling crc32fast v1.2.0
[RUSTC-TIMING] rustc_fs_util test:false 0.126
warning: `rustc_fs_util` (lib) generated 1 warning
---
   Compiling psm v0.1.16
   Compiling stacker v0.1.14
   Compiling rustc_llvm v0.0.0 (/checkout/compiler/rustc_llvm)
[RUSTC-TIMING] build_script_build test:false 0.296
   Compiling jemalloc-sys v0.4.3+5.3.0-rc.0 (https://github.com/BusyJay/jemallocator?branch=update-to-jemalloc-5.3#8387cdb4)
[RUSTC-TIMING] log test:false 0.360
[RUSTC-TIMING] build_script_build test:false 0.381
[RUSTC-TIMING] bitflags test:false 0.044
   Compiling unic-emoji-char v0.9.0
---
   Compiling rand v0.8.5
[RUSTC-TIMING] generic_array test:false 0.878
[RUSTC-TIMING] jobserver test:false 1.271
   Compiling crypto-common v0.1.2
warning: rustc_llvm.f5c36890-cgu.1: no profile data available for function _RNvXs2_NtCsivMPc7fkJD3_4core7convertNtNtNtB7_5alloc6layout11LayoutErrorINtB5_4FromBy_E4fromCs1LzBiHkV1Tw_10rustc_llvm Hash = 742261418966908927
[RUSTC-TIMING] rustc_parse_format test:false 1.026
   Compiling block-buffer v0.10.2
[RUSTC-TIMING] crypto_common test:false 0.090
[RUSTC-TIMING] rustc_llvm test:false 0.237
---
   Compiling rustc_ast v0.0.0 (/checkout/compiler/rustc_ast)
   Compiling rustc_error_messages v0.0.0 (/checkout/compiler/rustc_error_messages)
   Compiling rustc_feature v0.0.0 (/checkout/compiler/rustc_feature)
   Compiling rustc_target v0.0.0 (/checkout/compiler/rustc_target)
warning: rustc_error_messages.0ee3353c-cgu.11: no profile data available for function _RINvMsx_NtCsi2mL3jT7WVK_12tracing_core5fieldNtB6_8FieldSet9value_setATRNtB6_5FieldINtNtCsivMPc7fkJD3_4core6option6OptionRDNtB6_5ValueEL_EEj1_ECseT80zgfjLCO_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.0ee3353c-cgu.11: no profile data available for function _RINvMsx_NtCsi2mL3jT7WVK_12tracing_core5fieldNtB6_8FieldSet9value_setATRNtB6_5FieldINtNtCsivMPc7fkJD3_4core6option6OptionRDNtB6_5ValueEL_EEj2_ECseT80zgfjLCO_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.0ee3353c-cgu.11: no profile data available for function _RINvMsx_NtCsi2mL3jT7WVK_12tracing_core5fieldNtB6_8FieldSet9value_setATRNtB6_5FieldINtNtCsivMPc7fkJD3_4core6option6OptionRDNtB6_5ValueEL_EEj5_ECseT80zgfjLCO_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.0ee3353c-cgu.11: no profile data available for function _RINvNtCsi2mL3jT7WVK_12tracing_core5field5debugRINtNtCs9HAhSfvCYaR_5alloc3vec3VecNtNtCs9yGpLNr54nK_3std4path7PathBufEECseT80zgfjLCO_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.0ee3353c-cgu.11: no profile data available for function _RINvNtCsi2mL3jT7WVK_12tracing_core5field5debugRINtNtCsivMPc7fkJD3_4core6option6OptionNtCsfaXwht5LfIs_16unic_langid_impl18LanguageIdentifierEECseT80zgfjLCO_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.0ee3353c-cgu.11: no profile data available for function _RINvNtCsi2mL3jT7WVK_12tracing_core5field5debugRINtNtCsivMPc7fkJD3_4core6option6OptionNtNtCs9yGpLNr54nK_3std4path7PathBufEECseT80zgfjLCO_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.0ee3353c-cgu.11: no profile data available for function _RINvNtCsi2mL3jT7WVK_12tracing_core5field5debugRINtNtCsivMPc7fkJD3_4core6option6OptionRNtNtCs9yGpLNr54nK_3std4path4PathEECseT80zgfjLCO_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.0ee3353c-cgu.11: no profile data available for function _RINvNtCsi2mL3jT7WVK_12tracing_core5field5debugRNtCsfaXwht5LfIs_16unic_langid_impl18LanguageIdentifierECseT80zgfjLCO_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.0ee3353c-cgu.11: no profile data available for function _RINvNtCsi2mL3jT7WVK_12tracing_core5field5debugRNtNtCs2B6tHj1KwRV_13fluent_bundle8resource14FluentResourceECseT80zgfjLCO_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.0ee3353c-cgu.11: no profile data available for function _RINvNtCsi2mL3jT7WVK_12tracing_core5field5debugRNtNtCs9yGpLNr54nK_3std4path7PathBufECseT80zgfjLCO_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.0ee3353c-cgu.11: no profile data available for function _RINvNtCsi2mL3jT7WVK_12tracing_core5field5debugRQNtNtCs9yGpLNr54nK_3std4path7PathBufECseT80zgfjLCO_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.0ee3353c-cgu.11: no profile data available for function _RINvNtCsi2mL3jT7WVK_12tracing_core5field5debugRRSReECseT80zgfjLCO_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.0ee3353c-cgu.11: no profile data available for function _RINvNtCsivMPc7fkJD3_4core3ptr13drop_in_placeRINtNtB4_6option6OptionNtCsfaXwht5LfIs_16unic_langid_impl18LanguageIdentifierEECseT80zgfjLCO_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.0ee3353c-cgu.11: no profile data available for function _RINvNtCsivMPc7fkJD3_4core3ptr13drop_in_placeRINtNtB4_6option6OptionNtNtCs9yGpLNr54nK_3std4path7PathBufEECseT80zgfjLCO_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.0ee3353c-cgu.11: no profile data available for function _RINvNtCsivMPc7fkJD3_4core3ptr13drop_in_placeRINtNtB4_6option6OptionRNtNtCs9yGpLNr54nK_3std4path4PathEECseT80zgfjLCO_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.0ee3353c-cgu.11: no profile data available for function _RINvNtCsivMPc7fkJD3_4core3ptr13drop_in_placeRINtNtCs9HAhSfvCYaR_5alloc3vec3VecNtNtCs9yGpLNr54nK_3std4path7PathBufEECseT80zgfjLCO_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.0ee3353c-cgu.11: no profile data available for function _RINvNtCsivMPc7fkJD3_4core3ptr13drop_in_placeRNtCsfaXwht5LfIs_16unic_langid_impl18LanguageIdentifierECseT80zgfjLCO_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.0ee3353c-cgu.11: no profile data available for function _RINvNtCsivMPc7fkJD3_4core3ptr13drop_in_placeRNtNtCs2B6tHj1KwRV_13fluent_bundle8resource14FluentResourceECseT80zgfjLCO_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.0ee3353c-cgu.11: no profile data available for function _RINvNtCsivMPc7fkJD3_4core3ptr13drop_in_placeRNtNtCs9yGpLNr54nK_3std4path7PathBufECseT80zgfjLCO_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.0ee3353c-cgu.11: no profile data available for function _RINvNtCsivMPc7fkJD3_4core3ptr13drop_in_placeRQNtNtCs9yGpLNr54nK_3std4path7PathBufECseT80zgfjLCO_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.0ee3353c-cgu.11: no profile data available for function _RINvNtCsivMPc7fkJD3_4core3ptr13drop_in_placeRRSReECseT80zgfjLCO_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.0ee3353c-cgu.11: no profile data available for function _RNvXsk_NtCsi2mL3jT7WVK_12tracing_core5fieldINtB5_10DebugValueRINtNtCs9HAhSfvCYaR_5alloc3vec3VecNtNtCs9yGpLNr54nK_3std4path7PathBufEENtB5_5Value6recordCseT80zgfjLCO_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.0ee3353c-cgu.11: no profile data available for function _RNvXsk_NtCsi2mL3jT7WVK_12tracing_core5fieldINtB5_10DebugValueRINtNtCsivMPc7fkJD3_4core6option6OptionNtCsfaXwht5LfIs_16unic_langid_impl18LanguageIdentifierEENtB5_5Value6recordCseT80zgfjLCO_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.0ee3353c-cgu.11: no profile data available for function _RNvXsk_NtCsi2mL3jT7WVK_12tracing_core5fieldINtB5_10DebugValueRINtNtCsivMPc7fkJD3_4core6option6OptionNtNtCs9yGpLNr54nK_3std4path7PathBufEENtB5_5Value6recordCseT80zgfjLCO_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.0ee3353c-cgu.11: no profile data available for function _RNvXsk_NtCsi2mL3jT7WVK_12tracing_core5fieldINtB5_10DebugValueRINtNtCsivMPc7fkJD3_4core6option6OptionRNtNtCs9yGpLNr54nK_3std4path4PathEENtB5_5Value6recordCseT80zgfjLCO_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.0ee3353c-cgu.11: no profile data available for function _RNvXsk_NtCsi2mL3jT7WVK_12tracing_core5fieldINtB5_10DebugValueRNtCsfaXwht5LfIs_16unic_langid_impl18LanguageIdentifierENtB5_5Value6recordCseT80zgfjLCO_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.0ee3353c-cgu.11: no profile data available for function _RNvXsk_NtCsi2mL3jT7WVK_12tracing_core5fieldINtB5_10DebugValueRNtNtCs2B6tHj1KwRV_13fluent_bundle8resource14FluentResourceENtB5_5Value6recordCseT80zgfjLCO_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.0ee3353c-cgu.11: no profile data available for function _RNvXsk_NtCsi2mL3jT7WVK_12tracing_core5fieldINtB5_10DebugValueRNtNtCs9yGpLNr54nK_3std4path7PathBufENtB5_5Value6recordCseT80zgfjLCO_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.0ee3353c-cgu.11: no profile data available for function _RNvXsk_NtCsi2mL3jT7WVK_12tracing_core5fieldINtB5_10DebugValueRQNtNtCs9yGpLNr54nK_3std4path7PathBufENtB5_5Value6recordCseT80zgfjLCO_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.0ee3353c-cgu.11: no profile data available for function _RNvXsk_NtCsi2mL3jT7WVK_12tracing_core5fieldINtB5_10DebugValueRRSReENtB5_5Value6recordCseT80zgfjLCO_20rustc_error_messages Hash = 170957022131388415
[RUSTC-TIMING] chalk_engine test:false 1.514
[RUSTC-TIMING] rustc_error_messages test:false 1.279
warning: `rustc_error_messages` (lib) generated 30 warnings
[RUSTC-TIMING] rustc_feature test:false 1.508
