plain
[2022-05-28T20:03:35Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOBdoGS#serde:1.0.136" "--release" "--" "--skip-this-rustc"
Running serde-1.0.136: Debug + [Full]
[2022-05-28T20:03:36Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-28T20:03:36Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-28T20:03:36Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpuKYciE#serde:1.0.136" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-28T20:03:40Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-28T20:03:40Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-28T20:03:40Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmputjk1N#serde:1.0.136" "--release" "--" "--wrap-rustc-with" "Eprintln"
Finished benchmark serde-1.0.136 (6/7)
---
Preparing bitmaps-3.1.0
[2022-05-28T20:17:57Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-05-28T20:17:57Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-28T20:17:57Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-28T20:17:57Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpHKdMyM#bitmaps:3.1.0" "--" "--skip-this-rustc"
[2022-05-28T20:17:57Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpngY87t#bitmaps:3.1.0" "--profile" "check" "--" "--skip-this-rustc"
[2022-05-28T20:17:57Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKcWcS2#bitmaps:3.1.0" "--release" "--" "--skip-this-rustc"
[2022-05-28T20:17:57Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-28T20:17:57Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-05-28T20:17:57Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-05-28T20:17:58Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpevO7Vm#bitmaps:3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-28T20:18:00Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-05-28T20:18:00Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpevO7Vm#bitmaps:3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpevO7Vm/incremental-state"
[2022-05-28T20:18:03Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-05-28T20:18:03Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpevO7Vm#bitmaps:3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpevO7Vm/incremental-state"
[2022-05-28T20:18:04Z DEBUG collector::execute] applying println to "/tmp/.tmpevO7Vm"
[2022-05-28T20:18:04Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-28T20:18:04Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-28T20:18:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpevO7Vm#bitmaps:3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpevO7Vm/incremental-state"
[2022-05-28T20:18:05Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-28T20:18:05Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-28T20:18:05Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCJO5LW#bitmaps:3.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-28T20:18:08Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-28T20:18:08Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-28T20:18:08Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCJO5LW#bitmaps:3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpCJO5LW/incremental-state"
[2022-05-28T20:18:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-05-28T20:18:12Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCJO5LW#bitmaps:3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpCJO5LW/incremental-state"
[2022-05-28T20:18:13Z DEBUG collector::execute] applying println to "/tmp/.tmpCJO5LW"
[2022-05-28T20:18:13Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-28T20:18:13Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-28T20:18:13Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCJO5LW#bitmaps:3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpCJO5LW/incremental-state"
[2022-05-28T20:18:14Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-28T20:18:14Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-28T20:18:14Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpWa3Gft#bitmaps:3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-28T20:18:16Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-28T20:18:16Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-28T20:18:16Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpWa3Gft#bitmaps:3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpWa3Gft/incremental-state"
[2022-05-28T20:18:20Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-05-28T20:18:20Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpWa3Gft#bitmaps:3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpWa3Gft/incremental-state"
[2022-05-28T20:18:21Z DEBUG collector::execute] applying println to "/tmp/.tmpWa3Gft"
[2022-05-28T20:18:21Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-28T20:18:21Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-28T20:18:21Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpWa3Gft#bitmaps:3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpWa3Gft/incremental-state"
Executing benchmark cargo-0.60.0 (2/8)
Preparing cargo-0.60.0
[2022-05-28T20:18:22Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-05-28T20:18:22Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2022-05-28T20:19:36Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-28T20:19:37Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-05-28T20:19:37Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpUSSPAW#cargo:0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-28T20:20:00Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-05-28T20:20:00Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpUSSPAW#cargo:0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpUSSPAW/incremental-state"
[2022-05-28T20:20:28Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-05-28T20:20:28Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpUSSPAW#cargo:0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpUSSPAW/incremental-state"
[2022-05-28T20:20:33Z DEBUG collector::execute] applying println to "/tmp/.tmpUSSPAW"
[2022-05-28T20:20:33Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-28T20:20:33Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-28T20:20:33Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpUSSPAW#cargo:0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpUSSPAW/incremental-state"
[2022-05-28T20:20:45Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-28T20:20:45Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-28T20:20:45Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmplJFG2q#cargo:0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-28T20:21:41Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-28T20:21:41Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-28T20:21:41Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmplJFG2q#cargo:0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmplJFG2q/incremental-state"
[2022-05-28T20:22:50Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-05-28T20:22:50Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmplJFG2q#cargo:0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmplJFG2q/incremental-state"
[2022-05-28T20:23:02Z DEBUG collector::execute] applying println to "/tmp/.tmplJFG2q"
[2022-05-28T20:23:02Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-28T20:23:02Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-28T20:23:02Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmplJFG2q#cargo:0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmplJFG2q/incremental-state"
[2022-05-28T20:23:28Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-28T20:23:28Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-28T20:23:28Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpSyFyZN#cargo:0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-28T20:24:42Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-28T20:24:42Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-28T20:24:42Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpSyFyZN#cargo:0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpSyFyZN/incremental-state"
[2022-05-28T20:25:58Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-05-28T20:25:58Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpSyFyZN#cargo:0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpSyFyZN/incremental-state"
[2022-05-28T20:26:10Z DEBUG collector::execute] applying println to "/tmp/.tmpSyFyZN"
[2022-05-28T20:26:10Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-28T20:26:10Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-28T20:26:10Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpSyFyZN#cargo:0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpSyFyZN/incremental-state"
Executing benchmark ctfe-stress-5 (3/8)
Preparing ctfe-stress-5
[2022-05-28T20:26:43Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-28T20:26:43Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
---
[2022-05-28T20:26:44Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-28T20:26:44Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-05-28T20:26:44Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmptF3RhM#ctfe-stress-5:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-28T20:26:52Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-05-28T20:26:52Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmptF3RhM#ctfe-stress-5:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmptF3RhM/incremental-state"
[2022-05-28T20:27:02Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-05-28T20:27:02Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmptF3RhM#ctfe-stress-5:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmptF3RhM/incremental-state"
[2022-05-28T20:27:03Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-28T20:27:03Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-28T20:27:03Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpBVBw1M#ctfe-stress-5:0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-28T20:27:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-28T20:27:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-28T20:27:12Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpBVBw1M#ctfe-stress-5:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpBVBw1M/incremental-state"
[2022-05-28T20:27:22Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-05-28T20:27:22Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpBVBw1M#ctfe-stress-5:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpBVBw1M/incremental-state"
[2022-05-28T20:27:22Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-28T20:27:22Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-28T20:27:22Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpih9f8W#ctfe-stress-5:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-28T20:27:31Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
[2022-05-28T20:28:52Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-28T20:28:52Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-28T20:28:52Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpuaBU8n#diesel:1.4.8" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-28T20:29:13Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-28T20:29:13Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpuaBU8n#diesel:1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpuaBU8n/incremental-state"
[2022-05-28T20:29:38Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-05-28T20:29:38Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpuaBU8n#diesel:1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpuaBU8n/incremental-state"
[2022-05-28T20:29:42Z DEBUG collector::execute] applying println to "/tmp/.tmpuaBU8n"
[2022-05-28T20:29:42Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-28T20:29:42Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-28T20:29:42Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpuaBU8n#diesel:1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpuaBU8n/incremental-state"
[2022-05-28T20:29:47Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-28T20:29:47Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-28T20:29:47Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpTxqhG3#diesel:1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-28T20:30:07Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-28T20:30:07Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-28T20:30:07Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpTxqhG3#diesel:1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpTxqhG3/incremental-state"
[2022-05-28T20:30:32Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-05-28T20:30:32Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpTxqhG3#diesel:1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpTxqhG3/incremental-state"
[2022-05-28T20:30:36Z DEBUG collector::execute] applying println to "/tmp/.tmpTxqhG3"
[2022-05-28T20:30:36Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-28T20:30:36Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-28T20:30:36Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpTxqhG3#diesel:1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpTxqhG3/incremental-state"
Executing benchmark externs (5/8)
Preparing externs
[2022-05-28T20:30:41Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-05-28T20:30:41Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2022-05-28T20:30:43Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-28T20:30:43Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-28T20:30:43Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmphwo9sa#externs:0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-28T20:30:44Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-28T20:30:44Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmphwo9sa#externs:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmphwo9sa/incremental-state"
[2022-05-28T20:30:46Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-05-28T20:30:46Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmphwo9sa#externs:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmphwo9sa/incremental-state"
[2022-05-28T20:30:46Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-28T20:30:46Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-28T20:30:46Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpxMsJeY#externs:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-28T20:30:47Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-28T20:30:47Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-28T20:30:47Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpxMsJeY#externs:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpxMsJeY/incremental-state"
[2022-05-28T20:30:48Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-05-28T20:30:48Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpxMsJeY#externs:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpxMsJeY/incremental-state"
Executing benchmark match-stress (6/8)
Preparing match-stress
[2022-05-28T20:30:49Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-05-28T20:30:49Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2022-05-28T20:30:58Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-28T20:30:58Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-28T20:30:58Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp1pvJKA#match-stress:0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-28T20:31:02Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-28T20:31:02Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp1pvJKA#match-stress:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp1pvJKA/incremental-state"
[2022-05-28T20:31:06Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-05-28T20:31:06Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp1pvJKA#match-stress:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp1pvJKA/incremental-state"
[2022-05-28T20:31:07Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-28T20:31:07Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-28T20:31:07Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpeCio18#match-stress:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-28T20:31:11Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
[2022-05-28T20:31:17Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-28T20:31:17Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-05-28T20:31:17Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDSrsdh#token-stream-stress:0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-28T20:31:17Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-05-28T20:31:17Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDSrsdh#token-stream-stress:0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpDSrsdh/incremental-state"
[2022-05-28T20:31:18Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-05-28T20:31:18Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDSrsdh#token-stream-stress:0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpDSrsdh/incremental-state"
[2022-05-28T20:31:18Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-28T20:31:18Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-28T20:31:18Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpO8Cn2j#token-stream-stress:0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-28T20:31:18Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
---
[2022-05-28T20:31:19Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-28T20:31:19Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-28T20:31:19Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpcyjl6t#token-stream-stress:0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-28T20:31:19Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-28T20:31:19Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpcyjl6t#token-stream-stress:0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpcyjl6t/incremental-state"
[2022-05-28T20:31:19Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-05-28T20:31:20Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpcyjl6t#token-stream-stress:0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpcyjl6t/incremental-state"
Executing benchmark tuple-stress (8/8)
Preparing tuple-stress
[2022-05-28T20:31:20Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-28T20:31:20Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
---
[2022-05-28T20:31:39Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-28T20:31:39Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-28T20:31:39Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpGwWxeR#tuple-stress:0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-28T20:31:44Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-28T20:31:44Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpGwWxeR#tuple-stress:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpGwWxeR/incremental-state"
[2022-05-28T20:31:50Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-05-28T20:31:50Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpGwWxeR#tuple-stress:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpGwWxeR/incremental-state"
[2022-05-28T20:31:52Z DEBUG collector::execute] applying new row to "/tmp/.tmpGwWxeR"
[2022-05-28T20:31:52Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-05-28T20:31:52Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-05-28T20:31:52Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpGwWxeR#tuple-stress:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpGwWxeR/incremental-state"
[2022-05-28T20:31:59Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-28T20:31:59Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-28T20:31:59Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpZlLOOI#tuple-stress:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-28T20:32:04Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-28T20:32:04Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-28T20:32:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpZlLOOI#tuple-stress:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpZlLOOI/incremental-state"
[2022-05-28T20:32:10Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-05-28T20:32:10Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpZlLOOI#tuple-stress:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpZlLOOI/incremental-state"
[2022-05-28T20:32:12Z DEBUG collector::execute] applying new row to "/tmp/.tmpZlLOOI"
[2022-05-28T20:32:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-05-28T20:32:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-05-28T20:32:12Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpZlLOOI#tuple-stress:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpZlLOOI/incremental-state"
+ cd /checkout/obj
+ ./build/x86_64-unknown-linux-gnu/llvm/bin/llvm-profdata merge -o /tmp/rustc-pgo.profdata /tmp/rustc-pgo
+ rm -r ./build/x86_64-unknown-linux-gnu/llvm ./build/x86_64-unknown-linux-gnu/lld
+ python3 ../x.py dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu --include-default-paths build-manifest --rust-profile-use=/tmp/rustc-pgo.profdata --llvm-profile-use=/tmp/llvm-pgo.profdata
---
[RUSTC-TIMING] autocfg test:false 0.649
   Compiling memchr v2.4.1
[RUSTC-TIMING] arrayvec test:false 0.297
   Compiling regex-syntax v0.6.25
warning: rustc_fs_util.55615e79-cgu.4: no profile data available for function _RNvXs2_NtCsgmcG5dXCadN_4core7convertNtNtNtB7_5alloc6layout11LayoutErrorINtB5_4FromBy_E4fromCsbiuByCJqDUb_13rustc_fs_util Hash = 742261418966908927
[RUSTC-TIMING] unic_char_range test:false 0.194
   Compiling tinyvec v0.3.4
[RUSTC-TIMING] rustc_fs_util test:false 0.137
warning: `rustc_fs_util` (lib) generated 1 warning
warning: `rustc_fs_util` (lib) generated 1 warning
   Compiling ansi_term v0.12.1
warning: rustc_graphviz.88c63282-cgu.0: no profile data available for function _RNvXs2_NtCsgmcG5dXCadN_4core7convertNtNtNtB7_5alloc6layout11LayoutErrorINtB5_4FromBy_E4fromCsaK8dMtlh43e_14rustc_graphviz Hash = 742261418966908927
[RUSTC-TIMING] either test:false 0.195
   Compiling snap v1.0.1
   Compiling snap v1.0.1
warning: rustc_graphviz.88c63282-cgu.12: no profile data available for function _RNvMNtNtNtCsgmcG5dXCadN_4core4iter8adapters3revINtB2_3RevINtNtNtB8_3ops5range5RangejEE3newCsaK8dMtlh43e_14rustc_graphviz Hash = 742261418966908927
[RUSTC-TIMING] ppv_lite86 test:false 0.599
   Compiling crc32fast v1.2.0
[RUSTC-TIMING] tinystr test:false 0.583
   Compiling adler v0.2.3
---
[RUSTC-TIMING] ahash test:false 0.434
   Compiling matchers v0.1.0
[RUSTC-TIMING] rand_core test:false 0.440
   Compiling rand_chacha v0.3.0
warning: rustc_llvm.9a478636-cgu.2: no profile data available for function _RNvXs2_NtCsgmcG5dXCadN_4core7convertNtNtNtB7_5alloc6layout11LayoutErrorINtB5_4FromBy_E4fromCs5A7fH253dfC_10rustc_llvm Hash = 742261418966908927
[RUSTC-TIMING] matchers test:false 0.168
   Compiling rand_xoshiro v0.6.0
[RUSTC-TIMING] rustc_llvm test:false 0.274
warning: `rustc_llvm` (lib) generated 1 warning
---
   Compiling rustc_ast v0.0.0 (/checkout/compiler/rustc_ast)
   Compiling rustc_feature v0.0.0 (/checkout/compiler/rustc_feature)
   Compiling rustc_target v0.0.0 (/checkout/compiler/rustc_target)
   Compiling rustc_error_messages v0.0.0 (/checkout/compiler/rustc_error_messages)
warning: rustc_error_messages.08a1a549-cgu.11: no profile data available for function _RINvMsx_NtCs4MetWwXVPrY_12tracing_core5fieldNtB6_8FieldSet9value_setATRNtB6_5FieldINtNtCsgmcG5dXCadN_4core6option6OptionRDNtB6_5ValueEL_EEj1_ECsgpkREHUotZJ_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.08a1a549-cgu.11: no profile data available for function _RINvMsx_NtCs4MetWwXVPrY_12tracing_core5fieldNtB6_8FieldSet9value_setATRNtB6_5FieldINtNtCsgmcG5dXCadN_4core6option6OptionRDNtB6_5ValueEL_EEj2_ECsgpkREHUotZJ_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.08a1a549-cgu.11: no profile data available for function _RINvMsx_NtCs4MetWwXVPrY_12tracing_core5fieldNtB6_8FieldSet9value_setATRNtB6_5FieldINtNtCsgmcG5dXCadN_4core6option6OptionRDNtB6_5ValueEL_EEj5_ECsgpkREHUotZJ_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.08a1a549-cgu.11: no profile data available for function _RINvNtCs4MetWwXVPrY_12tracing_core5field5debugRINtNtCs5IAtDAEPLL0_5alloc3vec3VecNtNtCshFOMELpAQPt_3std4path7PathBufEECsgpkREHUotZJ_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.08a1a549-cgu.11: no profile data available for function _RINvNtCs4MetWwXVPrY_12tracing_core5field5debugRINtNtCsgmcG5dXCadN_4core6option6OptionNtCsZvZt4fBMkl_16unic_langid_impl18LanguageIdentifierEECsgpkREHUotZJ_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.08a1a549-cgu.11: no profile data available for function _RINvNtCs4MetWwXVPrY_12tracing_core5field5debugRINtNtCsgmcG5dXCadN_4core6option6OptionNtNtCshFOMELpAQPt_3std4path7PathBufEECsgpkREHUotZJ_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.08a1a549-cgu.11: no profile data available for function _RINvNtCs4MetWwXVPrY_12tracing_core5field5debugRINtNtCsgmcG5dXCadN_4core6option6OptionRNtNtCshFOMELpAQPt_3std4path4PathEECsgpkREHUotZJ_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.08a1a549-cgu.11: no profile data available for function _RINvNtCs4MetWwXVPrY_12tracing_core5field5debugRNtCsZvZt4fBMkl_16unic_langid_impl18LanguageIdentifierECsgpkREHUotZJ_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.08a1a549-cgu.11: no profile data available for function _RINvNtCs4MetWwXVPrY_12tracing_core5field5debugRNtNtCshFOMELpAQPt_3std4path7PathBufECsgpkREHUotZJ_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.08a1a549-cgu.11: no profile data available for function _RINvNtCs4MetWwXVPrY_12tracing_core5field5debugRNtNtCskcB8HSegukO_13fluent_bundle8resource14FluentResourceECsgpkREHUotZJ_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.08a1a549-cgu.11: no profile data available for function _RINvNtCs4MetWwXVPrY_12tracing_core5field5debugRQNtNtCshFOMELpAQPt_3std4path7PathBufECsgpkREHUotZJ_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.08a1a549-cgu.11: no profile data available for function _RINvNtCs4MetWwXVPrY_12tracing_core5field5debugRRSReECsgpkREHUotZJ_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.08a1a549-cgu.11: no profile data available for function _RINvNtCsgmcG5dXCadN_4core3ptr13drop_in_placeRINtNtB4_6option6OptionNtCsZvZt4fBMkl_16unic_langid_impl18LanguageIdentifierEECsgpkREHUotZJ_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.08a1a549-cgu.11: no profile data available for function _RINvNtCsgmcG5dXCadN_4core3ptr13drop_in_placeRINtNtB4_6option6OptionNtNtCshFOMELpAQPt_3std4path7PathBufEECsgpkREHUotZJ_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.08a1a549-cgu.11: no profile data available for function _RINvNtCsgmcG5dXCadN_4core3ptr13drop_in_placeRINtNtB4_6option6OptionRNtNtCshFOMELpAQPt_3std4path4PathEECsgpkREHUotZJ_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.08a1a549-cgu.11: no profile data available for function _RINvNtCsgmcG5dXCadN_4core3ptr13drop_in_placeRINtNtCs5IAtDAEPLL0_5alloc3vec3VecNtNtCshFOMELpAQPt_3std4path7PathBufEECsgpkREHUotZJ_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.08a1a549-cgu.11: no profile data available for function _RINvNtCsgmcG5dXCadN_4core3ptr13drop_in_placeRNtCsZvZt4fBMkl_16unic_langid_impl18LanguageIdentifierECsgpkREHUotZJ_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.08a1a549-cgu.11: no profile data available for function _RINvNtCsgmcG5dXCadN_4core3ptr13drop_in_placeRNtNtCshFOMELpAQPt_3std4path7PathBufECsgpkREHUotZJ_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.08a1a549-cgu.11: no profile data available for function _RINvNtCsgmcG5dXCadN_4core3ptr13drop_in_placeRNtNtCskcB8HSegukO_13fluent_bundle8resource14FluentResourceECsgpkREHUotZJ_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.08a1a549-cgu.11: no profile data available for function _RINvNtCsgmcG5dXCadN_4core3ptr13drop_in_placeRQNtNtCshFOMELpAQPt_3std4path7PathBufECsgpkREHUotZJ_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.08a1a549-cgu.11: no profile data available for function _RINvNtCsgmcG5dXCadN_4core3ptr13drop_in_placeRRSReECsgpkREHUotZJ_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.08a1a549-cgu.11: no profile data available for function _RNvXsk_NtCs4MetWwXVPrY_12tracing_core5fieldINtB5_10DebugValueRINtNtCs5IAtDAEPLL0_5alloc3vec3VecNtNtCshFOMELpAQPt_3std4path7PathBufEENtB5_5Value6recordCsgpkREHUotZJ_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.08a1a549-cgu.11: no profile data available for function _RNvXsk_NtCs4MetWwXVPrY_12tracing_core5fieldINtB5_10DebugValueRINtNtCsgmcG5dXCadN_4core6option6OptionNtCsZvZt4fBMkl_16unic_langid_impl18LanguageIdentifierEENtB5_5Value6recordCsgpkREHUotZJ_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.08a1a549-cgu.11: no profile data available for function _RNvXsk_NtCs4MetWwXVPrY_12tracing_core5fieldINtB5_10DebugValueRINtNtCsgmcG5dXCadN_4core6option6OptionNtNtCshFOMELpAQPt_3std4path7PathBufEENtB5_5Value6recordCsgpkREHUotZJ_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.08a1a549-cgu.11: no profile data available for function _RNvXsk_NtCs4MetWwXVPrY_12tracing_core5fieldINtB5_10DebugValueRINtNtCsgmcG5dXCadN_4core6option6OptionRNtNtCshFOMELpAQPt_3std4path4PathEENtB5_5Value6recordCsgpkREHUotZJ_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.08a1a549-cgu.11: no profile data available for function _RNvXsk_NtCs4MetWwXVPrY_12tracing_core5fieldINtB5_10DebugValueRNtCsZvZt4fBMkl_16unic_langid_impl18LanguageIdentifierENtB5_5Value6recordCsgpkREHUotZJ_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.08a1a549-cgu.11: no profile data available for function _RNvXsk_NtCs4MetWwXVPrY_12tracing_core5fieldINtB5_10DebugValueRNtNtCshFOMELpAQPt_3std4path7PathBufENtB5_5Value6recordCsgpkREHUotZJ_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.08a1a549-cgu.11: no profile data available for function _RNvXsk_NtCs4MetWwXVPrY_12tracing_core5fieldINtB5_10DebugValueRNtNtCskcB8HSegukO_13fluent_bundle8resource14FluentResourceENtB5_5Value6recordCsgpkREHUotZJ_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.08a1a549-cgu.11: no profile data available for function _RNvXsk_NtCs4MetWwXVPrY_12tracing_core5fieldINtB5_10DebugValueRQNtNtCshFOMELpAQPt_3std4path7PathBufENtB5_5Value6recordCsgpkREHUotZJ_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.08a1a549-cgu.11: no profile data available for function _RNvXsk_NtCs4MetWwXVPrY_12tracing_core5fieldINtB5_10DebugValueRRSReENtB5_5Value6recordCsgpkREHUotZJ_20rustc_error_messages Hash = 170957022131388415
[RUSTC-TIMING] rustc_error_messages test:false 0.919
warning: `rustc_error_messages` (lib) generated 30 warnings
[RUSTC-TIMING] rustc_feature test:false 1.118
[RUSTC-TIMING] rustc_span test:false 4.868
---
Dist extended stage1 (x86_64-unknown-linux-gnu)
Dist rust-nightly-x86_64-unknown-linux-gnu
 finished in 84.358 seconds
[TIMING] dist::Extended { stage: 2, host: x86_64-unknown-linux-gnu, target: x86_64-unknown-linux-gnu } -- 84.360
thread 'main' panicked at 'failed to copy `/checkout/src/tools/cargo/tests/testsuite/cargo_add/add-basic.in` to `/checkout/obj/build/tmp/tarball/rustc/src/image/src/tools/cargo/tests/testsuite/cargo_add/invalid_vers/in`: the source path is neither a regular file nor a symlink to a regular file', src/bootstrap/lib.rs:1434:17
Build completed unsuccessfully in 0:41:27
