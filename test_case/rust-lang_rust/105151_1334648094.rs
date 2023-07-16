plain
[2022-12-01T23:24:03Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-12-01T23:24:03Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-12-01T23:24:03Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpNMa3Kf#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-12-01T23:24:05Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-12-01T23:24:05Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpNMa3Kf#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpNMa3Kf/incremental-state"
[2022-12-01T23:24:08Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-12-01T23:24:08Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpNMa3Kf#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpNMa3Kf/incremental-state"
[2022-12-01T23:24:09Z DEBUG collector::execute] applying println to "/tmp/.tmpNMa3Kf"
[2022-12-01T23:24:09Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-12-01T23:24:09Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-12-01T23:24:09Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpNMa3Kf#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpNMa3Kf/incremental-state"
[2022-12-01T23:24:10Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-12-01T23:24:10Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-12-01T23:24:10Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpTF210J#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-12-01T23:24:13Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
---
[2022-12-01T23:26:36Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-12-01T23:26:37Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-12-01T23:26:37Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmptdVbor#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2022-12-01T23:27:32Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-12-01T23:27:32Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmptdVbor#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmptdVbor/incremental-state"
[2022-12-01T23:28:39Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-12-01T23:28:39Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmptdVbor#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmptdVbor/incremental-state"
[2022-12-01T23:28:50Z DEBUG collector::execute] applying println to "/tmp/.tmptdVbor"
[2022-12-01T23:28:50Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-12-01T23:28:50Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-12-01T23:28:50Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmptdVbor#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmptdVbor/incremental-state"
[2022-12-01T23:29:09Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-12-01T23:29:10Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-12-01T23:29:10Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpBn3fg4#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2022-12-01T23:30:17Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
[2022-12-01T23:32:02Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-12-01T23:32:02Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-12-01T23:32:02Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFKismL#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-12-01T23:32:11Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-12-01T23:32:11Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFKismL#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpFKismL/incremental-state"
[2022-12-01T23:32:21Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-12-01T23:32:21Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFKismL#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpFKismL/incremental-state"
[2022-12-01T23:32:22Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-12-01T23:32:22Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-12-01T23:32:22Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpmEJVN1#ctfe-stress-5@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-12-01T23:32:30Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
---
[2022-12-01T23:33:24Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-12-01T23:33:24Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-12-01T23:33:24Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpHJPYlW#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-12-01T23:33:41Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-12-01T23:33:41Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpHJPYlW#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpHJPYlW/incremental-state"
[2022-12-01T23:34:01Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-12-01T23:34:01Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpHJPYlW#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpHJPYlW/incremental-state"
[2022-12-01T23:34:05Z DEBUG collector::execute] applying println to "/tmp/.tmpHJPYlW"
[2022-12-01T23:34:05Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-12-01T23:34:05Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-12-01T23:34:05Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpHJPYlW#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpHJPYlW/incremental-state"
[2022-12-01T23:34:08Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-12-01T23:34:08Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-12-01T23:34:08Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpUOKgq8#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln"
[2022-12-01T23:34:28Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-12-01T23:34:28Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-12-01T23:34:28Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpUOKgq8#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpUOKgq8/incremental-state"
[2022-12-01T23:34:51Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-12-01T23:34:51Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpUOKgq8#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpUOKgq8/incremental-state"
[2022-12-01T23:34:55Z DEBUG collector::execute] applying println to "/tmp/.tmpUOKgq8"
[2022-12-01T23:34:55Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-12-01T23:34:55Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-12-01T23:34:55Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpUOKgq8#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpUOKgq8/incremental-state"
[2022-12-01T23:34:59Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-12-01T23:34:59Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-12-01T23:34:59Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmprSlv06#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-12-01T23:35:19Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
[2022-12-01T23:35:53Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-12-01T23:35:53Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-12-01T23:35:53Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzLIAII#externs@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-12-01T23:35:54Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-12-01T23:35:54Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzLIAII#externs@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpzLIAII/incremental-state"
[2022-12-01T23:35:55Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-12-01T23:35:55Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzLIAII#externs@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpzLIAII/incremental-state"
[2022-12-01T23:35:56Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-12-01T23:35:56Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-12-01T23:35:56Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp0hsxYV#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-12-01T23:35:56Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-12-01T23:35:56Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-12-01T23:35:56Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp0hsxYV#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp0hsxYV/incremental-state"
[2022-12-01T23:35:58Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-12-01T23:35:58Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp0hsxYV#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp0hsxYV/incremental-state"
Executing benchmark match-stress (6/8)
Preparing match-stress
[2022-12-01T23:35:58Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-12-01T23:35:58Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
---
[2022-12-01T23:36:06Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-12-01T23:36:06Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-12-01T23:36:06Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpXI2QRk#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-12-01T23:36:09Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-12-01T23:36:09Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpXI2QRk#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpXI2QRk/incremental-state"
[2022-12-01T23:36:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-12-01T23:36:12Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpXI2QRk#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpXI2QRk/incremental-state"
[2022-12-01T23:36:14Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-12-01T23:36:14Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-12-01T23:36:14Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjV0ysu#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-12-01T23:36:16Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
[2022-12-01T23:36:22Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-12-01T23:36:22Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-12-01T23:36:22Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpxEJUvd#token-stream-stress@0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2022-12-01T23:36:22Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-12-01T23:36:22Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpxEJUvd#token-stream-stress@0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpxEJUvd/incremental-state"
[2022-12-01T23:36:22Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-12-01T23:36:22Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpxEJUvd#token-stream-stress@0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpxEJUvd/incremental-state"
[2022-12-01T23:36:23Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-12-01T23:36:23Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-12-01T23:36:23Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp4I1FfO#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2022-12-01T23:36:23Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
---
[2022-12-01T23:37:00Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-12-01T23:37:00Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-12-01T23:37:00Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOGEBKS#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-12-01T23:37:05Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-12-01T23:37:05Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOGEBKS#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpOGEBKS/incremental-state"
[2022-12-01T23:37:11Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-12-01T23:37:11Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOGEBKS#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpOGEBKS/incremental-state"
[2022-12-01T23:37:12Z DEBUG collector::execute] applying new row to "/tmp/.tmpOGEBKS"
[2022-12-01T23:37:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-12-01T23:37:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-12-01T23:37:12Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOGEBKS#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpOGEBKS/incremental-state"
+ cd /checkout/obj
+ RUSTC_PROFILE_MERGED_FILE=/tmp/tmp-pgo/rustc-pgo.profdata
+ /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-profdata merge -o /tmp/tmp-pgo/rustc-pgo.profdata /tmp/tmp-pgo/rustc-pgo
+ echo 'Rustc PGO statistics'
---
[RUSTC-TIMING] git2_curl test:false 0.595
[RUSTC-TIMING] git2 test:false 5.203
[RUSTC-TIMING] toml_edit test:false 38.309
[RUSTC-TIMING] cargo test:false 50.160
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', compiler/rustc_monomorphize/src/collector.rs:934:13
   0:     0x7fb9117a6290 - std::backtrace_rs::backtrace::libunwind::trace::h72d1596dd528bb82
                               at /rustc/67ff019ab81a4122bdadb1c52c80f29215879df0/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7fb9117a6290 - std::backtrace_rs::backtrace::trace_unsynchronized::hee13e3b68af6f52f
                               at /rustc/67ff019ab81a4122bdadb1c52c80f29215879df0/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
                               at /rustc/67ff019ab81a4122bdadb1c52c80f29215879df0/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7fb9117a6290 - std::sys_common::backtrace::_print_fmt::h12232e77732816bc
                               at /rustc/67ff019ab81a4122bdadb1c52c80f29215879df0/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7fb9117a6290 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h296e2080d428f42d
                               at /rustc/67ff019ab81a4122bdadb1c52c80f29215879df0/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7fb91180873e - core::fmt::write::hcb131ddd0c15f718
                               at /rustc/67ff019ab81a4122bdadb1c52c80f29215879df0/library/core/src/fmt/mod.rs:1208:17
   5:     0x7fb9117965f5 - std::io::Write::write_fmt::h92d7e4cbd76dded8
                               at /rustc/67ff019ab81a4122bdadb1c52c80f29215879df0/library/std/src/io/mod.rs:1682:15
   6:     0x7fb9117a6055 - std::sys_common::backtrace::_print::hfd155602eae71eae
                               at /rustc/67ff019ab81a4122bdadb1c52c80f29215879df0/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x7fb9117a6055 - std::sys_common::backtrace::print::h6ba96d5b1ac612bf
                               at /rustc/67ff019ab81a4122bdadb1c52c80f29215879df0/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x7fb9117a8daf - std::panicking::default_hook::{{closure}}::hbcb3e10feaf14525
                               at /rustc/67ff019ab81a4122bdadb1c52c80f29215879df0/library/std/src/panicking.rs:267:22
   9:     0x7fb9117a8aea - std::panicking::default_hook::he45b9a8155940a82
                               at /rustc/67ff019ab81a4122bdadb1c52c80f29215879df0/library/std/src/panicking.rs:286:9
  10:     0x7fb90e9a94a6 - rustc_driver[db4a5a26c7bb9a9c]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7fb9117a95d9 - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::h87b8f61fb9274598
                               at /rustc/67ff019ab81a4122bdadb1c52c80f29215879df0/library/alloc/src/boxed.rs:2024:9
  12:     0x7fb9117a95d9 - std::panicking::rust_panic_with_hook::h46a63a5f115f0d1a
                               at /rustc/67ff019ab81a4122bdadb1c52c80f29215879df0/library/std/src/panicking.rs:692:13
  13:     0x7fb9117a9311 - std::panicking::begin_panic_handler::{{closure}}::hf78e69605879d797
                               at /rustc/67ff019ab81a4122bdadb1c52c80f29215879df0/library/std/src/panicking.rs:577:13
  14:     0x7fb9117a673c - std::sys_common::backtrace::__rust_end_short_backtrace::h97caef2d8425e271
                               at /rustc/67ff019ab81a4122bdadb1c52c80f29215879df0/library/std/src/sys_common/backtrace.rs:137:18
  15:     0x7fb9117a9072 - rust_begin_unwind
                               at /rustc/67ff019ab81a4122bdadb1c52c80f29215879df0/library/std/src/panicking.rs:575:5
  16:     0x7fb911805123 - core::panicking::panic_fmt::h852ef51d3640d2fc
                               at /rustc/67ff019ab81a4122bdadb1c52c80f29215879df0/library/core/src/panicking.rs:65:14
  17:     0x7fb9118051fd - core::panicking::panic::h7031aae91bc06edb
                               at /rustc/67ff019ab81a4122bdadb1c52c80f29215879df0/library/core/src/panicking.rs:114:5
  18:     0x7fb90f13e093 - <rustc_monomorphize[48dfcf2f23be60b3]::collector::MirNeighborCollector as rustc_middle[ca359ce8580ebda0]::mir::visit::Visitor>::visit_terminator
  19:     0x7fb90f144123 - rustc_monomorphize[48dfcf2f23be60b3]::collector::collect_neighbours
  20:     0x7fb90f14210c - rustc_monomorphize[48dfcf2f23be60b3]::collector::collect_items_rec
  21:     0x7fb90f142747 - rustc_monomorphize[48dfcf2f23be60b3]::collector::collect_items_rec
  22:     0x7fb90f142747 - rustc_monomorphize[48dfcf2f23be60b3]::collector::collect_items_rec
  23:     0x7fb90f142747 - rustc_monomorphize[48dfcf2f23be60b3]::collector::collect_items_rec
  24:     0x7fb90f142747 - rustc_monomorphize[48dfcf2f23be60b3]::collector::collect_items_rec
  25:     0x7fb90f142747 - rustc_monomorphize[48dfcf2f23be60b3]::collector::collect_items_rec
  26:     0x7fb90f145050 - std[a9f97bebfc7cd8c0]::panicking::try::<(), core[ae65f184c67bc919]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[49608448ea8a0fb8]::sync::par_for_each_in<alloc[c4b6aa748b2a1692]::vec::Vec<rustc_middle[ca359ce8580ebda0]::mir::mono::MonoItem>, rustc_monomorphize[48dfcf2f23be60b3]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}>>
  27:     0x7fb90f16a57f - <rustc_session[73e4673686d5debc]::session::Session>::time::<(), rustc_monomorphize[48dfcf2f23be60b3]::collector::collect_crate_mono_items::{closure#1}>
  28:     0x7fb90f13fcfd - rustc_monomorphize[48dfcf2f23be60b3]::collector::collect_crate_mono_items
  29:     0x7fb90f14fa6a - rustc_monomorphize[48dfcf2f23be60b3]::partitioning::collect_and_partition_mono_items
  30:     0x7fb90ff97792 - rustc_query_system[fccd579ae59229bc]::query::plumbing::try_execute_query::<rustc_query_impl[67f99e74d0bdbef1]::plumbing::QueryCtxt, rustc_query_system[fccd579ae59229bc]::query::caches::DefaultCache<(), (&std[a9f97bebfc7cd8c0]::collections::hash::set::HashSet<rustc_span[1c29d7493b5e59c8]::def_id::DefId, core[ae65f184c67bc919]::hash::BuildHasherDefault<rustc_hash[dbc584c8a978da26]::FxHasher>>, &[rustc_middle[ca359ce8580ebda0]::mir::mono::CodegenUnit])>>
  31:     0x7fb910053327 - rustc_query_system[fccd579ae59229bc]::query::plumbing::get_query::<rustc_query_impl[67f99e74d0bdbef1]::queries::collect_and_partition_mono_items, rustc_query_impl[67f99e74d0bdbef1]::plumbing::QueryCtxt>
  32:     0x7fb90fe3c48b - <rustc_query_impl[67f99e74d0bdbef1]::Queries as rustc_middle[ca359ce8580ebda0]::ty::query::QueryEngine>::collect_and_partition_mono_items
  33:     0x7fb90eb74367 - rustc_codegen_ssa[69bbb58b96e84bc8]::base::codegen_crate::<rustc_codegen_llvm[4b6fe2f02c92684f]::LlvmCodegenBackend>
  34:     0x7fb90eb280f8 - <rustc_codegen_llvm[4b6fe2f02c92684f]::LlvmCodegenBackend as rustc_codegen_ssa[69bbb58b96e84bc8]::traits::backend::CodegenBackend>::codegen_crate
  35:     0x7fb90ea43eaf - <rustc_session[73e4673686d5debc]::session::Session>::time::<alloc[c4b6aa748b2a1692]::boxed::Box<dyn core[ae65f184c67bc919]::any::Any>, rustc_interface[a506aaeab4659e18]::passes::start_codegen::{closure#0}>
  36:     0x7fb90ea753e7 - rustc_interface[a506aaeab4659e18]::passes::start_codegen
  37:     0x7fb90ea7bbcb - <rustc_interface[a506aaeab4659e18]::queries::Queries>::ongoing_codegen
  38:     0x7fb90e98ae15 - <rustc_interface[a506aaeab4659e18]::interface::Compiler>::enter::<rustc_driver[db4a5a26c7bb9a9c]::run_compiler::{closure#1}::{closure#2}, core[ae65f184c67bc919]::result::Result<core[ae65f184c67bc919]::option::Option<rustc_interface[a506aaeab4659e18]::queries::Linker>, rustc_errors[103a132b600ea385]::ErrorGuaranteed>>
  39:     0x7fb90e91fad2 - rustc_span[1c29d7493b5e59c8]::with_source_map::<core[ae65f184c67bc919]::result::Result<(), rustc_errors[103a132b600ea385]::ErrorGuaranteed>, rustc_interface[a506aaeab4659e18]::interface::run_compiler<core[ae65f184c67bc919]::result::Result<(), rustc_errors[103a132b600ea385]::ErrorGuaranteed>, rustc_driver[db4a5a26c7bb9a9c]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  40:     0x7fb90e97e148 - <scoped_tls[27cd29d9afbae785]::ScopedKey<rustc_span[1c29d7493b5e59c8]::SessionGlobals>>::set::<rustc_interface[a506aaeab4659e18]::interface::run_compiler<core[ae65f184c67bc919]::result::Result<(), rustc_errors[103a132b600ea385]::ErrorGuaranteed>, rustc_driver[db4a5a26c7bb9a9c]::run_compiler::{closure#1}>::{closure#0}, core[ae65f184c67bc919]::result::Result<(), rustc_errors[103a132b600ea385]::ErrorGuaranteed>>
  41:     0x7fb90e945e90 - std[a9f97bebfc7cd8c0]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[a506aaeab4659e18]::util::run_in_thread_pool_with_globals<rustc_interface[a506aaeab4659e18]::interface::run_compiler<core[ae65f184c67bc919]::result::Result<(), rustc_errors[103a132b600ea385]::ErrorGuaranteed>, rustc_driver[db4a5a26c7bb9a9c]::run_compiler::{closure#1}>::{closure#0}, core[ae65f184c67bc919]::result::Result<(), rustc_errors[103a132b600ea385]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[ae65f184c67bc919]::result::Result<(), rustc_errors[103a132b600ea385]::ErrorGuaranteed>>
  42:     0x7fb90e925e54 - <<std[a9f97bebfc7cd8c0]::thread::Builder>::spawn_unchecked_<rustc_interface[a506aaeab4659e18]::util::run_in_thread_pool_with_globals<rustc_interface[a506aaeab4659e18]::interface::run_compiler<core[ae65f184c67bc919]::result::Result<(), rustc_errors[103a132b600ea385]::ErrorGuaranteed>, rustc_driver[db4a5a26c7bb9a9c]::run_compiler::{closure#1}>::{closure#0}, core[ae65f184c67bc919]::result::Result<(), rustc_errors[103a132b600ea385]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[ae65f184c67bc919]::result::Result<(), rustc_errors[103a132b600ea385]::ErrorGuaranteed>>::{closure#1} as core[ae65f184c67bc919]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  43:     0x7fb9117b3403 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h7a85031506c31306
                               at /rustc/67ff019ab81a4122bdadb1c52c80f29215879df0/library/alloc/src/boxed.rs:1990:9
  44:     0x7fb9117b3403 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h2c3592766d4d2f9f
                               at /rustc/67ff019ab81a4122bdadb1c52c80f29215879df0/library/alloc/src/boxed.rs:1990:9
  45:     0x7fb9117b3403 - std::sys::unix::thread::Thread::new::thread_start::h1f529fb95785e7e8
                               at /rustc/67ff019ab81a4122bdadb1c52c80f29215879df0/library/std/src/sys/unix/thread.rs:108:17
  46:     0x7fb90d200ea5 - start_thread
  47:     0x7fb90cf29b0d - clone
  48:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (67ff019ab 2022-12-01) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type bin -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -Z unstable-options -C linker=clang -C symbol-mangling-version=v0 -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C link-args=-fuse-ld=lld -C split-debuginfo=off -Z binary-dep-depinfo -Z tls-model=initial-exec
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [collect_and_partition_mono_items] collect_and_partition_mono_items
[RUSTC-TIMING] cargo test:false 0.723
error: could not compile `cargo`
Build completed unsuccessfully in 0:36:27
