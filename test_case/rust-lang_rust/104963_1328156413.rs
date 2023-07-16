plain
[2022-11-26T23:59:36Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-11-26T23:59:36Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-11-26T23:59:36Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpUNtlKJ#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-11-26T23:59:38Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-11-26T23:59:38Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpUNtlKJ#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpUNtlKJ/incremental-state"
[2022-11-26T23:59:42Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-11-26T23:59:42Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpUNtlKJ#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpUNtlKJ/incremental-state"
[2022-11-26T23:59:43Z DEBUG collector::execute] applying println to "/tmp/.tmpUNtlKJ"
[2022-11-26T23:59:43Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-11-26T23:59:43Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-11-26T23:59:43Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpUNtlKJ#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpUNtlKJ/incremental-state"
[2022-11-26T23:59:44Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-11-26T23:59:44Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-11-26T23:59:44Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpj9qB4A#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-11-26T23:59:46Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
---
[2022-11-27T00:02:22Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-11-27T00:02:23Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-11-27T00:02:23Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOWYmZC#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2022-11-27T00:03:26Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-11-27T00:03:26Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOWYmZC#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpOWYmZC/incremental-state"
[2022-11-27T00:04:46Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-11-27T00:04:46Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOWYmZC#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpOWYmZC/incremental-state"
[2022-11-27T00:04:58Z DEBUG collector::execute] applying println to "/tmp/.tmpOWYmZC"
[2022-11-27T00:04:58Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-11-27T00:04:58Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-11-27T00:04:58Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOWYmZC#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpOWYmZC/incremental-state"
[2022-11-27T00:05:18Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-11-27T00:05:19Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-11-27T00:05:19Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp6Y4MZt#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2022-11-27T00:06:34Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
[2022-11-27T00:08:29Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-11-27T00:08:29Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-11-27T00:08:29Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpRaZyfu#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-11-27T00:08:38Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-11-27T00:08:38Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpRaZyfu#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpRaZyfu/incremental-state"
[2022-11-27T00:08:49Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-11-27T00:08:49Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpRaZyfu#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpRaZyfu/incremental-state"
[2022-11-27T00:08:49Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-11-27T00:08:49Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-11-27T00:08:49Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpTQNRTh#ctfe-stress-5@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-11-27T00:08:58Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-11-27T00:08:58Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-11-27T00:08:58Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpTQNRTh#ctfe-stress-5@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpTQNRTh/incremental-state"
[2022-11-27T00:09:09Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-11-27T00:09:09Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpTQNRTh#ctfe-stress-5@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpTQNRTh/incremental-state"
[2022-11-27T00:09:10Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-11-27T00:09:10Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-11-27T00:09:10Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpnLPq7M#ctfe-stress-5@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-11-27T00:09:18Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
[2022-11-27T00:09:53Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-11-27T00:09:54Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-11-27T00:09:54Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJHkDdl#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-11-27T00:10:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-11-27T00:10:12Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJHkDdl#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpJHkDdl/incremental-state"
[2022-11-27T00:10:35Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-11-27T00:10:35Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJHkDdl#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpJHkDdl/incremental-state"
[2022-11-27T00:10:39Z DEBUG collector::execute] applying println to "/tmp/.tmpJHkDdl"
[2022-11-27T00:10:39Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-11-27T00:10:39Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-11-27T00:10:39Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJHkDdl#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpJHkDdl/incremental-state"
[2022-11-27T00:10:43Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-11-27T00:10:43Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-11-27T00:10:43Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzphCEL#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln"
[2022-11-27T00:11:04Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-11-27T00:11:04Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-11-27T00:11:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzphCEL#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpzphCEL/incremental-state"
[2022-11-27T00:11:30Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-11-27T00:11:30Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzphCEL#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpzphCEL/incremental-state"
[2022-11-27T00:11:34Z DEBUG collector::execute] applying println to "/tmp/.tmpzphCEL"
[2022-11-27T00:11:34Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-11-27T00:11:34Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-11-27T00:11:34Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzphCEL#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpzphCEL/incremental-state"
[2022-11-27T00:11:39Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-11-27T00:11:39Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-11-27T00:11:39Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOWnjyl#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-11-27T00:12:01Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-11-27T00:12:01Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-11-27T00:12:01Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOWnjyl#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpOWnjyl/incremental-state"
[2022-11-27T00:12:28Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-11-27T00:12:28Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOWnjyl#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpOWnjyl/incremental-state"
[2022-11-27T00:12:32Z DEBUG collector::execute] applying println to "/tmp/.tmpOWnjyl"
[2022-11-27T00:12:32Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-11-27T00:12:32Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-11-27T00:12:32Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOWnjyl#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpOWnjyl/incremental-state"
Executing benchmark externs (5/8)
Preparing externs
[2022-11-27T00:12:36Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-11-27T00:12:36Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
---
[2022-11-27T00:12:42Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-11-27T00:12:42Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-11-27T00:12:42Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp2tnbVT#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-11-27T00:12:43Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-11-27T00:12:43Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp2tnbVT#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp2tnbVT/incremental-state"
[2022-11-27T00:12:44Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-11-27T00:12:44Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp2tnbVT#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp2tnbVT/incremental-state"
Executing benchmark match-stress (6/8)
Preparing match-stress
[2022-11-27T00:12:45Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-11-27T00:12:45Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
---
[2022-11-27T00:12:45Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-11-27T00:12:45Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-11-27T00:12:45Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKiqjWw#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-11-27T00:12:48Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-11-27T00:12:48Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKiqjWw#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpKiqjWw/incremental-state"
[2022-11-27T00:12:51Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-11-27T00:12:51Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKiqjWw#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpKiqjWw/incremental-state"
[2022-11-27T00:12:53Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-11-27T00:12:53Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-11-27T00:12:53Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpoQ1JcR#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-11-27T00:12:56Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-11-27T00:12:56Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-11-27T00:12:56Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpoQ1JcR#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpoQ1JcR/incremental-state"
[2022-11-27T00:12:59Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-11-27T00:12:59Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpoQ1JcR#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpoQ1JcR/incremental-state"
[2022-11-27T00:13:01Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-11-27T00:13:01Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-11-27T00:13:01Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpcnozK9#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-11-27T00:13:04Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-11-27T00:13:04Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-11-27T00:13:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpcnozK9#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpcnozK9/incremental-state"
[2022-11-27T00:13:08Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-11-27T00:13:08Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpcnozK9#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpcnozK9/incremental-state"
Executing benchmark token-stream-stress (7/8)
Preparing token-stream-stress
[2022-11-27T00:13:09Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-11-27T00:13:09Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2022-11-27T00:13:10Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-11-27T00:13:10Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-11-27T00:13:10Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpMkSOSh#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2022-11-27T00:13:11Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-11-27T00:13:11Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpMkSOSh#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpMkSOSh/incremental-state"
[2022-11-27T00:13:11Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-11-27T00:13:11Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpMkSOSh#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpMkSOSh/incremental-state"
[2022-11-27T00:13:11Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-11-27T00:13:11Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-11-27T00:13:11Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpBfTEEQ#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2022-11-27T00:13:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-11-27T00:13:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-11-27T00:13:12Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpBfTEEQ#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpBfTEEQ/incremental-state"
[2022-11-27T00:13:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-11-27T00:13:12Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpBfTEEQ#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpBfTEEQ/incremental-state"
Executing benchmark tuple-stress (8/8)
Preparing tuple-stress
[2022-11-27T00:13:12Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-11-27T00:13:12Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2022-11-27T00:13:32Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-11-27T00:13:32Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-11-27T00:13:32Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzK4tLH#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-11-27T00:13:38Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-11-27T00:13:38Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzK4tLH#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpzK4tLH/incremental-state"
[2022-11-27T00:13:44Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-11-27T00:13:44Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzK4tLH#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpzK4tLH/incremental-state"
[2022-11-27T00:13:46Z DEBUG collector::execute] applying new row to "/tmp/.tmpzK4tLH"
[2022-11-27T00:13:46Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-11-27T00:13:46Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-11-27T00:13:46Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzK4tLH#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpzK4tLH/incremental-state"
[2022-11-27T00:13:52Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-11-27T00:13:52Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-11-27T00:13:52Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpnk8t8K#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-11-27T00:13:58Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
[TIMING] dist::RustcDocs { host: x86_64-unknown-linux-gnu } -- 21.497
Documenting stage2 std (x86_64-unknown-linux-gnu) in JSON format
 Documenting core v0.0.0 (/checkout/library/core)
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `Item { id: Id("0:943"), crate_id: 0, name: None, span: Some(Span { filename: "library/core/src/prelude/v1.rs", begin: (10, 24), end: (10, 28) }), visibility: Public, docs: None, links: {}, attrs: ["#[stable(feature = \"core_prelude\", since = \"1.4.0\")]", "#[doc(no_inline)]"], deprecation: None, inner: Import(Import { source: "crate::marker::Copy", name: "Copy", id: Some(Id("0:2653:107")), glob: false }) }`,
 right: `Item { id: Id("0:943"), crate_id: 0, name: None, span: Some(Span { filename: "library/core/src/prelude/v1.rs", begin: (10, 24), end: (10, 28) }), visibility: Public, docs: None, links: {}, attrs: ["#[stable(feature = \"core_prelude\", since = \"1.4.0\")]", "#[doc(no_inline)]"], deprecation: None, inner: Import(Import { source: "crate::marker::Copy", name: "Copy", id: Some(Id("0:2654:107")), glob: false }) }`', src/librustdoc/json/mod.rs:256:21
   0:     0x7fef8cd3232a - std::backtrace_rs::backtrace::libunwind::trace::hf4f00b847b23cdc0
                               at /rustc/92fa2a1be7dd7730c2f2fe2f6c0ea4c1568bf834/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7fef8cd3232a - std::backtrace_rs::backtrace::trace_unsynchronized::he613364cfc365167
                               at /rustc/92fa2a1be7dd7730c2f2fe2f6c0ea4c1568bf834/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
                               at /rustc/92fa2a1be7dd7730c2f2fe2f6c0ea4c1568bf834/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7fef8cd3232a - std::sys_common::backtrace::_print_fmt::heb2d76c04d9e9e14
                               at /rustc/92fa2a1be7dd7730c2f2fe2f6c0ea4c1568bf834/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7fef8cd3232a - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h1820a3fb609e682d
                               at /rustc/92fa2a1be7dd7730c2f2fe2f6c0ea4c1568bf834/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7fef8cd94d0e - core::fmt::write::h24bcee81c55ed0dd
                               at /rustc/92fa2a1be7dd7730c2f2fe2f6c0ea4c1568bf834/library/core/src/fmt/mod.rs:1208:17
   5:     0x7fef8cd22845 - std::io::Write::write_fmt::had4eba7df1597172
                               at /rustc/92fa2a1be7dd7730c2f2fe2f6c0ea4c1568bf834/library/std/src/io/mod.rs:1682:15
   6:     0x7fef8cd320f5 - std::sys_common::backtrace::_print::h7abefe07ec6549d5
                               at /rustc/92fa2a1be7dd7730c2f2fe2f6c0ea4c1568bf834/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x7fef8cd320f5 - std::sys_common::backtrace::print::hdaa61aab596647af
                               at /rustc/92fa2a1be7dd7730c2f2fe2f6c0ea4c1568bf834/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x7fef8cd34e1f - std::panicking::default_hook::{{closure}}::h351f5afdd14d922b
                               at /rustc/92fa2a1be7dd7730c2f2fe2f6c0ea4c1568bf834/library/std/src/panicking.rs:267:22
   9:     0x7fef8cd34b5b - std::panicking::default_hook::h9452ce12ab7760af
                               at /rustc/92fa2a1be7dd7730c2f2fe2f6c0ea4c1568bf834/library/std/src/panicking.rs:286:9
  10:     0x7fef8cd3563c - std::panicking::rust_panic_with_hook::h9c25cb52d9f3014e
                               at /rustc/92fa2a1be7dd7730c2f2fe2f6c0ea4c1568bf834/library/std/src/panicking.rs:688:13
  11:     0x7fef8cd353d9 - std::panicking::begin_panic_handler::{{closure}}::h19ce61b6d16835de
                               at /rustc/92fa2a1be7dd7730c2f2fe2f6c0ea4c1568bf834/library/std/src/panicking.rs:579:13
  12:     0x7fef8cd327dc - std::sys_common::backtrace::__rust_end_short_backtrace::h88bd1f25b42751de
                               at /rustc/92fa2a1be7dd7730c2f2fe2f6c0ea4c1568bf834/library/std/src/sys_common/backtrace.rs:137:18
  13:     0x7fef8cd350e2 - rust_begin_unwind
                               at /rustc/92fa2a1be7dd7730c2f2fe2f6c0ea4c1568bf834/library/std/src/panicking.rs:575:5
  14:     0x7fef8cd91723 - core::panicking::panic_fmt::h931136223def17fc
                               at /rustc/92fa2a1be7dd7730c2f2fe2f6c0ea4c1568bf834/library/core/src/panicking.rs:65:14
  15:     0x7fef8cd91b1f - core::panicking::assert_failed_inner::ha94485444293ddd7
  16:     0x55e45fa5329b - core[7a704040b660a257]::panicking::assert_failed::<rustdoc_json_types[3cf6129a2c91ea5]::Item, rustdoc_json_types[3cf6129a2c91ea5]::Item>
  17:     0x55e45fd7ece2 - <rustdoc[4e409546d1015151]::json::JsonRenderer as rustdoc[4e409546d1015151]::formats::renderer::FormatRenderer>::item
  18:     0x55e45fd7a369 - <rustdoc[4e409546d1015151]::json::JsonRenderer as rustdoc[4e409546d1015151]::formats::renderer::FormatRenderer>::item
  19:     0x55e45fd7a369 - <rustdoc[4e409546d1015151]::json::JsonRenderer as rustdoc[4e409546d1015151]::formats::renderer::FormatRenderer>::item
  20:     0x55e45fd7a369 - <rustdoc[4e409546d1015151]::json::JsonRenderer as rustdoc[4e409546d1015151]::formats::renderer::FormatRenderer>::item
  21:     0x55e45fd66eff - rustdoc[4e409546d1015151]::formats::renderer::run_format::<rustdoc[4e409546d1015151]::json::JsonRenderer>
  22:     0x55e45fa93ac5 - rustdoc[4e409546d1015151]::run_renderer::<rustdoc[4e409546d1015151]::json::JsonRenderer>
  23:     0x55e45fc64063 - <rustc_interface[75e4b2638ee3365b]::passes::QueryContext>::enter::<rustdoc[4e409546d1015151]::main_args::{closure#1}::{closure#0}::{closure#1}, core[7a704040b660a257]::result::Result<(), rustc_errors[495ed71886388759]::ErrorGuaranteed>>
  24:     0x55e45fbbe6ba - <rustc_interface[75e4b2638ee3365b]::interface::Compiler>::enter::<rustdoc[4e409546d1015151]::main_args::{closure#1}::{closure#0}, core[7a704040b660a257]::result::Result<(), rustc_errors[495ed71886388759]::ErrorGuaranteed>>
  25:     0x55e45fc1300f - rustc_span[b78f0289d1185483]::with_source_map::<core[7a704040b660a257]::result::Result<(), rustc_errors[495ed71886388759]::ErrorGuaranteed>, rustc_interface[75e4b2638ee3365b]::interface::run_compiler<core[7a704040b660a257]::result::Result<(), rustc_errors[495ed71886388759]::ErrorGuaranteed>, rustdoc[4e409546d1015151]::main_args::{closure#1}>::{closure#0}::{closure#0}>
  26:     0x55e45fa3d215 - <scoped_tls[8ba2aa4ce7f63757]::ScopedKey<rustc_span[b78f0289d1185483]::SessionGlobals>>::set::<rustc_interface[75e4b2638ee3365b]::interface::run_compiler<core[7a704040b660a257]::result::Result<(), rustc_errors[495ed71886388759]::ErrorGuaranteed>, rustdoc[4e409546d1015151]::main_args::{closure#1}>::{closure#0}, core[7a704040b660a257]::result::Result<(), rustc_errors[495ed71886388759]::ErrorGuaranteed>>
  27:     0x55e45fc7b5e0 - std[dcf2c8a04e49d717]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[75e4b2638ee3365b]::util::run_in_thread_pool_with_globals<rustc_interface[75e4b2638ee3365b]::interface::run_compiler<core[7a704040b660a257]::result::Result<(), rustc_errors[495ed71886388759]::ErrorGuaranteed>, rustdoc[4e409546d1015151]::main_args::{closure#1}>::{closure#0}, core[7a704040b660a257]::result::Result<(), rustc_errors[495ed71886388759]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[7a704040b660a257]::result::Result<(), rustc_errors[495ed71886388759]::ErrorGuaranteed>>
  28:     0x55e45fd586dd - <<std[dcf2c8a04e49d717]::thread::Builder>::spawn_unchecked_<rustc_interface[75e4b2638ee3365b]::util::run_in_thread_pool_with_globals<rustc_interface[75e4b2638ee3365b]::interface::run_compiler<core[7a704040b660a257]::result::Result<(), rustc_errors[495ed71886388759]::ErrorGuaranteed>, rustdoc[4e409546d1015151]::main_args::{closure#1}>::{closure#0}, core[7a704040b660a257]::result::Result<(), rustc_errors[495ed71886388759]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[7a704040b660a257]::result::Result<(), rustc_errors[495ed71886388759]::ErrorGuaranteed>>::{closure#1} as core[7a704040b660a257]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  29:     0x7fef8cd3f353 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::he653adde8efe5a33
                               at /rustc/92fa2a1be7dd7730c2f2fe2f6c0ea4c1568bf834/library/alloc/src/boxed.rs:2000:9
  30:     0x7fef8cd3f353 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hcde4110df9afc17e
                               at /rustc/92fa2a1be7dd7730c2f2fe2f6c0ea4c1568bf834/library/alloc/src/boxed.rs:2000:9
  31:     0x7fef8cd3f353 - std::sys::unix::thread::Thread::new::thread_start::h057af0c0aa13bb5a
                               at /rustc/92fa2a1be7dd7730c2f2fe2f6c0ea4c1568bf834/library/std/src/sys/unix/thread.rs:108:17
  32:     0x7fef8c3fcea5 - start_thread
  33:     0x7fef8c125b0d - clone
  34:                0x0 - <unknown>
error: could not document `core`
Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name core library/core/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/json-doc/x86_64-unknown-linux-gnu/doc -Zunstable-options --check-cfg 'names()' --check-cfg 'values()' --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -Z unstable-options --resource-suffix 1.67.0 --output-format json -C metadata=97357397889e81ef -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/json-doc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/json-doc/release/deps -Csymbol-mangling-version=legacy -Zunstable-options -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(stdarch_intel_sde)' '--check-cfg=values(no_fp_fmt_parse)' '--check-cfg=values(no_global_oom_handling)' '--check-cfg=values(no_rc)' '--check-cfg=values(no_sync)' '--check-cfg=values(freebsd12)' '--check-cfg=values(backtrace_in_libstd)' '--check-cfg=values(target_env,"libnx")' '--check-cfg=values(target_os,"watchos")' '--check-cfg=values(target_arch,"asmjs","spirv","nvptx","nvptx64","le32","xtensa")' '--check-cfg=values(dont_compile_me)' '--check-cfg=values(rustix_use_libc)' -Clink-arg=-fuse-ld=lld -Clink-arg=-Wl,--threads=1 -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.67.0-nightly
  (92fa2a1be
  2022-11-26)' '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")'` (exit status: 101)
