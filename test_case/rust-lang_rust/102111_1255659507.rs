plain
[2022-09-21T19:25:18Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-09-21T19:25:18Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-09-21T19:25:18Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOJCbzH#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-09-21T19:25:20Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-09-21T19:25:20Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOJCbzH#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpOJCbzH/incremental-state"
[2022-09-21T19:25:23Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-09-21T19:25:23Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOJCbzH#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpOJCbzH/incremental-state"
[2022-09-21T19:25:24Z DEBUG collector::execute] applying println to "/tmp/.tmpOJCbzH"
[2022-09-21T19:25:24Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-09-21T19:25:24Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-09-21T19:25:24Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOJCbzH#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpOJCbzH/incremental-state"
[2022-09-21T19:25:25Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-09-21T19:25:25Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-09-21T19:25:25Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpalpIyD#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-09-21T19:25:27Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-09-21T19:25:27Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-09-21T19:25:27Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpalpIyD#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpalpIyD/incremental-state"
[2022-09-21T19:25:30Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-09-21T19:25:30Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpalpIyD#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpalpIyD/incremental-state"
[2022-09-21T19:25:31Z DEBUG collector::execute] applying println to "/tmp/.tmpalpIyD"
[2022-09-21T19:25:31Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-09-21T19:25:31Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-09-21T19:25:31Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpalpIyD#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpalpIyD/incremental-state"
[2022-09-21T19:25:32Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-09-21T19:25:32Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-09-21T19:25:32Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpnlU0dR#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-09-21T19:25:34Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-09-21T19:25:34Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-09-21T19:25:34Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpnlU0dR#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpnlU0dR/incremental-state"
[2022-09-21T19:25:38Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-09-21T19:25:38Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpnlU0dR#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpnlU0dR/incremental-state"
[2022-09-21T19:25:39Z DEBUG collector::execute] applying println to "/tmp/.tmpnlU0dR"
[2022-09-21T19:25:39Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-09-21T19:25:39Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-09-21T19:25:39Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpnlU0dR#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpnlU0dR/incremental-state"
Executing benchmark cargo-0.60.0 (2/8)
Preparing cargo-0.60.0
[2022-09-21T19:25:40Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-09-21T19:25:40Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2022-09-21T19:27:39Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-09-21T19:27:40Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-09-21T19:27:40Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpXbB2sR#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2022-09-21T19:28:31Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-09-21T19:28:31Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpXbB2sR#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpXbB2sR/incremental-state"
[2022-09-21T19:29:39Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-09-21T19:29:39Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpXbB2sR#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpXbB2sR/incremental-state"
[2022-09-21T19:29:52Z DEBUG collector::execute] applying println to "/tmp/.tmpXbB2sR"
[2022-09-21T19:29:52Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-09-21T19:29:52Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-09-21T19:29:52Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpXbB2sR#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpXbB2sR/incremental-state"
[2022-09-21T19:30:12Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-09-21T19:30:13Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-09-21T19:30:13Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpGb9Boz#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2022-09-21T19:31:20Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-09-21T19:31:20Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-09-21T19:31:20Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpGb9Boz#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpGb9Boz/incremental-state"
[2022-09-21T19:32:24Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-09-21T19:32:24Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpGb9Boz#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpGb9Boz/incremental-state"
[2022-09-21T19:32:34Z DEBUG collector::execute] applying println to "/tmp/.tmpGb9Boz"
[2022-09-21T19:32:34Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-09-21T19:32:34Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-09-21T19:32:34Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpGb9Boz#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpGb9Boz/incremental-state"
Executing benchmark ctfe-stress-5 (3/8)
Preparing ctfe-stress-5
[2022-09-21T19:32:57Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-09-21T19:32:57Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
---
[2022-09-21T19:33:16Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-09-21T19:33:16Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-09-21T19:33:16Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpUVZmJS#ctfe-stress-5@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-09-21T19:33:25Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-09-21T19:33:25Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpUVZmJS#ctfe-stress-5@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpUVZmJS/incremental-state"
[2022-09-21T19:33:36Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-09-21T19:33:36Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpUVZmJS#ctfe-stress-5@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpUVZmJS/incremental-state"
[2022-09-21T19:33:37Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-09-21T19:33:37Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-09-21T19:33:37Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwZPBAf#ctfe-stress-5@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-09-21T19:33:45Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-09-21T19:33:45Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-09-21T19:33:45Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwZPBAf#ctfe-stress-5@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpwZPBAf/incremental-state"
[2022-09-21T19:33:55Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-09-21T19:33:55Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwZPBAf#ctfe-stress-5@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpwZPBAf/incremental-state"
Executing benchmark diesel-1.4.8 (4/8)
Preparing diesel-1.4.8
[2022-09-21T19:33:56Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-09-21T19:33:56Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
---
[2022-09-21T19:35:00Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-09-21T19:35:00Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-09-21T19:35:00Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp0NEuLp#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln"
[2022-09-21T19:35:18Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-09-21T19:35:18Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp0NEuLp#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp0NEuLp/incremental-state"
[2022-09-21T19:35:41Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-09-21T19:35:41Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp0NEuLp#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp0NEuLp/incremental-state"
[2022-09-21T19:35:45Z DEBUG collector::execute] applying println to "/tmp/.tmp0NEuLp"
[2022-09-21T19:35:45Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-09-21T19:35:45Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-09-21T19:35:45Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp0NEuLp#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp0NEuLp/incremental-state"
[2022-09-21T19:35:49Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-09-21T19:35:49Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-09-21T19:35:49Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpco4Dcz#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-09-21T19:36:07Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-09-21T19:36:07Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-09-21T19:36:07Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpco4Dcz#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpco4Dcz/incremental-state"
[2022-09-21T19:36:30Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-09-21T19:36:30Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpco4Dcz#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpco4Dcz/incremental-state"
[2022-09-21T19:36:34Z DEBUG collector::execute] applying println to "/tmp/.tmpco4Dcz"
[2022-09-21T19:36:34Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-09-21T19:36:34Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-09-21T19:36:34Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpco4Dcz#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpco4Dcz/incremental-state"
Executing benchmark externs (5/8)
Preparing externs
[2022-09-21T19:36:38Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-09-21T19:36:38Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
---
[2022-09-21T19:36:39Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-09-21T19:36:39Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-09-21T19:36:39Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpWC9LXt#externs@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-09-21T19:36:40Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-09-21T19:36:40Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpWC9LXt#externs@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpWC9LXt/incremental-state"
[2022-09-21T19:36:40Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-09-21T19:36:40Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpWC9LXt#externs@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpWC9LXt/incremental-state"
[2022-09-21T19:36:41Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-09-21T19:36:41Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-09-21T19:36:41Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmptw8a91#externs@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-09-21T19:36:42Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
---
[2022-09-21T19:36:43Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-09-21T19:36:43Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-09-21T19:36:43Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpkANT8n#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-09-21T19:36:44Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-09-21T19:36:44Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpkANT8n#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpkANT8n/incremental-state"
[2022-09-21T19:36:45Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-09-21T19:36:45Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpkANT8n#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpkANT8n/incremental-state"
Executing benchmark match-stress (6/8)
Preparing match-stress
[2022-09-21T19:36:46Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-09-21T19:36:46Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
---
[2022-09-21T19:36:46Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-09-21T19:36:46Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-09-21T19:36:46Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpoD3Yxr#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-09-21T19:36:49Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-09-21T19:36:49Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpoD3Yxr#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpoD3Yxr/incremental-state"
[2022-09-21T19:36:52Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-09-21T19:36:52Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpoD3Yxr#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpoD3Yxr/incremental-state"
[2022-09-21T19:36:53Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-09-21T19:36:53Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-09-21T19:36:53Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpXNWpVk#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-09-21T19:36:56Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-09-21T19:36:56Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-09-21T19:36:56Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpXNWpVk#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpXNWpVk/incremental-state"
[2022-09-21T19:36:59Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-09-21T19:36:59Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpXNWpVk#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpXNWpVk/incremental-state"
[2022-09-21T19:37:01Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-09-21T19:37:01Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-09-21T19:37:01Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp8uCl4M#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-09-21T19:37:03Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
[2022-09-21T19:37:10Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-09-21T19:37:10Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-09-21T19:37:10Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKgzara#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2022-09-21T19:37:10Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-09-21T19:37:10Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKgzara#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpKgzara/incremental-state"
[2022-09-21T19:37:10Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-09-21T19:37:10Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKgzara#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpKgzara/incremental-state"
[2022-09-21T19:37:11Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-09-21T19:37:11Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-09-21T19:37:11Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpRNs1P5#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2022-09-21T19:37:11Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
[2022-09-21T19:37:12Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-09-21T19:37:12Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-09-21T19:37:12Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpvR5rPa#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-09-21T19:37:18Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-09-21T19:37:18Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpvR5rPa#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpvR5rPa/incremental-state"
[2022-09-21T19:37:25Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-09-21T19:37:25Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpvR5rPa#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpvR5rPa/incremental-state"
[2022-09-21T19:37:27Z DEBUG collector::execute] applying new row to "/tmp/.tmpvR5rPa"
[2022-09-21T19:37:27Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-09-21T19:37:27Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-09-21T19:37:27Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpvR5rPa#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpvR5rPa/incremental-state"
[2022-09-21T19:37:34Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-09-21T19:37:34Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-09-21T19:37:34Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp77Dmrh#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-09-21T19:37:39Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
---
[2022-09-21T19:37:55Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-09-21T19:37:55Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-09-21T19:37:55Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmptKTJuO#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-09-21T19:38:00Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-09-21T19:38:00Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmptKTJuO#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmptKTJuO/incremental-state"
[2022-09-21T19:38:07Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-09-21T19:38:07Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmptKTJuO#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmptKTJuO/incremental-state"
[2022-09-21T19:38:09Z DEBUG collector::execute] applying new row to "/tmp/.tmptKTJuO"
[2022-09-21T19:38:09Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-09-21T19:38:09Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-09-21T19:38:09Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmptKTJuO#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmptKTJuO/incremental-state"
+ cd /checkout/obj
+ RUSTC_PROFILE_MERGED_FILE=/tmp/tmp-pgo/rustc-pgo.profdata
+ /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-profdata merge -o /tmp/tmp-pgo/rustc-pgo.profdata /tmp/tmp-pgo/rustc-pgo
+ echo 'Rustc PGO statistics'
---
[RUSTC-TIMING] unicode_xid test:false 0.128
   Compiling rustc_fs_util v0.0.0 (/checkout/compiler/rustc_fs_util)
[RUSTC-TIMING] self_cell test:false 0.079
   Compiling either v1.6.0
warning: rustc_graphviz.577eeeca-cgu.0: no profile data available for function _RNvMNtNtNtCskeEKH0oMbfI_4core4iter8adapters3revINtB2_3RevINtNtNtB8_3ops5range5RangejEE3newCsaOG5edl3jVm_14rustc_graphviz Hash = 742261418966908927 up to 0 count discarded
[RUSTC-TIMING] lazy_static test:false 0.089
   Compiling datafrog v2.0.1
[RUSTC-TIMING] build_script_build test:false 0.299
   Compiling memchr v2.5.0
---
   Compiling object v0.29.0
   Compiling gimli v0.26.1
[RUSTC-TIMING] regex_automata test:false 3.472
[RUSTC-TIMING] tempfile test:false 0.714
warning: rustc_serialize.01b8a8a2-cgu.7: no profile data available for function _RINvNtCskeEKH0oMbfI_4core3ptr13drop_in_placeRjECsfGZopAGsNyw_15rustc_serialize Hash = 742261418966908927 up to 0 count discarded

warning: rustc_serialize.01b8a8a2-cgu.7: no profile data available for function _RINvNtCskeEKH0oMbfI_4core9panicking13assert_failedjjECsfGZopAGsNyw_15rustc_serialize Hash = 742261418966908927 up to 0 count discarded

warning: rustc_serialize.01b8a8a2-cgu.10: no profile data available for function _RNvXsV_NtCskeEKH0oMbfI_4core3fmtRjNtB5_5Debug3fmtCsfGZopAGsNyw_15rustc_serialize Hash = 1124680650125156080 up to 0 count discarded
[RUSTC-TIMING] rustc_serialize test:false 0.671
warning: `rustc_serialize` (lib) generated 3 warnings
[RUSTC-TIMING] regex_syntax test:false 7.609
[RUSTC-TIMING] sha2 test:false 1.570
---
[RUSTC-TIMING] git2_curl test:false 0.591
[RUSTC-TIMING] git2 test:false 5.532
[RUSTC-TIMING] toml_edit test:false 40.427
[RUSTC-TIMING] cargo test:false 48.370
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', compiler/rustc_monomorphize/src/collector.rs:940:13
   0:     0x7f5d8796c8b0 - std::backtrace_rs::backtrace::libunwind::trace::hd86347086d4d9848
                               at /rustc/692c0be0a784dcddfc83df99a6f9bf48d56f7dab/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   1:     0x7f5d8796c8b0 - std::backtrace_rs::backtrace::trace_unsynchronized::h471f843ea3342d4e
                               at /rustc/692c0be0a784dcddfc83df99a6f9bf48d56f7dab/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
                               at /rustc/692c0be0a784dcddfc83df99a6f9bf48d56f7dab/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f5d8796c8b0 - std::sys_common::backtrace::_print_fmt::ha5b71df9f208c9e1
                               at /rustc/692c0be0a784dcddfc83df99a6f9bf48d56f7dab/library/std/src/sys_common/backtrace.rs:66:5
   3:     0x7f5d8796c8b0 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h4f8935cce220fb7c
                               at /rustc/692c0be0a784dcddfc83df99a6f9bf48d56f7dab/library/std/src/sys_common/backtrace.rs:45:22
   4:     0x7f5d879cba5c - core::fmt::write::hf970a4d7eb447230
                               at /rustc/692c0be0a784dcddfc83df99a6f9bf48d56f7dab/library/core/src/fmt/mod.rs:1202:17
   5:     0x7f5d8795cf95 - std::io::Write::write_fmt::ha54260ad7296009e
                               at /rustc/692c0be0a784dcddfc83df99a6f9bf48d56f7dab/library/std/src/io/mod.rs:1679:15
   6:     0x7f5d8796f7e1 - std::sys_common::backtrace::_print::hdfdcc8a9593c812b
                               at /rustc/692c0be0a784dcddfc83df99a6f9bf48d56f7dab/library/std/src/sys_common/backtrace.rs:48:5
   7:     0x7f5d8796f7e1 - std::sys_common::backtrace::print::h4fc4cbbeedef9395
                               at /rustc/692c0be0a784dcddfc83df99a6f9bf48d56f7dab/library/std/src/sys_common/backtrace.rs:35:9
   8:     0x7f5d8796f7e1 - std::panicking::default_hook::{{closure}}::h9e815e095d29d239
                               at /rustc/692c0be0a784dcddfc83df99a6f9bf48d56f7dab/library/std/src/panicking.rs:267:22
   9:     0x7f5d8796f49e - std::panicking::default_hook::hc19acee535433356
                               at /rustc/692c0be0a784dcddfc83df99a6f9bf48d56f7dab/library/std/src/panicking.rs:286:9
  10:     0x7f5d849362b6 - rustc_driver[e88ab1e0ee5011ab]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7f5d87970058 - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::hdc766bdbadd67bed
                               at /rustc/692c0be0a784dcddfc83df99a6f9bf48d56f7dab/library/alloc/src/boxed.rs:1954:9
  12:     0x7f5d87970058 - std::panicking::rust_panic_with_hook::hbe4a665e90dad3db
                               at /rustc/692c0be0a784dcddfc83df99a6f9bf48d56f7dab/library/std/src/panicking.rs:673:13
  13:     0x7f5d8796fe59 - std::panicking::begin_panic_handler::{{closure}}::h1f0acd1fe5441dca
                               at /rustc/692c0be0a784dcddfc83df99a6f9bf48d56f7dab/library/std/src/panicking.rs:558:13
  14:     0x7f5d8796cdd4 - std::sys_common::backtrace::__rust_end_short_backtrace::hca6b38c154bab897
                               at /rustc/692c0be0a784dcddfc83df99a6f9bf48d56f7dab/library/std/src/sys_common/backtrace.rs:138:18
  15:     0x7f5d8796fb92 - rust_begin_unwind
                               at /rustc/692c0be0a784dcddfc83df99a6f9bf48d56f7dab/library/std/src/panicking.rs:556:5
  16:     0x7f5d879c85f3 - core::panicking::panic_fmt::h7574974d27e60ac2
                               at /rustc/692c0be0a784dcddfc83df99a6f9bf48d56f7dab/library/core/src/panicking.rs:142:14
  17:     0x7f5d879c843d - core::panicking::panic::h3d77d8ba5e42b73d
                               at /rustc/692c0be0a784dcddfc83df99a6f9bf48d56f7dab/library/core/src/panicking.rs:48:5
  18:     0x7f5d84db1b92 - <rustc_monomorphize[b6fb6c5a7eb33ed1]::collector::MirNeighborCollector as rustc_middle[c82706f5e99477f9]::mir::visit::Visitor>::visit_terminator
  19:     0x7f5d84db7e71 - rustc_monomorphize[b6fb6c5a7eb33ed1]::collector::collect_neighbours
  20:     0x7f5d84db58eb - rustc_monomorphize[b6fb6c5a7eb33ed1]::collector::collect_items_rec
  21:     0x7f5d84db600d - rustc_monomorphize[b6fb6c5a7eb33ed1]::collector::collect_items_rec
  22:     0x7f5d84db600d - rustc_monomorphize[b6fb6c5a7eb33ed1]::collector::collect_items_rec
  23:     0x7f5d84db600d - rustc_monomorphize[b6fb6c5a7eb33ed1]::collector::collect_items_rec
  24:     0x7f5d84db600d - rustc_monomorphize[b6fb6c5a7eb33ed1]::collector::collect_items_rec
  25:     0x7f5d84db600d - rustc_monomorphize[b6fb6c5a7eb33ed1]::collector::collect_items_rec
  26:     0x7f5d84deee4d - <core[2bb8d0aa17313d7]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[75a2095c119064ec]::sync::par_for_each_in<alloc[50cd12edb6646b8b]::vec::Vec<rustc_middle[c82706f5e99477f9]::mir::mono::MonoItem>, rustc_monomorphize[b6fb6c5a7eb33ed1]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}> as core[2bb8d0aa17313d7]::ops::function::FnOnce<()>>::call_once
  27:     0x7f5d84dea715 - std[734a5d07e7dea0b5]::panicking::try::<(), core[2bb8d0aa17313d7]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[75a2095c119064ec]::sync::par_for_each_in<alloc[50cd12edb6646b8b]::vec::Vec<rustc_middle[c82706f5e99477f9]::mir::mono::MonoItem>, rustc_monomorphize[b6fb6c5a7eb33ed1]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}>>
  28:     0x7f5d84dcdbb1 - <rustc_session[860b62581d5940c8]::session::Session>::time::<(), rustc_monomorphize[b6fb6c5a7eb33ed1]::collector::collect_crate_mono_items::{closure#1}>
  29:     0x7f5d84db3712 - rustc_monomorphize[b6fb6c5a7eb33ed1]::collector::collect_crate_mono_items
  30:     0x7f5d84dc9065 - rustc_monomorphize[b6fb6c5a7eb33ed1]::partitioning::collect_and_partition_mono_items
  31:     0x7f5d86147b5d - rustc_query_system[ff407481ba3fdf0a]::query::plumbing::try_execute_query::<rustc_query_impl[352f51d90f435fc8]::plumbing::QueryCtxt, rustc_query_system[ff407481ba3fdf0a]::query::caches::DefaultCache<(), (&std[734a5d07e7dea0b5]::collections::hash::set::HashSet<rustc_span[834d17fc4ad3406c]::def_id::DefId, core[2bb8d0aa17313d7]::hash::BuildHasherDefault<rustc_hash[5034897b462db3a3]::FxHasher>>, &[rustc_middle[c82706f5e99477f9]::mir::mono::CodegenUnit])>>
  32:     0x7f5d861d86ce - rustc_query_system[ff407481ba3fdf0a]::query::plumbing::get_query::<rustc_query_impl[352f51d90f435fc8]::queries::collect_and_partition_mono_items, rustc_query_impl[352f51d90f435fc8]::plumbing::QueryCtxt>
  33:     0x7f5d8639f0bb - <rustc_query_impl[352f51d90f435fc8]::Queries as rustc_middle[c82706f5e99477f9]::ty::query::QueryEngine>::collect_and_partition_mono_items
  34:     0x7f5d84bf2f37 - rustc_codegen_ssa[8a38c4617f0ae3b7]::base::codegen_crate::<rustc_codegen_llvm[e64b4b2756031fdd]::LlvmCodegenBackend>
  35:     0x7f5d84ca62ed - <rustc_codegen_llvm[e64b4b2756031fdd]::LlvmCodegenBackend as rustc_codegen_ssa[8a38c4617f0ae3b7]::traits::backend::CodegenBackend>::codegen_crate
  36:     0x7f5d84ab3295 - <rustc_interface[af109c0bf5e7882b]::passes::QueryContext>::enter::<<rustc_interface[af109c0bf5e7882b]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[2bb8d0aa17313d7]::result::Result<alloc[50cd12edb6646b8b]::boxed::Box<dyn core[2bb8d0aa17313d7]::any::Any>, rustc_errors[f490fc5d2ab7927a]::ErrorGuaranteed>>
  37:     0x7f5d84a9d51b - <rustc_interface[af109c0bf5e7882b]::queries::Queries>::ongoing_codegen
  38:     0x7f5d84969bf0 - rustc_interface[af109c0bf5e7882b]::interface::create_compiler_and_run::<core[2bb8d0aa17313d7]::result::Result<(), rustc_errors[f490fc5d2ab7927a]::ErrorGuaranteed>, rustc_driver[e88ab1e0ee5011ab]::run_compiler::{closure#1}>
  39:     0x7f5d849d9db2 - <scoped_tls[da5adb31d26bd224]::ScopedKey<rustc_span[834d17fc4ad3406c]::SessionGlobals>>::set::<rustc_interface[af109c0bf5e7882b]::interface::run_compiler<core[2bb8d0aa17313d7]::result::Result<(), rustc_errors[f490fc5d2ab7927a]::ErrorGuaranteed>, rustc_driver[e88ab1e0ee5011ab]::run_compiler::{closure#1}>::{closure#0}, core[2bb8d0aa17313d7]::result::Result<(), rustc_errors[f490fc5d2ab7927a]::ErrorGuaranteed>>
  40:     0x7f5d8498a11f - std[734a5d07e7dea0b5]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[af109c0bf5e7882b]::util::run_in_thread_pool_with_globals<rustc_interface[af109c0bf5e7882b]::interface::run_compiler<core[2bb8d0aa17313d7]::result::Result<(), rustc_errors[f490fc5d2ab7927a]::ErrorGuaranteed>, rustc_driver[e88ab1e0ee5011ab]::run_compiler::{closure#1}>::{closure#0}, core[2bb8d0aa17313d7]::result::Result<(), rustc_errors[f490fc5d2ab7927a]::ErrorGuaranteed>>::{closure#0}, core[2bb8d0aa17313d7]::result::Result<(), rustc_errors[f490fc5d2ab7927a]::ErrorGuaranteed>>
  41:     0x7f5d8498b339 - <<std[734a5d07e7dea0b5]::thread::Builder>::spawn_unchecked_<rustc_interface[af109c0bf5e7882b]::util::run_in_thread_pool_with_globals<rustc_interface[af109c0bf5e7882b]::interface::run_compiler<core[2bb8d0aa17313d7]::result::Result<(), rustc_errors[f490fc5d2ab7927a]::ErrorGuaranteed>, rustc_driver[e88ab1e0ee5011ab]::run_compiler::{closure#1}>::{closure#0}, core[2bb8d0aa17313d7]::result::Result<(), rustc_errors[f490fc5d2ab7927a]::ErrorGuaranteed>>::{closure#0}, core[2bb8d0aa17313d7]::result::Result<(), rustc_errors[f490fc5d2ab7927a]::ErrorGuaranteed>>::{closure#1} as core[2bb8d0aa17313d7]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  42:     0x7f5d8797a563 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hb278eb53bcad769e
                               at /rustc/692c0be0a784dcddfc83df99a6f9bf48d56f7dab/library/alloc/src/boxed.rs:1940:9
  43:     0x7f5d8797a563 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hf56f5a379975cbe0
                               at /rustc/692c0be0a784dcddfc83df99a6f9bf48d56f7dab/library/alloc/src/boxed.rs:1940:9
  44:     0x7f5d8797a563 - std::sys::unix::thread::Thread::new::thread_start::h5bbd14dda35b01a2
                               at /rustc/692c0be0a784dcddfc83df99a6f9bf48d56f7dab/library/std/src/sys/unix/thread.rs:108:17
  45:     0x7f5d83112ea5 - start_thread
  46:     0x7f5d82e3bb0d - clone
  47:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.66.0-nightly (692c0be0a 2022-09-21) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type bin -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -Z unstable-options -C linker=clang -C symbol-mangling-version=v0 -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C link-args=-fuse-ld=lld -C split-debuginfo=off -Z binary-dep-depinfo -Z tls-model=initial-exec
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [collect_and_partition_mono_items] collect_and_partition_mono_items
[RUSTC-TIMING] cargo test:false 0.788
error: could not compile `cargo`
Build completed unsuccessfully in 0:31:38
