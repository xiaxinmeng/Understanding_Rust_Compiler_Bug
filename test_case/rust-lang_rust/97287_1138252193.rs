plain
[2022-05-26T06:42:35Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp1y23DU#hyper:0.14.18" "--features=client,http1,http2,server,stream" "--" "--wrap-rustc-with" "Eprintln"
Running hyper-0.14.18: Opt + [Full]
[2022-05-26T06:42:40Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T06:42:40Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-26T06:42:40Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpZPxxVn#hyper:0.14.18" "--release" "--features=client,http1,http2,server,stream" "--" "--wrap-rustc-with" "Eprintln"
Executing benchmark regex-1.5.5 (4/7)
Preparing regex-1.5.5
[2022-05-26T06:42:50Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-26T06:42:50Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2022-05-26T06:43:01Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmppISKCI#regex:1.5.5" "--" "--wrap-rustc-with" "Eprintln"
Running regex-1.5.5: Opt + [Full]
[2022-05-26T06:43:04Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T06:43:04Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-26T06:43:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpefTlGx#regex:1.5.5" "--release" "--" "--wrap-rustc-with" "Eprintln"
Executing benchmark ripgrep-13.0.0 (5/7)
Preparing ripgrep-13.0.0
[2022-05-26T06:43:18Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-26T06:43:18Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
---
Preparing bitmaps-3.1.0
[2022-05-26T06:56:54Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-05-26T06:56:54Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-26T06:56:54Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-26T06:56:54Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpEpEZag#bitmaps:3.1.0" "--release" "--" "--skip-this-rustc"
[2022-05-26T06:56:54Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpahGb0H#bitmaps:3.1.0" "--" "--skip-this-rustc"
[2022-05-26T06:56:54Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpn9Wio1#bitmaps:3.1.0" "--profile" "check" "--" "--skip-this-rustc"
[2022-05-26T06:56:55Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T06:56:55Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-05-26T06:56:55Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-05-26T06:56:55Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzvsDAk#bitmaps:3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T06:56:56Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-05-26T06:56:57Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzvsDAk#bitmaps:3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpzvsDAk/incremental-state"
[2022-05-26T06:57:00Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-05-26T06:57:00Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzvsDAk#bitmaps:3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpzvsDAk/incremental-state"
[2022-05-26T06:57:00Z DEBUG collector::execute] applying println to "/tmp/.tmpzvsDAk"
[2022-05-26T06:57:00Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-26T06:57:00Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-26T06:57:00Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzvsDAk#bitmaps:3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpzvsDAk/incremental-state"
[2022-05-26T06:57:01Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T06:57:01Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-26T06:57:01Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpES77bn#bitmaps:3.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T06:57:03Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-26T06:57:03Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-26T06:57:03Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpES77bn#bitmaps:3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpES77bn/incremental-state"
[2022-05-26T06:57:06Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-05-26T06:57:06Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpES77bn#bitmaps:3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpES77bn/incremental-state"
[2022-05-26T06:57:07Z DEBUG collector::execute] applying println to "/tmp/.tmpES77bn"
[2022-05-26T06:57:07Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-26T06:57:07Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-26T06:57:07Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpES77bn#bitmaps:3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpES77bn/incremental-state"
[2022-05-26T06:57:08Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T06:57:08Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-26T06:57:08Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpGRHoeE#bitmaps:3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T06:57:11Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-26T06:57:11Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-26T06:57:11Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpGRHoeE#bitmaps:3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpGRHoeE/incremental-state"
[2022-05-26T06:57:14Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-05-26T06:57:14Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpGRHoeE#bitmaps:3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpGRHoeE/incremental-state"
[2022-05-26T06:57:14Z DEBUG collector::execute] applying println to "/tmp/.tmpGRHoeE"
[2022-05-26T06:57:14Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-26T06:57:14Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-26T06:57:15Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpGRHoeE#bitmaps:3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpGRHoeE/incremental-state"
Executing benchmark cargo-0.60.0 (2/8)
Preparing cargo-0.60.0
[2022-05-26T06:57:16Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-05-26T06:57:16Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2022-05-26T06:58:11Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T06:58:12Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-05-26T06:58:12Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJRavKR#cargo:0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T06:58:32Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-05-26T06:58:32Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJRavKR#cargo:0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpJRavKR/incremental-state"
[2022-05-26T06:58:58Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-05-26T06:58:58Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJRavKR#cargo:0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpJRavKR/incremental-state"
[2022-05-26T06:59:02Z DEBUG collector::execute] applying println to "/tmp/.tmpJRavKR"
[2022-05-26T06:59:02Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-26T06:59:02Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-26T06:59:02Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJRavKR#cargo:0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpJRavKR/incremental-state"
[2022-05-26T06:59:13Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T06:59:14Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-26T06:59:14Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpUvMef5#cargo:0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T07:00:02Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-26T07:00:02Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-26T07:00:03Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpUvMef5#cargo:0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpUvMef5/incremental-state"
[2022-05-26T07:01:02Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-05-26T07:01:02Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpUvMef5#cargo:0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpUvMef5/incremental-state"
[2022-05-26T07:01:15Z DEBUG collector::execute] applying println to "/tmp/.tmpUvMef5"
[2022-05-26T07:01:15Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-26T07:01:15Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-26T07:01:15Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpUvMef5#cargo:0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpUvMef5/incremental-state"
[2022-05-26T07:01:40Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T07:01:41Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-26T07:01:41Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpoyuSow#cargo:0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T07:02:48Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-26T07:02:48Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-26T07:02:48Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpoyuSow#cargo:0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpoyuSow/incremental-state"
[2022-05-26T07:03:51Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-05-26T07:03:51Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpoyuSow#cargo:0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpoyuSow/incremental-state"
[2022-05-26T07:04:03Z DEBUG collector::execute] applying println to "/tmp/.tmpoyuSow"
[2022-05-26T07:04:03Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-26T07:04:03Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-26T07:04:03Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpoyuSow#cargo:0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpoyuSow/incremental-state"
Executing benchmark ctfe-stress-5 (3/8)
Preparing ctfe-stress-5
[2022-05-26T07:04:33Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-26T07:04:33Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2022-05-26T07:04:34Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T07:04:34Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-05-26T07:04:34Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwa7bxi#ctfe-stress-5:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T07:04:42Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-05-26T07:04:42Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwa7bxi#ctfe-stress-5:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpwa7bxi/incremental-state"
[2022-05-26T07:04:52Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-05-26T07:04:52Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwa7bxi#ctfe-stress-5:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpwa7bxi/incremental-state"
[2022-05-26T07:04:53Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T07:04:53Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-26T07:04:53Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpxep2wM#ctfe-stress-5:0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T07:05:02Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-26T07:05:02Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-26T07:05:02Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpxep2wM#ctfe-stress-5:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpxep2wM/incremental-state"
[2022-05-26T07:05:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-05-26T07:05:12Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpxep2wM#ctfe-stress-5:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpxep2wM/incremental-state"
[2022-05-26T07:05:13Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T07:05:13Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-26T07:05:13Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpqfwERp#ctfe-stress-5:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T07:05:21Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-26T07:05:21Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-26T07:05:21Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpqfwERp#ctfe-stress-5:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpqfwERp/incremental-state"
[2022-05-26T07:05:30Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-05-26T07:05:30Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpqfwERp#ctfe-stress-5:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpqfwERp/incremental-state"
Executing benchmark diesel-1.4.8 (4/8)
Preparing diesel-1.4.8
[2022-05-26T07:05:30Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-05-26T07:05:30Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-26T07:05:30Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-26T07:05:30Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-26T07:05:30Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpSWg4c2#diesel:1.4.8" "--release" "--" "--skip-this-rustc"
[2022-05-26T07:05:30Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpHgKAhB#diesel:1.4.8" "--profile" "check" "--" "--skip-this-rustc"
[2022-05-26T07:05:30Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpAcbRAa#diesel:1.4.8" "--" "--skip-this-rustc"
[2022-05-26T07:05:50Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T07:05:50Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-05-26T07:05:50Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp76Z5wX#diesel:1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T07:06:06Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
---
[2022-05-26T07:06:32Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T07:06:32Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-26T07:06:32Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpvJ4jRq#diesel:1.4.8" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T07:06:53Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-26T07:06:53Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpvJ4jRq#diesel:1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpvJ4jRq/incremental-state"
[2022-05-26T07:07:15Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-05-26T07:07:15Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpvJ4jRq#diesel:1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpvJ4jRq/incremental-state"
[2022-05-26T07:07:19Z DEBUG collector::execute] applying println to "/tmp/.tmpvJ4jRq"
[2022-05-26T07:07:19Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-26T07:07:19Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-26T07:07:19Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpvJ4jRq#diesel:1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpvJ4jRq/incremental-state"
[2022-05-26T07:07:23Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T07:07:23Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-26T07:07:23Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-26T07:07:23Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpImJOID#diesel:1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T07:07:42Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-26T07:07:42Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpImJOID#diesel:1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpImJOID/incremental-state"
[2022-05-26T07:08:03Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-05-26T07:08:03Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpImJOID#diesel:1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpImJOID/incremental-state"
[2022-05-26T07:08:07Z DEBUG collector::execute] applying println to "/tmp/.tmpImJOID"
[2022-05-26T07:08:07Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-26T07:08:07Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-26T07:08:07Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpImJOID#diesel:1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpImJOID/incremental-state"
Executing benchmark externs (5/8)
Preparing externs
[2022-05-26T07:08:11Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-26T07:08:11Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
---
[2022-05-26T07:08:27Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T07:08:27Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-26T07:08:27Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpF7z8rq#match-stress:0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T07:08:30Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-26T07:08:30Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpF7z8rq#match-stress:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpF7z8rq/incremental-state"
[2022-05-26T07:08:34Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-05-26T07:08:34Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpF7z8rq#match-stress:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpF7z8rq/incremental-state"
[2022-05-26T07:08:35Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T07:08:35Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-26T07:08:35Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpc6OIdu#match-stress:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T07:08:39Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-26T07:08:39Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-26T07:08:39Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpc6OIdu#match-stress:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpc6OIdu/incremental-state"
[2022-05-26T07:08:42Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-05-26T07:08:42Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpc6OIdu#match-stress:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpc6OIdu/incremental-state"
Executing benchmark token-stream-stress (7/8)
Preparing token-stream-stress
[2022-05-26T07:08:44Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-05-26T07:08:44Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-26T07:08:44Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-26T07:08:44Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-26T07:08:44Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpRMHbAJ#token-stream-stress:0.0.0" "--bin" "token-stream-stress-bin" "--" "--skip-this-rustc"
[2022-05-26T07:08:44Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpW6bIxt#token-stream-stress:0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--skip-this-rustc"
[2022-05-26T07:08:44Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp9iPsFW#token-stream-stress:0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--skip-this-rustc"
[2022-05-26T07:08:45Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T07:08:45Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-05-26T07:08:45Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpI8ezfr#token-stream-stress:0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T07:08:45Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-05-26T07:08:45Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-05-26T07:08:45Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpI8ezfr#token-stream-stress:0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpI8ezfr/incremental-state"
[2022-05-26T07:08:45Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-05-26T07:08:45Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpI8ezfr#token-stream-stress:0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpI8ezfr/incremental-state"
[2022-05-26T07:08:45Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T07:08:45Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-26T07:08:45Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQPtpUM#token-stream-stress:0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T07:08:46Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-26T07:08:46Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-26T07:08:46Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQPtpUM#token-stream-stress:0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpQPtpUM/incremental-state"
[2022-05-26T07:08:46Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-05-26T07:08:46Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQPtpUM#token-stream-stress:0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpQPtpUM/incremental-state"
[2022-05-26T07:08:46Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T07:08:46Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-26T07:08:46Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpF9g2Bv#token-stream-stress:0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T07:08:47Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
[2022-05-26T07:08:48Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T07:08:48Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-05-26T07:08:48Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpawqH1w#tuple-stress:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T07:08:53Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-05-26T07:08:53Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpawqH1w#tuple-stress:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpawqH1w/incremental-state"
[2022-05-26T07:08:59Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-05-26T07:08:59Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpawqH1w#tuple-stress:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpawqH1w/incremental-state"
[2022-05-26T07:09:01Z DEBUG collector::execute] applying new row to "/tmp/.tmpawqH1w"
[2022-05-26T07:09:01Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-05-26T07:09:01Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-05-26T07:09:01Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpawqH1w#tuple-stress:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpawqH1w/incremental-state"
[2022-05-26T07:09:07Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T07:09:07Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-26T07:09:07Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpP8rE3E#tuple-stress:0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T07:09:13Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
---
[2022-05-26T07:09:28Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T07:09:28Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-26T07:09:28Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpyMtUa7#tuple-stress:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T07:09:33Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-26T07:09:33Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpyMtUa7#tuple-stress:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpyMtUa7/incremental-state"
[2022-05-26T07:09:39Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-05-26T07:09:39Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpyMtUa7#tuple-stress:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpyMtUa7/incremental-state"
[2022-05-26T07:09:41Z DEBUG collector::execute] applying new row to "/tmp/.tmpyMtUa7"
[2022-05-26T07:09:42Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-05-26T07:09:42Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-05-26T07:09:42Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpyMtUa7#tuple-stress:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpyMtUa7/incremental-state"
+ cd /checkout/obj
+ ./build/x86_64-unknown-linux-gnu/llvm/bin/llvm-profdata merge -o /tmp/rustc-pgo.profdata /tmp/rustc-pgo
+ rm -r ./build/x86_64-unknown-linux-gnu/llvm ./build/x86_64-unknown-linux-gnu/lld
+ python3 ../x.py dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu --include-default-paths build-manifest --rust-profile-use=/tmp/rustc-pgo.profdata --llvm-profile-use=/tmp/llvm-pgo.profdata
---
[RUSTC-TIMING] unicode_xid test:false 0.101
   Compiling tinyvec v0.3.4
[RUSTC-TIMING] ppv_lite86 test:false 0.463
   Compiling ansi_term v0.12.1
warning: rustc_graphviz.6dcfd60b-cgu.10: no profile data available for function _RNvXs2_NtCsgxC4CXDL7az_4core7convertNtNtNtB7_5alloc6layout11LayoutErrorINtB5_4FromBy_E4fromCslL8gIRdJGaJ_14rustc_graphviz Hash = 742261418966908927

warning: rustc_fs_util.cceb9b64-cgu.5: no profile data available for function _RNvXs2_NtCsgxC4CXDL7az_4core7convertNtNtNtB7_5alloc6layout11LayoutErrorINtB5_4FromBy_E4fromCsdhfY1dVdlya_13rustc_fs_util Hash = 742261418966908927
[RUSTC-TIMING] unic_char_range test:false 0.154
   Compiling crc32fast v1.2.0
   Compiling crc32fast v1.2.0
warning: rustc_graphviz.6dcfd60b-cgu.8: no profile data available for function _RNvMNtNtNtCsgxC4CXDL7az_4core4iter8adapters3revINtB2_3RevINtNtNtB8_3ops5range5RangejEE3newCslL8gIRdJGaJ_14rustc_graphviz Hash = 742261418966908927
[RUSTC-TIMING] rustc_fs_util test:false 0.138
warning: `rustc_fs_util` (lib) generated 1 warning
   Compiling snap v1.0.1
[RUSTC-TIMING] either test:false 0.141
---
[RUSTC-TIMING] matchers test:false 0.123
   Compiling block-buffer v0.10.2
[RUSTC-TIMING] hashbrown test:false 0.574
[RUSTC-TIMING] crypto_common test:false 0.089
warning: rustc_llvm.bc2429ff-cgu.6: no profile data available for function _RNvXs2_NtCsgxC4CXDL7az_4core7convertNtNtNtB7_5alloc6layout11LayoutErrorINtB5_4FromBy_E4fromCschkCTJmjI1D_10rustc_llvm Hash = 742261418966908927
[RUSTC-TIMING] block_buffer test:false 0.117
   Compiling digest v0.10.2
[RUSTC-TIMING] rustc_llvm test:false 0.256
warning: `rustc_llvm` (lib) generated 1 warning
---
   Compiling rustc_target v0.0.0 (/checkout/compiler/rustc_target)
   Compiling rustc_feature v0.0.0 (/checkout/compiler/rustc_feature)
   Compiling chalk-engine v0.80.0
[RUSTC-TIMING] chalk_solve test:false 3.772
warning: rustc_error_messages.5497b15d-cgu.11: no profile data available for function _RINvMsx_NtCsbYMOjsVZYrT_12tracing_core5fieldNtB6_8FieldSet9value_setATRNtB6_5FieldINtNtCsgxC4CXDL7az_4core6option6OptionRDNtB6_5ValueEL_EEj1_ECsfKaVhy5PjZh_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.5497b15d-cgu.11: no profile data available for function _RINvMsx_NtCsbYMOjsVZYrT_12tracing_core5fieldNtB6_8FieldSet9value_setATRNtB6_5FieldINtNtCsgxC4CXDL7az_4core6option6OptionRDNtB6_5ValueEL_EEj2_ECsfKaVhy5PjZh_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.5497b15d-cgu.11: no profile data available for function _RINvMsx_NtCsbYMOjsVZYrT_12tracing_core5fieldNtB6_8FieldSet9value_setATRNtB6_5FieldINtNtCsgxC4CXDL7az_4core6option6OptionRDNtB6_5ValueEL_EEj5_ECsfKaVhy5PjZh_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.5497b15d-cgu.11: no profile data available for function _RINvNtCsbYMOjsVZYrT_12tracing_core5field5debugRINtNtCs5Q2FUy7BXG_5alloc3vec3VecNtNtCs3yiPweuYEgW_3std4path7PathBufEECsfKaVhy5PjZh_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.5497b15d-cgu.11: no profile data available for function _RINvNtCsbYMOjsVZYrT_12tracing_core5field5debugRINtNtCsgxC4CXDL7az_4core6option6OptionNtCsk6xmqXx2eYv_16unic_langid_impl18LanguageIdentifierEECsfKaVhy5PjZh_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.5497b15d-cgu.11: no profile data available for function _RINvNtCsbYMOjsVZYrT_12tracing_core5field5debugRINtNtCsgxC4CXDL7az_4core6option6OptionNtNtCs3yiPweuYEgW_3std4path7PathBufEECsfKaVhy5PjZh_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.5497b15d-cgu.11: no profile data available for function _RINvNtCsbYMOjsVZYrT_12tracing_core5field5debugRINtNtCsgxC4CXDL7az_4core6option6OptionRNtNtCs3yiPweuYEgW_3std4path4PathEECsfKaVhy5PjZh_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.5497b15d-cgu.11: no profile data available for function _RINvNtCsbYMOjsVZYrT_12tracing_core5field5debugRNtCsk6xmqXx2eYv_16unic_langid_impl18LanguageIdentifierECsfKaVhy5PjZh_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.5497b15d-cgu.11: no profile data available for function _RINvNtCsbYMOjsVZYrT_12tracing_core5field5debugRNtNtCs2bIKvJD7gFy_13fluent_bundle8resource14FluentResourceECsfKaVhy5PjZh_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.5497b15d-cgu.11: no profile data available for function _RINvNtCsbYMOjsVZYrT_12tracing_core5field5debugRNtNtCs3yiPweuYEgW_3std4path7PathBufECsfKaVhy5PjZh_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.5497b15d-cgu.11: no profile data available for function _RINvNtCsbYMOjsVZYrT_12tracing_core5field5debugRQNtNtCs3yiPweuYEgW_3std4path7PathBufECsfKaVhy5PjZh_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.5497b15d-cgu.11: no profile data available for function _RINvNtCsbYMOjsVZYrT_12tracing_core5field5debugRRSReECsfKaVhy5PjZh_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.5497b15d-cgu.11: no profile data available for function _RINvNtCsgxC4CXDL7az_4core3ptr13drop_in_placeRINtNtB4_6option6OptionNtCsk6xmqXx2eYv_16unic_langid_impl18LanguageIdentifierEECsfKaVhy5PjZh_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.5497b15d-cgu.11: no profile data available for function _RINvNtCsgxC4CXDL7az_4core3ptr13drop_in_placeRINtNtB4_6option6OptionNtNtCs3yiPweuYEgW_3std4path7PathBufEECsfKaVhy5PjZh_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.5497b15d-cgu.11: no profile data available for function _RINvNtCsgxC4CXDL7az_4core3ptr13drop_in_placeRINtNtB4_6option6OptionRNtNtCs3yiPweuYEgW_3std4path4PathEECsfKaVhy5PjZh_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.5497b15d-cgu.11: no profile data available for function _RINvNtCsgxC4CXDL7az_4core3ptr13drop_in_placeRINtNtCs5Q2FUy7BXG_5alloc3vec3VecNtNtCs3yiPweuYEgW_3std4path7PathBufEECsfKaVhy5PjZh_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.5497b15d-cgu.11: no profile data available for function _RINvNtCsgxC4CXDL7az_4core3ptr13drop_in_placeRNtCsk6xmqXx2eYv_16unic_langid_impl18LanguageIdentifierECsfKaVhy5PjZh_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.5497b15d-cgu.11: no profile data available for function _RINvNtCsgxC4CXDL7az_4core3ptr13drop_in_placeRNtNtCs2bIKvJD7gFy_13fluent_bundle8resource14FluentResourceECsfKaVhy5PjZh_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.5497b15d-cgu.11: no profile data available for function _RINvNtCsgxC4CXDL7az_4core3ptr13drop_in_placeRNtNtCs3yiPweuYEgW_3std4path7PathBufECsfKaVhy5PjZh_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.5497b15d-cgu.11: no profile data available for function _RINvNtCsgxC4CXDL7az_4core3ptr13drop_in_placeRQNtNtCs3yiPweuYEgW_3std4path7PathBufECsfKaVhy5PjZh_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.5497b15d-cgu.11: no profile data available for function _RINvNtCsgxC4CXDL7az_4core3ptr13drop_in_placeRRSReECsfKaVhy5PjZh_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.5497b15d-cgu.11: no profile data available for function _RNvXsk_NtCsbYMOjsVZYrT_12tracing_core5fieldINtB5_10DebugValueRINtNtCs5Q2FUy7BXG_5alloc3vec3VecNtNtCs3yiPweuYEgW_3std4path7PathBufEENtB5_5Value6recordCsfKaVhy5PjZh_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.5497b15d-cgu.11: no profile data available for function _RNvXsk_NtCsbYMOjsVZYrT_12tracing_core5fieldINtB5_10DebugValueRINtNtCsgxC4CXDL7az_4core6option6OptionNtCsk6xmqXx2eYv_16unic_langid_impl18LanguageIdentifierEENtB5_5Value6recordCsfKaVhy5PjZh_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.5497b15d-cgu.11: no profile data available for function _RNvXsk_NtCsbYMOjsVZYrT_12tracing_core5fieldINtB5_10DebugValueRINtNtCsgxC4CXDL7az_4core6option6OptionNtNtCs3yiPweuYEgW_3std4path7PathBufEENtB5_5Value6recordCsfKaVhy5PjZh_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.5497b15d-cgu.11: no profile data available for function _RNvXsk_NtCsbYMOjsVZYrT_12tracing_core5fieldINtB5_10DebugValueRINtNtCsgxC4CXDL7az_4core6option6OptionRNtNtCs3yiPweuYEgW_3std4path4PathEENtB5_5Value6recordCsfKaVhy5PjZh_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.5497b15d-cgu.11: no profile data available for function _RNvXsk_NtCsbYMOjsVZYrT_12tracing_core5fieldINtB5_10DebugValueRNtCsk6xmqXx2eYv_16unic_langid_impl18LanguageIdentifierENtB5_5Value6recordCsfKaVhy5PjZh_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.5497b15d-cgu.11: no profile data available for function _RNvXsk_NtCsbYMOjsVZYrT_12tracing_core5fieldINtB5_10DebugValueRNtNtCs2bIKvJD7gFy_13fluent_bundle8resource14FluentResourceENtB5_5Value6recordCsfKaVhy5PjZh_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.5497b15d-cgu.11: no profile data available for function _RNvXsk_NtCsbYMOjsVZYrT_12tracing_core5fieldINtB5_10DebugValueRNtNtCs3yiPweuYEgW_3std4path7PathBufENtB5_5Value6recordCsfKaVhy5PjZh_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.5497b15d-cgu.11: no profile data available for function _RNvXsk_NtCsbYMOjsVZYrT_12tracing_core5fieldINtB5_10DebugValueRQNtNtCs3yiPweuYEgW_3std4path7PathBufENtB5_5Value6recordCsfKaVhy5PjZh_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.5497b15d-cgu.11: no profile data available for function _RNvXsk_NtCsbYMOjsVZYrT_12tracing_core5fieldINtB5_10DebugValueRRSReENtB5_5Value6recordCsfKaVhy5PjZh_20rustc_error_messages Hash = 170957022131388415
[RUSTC-TIMING] rustc_error_messages test:false 1.402
warning: `rustc_error_messages` (lib) generated 30 warnings
[RUSTC-TIMING] rustc_feature test:false 1.496
[RUSTC-TIMING] chalk_engine test:false 1.555
---
    Checking rustc_type_ir v0.0.0 (/checkout/compiler/rustc_type_ir)
 Documenting rustc_type_ir v0.0.0 (/checkout/compiler/rustc_type_ir)
 Documenting rustc_span v0.0.0 (/checkout/compiler/rustc_span)
[RUSTC-TIMING] rustc_type_ir test:false 0.365
error: unresolved link to `rustc_hir::TyKind`
  --> compiler/rustc_type_ir/src/sty.rs:18:60
   |
18 | /// Types written by the user start out as [`hir::TyKind`](rustc_hir::TyKind) and get
   |                                                            ^^^^^^^^^^^^^^^^^ no item named `rustc_hir` in scope
   |
   = note: `-D rustdoc::broken-intra-doc-links` implied by `-D warnings`

error: unresolved link to `FnDef`
  --> compiler/rustc_type_ir/src/sty.rs:84:10
   |
84 |     /// [FnDef] or [Closure] which can be then be coerced to this variant.
   |          ^^^^^ no item named `FnDef` in scope
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `Closure`
error: unresolved link to `Closure`
  --> compiler/rustc_type_ir/src/sty.rs:84:21
   |
84 |     /// [FnDef] or [Closure] which can be then be coerced to this variant.
   |                     ^^^^^^^ no item named `Closure` in scope
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

error: unresolved link to `ClosureSubsts`
   --> compiler/rustc_type_ir/src/sty.rs:101:10
    |
101 |     /// [ClosureSubsts] for more details.
    |          ^^^^^^^^^^^^^ no item named `ClosureSubsts` in scope
    |
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

error: unresolved link to `GeneratorSubsts`
   --> compiler/rustc_type_ir/src/sty.rs:108:10
    |
108 |     /// [GeneratorSubsts].
    |          ^^^^^^^^^^^^^^^ no item named `GeneratorSubsts` in scope
    |
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

error: unresolved link to `GeneratorSubsts`
   --> compiler/rustc_type_ir/src/sty.rs:112:49
    |
112 |     /// This should only appear as part of the [GeneratorSubsts].
    |                                                 ^^^^^^^^^^^^^^^ no item named `GeneratorSubsts` in scope
    |
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: could not document `rustc_type_ir`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name rustc_type_ir compiler/rustc_type_ir/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/doc --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -C metadata=6d581c1b7631f9e2 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-8db9ec982a978a4d.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-650bfd0594a5fe32.rmeta --extern rustc_index=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_index-351f3773c639cba7.rmeta --extern rustc_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps/librustc_macros-5d871d1088043d1f.so --extern rustc_serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_serialize-fdb4862e2b55fc87.rmeta --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-3876c03f3e7b9b20.rmeta --extern-html-root-url 'bitflags=https://docs.rs/bitflags/1.2.1/' --extern-html-root-url 'smallvec=https://docs.rs/smallvec/1.7.0/' -Zunstable-options -Csymbol-mangling-version=v0 -Zunstable-options '--check-cfg=names()' '--check-cfg=values()' '--check-cfg=values(bootstrap)' '--check-cfg=values(parallel_compiler)' '--check-cfg=values(no_btreemap_remove_entry)' '--check-cfg=values(crossbeam_loom)' '--check-cfg=values(span_locations)' -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.63.0-nightly
  (fbb5bdda1
  2022-05-26)' --document-private-items '-Arustdoc::private-intra-doc-links' --enable-index-page -Zunstable-options -Znormalize-docs --show-type-layout --generate-link-to-definition --extern-html-root-url 'ena=https://docs.rs/ena/latest/'` (exit status: 1)
[RUSTC-TIMING] rustc_span test:false 1.382
[RUSTC-TIMING] chalk_solve test:false 2.236
[RUSTC-TIMING] serde test:false 2.364
Build completed unsuccessfully in 0:21:39
