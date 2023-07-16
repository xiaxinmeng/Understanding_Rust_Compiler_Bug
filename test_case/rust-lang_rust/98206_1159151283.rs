plain
[2022-06-17T17:32:20Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpESHact#ripgrep@13.0.0" "--" "--wrap-rustc-with" "Eprintln"
Running ripgrep-13.0.0: Opt + [Full]
[2022-06-17T17:32:26Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-17T17:32:27Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-06-17T17:32:27Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKMChTQ#ripgrep@13.0.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
Executing benchmark serde-1.0.136 (6/7)
Preparing serde-1.0.136
[2022-06-17T17:33:01Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-06-17T17:33:01Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
---
Preparing bitmaps-3.1.0
[2022-06-17T17:48:39Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-06-17T17:48:39Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-06-17T17:48:39Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-06-17T17:48:39Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmplQIssK#bitmaps@3.1.0" "--profile" "check" "--" "--skip-this-rustc"
[2022-06-17T17:48:39Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp1yQmpI#bitmaps@3.1.0" "--release" "--" "--skip-this-rustc"
[2022-06-17T17:48:39Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpsq9xix#bitmaps@3.1.0" "--" "--skip-this-rustc"
[2022-06-17T17:48:39Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-17T17:48:39Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-06-17T17:48:39Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp5kuK66#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-17T17:48:42Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
---
[2022-06-17T17:48:48Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-17T17:48:48Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-06-17T17:48:48Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpeWrR1q#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-17T17:48:50Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-06-17T17:48:50Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpeWrR1q#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpeWrR1q/incremental-state"
[2022-06-17T17:48:55Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-06-17T17:48:55Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpeWrR1q#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpeWrR1q/incremental-state"
[2022-06-17T17:48:55Z DEBUG collector::execute] applying println to "/tmp/.tmpeWrR1q"
[2022-06-17T17:48:55Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-06-17T17:48:55Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-06-17T17:48:55Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpeWrR1q#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpeWrR1q/incremental-state"
[2022-06-17T17:48:57Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-17T17:48:57Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-06-17T17:48:57Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwYqHOR#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-17T17:49:00Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-06-17T17:49:00Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-06-17T17:49:00Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwYqHOR#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpwYqHOR/incremental-state"
[2022-06-17T17:49:04Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-06-17T17:49:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwYqHOR#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpwYqHOR/incremental-state"
[2022-06-17T17:49:05Z DEBUG collector::execute] applying println to "/tmp/.tmpwYqHOR"
[2022-06-17T17:49:05Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-06-17T17:49:05Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-06-17T17:49:05Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwYqHOR#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpwYqHOR/incremental-state"
Executing benchmark cargo-0.60.0 (2/8)
Preparing cargo-0.60.0
[2022-06-17T17:49:06Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-06-17T17:49:06Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2022-06-17T17:50:21Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-17T17:50:22Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-06-17T17:50:22Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpnz0GLH#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-17T17:50:48Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-06-17T17:50:48Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpnz0GLH#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpnz0GLH/incremental-state"
[2022-06-17T17:51:19Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-06-17T17:51:19Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpnz0GLH#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpnz0GLH/incremental-state"
[2022-06-17T17:51:25Z DEBUG collector::execute] applying println to "/tmp/.tmpnz0GLH"
[2022-06-17T17:51:25Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-06-17T17:51:25Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-06-17T17:51:25Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpnz0GLH#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpnz0GLH/incremental-state"
[2022-06-17T17:51:39Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-17T17:51:40Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-06-17T17:51:40Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmppsQjTh#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-17T17:52:47Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-06-17T17:52:47Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-06-17T17:52:47Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmppsQjTh#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmppsQjTh/incremental-state"
[2022-06-17T17:54:06Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-06-17T17:54:06Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmppsQjTh#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmppsQjTh/incremental-state"
[2022-06-17T17:54:20Z DEBUG collector::execute] applying println to "/tmp/.tmppsQjTh"
[2022-06-17T17:54:20Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-06-17T17:54:20Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-06-17T17:54:20Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmppsQjTh#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmppsQjTh/incremental-state"
[2022-06-17T17:54:49Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-17T17:54:50Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-06-17T17:54:50Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpNTRYKJ#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-17T17:56:08Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-06-17T17:56:08Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-06-17T17:56:08Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpNTRYKJ#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpNTRYKJ/incremental-state"
[2022-06-17T17:57:38Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-06-17T17:57:38Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpNTRYKJ#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpNTRYKJ/incremental-state"
[2022-06-17T17:57:52Z DEBUG collector::execute] applying println to "/tmp/.tmpNTRYKJ"
[2022-06-17T17:57:52Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-06-17T17:57:52Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-06-17T17:57:52Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpNTRYKJ#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpNTRYKJ/incremental-state"
Executing benchmark ctfe-stress-5 (3/8)
Preparing ctfe-stress-5
[2022-06-17T17:58:29Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-06-17T17:58:29Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
---
[2022-06-17T17:58:55Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-17T17:58:55Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-06-17T17:58:55Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpZWBhtF#ctfe-stress-5@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-17T17:59:06Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-06-17T17:59:06Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpZWBhtF#ctfe-stress-5@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpZWBhtF/incremental-state"
[2022-06-17T17:59:22Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-06-17T17:59:22Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpZWBhtF#ctfe-stress-5@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpZWBhtF/incremental-state"
[2022-06-17T17:59:22Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-17T17:59:22Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-06-17T17:59:22Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmppWNDE5#ctfe-stress-5@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-17T17:59:33Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-06-17T17:59:33Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-06-17T17:59:33Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmppWNDE5#ctfe-stress-5@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmppWNDE5/incremental-state"
[2022-06-17T17:59:45Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-06-17T17:59:45Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmppWNDE5#ctfe-stress-5@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmppWNDE5/incremental-state"
Executing benchmark diesel-1.4.8 (4/8)
Preparing diesel-1.4.8
[2022-06-17T17:59:46Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-06-17T17:59:46Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2022-06-17T18:01:00Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpbFfABD#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpbFfABD/incremental-state"
Running diesel-1.4.8: Debug + [Full, IncrFull, IncrUnchanged, IncrPatched]
[2022-06-17T18:01:04Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-17T18:01:04Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-06-17T18:01:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpE2UbUx#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-17T18:01:27Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-06-17T18:01:27Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpE2UbUx#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpE2UbUx/incremental-state"
[2022-06-17T18:01:54Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-06-17T18:01:54Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpE2UbUx#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpE2UbUx/incremental-state"
[2022-06-17T18:01:59Z DEBUG collector::execute] applying println to "/tmp/.tmpE2UbUx"
[2022-06-17T18:01:59Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-06-17T18:01:59Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-06-17T18:01:59Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpE2UbUx#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpE2UbUx/incremental-state"
[2022-06-17T18:02:04Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-17T18:02:04Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-06-17T18:02:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpWn0sOv#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-17T18:02:27Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
[2022-06-17T18:03:05Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-17T18:03:05Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-06-17T18:03:05Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpHUriVC#externs@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-17T18:03:06Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-06-17T18:03:06Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpHUriVC#externs@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpHUriVC/incremental-state"
[2022-06-17T18:03:07Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-06-17T18:03:07Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpHUriVC#externs@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpHUriVC/incremental-state"
[2022-06-17T18:03:08Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-17T18:03:08Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-06-17T18:03:08Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpU9iNmu#externs@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-17T18:03:09Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
---
[2022-06-17T18:03:11Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-17T18:03:11Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-06-17T18:03:11Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpRQtVjI#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-17T18:03:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-06-17T18:03:12Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpRQtVjI#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpRQtVjI/incremental-state"
[2022-06-17T18:03:14Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-06-17T18:03:14Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpRQtVjI#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpRQtVjI/incremental-state"
Executing benchmark match-stress (6/8)
Preparing match-stress
[2022-06-17T18:03:14Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-06-17T18:03:14Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
---
[2022-06-17T18:03:15Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-17T18:03:15Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-06-17T18:03:15Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFENtnF#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-17T18:03:18Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-06-17T18:03:18Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFENtnF#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpFENtnF/incremental-state"
[2022-06-17T18:03:23Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-06-17T18:03:23Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFENtnF#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpFENtnF/incremental-state"
[2022-06-17T18:03:24Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-17T18:03:24Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-06-17T18:03:24Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwPK5PY#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-17T18:03:27Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-06-17T18:03:27Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-06-17T18:03:28Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwPK5PY#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpwPK5PY/incremental-state"
[2022-06-17T18:03:32Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-06-17T18:03:32Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwPK5PY#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpwPK5PY/incremental-state"
[2022-06-17T18:03:33Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-17T18:03:33Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-06-17T18:03:33Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp9xFLzj#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-17T18:03:37Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-06-17T18:03:37Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-06-17T18:03:37Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp9xFLzj#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp9xFLzj/incremental-state"
[2022-06-17T18:03:42Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-06-17T18:03:42Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp9xFLzj#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp9xFLzj/incremental-state"
Executing benchmark token-stream-stress (7/8)
Preparing token-stream-stress
[2022-06-17T18:03:43Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-06-17T18:03:43Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
---
[2022-06-17T18:03:44Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-17T18:03:44Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-06-17T18:03:44Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpM3GOUv#token-stream-stress@0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-17T18:03:44Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-06-17T18:03:44Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpM3GOUv#token-stream-stress@0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpM3GOUv/incremental-state"
[2022-06-17T18:03:45Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-06-17T18:03:45Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpM3GOUv#token-stream-stress@0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpM3GOUv/incremental-state"
[2022-06-17T18:03:45Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-17T18:03:45Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-06-17T18:03:45Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpkDuqgi#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-17T18:03:45Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-06-17T18:03:45Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-06-17T18:03:45Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpkDuqgi#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpkDuqgi/incremental-state"
[2022-06-17T18:03:46Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-06-17T18:03:46Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpkDuqgi#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpkDuqgi/incremental-state"
[2022-06-17T18:03:46Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-17T18:03:46Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-06-17T18:03:46Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpxcEsg2#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-17T18:03:46Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-06-17T18:03:46Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-06-17T18:03:46Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpxcEsg2#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpxcEsg2/incremental-state"
[2022-06-17T18:03:47Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-06-17T18:03:47Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpxcEsg2#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpxcEsg2/incremental-state"
Executing benchmark tuple-stress (8/8)
Preparing tuple-stress
[2022-06-17T18:03:47Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-06-17T18:03:47Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
---
[2022-06-17T18:04:09Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-17T18:04:09Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-06-17T18:04:09Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpqofwiP#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-17T18:04:15Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-06-17T18:04:15Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpqofwiP#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpqofwiP/incremental-state"
[2022-06-17T18:04:23Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-06-17T18:04:23Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpqofwiP#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpqofwiP/incremental-state"
[2022-06-17T18:04:25Z DEBUG collector::execute] applying new row to "/tmp/.tmpqofwiP"
[2022-06-17T18:04:25Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-06-17T18:04:25Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-06-17T18:04:25Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpqofwiP#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpqofwiP/incremental-state"
[2022-06-17T18:04:32Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-17T18:04:32Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-06-17T18:04:32Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp7wL2gt#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-17T18:04:37Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-06-17T18:04:37Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-06-17T18:04:37Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp7wL2gt#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp7wL2gt/incremental-state"
[2022-06-17T18:04:44Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-06-17T18:04:44Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp7wL2gt#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp7wL2gt/incremental-state"
[2022-06-17T18:04:46Z DEBUG collector::execute] applying new row to "/tmp/.tmp7wL2gt"
[2022-06-17T18:04:46Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-06-17T18:04:46Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-06-17T18:04:46Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp7wL2gt#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp7wL2gt/incremental-state"
+ cd /checkout/obj
+ RUSTC_PROFILE_MERGED_FILE=/tmp/rustc-pgo.profdata
+ ./build/x86_64-unknown-linux-gnu/llvm/bin/llvm-profdata merge -o /tmp/rustc-pgo.profdata /tmp/rustc-pgo
+ echo 'Rustc PGO statistics'
---
[RUSTC-TIMING] unic_char_range test:false 0.162
   Compiling either v1.6.0
[RUSTC-TIMING] self_cell test:false 0.081
   Compiling datafrog v2.0.1
warning: rustc_fs_util.cae42a31-cgu.1: no profile data available for function _RNvXs2_NtCs3twH93Ieuaq_4core7convertNtNtNtB7_5alloc6layout11LayoutErrorINtB5_4FromBy_E4fromCsiXvTMdmMa19_13rustc_fs_util Hash = 742261418966908927
[RUSTC-TIMING] itoa test:false 0.151
   Compiling memchr v2.4.1
   Compiling memchr v2.4.1
warning: rustc_graphviz.2120d47b-cgu.4: no profile data available for function _RNvXs2_NtCs3twH93Ieuaq_4core7convertNtNtNtB7_5alloc6layout11LayoutErrorINtB5_4FromBy_E4fromCs6b8QdMYvBMZ_14rustc_graphviz Hash = 742261418966908927

warning: rustc_graphviz.2120d47b-cgu.2: no profile data available for function _RNvMNtNtNtCs3twH93Ieuaq_4core4iter8adapters3revINtB2_3RevINtNtNtB8_3ops5range5RangejEE3newCs6b8QdMYvBMZ_14rustc_graphviz Hash = 742261418966908927
[RUSTC-TIMING] rustc_fs_util test:false 0.122
warning: `rustc_fs_util` (lib) generated 1 warning
   Compiling regex-syntax v0.6.25
[RUSTC-TIMING] ppv_lite86 test:false 0.582
---
[RUSTC-TIMING] generic_array test:false 1.039
[RUSTC-TIMING] rustc_parse_format test:false 1.262
[RUSTC-TIMING] matchers test:false 0.158
   Compiling crypto-common v0.1.2
warning: rustc_llvm.6720fe17-cgu.5: no profile data available for function _RNvXs2_NtCs3twH93Ieuaq_4core7convertNtNtNtB7_5alloc6layout11LayoutErrorINtB5_4FromBy_E4fromCsbhlc3Ow5w57_10rustc_llvm Hash = 742261418966908927
[RUSTC-TIMING] cc test:false 5.261
   Compiling block-buffer v0.10.2
[RUSTC-TIMING] crypto_common test:false 0.105
[RUSTC-TIMING] rustc_llvm test:false 0.269
---
   Compiling object v0.28.4
[RUSTC-TIMING] rand test:false 1.543
   Compiling tempfile v3.2.0
[RUSTC-TIMING] sha1 test:false 0.453
warning: rustc_serialize.ad3337cb-cgu.5: no profile data available for function _RINvNtCs3twH93Ieuaq_4core3ptr13drop_in_placeRjECsaOw1z4aYKr7_15rustc_serialize Hash = 742261418966908927

warning: rustc_serialize.ad3337cb-cgu.5: no profile data available for function _RINvNtCs3twH93Ieuaq_4core9panicking13assert_failedjjECsaOw1z4aYKr7_15rustc_serialize Hash = 742261418966908927

warning: rustc_serialize.ad3337cb-cgu.7: no profile data available for function _RNvXsV_NtCs3twH93Ieuaq_4core3fmtRjNtB5_5Debug3fmtCsaOw1z4aYKr7_15rustc_serialize Hash = 1124680650125156080
[RUSTC-TIMING] aho_corasick test:false 3.212
[RUSTC-TIMING] rustc_serialize test:false 0.800
warning: `rustc_serialize` (lib) generated 3 warnings
[RUSTC-TIMING] regex_automata test:false 3.623
---
   Compiling rustc_ast v0.0.0 (/checkout/compiler/rustc_ast)
   Compiling rustc_target v0.0.0 (/checkout/compiler/rustc_target)
   Compiling rustc_error_messages v0.0.0 (/checkout/compiler/rustc_error_messages)
   Compiling rustc_feature v0.0.0 (/checkout/compiler/rustc_feature)
warning: rustc_error_messages.a9bd464d-cgu.11: no profile data available for function _RINvMsx_NtCsbmRcNT3iGNU_12tracing_core5fieldNtB6_8FieldSet9value_setATRNtB6_5FieldINtNtCs3twH93Ieuaq_4core6option6OptionRDNtB6_5ValueEL_EEj1_ECshND7PHJU69l_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.a9bd464d-cgu.11: no profile data available for function _RINvMsx_NtCsbmRcNT3iGNU_12tracing_core5fieldNtB6_8FieldSet9value_setATRNtB6_5FieldINtNtCs3twH93Ieuaq_4core6option6OptionRDNtB6_5ValueEL_EEj2_ECshND7PHJU69l_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.a9bd464d-cgu.11: no profile data available for function _RINvMsx_NtCsbmRcNT3iGNU_12tracing_core5fieldNtB6_8FieldSet9value_setATRNtB6_5FieldINtNtCs3twH93Ieuaq_4core6option6OptionRDNtB6_5ValueEL_EEj5_ECshND7PHJU69l_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.a9bd464d-cgu.11: no profile data available for function _RINvNtCs3twH93Ieuaq_4core3ptr13drop_in_placeRINtNtB4_6option6OptionNtCsADXiRGsqjR_16unic_langid_impl18LanguageIdentifierEECshND7PHJU69l_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.a9bd464d-cgu.11: no profile data available for function _RINvNtCs3twH93Ieuaq_4core3ptr13drop_in_placeRINtNtB4_6option6OptionNtNtCs7Iy1M2Wve7y_3std4path7PathBufEECshND7PHJU69l_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.a9bd464d-cgu.11: no profile data available for function _RINvNtCs3twH93Ieuaq_4core3ptr13drop_in_placeRINtNtB4_6option6OptionRNtNtCs7Iy1M2Wve7y_3std4path4PathEECshND7PHJU69l_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.a9bd464d-cgu.11: no profile data available for function _RINvNtCs3twH93Ieuaq_4core3ptr13drop_in_placeRINtNtCslABs5WeHvBX_5alloc3vec3VecNtNtCs7Iy1M2Wve7y_3std4path7PathBufEECshND7PHJU69l_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.a9bd464d-cgu.11: no profile data available for function _RINvNtCs3twH93Ieuaq_4core3ptr13drop_in_placeRNtCsADXiRGsqjR_16unic_langid_impl18LanguageIdentifierECshND7PHJU69l_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.a9bd464d-cgu.11: no profile data available for function _RINvNtCs3twH93Ieuaq_4core3ptr13drop_in_placeRNtNtCs7Iy1M2Wve7y_3std4path7PathBufECshND7PHJU69l_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.a9bd464d-cgu.11: no profile data available for function _RINvNtCs3twH93Ieuaq_4core3ptr13drop_in_placeRNtNtCslgieBiDyeGh_13fluent_bundle8resource14FluentResourceECshND7PHJU69l_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.a9bd464d-cgu.11: no profile data available for function _RINvNtCs3twH93Ieuaq_4core3ptr13drop_in_placeRQNtNtCs7Iy1M2Wve7y_3std4path7PathBufECshND7PHJU69l_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.a9bd464d-cgu.11: no profile data available for function _RINvNtCs3twH93Ieuaq_4core3ptr13drop_in_placeRRSReECshND7PHJU69l_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.a9bd464d-cgu.11: no profile data available for function _RINvNtCsbmRcNT3iGNU_12tracing_core5field5debugRINtNtCs3twH93Ieuaq_4core6option6OptionNtCsADXiRGsqjR_16unic_langid_impl18LanguageIdentifierEECshND7PHJU69l_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.a9bd464d-cgu.11: no profile data available for function _RINvNtCsbmRcNT3iGNU_12tracing_core5field5debugRINtNtCs3twH93Ieuaq_4core6option6OptionNtNtCs7Iy1M2Wve7y_3std4path7PathBufEECshND7PHJU69l_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.a9bd464d-cgu.11: no profile data available for function _RINvNtCsbmRcNT3iGNU_12tracing_core5field5debugRINtNtCs3twH93Ieuaq_4core6option6OptionRNtNtCs7Iy1M2Wve7y_3std4path4PathEECshND7PHJU69l_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.a9bd464d-cgu.11: no profile data available for function _RINvNtCsbmRcNT3iGNU_12tracing_core5field5debugRINtNtCslABs5WeHvBX_5alloc3vec3VecNtNtCs7Iy1M2Wve7y_3std4path7PathBufEECshND7PHJU69l_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.a9bd464d-cgu.11: no profile data available for function _RINvNtCsbmRcNT3iGNU_12tracing_core5field5debugRNtCsADXiRGsqjR_16unic_langid_impl18LanguageIdentifierECshND7PHJU69l_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.a9bd464d-cgu.11: no profile data available for function _RINvNtCsbmRcNT3iGNU_12tracing_core5field5debugRNtNtCs7Iy1M2Wve7y_3std4path7PathBufECshND7PHJU69l_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.a9bd464d-cgu.11: no profile data available for function _RINvNtCsbmRcNT3iGNU_12tracing_core5field5debugRNtNtCslgieBiDyeGh_13fluent_bundle8resource14FluentResourceECshND7PHJU69l_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.a9bd464d-cgu.11: no profile data available for function _RINvNtCsbmRcNT3iGNU_12tracing_core5field5debugRQNtNtCs7Iy1M2Wve7y_3std4path7PathBufECshND7PHJU69l_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.a9bd464d-cgu.11: no profile data available for function _RINvNtCsbmRcNT3iGNU_12tracing_core5field5debugRRSReECshND7PHJU69l_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.a9bd464d-cgu.11: no profile data available for function _RNvXsk_NtCsbmRcNT3iGNU_12tracing_core5fieldINtB5_10DebugValueRINtNtCs3twH93Ieuaq_4core6option6OptionNtCsADXiRGsqjR_16unic_langid_impl18LanguageIdentifierEENtB5_5Value6recordCshND7PHJU69l_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.a9bd464d-cgu.11: no profile data available for function _RNvXsk_NtCsbmRcNT3iGNU_12tracing_core5fieldINtB5_10DebugValueRINtNtCs3twH93Ieuaq_4core6option6OptionNtNtCs7Iy1M2Wve7y_3std4path7PathBufEENtB5_5Value6recordCshND7PHJU69l_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.a9bd464d-cgu.11: no profile data available for function _RNvXsk_NtCsbmRcNT3iGNU_12tracing_core5fieldINtB5_10DebugValueRINtNtCs3twH93Ieuaq_4core6option6OptionRNtNtCs7Iy1M2Wve7y_3std4path4PathEENtB5_5Value6recordCshND7PHJU69l_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.a9bd464d-cgu.11: no profile data available for function _RNvXsk_NtCsbmRcNT3iGNU_12tracing_core5fieldINtB5_10DebugValueRINtNtCslABs5WeHvBX_5alloc3vec3VecNtNtCs7Iy1M2Wve7y_3std4path7PathBufEENtB5_5Value6recordCshND7PHJU69l_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.a9bd464d-cgu.11: no profile data available for function _RNvXsk_NtCsbmRcNT3iGNU_12tracing_core5fieldINtB5_10DebugValueRNtCsADXiRGsqjR_16unic_langid_impl18LanguageIdentifierENtB5_5Value6recordCshND7PHJU69l_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.a9bd464d-cgu.11: no profile data available for function _RNvXsk_NtCsbmRcNT3iGNU_12tracing_core5fieldINtB5_10DebugValueRNtNtCs7Iy1M2Wve7y_3std4path7PathBufENtB5_5Value6recordCshND7PHJU69l_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.a9bd464d-cgu.11: no profile data available for function _RNvXsk_NtCsbmRcNT3iGNU_12tracing_core5fieldINtB5_10DebugValueRNtNtCslgieBiDyeGh_13fluent_bundle8resource14FluentResourceENtB5_5Value6recordCshND7PHJU69l_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.a9bd464d-cgu.11: no profile data available for function _RNvXsk_NtCsbmRcNT3iGNU_12tracing_core5fieldINtB5_10DebugValueRQNtNtCs7Iy1M2Wve7y_3std4path7PathBufENtB5_5Value6recordCshND7PHJU69l_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.a9bd464d-cgu.11: no profile data available for function _RNvXsk_NtCsbmRcNT3iGNU_12tracing_core5fieldINtB5_10DebugValueRRSReENtB5_5Value6recordCshND7PHJU69l_20rustc_error_messages Hash = 170957022131388415
[RUSTC-TIMING] rustc_error_messages test:false 1.040
warning: `rustc_error_messages` (lib) generated 30 warnings
[RUSTC-TIMING] rustc_feature test:false 1.174
[RUSTC-TIMING] jemalloc_sys test:false 0.092
---
[RUSTC-TIMING] build_script_build test:false 0.335
error: unused import: `TypeFoldable`
  --> src/tools/clippy/clippy_utils/src/ty.rs:16:89
   |
16 |     self, AdtDef, Binder, FnSig, IntTy, ParamEnv, Predicate, PredicateKind, Ty, TyCtxt, TypeFoldable, UintTy,
   |
   |
   = note: `-D unused-imports` implied by `-D warnings`
error: unused import: `TypeFoldable`
  --> src/tools/clippy/clippy_utils/src/lib.rs:96:91
   |
   |
96 | use rustc_middle::ty::{layout::IntegerExt, BorrowKind, DefIdTree, Ty, TyCtxt, TypeAndMut, TypeFoldable, UpvarCapture};

[RUSTC-TIMING] rustc_tools_util test:false 0.414
[RUSTC-TIMING] build_script_build test:false 0.260
[RUSTC-TIMING] cargo_platform test:false 0.667
[RUSTC-TIMING] cargo_platform test:false 0.667
   Compiling cargo_metadata v0.14.0
[RUSTC-TIMING] unicode_script test:false 0.833
[RUSTC-TIMING] quine_mc_cluskey test:false 0.983
error[E0599]: no method named `needs_infer` found for struct `rustc_middle::ty::Ty` in the current scope
   --> src/tools/clippy/clippy_utils/src/ty.rs:168:17
    |
168 |     assert!(!ty.needs_infer());
    |
    = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
    |
    |
5   | use crate::rustc_middle::ty::TypeVisitable;
    |

error[E0599]: no method named `has_escaping_bound_vars` found for struct `rustc_middle::ty::Ty` in the current scope
    |
    |
171 |     if ty.has_escaping_bound_vars() {
    |
    = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
    |
    |
5   | use crate::rustc_middle::ty::TypeVisitable;
    |

[RUSTC-TIMING] camino test:false 0.856
error[E0599]: no method named `is_global` found for reference `&rustc_middle::ty::Predicate<'_>` in the current scope
     |
     |
1901 |         .filter_map(|(p, _)| if p.is_global() { Some(*p) } else { None });
     |                                   ^^^^^^^^^ method not found in `&rustc_middle::ty::Predicate<'_>`
     = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
     |
20   | use crate::rustc_middle::ty::TypeVisitable;
---
[RUSTC-TIMING] build_script_build test:false 0.296
warning: unused import: `TypeFoldable`
  --> src/tools/clippy/clippy_utils/src/ty.rs:16:89
   |
16 |     self, AdtDef, Binder, FnSig, IntTy, ParamEnv, Predicate, PredicateKind, Ty, TyCtxt, TypeFoldable, UintTy,
   |
   = note: `#[warn(unused_imports)]` on by default

warning: unused import: `TypeFoldable`
warning: unused import: `TypeFoldable`
  --> src/tools/clippy/clippy_utils/src/lib.rs:96:91
   |
96 | use rustc_middle::ty::{layout::IntegerExt, BorrowKind, DefIdTree, Ty, TyCtxt, TypeAndMut, TypeFoldable, UpvarCapture};

[RUSTC-TIMING] futures_core test:false 0.133
[RUSTC-TIMING] num_cpus test:false 1.535
[RUSTC-TIMING] getrandom test:false 0.433
---
[RUSTC-TIMING] bitmaps test:false 1.674
error[E0599]: no method named `needs_infer` found for struct `rustc_middle::ty::Ty` in the current scope
   --> src/tools/clippy/clippy_utils/src/ty.rs:168:17
    |
168 |     assert!(!ty.needs_infer());
    |
    = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
    |
    |
5   | use crate::rustc_middle::ty::TypeVisitable;
    |

error[E0599]: no method named `has_escaping_bound_vars` found for struct `rustc_middle::ty::Ty` in the current scope
    |
    |
171 |     if ty.has_escaping_bound_vars() {
    |
    = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
    |
    |
5   | use crate::rustc_middle::ty::TypeVisitable;
    |

[RUSTC-TIMING] heck test:false 0.930
error[E0599]: no method named `is_global` found for reference `&rustc_middle::ty::Predicate<'_>` in the current scope
     |
     |
1901 |         .filter_map(|(p, _)| if p.is_global() { Some(*p) } else { None });
     |                                   ^^^^^^^^^ method not found in `&rustc_middle::ty::Predicate<'_>`
     = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
     |
20   | use crate::rustc_middle::ty::TypeVisitable;
---
[TIMING] tool::RustDemangler { compiler: Compiler { stage: 1, host: x86_64-unknown-linux-gnu }, target: x86_64-unknown-linux-gnu, extra_features: [] } -- 0.000
Dist rust-demangler-nightly-x86_64-unknown-linux-gnu
 finished in 2.051 seconds
[TIMING] dist::RustDemangler { compiler: Compiler { stage: 1, host: x86_64-unknown-linux-gnu }, target: x86_64-unknown-linux-gnu } -- 2.058
thread 'main' panicked at 'clippy expected to build - essential tool', src/bootstrap/dist.rs:1095:14
Build completed unsuccessfully in 0:40:10
