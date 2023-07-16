plain
Executing benchmark hyper-0.14.18 (3/7)
Preparing hyper-0.14.18
[2022-07-02T02:13:03Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-07-02T02:13:03Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-07-02T02:13:03Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpuD3cDz#hyper@0.14.18" "--features=client,http1,http2,server,stream" "--" "--skip-this-rustc"
[2022-07-02T02:13:03Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzNXYSz#hyper@0.14.18" "--release" "--features=client,http1,http2,server,stream" "--" "--skip-this-rustc"
[2022-07-02T02:13:49Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-02T02:13:49Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-07-02T02:13:49Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpAWv4wf#hyper@0.14.18" "--features=client,http1,http2,server,stream" "--" "--wrap-rustc-with" "Eprintln"
Running hyper-0.14.18: Opt + [Full]
---
[2022-07-02T02:14:25Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmps8hlWi#regex@1.5.5" "--" "--wrap-rustc-with" "Eprintln"
Running regex-1.5.5: Opt + [Full]
[2022-07-02T02:14:29Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-02T02:14:29Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-07-02T02:14:29Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpeFMNOQ#regex@1.5.5" "--release" "--" "--wrap-rustc-with" "Eprintln"
Executing benchmark ripgrep-13.0.0 (5/7)
Preparing ripgrep-13.0.0
[2022-07-02T02:14:48Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-07-02T02:14:48Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
---
Preparing bitmaps-3.1.0
[2022-07-02T02:33:54Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-07-02T02:33:54Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-07-02T02:33:54Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-07-02T02:33:54Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpZpeFh7#bitmaps@3.1.0" "--" "--skip-this-rustc"
[2022-07-02T02:33:54Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp8P3Y1Z#bitmaps@3.1.0" "--release" "--" "--skip-this-rustc"
[2022-07-02T02:33:54Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpddacqs#bitmaps@3.1.0" "--profile" "check" "--" "--skip-this-rustc"
[2022-07-02T02:33:55Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-02T02:33:55Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-07-02T02:33:55Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpcT6db6#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-07-02T02:33:57Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
---
[2022-07-02T02:34:04Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-02T02:34:04Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-07-02T02:34:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpp8KvBG#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-07-02T02:34:07Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-07-02T02:34:07Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpp8KvBG#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpp8KvBG/incremental-state"
[2022-07-02T02:34:11Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-07-02T02:34:11Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpp8KvBG#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpp8KvBG/incremental-state"
[2022-07-02T02:34:12Z DEBUG collector::execute] applying println to "/tmp/.tmpp8KvBG"
[2022-07-02T02:34:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-07-02T02:34:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-07-02T02:34:12Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpp8KvBG#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpp8KvBG/incremental-state"
[2022-07-02T02:34:13Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-02T02:34:13Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-07-02T02:34:13Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpWBvchI#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-07-02T02:34:16Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-07-02T02:34:16Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-07-02T02:34:16Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpWBvchI#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpWBvchI/incremental-state"
[2022-07-02T02:34:20Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-07-02T02:34:20Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpWBvchI#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpWBvchI/incremental-state"
[2022-07-02T02:34:21Z DEBUG collector::execute] applying println to "/tmp/.tmpWBvchI"
[2022-07-02T02:34:21Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-07-02T02:34:21Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-07-02T02:34:21Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpWBvchI#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpWBvchI/incremental-state"
Executing benchmark cargo-0.60.0 (2/8)
Preparing cargo-0.60.0
[2022-07-02T02:34:22Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-07-02T02:34:22Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
---
[2022-07-02T02:35:43Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-02T02:35:44Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-07-02T02:35:44Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJoqCwj#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2022-07-02T02:36:08Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-07-02T02:36:08Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJoqCwj#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpJoqCwj/incremental-state"
[2022-07-02T02:36:37Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-07-02T02:36:37Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJoqCwj#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpJoqCwj/incremental-state"
[2022-07-02T02:36:42Z DEBUG collector::execute] applying println to "/tmp/.tmpJoqCwj"
[2022-07-02T02:36:42Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-07-02T02:36:42Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-07-02T02:36:42Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJoqCwj#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpJoqCwj/incremental-state"
[2022-07-02T02:36:55Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-02T02:36:56Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-07-02T02:36:56Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmprC5Wg5#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2022-07-02T02:38:07Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-07-02T02:38:07Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-07-02T02:38:07Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmprC5Wg5#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmprC5Wg5/incremental-state"
[2022-07-02T02:39:24Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-07-02T02:39:24Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmprC5Wg5#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmprC5Wg5/incremental-state"
[2022-07-02T02:39:38Z DEBUG collector::execute] applying println to "/tmp/.tmprC5Wg5"
[2022-07-02T02:39:38Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-07-02T02:39:38Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-07-02T02:39:38Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmprC5Wg5#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmprC5Wg5/incremental-state"
[2022-07-02T02:40:08Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-02T02:40:09Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-07-02T02:40:09Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp54DCe9#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2022-07-02T02:41:35Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
[2022-07-02T02:43:56Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-02T02:43:56Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-07-02T02:43:56Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmppTR6gG#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-07-02T02:44:06Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-07-02T02:44:06Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmppTR6gG#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmppTR6gG/incremental-state"
[2022-07-02T02:44:19Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-07-02T02:44:19Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmppTR6gG#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmppTR6gG/incremental-state"
[2022-07-02T02:44:19Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-02T02:44:19Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-07-02T02:44:19Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpatGMHV#ctfe-stress-5@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-07-02T02:44:29Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-07-02T02:44:29Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-07-02T02:44:29Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpatGMHV#ctfe-stress-5@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpatGMHV/incremental-state"
[2022-07-02T02:44:41Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-07-02T02:44:41Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpatGMHV#ctfe-stress-5@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpatGMHV/incremental-state"
[2022-07-02T02:44:42Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-02T02:44:42Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-07-02T02:44:42Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpla56XE#ctfe-stress-5@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-07-02T02:44:51Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
Preparing diesel-1.4.8
[2022-07-02T02:45:04Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-07-02T02:45:04Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-07-02T02:45:04Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-07-02T02:45:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmppOWLfG#diesel@1.4.8" "--" "--skip-this-rustc"
[2022-07-02T02:45:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpdRuwnw#diesel@1.4.8" "--profile" "check" "--" "--skip-this-rustc"
[2022-07-02T02:45:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpD03P2h#diesel@1.4.8" "--release" "--" "--skip-this-rustc"
[2022-07-02T02:45:30Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-02T02:45:30Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-07-02T02:45:30Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpTMr6dG#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-07-02T02:45:49Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-07-02T02:45:49Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-07-02T02:45:49Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpTMr6dG#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpTMr6dG/incremental-state"
[2022-07-02T02:46:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-07-02T02:46:12Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpTMr6dG#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpTMr6dG/incremental-state"
[2022-07-02T02:46:16Z DEBUG collector::execute] applying println to "/tmp/.tmpTMr6dG"
[2022-07-02T02:46:16Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-07-02T02:46:16Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-07-02T02:46:16Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpTMr6dG#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpTMr6dG/incremental-state"
[2022-07-02T02:46:20Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-02T02:46:20Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-07-02T02:46:20Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpcrb2Dh#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln"
[2022-07-02T02:46:42Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-07-02T02:46:42Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-07-02T02:46:42Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpcrb2Dh#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpcrb2Dh/incremental-state"
[2022-07-02T02:47:09Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-07-02T02:47:09Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpcrb2Dh#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpcrb2Dh/incremental-state"
[2022-07-02T02:47:13Z DEBUG collector::execute] applying println to "/tmp/.tmpcrb2Dh"
[2022-07-02T02:47:13Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-07-02T02:47:13Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-07-02T02:47:13Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpcrb2Dh#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpcrb2Dh/incremental-state"
[2022-07-02T02:47:18Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-02T02:47:18Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-07-02T02:47:18Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp9Vm4G8#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-07-02T02:47:40Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
[2022-07-02T02:48:21Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-02T02:48:21Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-07-02T02:48:21Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpqK0Jsr#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-07-02T02:48:22Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-07-02T02:48:22Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpqK0Jsr#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpqK0Jsr/incremental-state"
[2022-07-02T02:48:24Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-07-02T02:48:24Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpqK0Jsr#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpqK0Jsr/incremental-state"
Executing benchmark match-stress (6/8)
Preparing match-stress
[2022-07-02T02:48:25Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-07-02T02:48:25Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2022-07-02T02:48:25Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-02T02:48:25Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-07-02T02:48:25Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpLOnELW#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-07-02T02:48:28Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-07-02T02:48:28Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpLOnELW#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpLOnELW/incremental-state"
[2022-07-02T02:48:33Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-07-02T02:48:33Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpLOnELW#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpLOnELW/incremental-state"
[2022-07-02T02:48:34Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-02T02:48:34Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-07-02T02:48:34Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpaIs5CE#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-07-02T02:48:38Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-07-02T02:48:38Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-07-02T02:48:38Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpaIs5CE#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpaIs5CE/incremental-state"
[2022-07-02T02:48:42Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-07-02T02:48:42Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpaIs5CE#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpaIs5CE/incremental-state"
[2022-07-02T02:48:43Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-02T02:48:43Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-07-02T02:48:43Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpIV1Ye7#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-07-02T02:48:47Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
[2022-07-02T02:48:54Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-02T02:48:54Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-07-02T02:48:54Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpgVlkO4#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2022-07-02T02:48:54Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-07-02T02:48:54Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpgVlkO4#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpgVlkO4/incremental-state"
[2022-07-02T02:48:55Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-07-02T02:48:55Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpgVlkO4#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpgVlkO4/incremental-state"
[2022-07-02T02:48:55Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-02T02:48:55Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-07-02T02:48:55Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQAhzAN#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2022-07-02T02:48:56Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-07-02T02:48:56Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-07-02T02:48:56Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQAhzAN#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpQAhzAN/incremental-state"
[2022-07-02T02:48:56Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-07-02T02:48:56Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQAhzAN#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpQAhzAN/incremental-state"
Executing benchmark tuple-stress (8/8)
Preparing tuple-stress
[2022-07-02T02:48:56Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-07-02T02:48:56Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2022-07-02T02:48:57Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-02T02:48:57Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-07-02T02:48:57Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp5qNAFK#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-07-02T02:49:03Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-07-02T02:49:03Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp5qNAFK#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp5qNAFK/incremental-state"
[2022-07-02T02:49:09Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-07-02T02:49:09Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp5qNAFK#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp5qNAFK/incremental-state"
[2022-07-02T02:49:11Z DEBUG collector::execute] applying new row to "/tmp/.tmp5qNAFK"
[2022-07-02T02:49:11Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-07-02T02:49:11Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-07-02T02:49:11Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp5qNAFK#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp5qNAFK/incremental-state"
[2022-07-02T02:49:17Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-02T02:49:17Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-07-02T02:49:17Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpUbnlLf#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-07-02T02:49:23Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-07-02T02:49:23Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-07-02T02:49:23Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpUbnlLf#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpUbnlLf/incremental-state"
[2022-07-02T02:49:30Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-07-02T02:49:30Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpUbnlLf#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpUbnlLf/incremental-state"
[2022-07-02T02:49:32Z DEBUG collector::execute] applying new row to "/tmp/.tmpUbnlLf"
[2022-07-02T02:49:32Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-07-02T02:49:32Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-07-02T02:49:32Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpUbnlLf#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpUbnlLf/incremental-state"
[2022-07-02T02:49:39Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-02T02:49:39Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-07-02T02:49:39Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpRNmmeo#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-07-02T02:49:44Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-07-02T02:49:44Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-07-02T02:49:44Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpRNmmeo#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpRNmmeo/incremental-state"
[2022-07-02T02:49:51Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-07-02T02:49:51Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpRNmmeo#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpRNmmeo/incremental-state"
[2022-07-02T02:49:53Z DEBUG collector::execute] applying new row to "/tmp/.tmpRNmmeo"
[2022-07-02T02:49:53Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-07-02T02:49:53Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-07-02T02:49:53Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpRNmmeo#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpRNmmeo/incremental-state"
+ cd /checkout/obj
+ RUSTC_PROFILE_MERGED_FILE=/tmp/rustc-pgo.profdata
+ ./build/x86_64-unknown-linux-gnu/llvm/bin/llvm-profdata merge -o /tmp/rustc-pgo.profdata /tmp/rustc-pgo
+ echo 'Rustc PGO statistics'
---
[RUSTC-TIMING] cpufeatures test:false 0.358
   Compiling ansi_term v0.12.1
[RUSTC-TIMING] unicode_width test:false 0.337
   Compiling snap v1.0.1
warning: rustc_graphviz.dbf8342d-cgu.8: no profile data available for function _RNvXs2_NtCsjfX2UXV8h5R_4core7convertNtNtNtB7_5alloc6layout11LayoutErrorINtB5_4FromBy_E4fromCscBydYYIUpIv_14rustc_graphviz Hash = 742261418966908927

warning: rustc_graphviz.dbf8342d-cgu.5: no profile data available for function _RNvMNtNtNtCsjfX2UXV8h5R_4core4iter8adapters3revINtB2_3RevINtNtNtB8_3ops5range5RangejEE3newCscBydYYIUpIv_14rustc_graphviz Hash = 742261418966908927
   Compiling adler v0.2.3
[RUSTC-TIMING] unicode_xid test:false 0.335
[RUSTC-TIMING] unicode_xid test:false 0.335
warning: rustc_fs_util.a24ac5b6-cgu.3: no profile data available for function _RNvXs2_NtCsjfX2UXV8h5R_4core7convertNtNtNtB7_5alloc6layout11LayoutErrorINtB5_4FromBy_E4fromCsaiBh8ijmwa8_13rustc_fs_util Hash = 742261418966908927
   Compiling unicode-script v0.5.3
[RUSTC-TIMING] build_script_build test:false 1.005
   Compiling fixedbitset v0.2.0
[RUSTC-TIMING] smallvec test:false 0.600
---
[RUSTC-TIMING] jobserver test:false 2.670
[RUSTC-TIMING] psm test:false 2.001
[RUSTC-TIMING] stacker test:false 0.691
   Compiling rand v0.8.5
warning: rustc_llvm.fd41b93c-cgu.3: no profile data available for function _RNvXs2_NtCsjfX2UXV8h5R_4core7convertNtNtNtB7_5alloc6layout11LayoutErrorINtB5_4FromBy_E4fromCsko943KsPBHk_10rustc_llvm Hash = 742261418966908927
[RUSTC-TIMING] crossbeam_channel test:false 2.287
   Compiling digest v0.10.2
[RUSTC-TIMING] rustc_lexer test:false 2.707
[RUSTC-TIMING] generic_array test:false 2.453
---
   Compiling unic-langid v0.9.0
[RUSTC-TIMING] unic_langid_macros test:false 0.172
[RUSTC-TIMING] chalk_derive test:false 2.116
   Compiling chalk-ir v0.80.0
warning: rustc_serialize.db975148-cgu.5: no profile data available for function _RINvNtCsjfX2UXV8h5R_4core3ptr13drop_in_placeRjECs4n6vzr0rEYm_15rustc_serialize Hash = 742261418966908927

warning: rustc_serialize.db975148-cgu.5: no profile data available for function _RINvNtCsjfX2UXV8h5R_4core9panicking13assert_failedjjECs4n6vzr0rEYm_15rustc_serialize Hash = 742261418966908927

warning: rustc_serialize.db975148-cgu.6: no profile data available for function _RNvXsV_NtCsjfX2UXV8h5R_4core3fmtRjNtB5_5Debug3fmtCs4n6vzr0rEYm_15rustc_serialize Hash = 1124680650125156080
[RUSTC-TIMING] unic_langid_macros test:false 0.220
   Compiling intl-memoizer v0.5.1
   Compiling fluent-langneg v0.13.0
   Compiling intl_pluralrules v7.0.1
---
   Compiling rustc_ast v0.0.0 (/checkout/compiler/rustc_ast)
   Compiling rustc_error_messages v0.0.0 (/checkout/compiler/rustc_error_messages)
   Compiling rustc_target v0.0.0 (/checkout/compiler/rustc_target)
   Compiling rustc_feature v0.0.0 (/checkout/compiler/rustc_feature)
warning: rustc_error_messages.003cc23e-cgu.11: no profile data available for function _RINvMsx_NtCsNvAsZyCleD_12tracing_core5fieldNtB6_8FieldSet9value_setATRNtB6_5FieldINtNtCsjfX2UXV8h5R_4core6option6OptionRDNtB6_5ValueEL_EEj1_ECs88p0SYWuc9K_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.003cc23e-cgu.11: no profile data available for function _RINvMsx_NtCsNvAsZyCleD_12tracing_core5fieldNtB6_8FieldSet9value_setATRNtB6_5FieldINtNtCsjfX2UXV8h5R_4core6option6OptionRDNtB6_5ValueEL_EEj2_ECs88p0SYWuc9K_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.003cc23e-cgu.11: no profile data available for function _RINvMsx_NtCsNvAsZyCleD_12tracing_core5fieldNtB6_8FieldSet9value_setATRNtB6_5FieldINtNtCsjfX2UXV8h5R_4core6option6OptionRDNtB6_5ValueEL_EEj5_ECs88p0SYWuc9K_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.003cc23e-cgu.11: no profile data available for function _RINvNtCsNvAsZyCleD_12tracing_core5field5debugRINtNtCs4gF7tHSht2S_5alloc3vec3VecNtNtCsima656YYXsR_3std4path7PathBufEECs88p0SYWuc9K_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.003cc23e-cgu.11: no profile data available for function _RINvNtCsNvAsZyCleD_12tracing_core5field5debugRINtNtCsjfX2UXV8h5R_4core6option6OptionNtCs57dJSuulQkf_16unic_langid_impl18LanguageIdentifierEECs88p0SYWuc9K_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.003cc23e-cgu.11: no profile data available for function _RINvNtCsNvAsZyCleD_12tracing_core5field5debugRINtNtCsjfX2UXV8h5R_4core6option6OptionNtNtCsima656YYXsR_3std4path7PathBufEECs88p0SYWuc9K_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.003cc23e-cgu.11: no profile data available for function _RINvNtCsNvAsZyCleD_12tracing_core5field5debugRINtNtCsjfX2UXV8h5R_4core6option6OptionRNtNtCsima656YYXsR_3std4path4PathEECs88p0SYWuc9K_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.003cc23e-cgu.11: no profile data available for function _RINvNtCsNvAsZyCleD_12tracing_core5field5debugRNtCs57dJSuulQkf_16unic_langid_impl18LanguageIdentifierECs88p0SYWuc9K_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.003cc23e-cgu.11: no profile data available for function _RINvNtCsNvAsZyCleD_12tracing_core5field5debugRNtNtCs9H1M1zC8Dso_13fluent_bundle8resource14FluentResourceECs88p0SYWuc9K_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.003cc23e-cgu.11: no profile data available for function _RINvNtCsNvAsZyCleD_12tracing_core5field5debugRNtNtCsima656YYXsR_3std4path7PathBufECs88p0SYWuc9K_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.003cc23e-cgu.11: no profile data available for function _RINvNtCsNvAsZyCleD_12tracing_core5field5debugRQNtNtCsima656YYXsR_3std4path7PathBufECs88p0SYWuc9K_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.003cc23e-cgu.11: no profile data available for function _RINvNtCsNvAsZyCleD_12tracing_core5field5debugRRSReECs88p0SYWuc9K_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.003cc23e-cgu.11: no profile data available for function _RINvNtCsjfX2UXV8h5R_4core3ptr13drop_in_placeRINtNtB4_6option6OptionNtCs57dJSuulQkf_16unic_langid_impl18LanguageIdentifierEECs88p0SYWuc9K_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.003cc23e-cgu.11: no profile data available for function _RINvNtCsjfX2UXV8h5R_4core3ptr13drop_in_placeRINtNtB4_6option6OptionNtNtCsima656YYXsR_3std4path7PathBufEECs88p0SYWuc9K_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.003cc23e-cgu.11: no profile data available for function _RINvNtCsjfX2UXV8h5R_4core3ptr13drop_in_placeRINtNtB4_6option6OptionRNtNtCsima656YYXsR_3std4path4PathEECs88p0SYWuc9K_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.003cc23e-cgu.11: no profile data available for function _RINvNtCsjfX2UXV8h5R_4core3ptr13drop_in_placeRINtNtCs4gF7tHSht2S_5alloc3vec3VecNtNtCsima656YYXsR_3std4path7PathBufEECs88p0SYWuc9K_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.003cc23e-cgu.11: no profile data available for function _RINvNtCsjfX2UXV8h5R_4core3ptr13drop_in_placeRNtCs57dJSuulQkf_16unic_langid_impl18LanguageIdentifierECs88p0SYWuc9K_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.003cc23e-cgu.11: no profile data available for function _RINvNtCsjfX2UXV8h5R_4core3ptr13drop_in_placeRNtNtCs9H1M1zC8Dso_13fluent_bundle8resource14FluentResourceECs88p0SYWuc9K_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.003cc23e-cgu.11: no profile data available for function _RINvNtCsjfX2UXV8h5R_4core3ptr13drop_in_placeRNtNtCsima656YYXsR_3std4path7PathBufECs88p0SYWuc9K_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.003cc23e-cgu.11: no profile data available for function _RINvNtCsjfX2UXV8h5R_4core3ptr13drop_in_placeRQNtNtCsima656YYXsR_3std4path7PathBufECs88p0SYWuc9K_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.003cc23e-cgu.11: no profile data available for function _RINvNtCsjfX2UXV8h5R_4core3ptr13drop_in_placeRRSReECs88p0SYWuc9K_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.003cc23e-cgu.11: no profile data available for function _RNvXsk_NtCsNvAsZyCleD_12tracing_core5fieldINtB5_10DebugValueRINtNtCs4gF7tHSht2S_5alloc3vec3VecNtNtCsima656YYXsR_3std4path7PathBufEENtB5_5Value6recordCs88p0SYWuc9K_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.003cc23e-cgu.11: no profile data available for function _RNvXsk_NtCsNvAsZyCleD_12tracing_core5fieldINtB5_10DebugValueRINtNtCsjfX2UXV8h5R_4core6option6OptionNtCs57dJSuulQkf_16unic_langid_impl18LanguageIdentifierEENtB5_5Value6recordCs88p0SYWuc9K_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.003cc23e-cgu.11: no profile data available for function _RNvXsk_NtCsNvAsZyCleD_12tracing_core5fieldINtB5_10DebugValueRINtNtCsjfX2UXV8h5R_4core6option6OptionNtNtCsima656YYXsR_3std4path7PathBufEENtB5_5Value6recordCs88p0SYWuc9K_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.003cc23e-cgu.11: no profile data available for function _RNvXsk_NtCsNvAsZyCleD_12tracing_core5fieldINtB5_10DebugValueRINtNtCsjfX2UXV8h5R_4core6option6OptionRNtNtCsima656YYXsR_3std4path4PathEENtB5_5Value6recordCs88p0SYWuc9K_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.003cc23e-cgu.11: no profile data available for function _RNvXsk_NtCsNvAsZyCleD_12tracing_core5fieldINtB5_10DebugValueRNtCs57dJSuulQkf_16unic_langid_impl18LanguageIdentifierENtB5_5Value6recordCs88p0SYWuc9K_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.003cc23e-cgu.11: no profile data available for function _RNvXsk_NtCsNvAsZyCleD_12tracing_core5fieldINtB5_10DebugValueRNtNtCs9H1M1zC8Dso_13fluent_bundle8resource14FluentResourceENtB5_5Value6recordCs88p0SYWuc9K_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.003cc23e-cgu.11: no profile data available for function _RNvXsk_NtCsNvAsZyCleD_12tracing_core5fieldINtB5_10DebugValueRNtNtCsima656YYXsR_3std4path7PathBufENtB5_5Value6recordCs88p0SYWuc9K_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.003cc23e-cgu.11: no profile data available for function _RNvXsk_NtCsNvAsZyCleD_12tracing_core5fieldINtB5_10DebugValueRQNtNtCsima656YYXsR_3std4path7PathBufENtB5_5Value6recordCs88p0SYWuc9K_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.003cc23e-cgu.11: no profile data available for function _RNvXsk_NtCsNvAsZyCleD_12tracing_core5fieldINtB5_10DebugValueRRSReENtB5_5Value6recordCs88p0SYWuc9K_20rustc_error_messages Hash = 170957022131388415
[RUSTC-TIMING] rustc_error_messages test:false 1.185
warning: `rustc_error_messages` (lib) generated 30 warnings
[RUSTC-TIMING] rustc_feature test:false 1.394
[RUSTC-TIMING] rustc_span test:false 5.310
