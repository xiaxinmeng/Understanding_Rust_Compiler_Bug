plain
[2022-10-10T17:01:34Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-10-10T17:01:34Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-10-10T17:01:34Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpAEzCZE#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-10-10T17:01:36Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-10-10T17:01:36Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpAEzCZE#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpAEzCZE/incremental-state"
[2022-10-10T17:01:39Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-10-10T17:01:39Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpAEzCZE#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpAEzCZE/incremental-state"
[2022-10-10T17:01:40Z DEBUG collector::execute] applying println to "/tmp/.tmpAEzCZE"
[2022-10-10T17:01:40Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-10-10T17:01:40Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-10-10T17:01:40Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpAEzCZE#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpAEzCZE/incremental-state"
[2022-10-10T17:01:41Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-10-10T17:01:41Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-10-10T17:01:41Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpP3IUUz#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-10-10T17:01:43Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
---
[2022-10-10T17:01:49Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-10-10T17:01:49Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-10-10T17:01:49Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp6WEtjo#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-10-10T17:01:51Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-10-10T17:01:51Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp6WEtjo#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp6WEtjo/incremental-state"
[2022-10-10T17:01:54Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-10-10T17:01:54Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp6WEtjo#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp6WEtjo/incremental-state"
[2022-10-10T17:01:55Z DEBUG collector::execute] applying println to "/tmp/.tmp6WEtjo"
[2022-10-10T17:01:55Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-10-10T17:01:55Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-10-10T17:01:55Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp6WEtjo#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp6WEtjo/incremental-state"
Executing benchmark cargo-0.60.0 (2/8)
Preparing cargo-0.60.0
[2022-10-10T17:01:56Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-10-10T17:01:56Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
---
[2022-10-10T17:09:33Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-10-10T17:09:33Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-10-10T17:09:33Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpahABcy#ctfe-stress-5@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-10-10T17:09:42Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-10-10T17:09:42Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpahABcy#ctfe-stress-5@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpahABcy/incremental-state"
[2022-10-10T17:09:53Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-10-10T17:09:53Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpahABcy#ctfe-stress-5@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpahABcy/incremental-state"
[2022-10-10T17:09:54Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-10-10T17:09:54Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-10-10T17:09:54Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJKlO4s#ctfe-stress-5@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-10-10T17:10:03Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-10-10T17:10:03Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-10-10T17:10:03Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJKlO4s#ctfe-stress-5@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpJKlO4s/incremental-state"
[2022-10-10T17:10:13Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-10-10T17:10:13Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJKlO4s#ctfe-stress-5@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpJKlO4s/incremental-state"
Executing benchmark diesel-1.4.8 (4/8)
Preparing diesel-1.4.8
[2022-10-10T17:10:14Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-10-10T17:10:14Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
---
[2022-10-10T17:10:35Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-10-10T17:10:35Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-10-10T17:10:35Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQIRJkn#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-10-10T17:10:51Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-10-10T17:10:51Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQIRJkn#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpQIRJkn/incremental-state"
[2022-10-10T17:11:11Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-10-10T17:11:11Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQIRJkn#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpQIRJkn/incremental-state"
[2022-10-10T17:11:14Z DEBUG collector::execute] applying println to "/tmp/.tmpQIRJkn"
[2022-10-10T17:11:14Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-10-10T17:11:14Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-10-10T17:11:14Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQIRJkn#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpQIRJkn/incremental-state"
[2022-10-10T17:11:18Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-10-10T17:11:18Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-10-10T17:11:18Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwLkzsd#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln"
[2022-10-10T17:11:36Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-10-10T17:11:36Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-10-10T17:11:36Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwLkzsd#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpwLkzsd/incremental-state"
[2022-10-10T17:11:59Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-10-10T17:11:59Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwLkzsd#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpwLkzsd/incremental-state"
[2022-10-10T17:12:03Z DEBUG collector::execute] applying println to "/tmp/.tmpwLkzsd"
[2022-10-10T17:12:03Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-10-10T17:12:03Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-10-10T17:12:03Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwLkzsd#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpwLkzsd/incremental-state"
[2022-10-10T17:12:07Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-10-10T17:12:07Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-10-10T17:12:07Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpy7pBbI#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-10-10T17:12:25Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
[2022-10-10T17:13:04Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-10-10T17:13:04Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-10-10T17:13:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpnukv7N#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-10-10T17:13:06Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-10-10T17:13:06Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpnukv7N#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpnukv7N/incremental-state"
[2022-10-10T17:13:09Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-10-10T17:13:09Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpnukv7N#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpnukv7N/incremental-state"
[2022-10-10T17:13:11Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-10-10T17:13:11Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-10-10T17:13:11Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpImLmX2#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-10-10T17:13:13Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-10-10T17:13:13Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-10-10T17:13:13Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpImLmX2#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpImLmX2/incremental-state"
[2022-10-10T17:13:17Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-10-10T17:13:17Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpImLmX2#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpImLmX2/incremental-state"
[2022-10-10T17:13:18Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-10-10T17:13:18Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-10-10T17:13:18Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpMAosTe#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-10-10T17:13:21Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-10-10T17:13:21Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-10-10T17:13:21Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpMAosTe#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpMAosTe/incremental-state"
[2022-10-10T17:13:24Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-10-10T17:13:24Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpMAosTe#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpMAosTe/incremental-state"
Executing benchmark token-stream-stress (7/8)
Preparing token-stream-stress
[2022-10-10T17:13:25Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-10-10T17:13:25Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2022-10-10T17:13:27Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-10-10T17:13:27Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-10-10T17:13:27Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpdxIIl9#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2022-10-10T17:13:27Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-10-10T17:13:27Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpdxIIl9#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpdxIIl9/incremental-state"
[2022-10-10T17:13:27Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-10-10T17:13:27Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpdxIIl9#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpdxIIl9/incremental-state"
[2022-10-10T17:13:27Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-10-10T17:13:28Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-10-10T17:13:28Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmphhiBQX#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2022-10-10T17:13:28Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-10-10T17:13:28Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-10-10T17:13:28Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmphhiBQX#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmphhiBQX/incremental-state"
[2022-10-10T17:13:28Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-10-10T17:13:28Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmphhiBQX#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmphhiBQX/incremental-state"
Executing benchmark tuple-stress (8/8)
Preparing tuple-stress
[2022-10-10T17:13:28Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-10-10T17:13:28Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2022-10-10T17:13:29Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-10-10T17:13:29Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-10-10T17:13:29Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpRgScMJ#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-10-10T17:13:34Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-10-10T17:13:34Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpRgScMJ#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpRgScMJ/incremental-state"
[2022-10-10T17:13:40Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-10-10T17:13:40Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpRgScMJ#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpRgScMJ/incremental-state"
[2022-10-10T17:13:42Z DEBUG collector::execute] applying new row to "/tmp/.tmpRgScMJ"
[2022-10-10T17:13:42Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-10-10T17:13:42Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-10-10T17:13:42Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpRgScMJ#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpRgScMJ/incremental-state"
[2022-10-10T17:13:48Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-10-10T17:13:48Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-10-10T17:13:48Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpoHLGeb#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-10-10T17:13:53Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-10-10T17:13:53Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-10-10T17:13:53Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpoHLGeb#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpoHLGeb/incremental-state"
[2022-10-10T17:13:59Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-10-10T17:13:59Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpoHLGeb#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpoHLGeb/incremental-state"
[2022-10-10T17:14:01Z DEBUG collector::execute] applying new row to "/tmp/.tmpoHLGeb"
[2022-10-10T17:14:01Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-10-10T17:14:01Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-10-10T17:14:01Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpoHLGeb#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpoHLGeb/incremental-state"
[2022-10-10T17:14:07Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-10-10T17:14:07Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-10-10T17:14:07Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpeyfJT0#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-10-10T17:14:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-10-10T17:14:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-10-10T17:14:12Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpeyfJT0#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpeyfJT0/incremental-state"
[2022-10-10T17:14:19Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-10-10T17:14:19Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpeyfJT0#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpeyfJT0/incremental-state"
[2022-10-10T17:14:20Z DEBUG collector::execute] applying new row to "/tmp/.tmpeyfJT0"
[2022-10-10T17:14:20Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-10-10T17:14:20Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-10-10T17:14:20Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpeyfJT0#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpeyfJT0/incremental-state"
+ cd /checkout/obj
+ RUSTC_PROFILE_MERGED_FILE=/tmp/tmp-pgo/rustc-pgo.profdata
+ /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-profdata merge -o /tmp/tmp-pgo/rustc-pgo.profdata /tmp/tmp-pgo/rustc-pgo
+ echo 'Rustc PGO statistics'
---
BOLT-INFO: the input contains 7252 (dynamic count : 19737716726) opportunities for macro-fusion optimization. Will fix instances on a hot path.
BOLT-INFO: 586621 instructions were shortened
BOLT-INFO: removed 1615 empty blocks
BOLT-INFO: removed 18 'repz' prefixes with estimated execution count of 69824083 times.
BOLT-INFO: ICF folded 1253 out of 106155 functions in 3 passes. 1 functions had jump tables.
BOLT-INFO: Removing all identical functions will save 123.26 KB of code space. Folded functions were called 1194107331 times based on profile.
BOLT-INFO: basic block reordering modified layout of 8580 (8.18%) functions
BOLT-INFO: UCE removed 5 blocks and 145 bytes of code.
BOLT-INFO: splitting separates 11449708 hot bytes from 9052836 cold bytes (55.85% of split functions is hot).
BOLT-INFO: 83 Functions were reordered by LoopInversionPass
BOLT-INFO: hfsort+ reduced the number of chains from 15219 to 6752
BOLT-INFO: program-wide dynostats after all optimizations before SCTC and FOP:
       1151927418125 : executed forward branches
        128679685423 : taken forward branches
        240479789396 : executed backward branches
        150775560709 : taken backward branches
---
[RUSTC-TIMING] build_script_build test:false 0.390
   Compiling lock_api v0.4.7
[RUSTC-TIMING] unicode_width test:false 0.484
   Compiling ena v0.14.0
warning: rustc_graphviz.769c8373-cgu.10: no profile data available for function _RNvMNtNtNtCs8fE5ZIj2Dja_4core4iter8adapters3revINtB2_3RevINtNtNtB8_3ops5range5RangejEE3newCsd4miHoBBJsd_14rustc_graphviz Hash = 742261418966908927 up to 0 count discarded
[RUSTC-TIMING] smallvec test:false 0.652
   Compiling crossbeam-channel v0.5.4
   Compiling unic-ucd-version v0.9.0
[RUSTC-TIMING] build_script_main test:false 1.072
---
   Compiling petgraph v0.5.1
   Compiling object v0.29.0
   Compiling gimli v0.26.1
[RUSTC-TIMING] indexmap test:false 0.820
warning: rustc_serialize.a317d961-cgu.3: no profile data available for function _RINvNtCs8fE5ZIj2Dja_4core3ptr13drop_in_placeRjECshLydsfzbhQb_15rustc_serialize Hash = 742261418966908927 up to 0 count discarded

warning: rustc_serialize.a317d961-cgu.3: no profile data available for function _RINvNtCs8fE5ZIj2Dja_4core9panicking13assert_failedjjECshLydsfzbhQb_15rustc_serialize Hash = 742261418966908927 up to 0 count discarded

warning: rustc_serialize.a317d961-cgu.5: no profile data available for function _RNvXsV_NtCs8fE5ZIj2Dja_4core3fmtRjNtB5_5Debug3fmtCshLydsfzbhQb_15rustc_serialize Hash = 1124680650125156080 up to 0 count discarded
[RUSTC-TIMING] rustc_serialize test:false 0.611
warning: `rustc_serialize` (lib) generated 3 warnings
[RUSTC-TIMING] fluent_bundle test:false 0.916
   Compiling rustc_macros v0.1.0 (/checkout/compiler/rustc_macros)
---
   Compiling rustc_feature v0.0.0 (/checkout/compiler/rustc_feature)
   Compiling rustc_ast v0.0.0 (/checkout/compiler/rustc_ast)
   Compiling rustc_error_messages v0.0.0 (/checkout/compiler/rustc_error_messages)
   Compiling rustc_target v0.0.0 (/checkout/compiler/rustc_target)
warning: rustc_error_messages.4576acd1-cgu.14: no profile data available for function _RINvMsH_NtCsdNuRplmtRUE_12tracing_core5fieldNtB6_8FieldSet9value_setATRNtB6_5FieldINtNtCs8fE5ZIj2Dja_4core6option6OptionRDNtB6_5ValueEL_EEj1_ECs6YUQPUIOf2D_20rustc_error_messages Hash = 742261418966908927 up to 0 count discarded

warning: rustc_error_messages.4576acd1-cgu.14: no profile data available for function _RINvMsH_NtCsdNuRplmtRUE_12tracing_core5fieldNtB6_8FieldSet9value_setATRNtB6_5FieldINtNtCs8fE5ZIj2Dja_4core6option6OptionRDNtB6_5ValueEL_EEj2_ECs6YUQPUIOf2D_20rustc_error_messages Hash = 742261418966908927 up to 0 count discarded

warning: rustc_error_messages.4576acd1-cgu.14: no profile data available for function _RINvMsH_NtCsdNuRplmtRUE_12tracing_core5fieldNtB6_8FieldSet9value_setATRNtB6_5FieldINtNtCs8fE5ZIj2Dja_4core6option6OptionRDNtB6_5ValueEL_EEj5_ECs6YUQPUIOf2D_20rustc_error_messages Hash = 742261418966908927 up to 0 count discarded

warning: rustc_error_messages.4576acd1-cgu.14: no profile data available for function _RINvNtCs8fE5ZIj2Dja_4core3ptr13drop_in_placeRINtNtB4_6option6OptionNtCshq6lv1zH0m8_16unic_langid_impl18LanguageIdentifierEECs6YUQPUIOf2D_20rustc_error_messages Hash = 742261418966908927 up to 0 count discarded

warning: rustc_error_messages.4576acd1-cgu.14: no profile data available for function _RINvNtCs8fE5ZIj2Dja_4core3ptr13drop_in_placeRINtNtB4_6option6OptionNtNtCskDg8l1urSmw_3std4path7PathBufEECs6YUQPUIOf2D_20rustc_error_messages Hash = 742261418966908927 up to 0 count discarded

warning: rustc_error_messages.4576acd1-cgu.14: no profile data available for function _RINvNtCs8fE5ZIj2Dja_4core3ptr13drop_in_placeRINtNtB4_6option6OptionRNtNtCskDg8l1urSmw_3std4path4PathEECs6YUQPUIOf2D_20rustc_error_messages Hash = 742261418966908927 up to 0 count discarded

warning: rustc_error_messages.4576acd1-cgu.14: no profile data available for function _RINvNtCs8fE5ZIj2Dja_4core3ptr13drop_in_placeRINtNtCs3j0Se0hlEr3_5alloc3vec3VecNtNtCskDg8l1urSmw_3std4path7PathBufEECs6YUQPUIOf2D_20rustc_error_messages Hash = 742261418966908927 up to 0 count discarded

warning: rustc_error_messages.4576acd1-cgu.14: no profile data available for function _RINvNtCs8fE5ZIj2Dja_4core3ptr13drop_in_placeRNtCshq6lv1zH0m8_16unic_langid_impl18LanguageIdentifierECs6YUQPUIOf2D_20rustc_error_messages Hash = 742261418966908927 up to 0 count discarded

warning: rustc_error_messages.4576acd1-cgu.14: no profile data available for function _RINvNtCs8fE5ZIj2Dja_4core3ptr13drop_in_placeRNtNtCsbFIzVOtJsMp_13fluent_bundle8resource14FluentResourceECs6YUQPUIOf2D_20rustc_error_messages Hash = 742261418966908927 up to 0 count discarded

warning: rustc_error_messages.4576acd1-cgu.14: no profile data available for function _RINvNtCs8fE5ZIj2Dja_4core3ptr13drop_in_placeRNtNtCskDg8l1urSmw_3std4path7PathBufECs6YUQPUIOf2D_20rustc_error_messages Hash = 742261418966908927 up to 0 count discarded

warning: rustc_error_messages.4576acd1-cgu.14: no profile data available for function _RINvNtCs8fE5ZIj2Dja_4core3ptr13drop_in_placeRQNtNtCskDg8l1urSmw_3std4path7PathBufECs6YUQPUIOf2D_20rustc_error_messages Hash = 742261418966908927 up to 0 count discarded

warning: rustc_error_messages.4576acd1-cgu.14: no profile data available for function _RINvNtCs8fE5ZIj2Dja_4core3ptr13drop_in_placeRRSReECs6YUQPUIOf2D_20rustc_error_messages Hash = 742261418966908927 up to 0 count discarded

warning: rustc_error_messages.4576acd1-cgu.14: no profile data available for function _RINvNtCsdNuRplmtRUE_12tracing_core5field5debugRINtNtCs3j0Se0hlEr3_5alloc3vec3VecNtNtCskDg8l1urSmw_3std4path7PathBufEECs6YUQPUIOf2D_20rustc_error_messages Hash = 742261418966908927 up to 0 count discarded

warning: rustc_error_messages.4576acd1-cgu.14: no profile data available for function _RINvNtCsdNuRplmtRUE_12tracing_core5field5debugRINtNtCs8fE5ZIj2Dja_4core6option6OptionNtCshq6lv1zH0m8_16unic_langid_impl18LanguageIdentifierEECs6YUQPUIOf2D_20rustc_error_messages Hash = 742261418966908927 up to 0 count discarded

warning: rustc_error_messages.4576acd1-cgu.14: no profile data available for function _RINvNtCsdNuRplmtRUE_12tracing_core5field5debugRINtNtCs8fE5ZIj2Dja_4core6option6OptionNtNtCskDg8l1urSmw_3std4path7PathBufEECs6YUQPUIOf2D_20rustc_error_messages Hash = 742261418966908927 up to 0 count discarded

warning: rustc_error_messages.4576acd1-cgu.14: no profile data available for function _RINvNtCsdNuRplmtRUE_12tracing_core5field5debugRINtNtCs8fE5ZIj2Dja_4core6option6OptionRNtNtCskDg8l1urSmw_3std4path4PathEECs6YUQPUIOf2D_20rustc_error_messages Hash = 742261418966908927 up to 0 count discarded

warning: rustc_error_messages.4576acd1-cgu.14: no profile data available for function _RINvNtCsdNuRplmtRUE_12tracing_core5field5debugRNtCshq6lv1zH0m8_16unic_langid_impl18LanguageIdentifierECs6YUQPUIOf2D_20rustc_error_messages Hash = 742261418966908927 up to 0 count discarded

warning: rustc_error_messages.4576acd1-cgu.14: no profile data available for function _RINvNtCsdNuRplmtRUE_12tracing_core5field5debugRNtNtCsbFIzVOtJsMp_13fluent_bundle8resource14FluentResourceECs6YUQPUIOf2D_20rustc_error_messages Hash = 742261418966908927 up to 0 count discarded

warning: rustc_error_messages.4576acd1-cgu.14: no profile data available for function _RINvNtCsdNuRplmtRUE_12tracing_core5field5debugRNtNtCskDg8l1urSmw_3std4path7PathBufECs6YUQPUIOf2D_20rustc_error_messages Hash = 742261418966908927 up to 0 count discarded

warning: rustc_error_messages.4576acd1-cgu.14: no profile data available for function _RINvNtCsdNuRplmtRUE_12tracing_core5field5debugRQNtNtCskDg8l1urSmw_3std4path7PathBufECs6YUQPUIOf2D_20rustc_error_messages Hash = 742261418966908927 up to 0 count discarded

warning: rustc_error_messages.4576acd1-cgu.14: no profile data available for function _RINvNtCsdNuRplmtRUE_12tracing_core5field5debugRRSReECs6YUQPUIOf2D_20rustc_error_messages Hash = 742261418966908927 up to 0 count discarded

warning: rustc_error_messages.4576acd1-cgu.14: no profile data available for function _RNvXsu_NtCsdNuRplmtRUE_12tracing_core5fieldINtB5_10DebugValueRINtNtCs3j0Se0hlEr3_5alloc3vec3VecNtNtCskDg8l1urSmw_3std4path7PathBufEENtB5_5Value6recordCs6YUQPUIOf2D_20rustc_error_messages Hash = 170957022131388415 up to 0 count discarded

warning: rustc_error_messages.4576acd1-cgu.14: no profile data available for function _RNvXsu_NtCsdNuRplmtRUE_12tracing_core5fieldINtB5_10DebugValueRINtNtCs8fE5ZIj2Dja_4core6option6OptionNtCshq6lv1zH0m8_16unic_langid_impl18LanguageIdentifierEENtB5_5Value6recordCs6YUQPUIOf2D_20rustc_error_messages Hash = 170957022131388415 up to 0 count discarded

warning: rustc_error_messages.4576acd1-cgu.14: no profile data available for function _RNvXsu_NtCsdNuRplmtRUE_12tracing_core5fieldINtB5_10DebugValueRINtNtCs8fE5ZIj2Dja_4core6option6OptionNtNtCskDg8l1urSmw_3std4path7PathBufEENtB5_5Value6recordCs6YUQPUIOf2D_20rustc_error_messages Hash = 170957022131388415 up to 0 count discarded

warning: rustc_error_messages.4576acd1-cgu.14: no profile data available for function _RNvXsu_NtCsdNuRplmtRUE_12tracing_core5fieldINtB5_10DebugValueRINtNtCs8fE5ZIj2Dja_4core6option6OptionRNtNtCskDg8l1urSmw_3std4path4PathEENtB5_5Value6recordCs6YUQPUIOf2D_20rustc_error_messages Hash = 170957022131388415 up to 0 count discarded

warning: rustc_error_messages.4576acd1-cgu.14: no profile data available for function _RNvXsu_NtCsdNuRplmtRUE_12tracing_core5fieldINtB5_10DebugValueRNtCshq6lv1zH0m8_16unic_langid_impl18LanguageIdentifierENtB5_5Value6recordCs6YUQPUIOf2D_20rustc_error_messages Hash = 170957022131388415 up to 0 count discarded

warning: rustc_error_messages.4576acd1-cgu.14: no profile data available for function _RNvXsu_NtCsdNuRplmtRUE_12tracing_core5fieldINtB5_10DebugValueRNtNtCsbFIzVOtJsMp_13fluent_bundle8resource14FluentResourceENtB5_5Value6recordCs6YUQPUIOf2D_20rustc_error_messages Hash = 170957022131388415 up to 0 count discarded

warning: rustc_error_messages.4576acd1-cgu.14: no profile data available for function _RNvXsu_NtCsdNuRplmtRUE_12tracing_core5fieldINtB5_10DebugValueRNtNtCskDg8l1urSmw_3std4path7PathBufENtB5_5Value6recordCs6YUQPUIOf2D_20rustc_error_messages Hash = 170957022131388415 up to 0 count discarded

warning: rustc_error_messages.4576acd1-cgu.14: no profile data available for function _RNvXsu_NtCsdNuRplmtRUE_12tracing_core5fieldINtB5_10DebugValueRQNtNtCskDg8l1urSmw_3std4path7PathBufENtB5_5Value6recordCs6YUQPUIOf2D_20rustc_error_messages Hash = 170957022131388415 up to 0 count discarded

warning: rustc_error_messages.4576acd1-cgu.14: no profile data available for function _RNvXsu_NtCsdNuRplmtRUE_12tracing_core5fieldINtB5_10DebugValueRRSReENtB5_5Value6recordCs6YUQPUIOf2D_20rustc_error_messages Hash = 170957022131388415 up to 0 count discarded
[RUSTC-TIMING] rustc_feature test:false 1.985
[RUSTC-TIMING] rustc_error_messages test:false 2.176
warning: `rustc_error_messages` (lib) generated 30 warnings
[RUSTC-TIMING] rustc_span test:false 5.269
---
[RUSTC-TIMING] rustc_borrowck test:false 84.827
[RUSTC-TIMING] rustc_const_eval test:false 106.349
[RUSTC-TIMING] rustc_hir_analysis test:false 92.760
   Compiling rustc_driver v0.0.0 (/checkout/compiler/rustc_driver)
error: couldn't create a temp dir: Read-only file system (os error 30) at path "/tmp/rustc8Kb1o1"
error: could not compile `rustc_driver` due to previous error
Build completed unsuccessfully in 0:21:28
Build completed unsuccessfully in 0:21:28
== clock drift check ==
  local time: Mon Oct 10 18:14:03 UTC 2022
  network time: Mon, 10 Oct 2022 18:14:03 GMT
== end clock drift check ==
time="2022-10-10T18:14:03Z" level=error msg="Error waiting for container: container abd70ba4b959ef1b78266f3083cecaca09ac68545c04ba222961f47ff45bf45b: driver \"overlay2\" failed to remove root filesystem: unlinkat /var/lib/docker/overlay2/156012a8a8f6e5143bf2be0f6a1c7036db3da697540299d5325cc73ed8681cb8: read-only file system"
##[error]Process completed with exit code 125.
##[error]Read-only file system : '/home/runner/runners/2.298.2/_diag/pages/70ba09fd-1fda-4ebd-8f21-49fc09267c3d_05e30b09-217f-43ae-826e-dea76afd7ad3_1.log'
