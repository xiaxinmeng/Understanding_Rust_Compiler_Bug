plain
Executing benchmark regex-1.5.5 (4/7)
Preparing regex-1.5.5
[2022-05-29T07:16:13Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-29T07:16:13Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-29T07:16:13Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpMXrqLE#regex:1.5.5" "--release" "--" "--skip-this-rustc"
[2022-05-29T07:16:13Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp6NwqNF#regex:1.5.5" "--" "--skip-this-rustc"
[2022-05-29T07:16:28Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-29T07:16:28Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-29T07:16:28Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-29T07:16:28Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpWPQMUa#regex:1.5.5" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-29T07:16:32Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-29T07:16:32Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-29T07:16:32Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp0ffvqg#regex:1.5.5" "--release" "--" "--wrap-rustc-with" "Eprintln"
Finished benchmark regex-1.5.5 (4/7)
---
Preparing bitmaps-3.1.0
[2022-05-29T07:33:41Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-05-29T07:33:41Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-29T07:33:41Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-29T07:33:41Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpy8eSsa#bitmaps:3.1.0" "--" "--skip-this-rustc"
[2022-05-29T07:33:41Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpYFnUBa#bitmaps:3.1.0" "--profile" "check" "--" "--skip-this-rustc"
[2022-05-29T07:33:41Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp14NIiW#bitmaps:3.1.0" "--release" "--" "--skip-this-rustc"
[2022-05-29T07:33:42Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-29T07:33:42Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-05-29T07:33:42Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpHC2TAT#bitmaps:3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-29T07:33:44Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-05-29T07:33:44Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-05-29T07:33:44Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpHC2TAT#bitmaps:3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpHC2TAT/incremental-state"
[2022-05-29T07:33:49Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-05-29T07:33:49Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpHC2TAT#bitmaps:3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpHC2TAT/incremental-state"
[2022-05-29T07:33:49Z DEBUG collector::execute] applying println to "/tmp/.tmpHC2TAT"
[2022-05-29T07:33:49Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-29T07:33:49Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-29T07:33:50Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpHC2TAT#bitmaps:3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpHC2TAT/incremental-state"
[2022-05-29T07:33:51Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-29T07:33:51Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-29T07:33:51Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzgF2G8#bitmaps:3.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-29T07:33:54Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-29T07:33:54Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-29T07:33:54Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzgF2G8#bitmaps:3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpzgF2G8/incremental-state"
[2022-05-29T07:33:58Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-05-29T07:33:58Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzgF2G8#bitmaps:3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpzgF2G8/incremental-state"
[2022-05-29T07:33:59Z DEBUG collector::execute] applying println to "/tmp/.tmpzgF2G8"
[2022-05-29T07:33:59Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-29T07:33:59Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-29T07:33:59Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzgF2G8#bitmaps:3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpzgF2G8/incremental-state"
[2022-05-29T07:34:00Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-29T07:34:00Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-29T07:34:00Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-29T07:34:00Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOVagpi#bitmaps:3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-29T07:34:03Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-29T07:34:03Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOVagpi#bitmaps:3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpOVagpi/incremental-state"
[2022-05-29T07:34:08Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-05-29T07:34:08Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOVagpi#bitmaps:3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpOVagpi/incremental-state"
[2022-05-29T07:34:09Z DEBUG collector::execute] applying println to "/tmp/.tmpOVagpi"
[2022-05-29T07:34:09Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-29T07:34:09Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-29T07:34:09Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOVagpi#bitmaps:3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpOVagpi/incremental-state"
Executing benchmark cargo-0.60.0 (2/8)
Preparing cargo-0.60.0
[2022-05-29T07:34:10Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-29T07:34:10Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
---
[2022-05-29T07:35:24Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-29T07:35:24Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-05-29T07:35:24Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpYVdys4#cargo:0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-29T07:35:51Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-05-29T07:35:51Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpYVdys4#cargo:0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpYVdys4/incremental-state"
[2022-05-29T07:36:23Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-05-29T07:36:23Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpYVdys4#cargo:0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpYVdys4/incremental-state"
[2022-05-29T07:36:29Z DEBUG collector::execute] applying println to "/tmp/.tmpYVdys4"
[2022-05-29T07:36:29Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-29T07:36:29Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-29T07:36:29Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpYVdys4#cargo:0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpYVdys4/incremental-state"
[2022-05-29T07:36:43Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-29T07:36:44Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-29T07:36:44Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmppqCdSI#cargo:0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-29T07:37:47Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-29T07:37:47Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-29T07:37:48Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmppqCdSI#cargo:0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmppqCdSI/incremental-state"
[2022-05-29T07:39:06Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-05-29T07:39:06Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmppqCdSI#cargo:0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmppqCdSI/incremental-state"
[2022-05-29T07:39:21Z DEBUG collector::execute] applying println to "/tmp/.tmppqCdSI"
[2022-05-29T07:39:21Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-29T07:39:21Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-29T07:39:21Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmppqCdSI#cargo:0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmppqCdSI/incremental-state"
[2022-05-29T07:39:51Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-29T07:39:51Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-29T07:39:51Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpuJAEXG#cargo:0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-29T07:41:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-29T07:41:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-29T07:41:12Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpuJAEXG#cargo:0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpuJAEXG/incremental-state"
[2022-05-29T07:42:39Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-05-29T07:42:39Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpuJAEXG#cargo:0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpuJAEXG/incremental-state"
[2022-05-29T07:42:53Z DEBUG collector::execute] applying println to "/tmp/.tmpuJAEXG"
[2022-05-29T07:42:53Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-29T07:42:53Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-29T07:42:53Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpuJAEXG#cargo:0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpuJAEXG/incremental-state"
Executing benchmark ctfe-stress-5 (3/8)
Preparing ctfe-stress-5
[2022-05-29T07:43:31Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-29T07:43:31Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
---
[2022-05-29T07:43:55Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-29T07:43:55Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-29T07:43:55Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmptbdoCr#ctfe-stress-5:0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-29T07:44:05Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-29T07:44:05Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmptbdoCr#ctfe-stress-5:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmptbdoCr/incremental-state"
[2022-05-29T07:44:17Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-05-29T07:44:17Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmptbdoCr#ctfe-stress-5:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmptbdoCr/incremental-state"
[2022-05-29T07:44:17Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-29T07:44:17Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-29T07:44:17Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpknIsFt#ctfe-stress-5:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-29T07:44:27Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-29T07:44:27Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-29T07:44:27Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpknIsFt#ctfe-stress-5:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpknIsFt/incremental-state"
[2022-05-29T07:44:38Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-05-29T07:44:38Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpknIsFt#ctfe-stress-5:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpknIsFt/incremental-state"
Executing benchmark diesel-1.4.8 (4/8)
Preparing diesel-1.4.8
[2022-05-29T07:44:39Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-29T07:44:39Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-29T07:44:39Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-29T07:44:39Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-05-29T07:44:39Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpRSaDfI#diesel:1.4.8" "--" "--skip-this-rustc"
[2022-05-29T07:44:39Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzfsdsY#diesel:1.4.8" "--release" "--" "--skip-this-rustc"
[2022-05-29T07:44:39Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp0Lbttb#diesel:1.4.8" "--profile" "check" "--" "--skip-this-rustc"
[2022-05-29T07:45:04Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-29T07:45:04Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-05-29T07:45:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQtLeU8#diesel:1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-29T07:45:25Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-05-29T07:45:25Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-05-29T07:45:25Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQtLeU8#diesel:1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpQtLeU8/incremental-state"
[2022-05-29T07:45:50Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-05-29T07:45:50Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQtLeU8#diesel:1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpQtLeU8/incremental-state"
[2022-05-29T07:45:54Z DEBUG collector::execute] applying println to "/tmp/.tmpQtLeU8"
[2022-05-29T07:45:54Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-29T07:45:54Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-29T07:45:54Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQtLeU8#diesel:1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpQtLeU8/incremental-state"
[2022-05-29T07:45:59Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-29T07:45:59Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-29T07:45:59Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-29T07:45:59Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpaHBDih#diesel:1.4.8" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-29T07:46:22Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-29T07:46:22Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpaHBDih#diesel:1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpaHBDih/incremental-state"
[2022-05-29T07:46:51Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-05-29T07:46:51Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpaHBDih#diesel:1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpaHBDih/incremental-state"
[2022-05-29T07:46:56Z DEBUG collector::execute] applying println to "/tmp/.tmpaHBDih"
[2022-05-29T07:46:56Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-29T07:46:56Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-29T07:46:56Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpaHBDih#diesel:1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpaHBDih/incremental-state"
[2022-05-29T07:47:01Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-29T07:47:01Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-29T07:47:01Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-29T07:47:01Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpImqxbA#diesel:1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-29T07:47:25Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-29T07:47:25Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpImqxbA#diesel:1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpImqxbA/incremental-state"
[2022-05-29T07:47:54Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-05-29T07:47:54Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpImqxbA#diesel:1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpImqxbA/incremental-state"
[2022-05-29T07:47:58Z DEBUG collector::execute] applying println to "/tmp/.tmpImqxbA"
[2022-05-29T07:47:58Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-29T07:47:58Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-29T07:47:58Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpImqxbA#diesel:1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpImqxbA/incremental-state"
Executing benchmark externs (5/8)
Preparing externs
[2022-05-29T07:48:04Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-05-29T07:48:04Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2022-05-29T07:48:04Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-29T07:48:04Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-05-29T07:48:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp5hGFYO#externs:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-29T07:48:05Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-05-29T07:48:05Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp5hGFYO#externs:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp5hGFYO/incremental-state"
[2022-05-29T07:48:06Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-05-29T07:48:06Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp5hGFYO#externs:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp5hGFYO/incremental-state"
[2022-05-29T07:48:07Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-29T07:48:07Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-29T07:48:07Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpAl8aEO#externs:0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-29T07:48:08Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-29T07:48:08Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-29T07:48:08Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpAl8aEO#externs:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpAl8aEO/incremental-state"
[2022-05-29T07:48:09Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-05-29T07:48:09Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpAl8aEO#externs:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpAl8aEO/incremental-state"
[2022-05-29T07:48:10Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-29T07:48:10Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-29T07:48:10Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp4TiVpM#externs:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-29T07:48:11Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-29T07:48:11Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-29T07:48:11Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp4TiVpM#externs:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp4TiVpM/incremental-state"
[2022-05-29T07:48:13Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-05-29T07:48:13Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp4TiVpM#externs:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp4TiVpM/incremental-state"
Executing benchmark match-stress (6/8)
Preparing match-stress
[2022-05-29T07:48:13Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-29T07:48:13Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-05-29T07:48:13Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-05-29T07:48:13Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-29T07:48:13Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpmCELhb#match-stress:0.1.0" "--" "--skip-this-rustc"
[2022-05-29T07:48:13Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJpunyV#match-stress:0.1.0" "--profile" "check" "--" "--skip-this-rustc"
[2022-05-29T07:48:13Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpqzw4u2#match-stress:0.1.0" "--release" "--" "--skip-this-rustc"
[2022-05-29T07:48:14Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-29T07:48:14Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-05-29T07:48:14Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDI2Yja#match-stress:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-29T07:48:17Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-05-29T07:48:17Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-05-29T07:48:17Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDI2Yja#match-stress:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpDI2Yja/incremental-state"
[2022-05-29T07:48:22Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-05-29T07:48:22Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDI2Yja#match-stress:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpDI2Yja/incremental-state"
[2022-05-29T07:48:23Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-29T07:48:23Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-29T07:48:23Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpSD2Q9K#match-stress:0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-29T07:48:27Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
---
[2022-05-29T07:48:33Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-29T07:48:33Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-29T07:48:33Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpWIYsDQ#match-stress:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-29T07:48:37Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-29T07:48:37Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpWIYsDQ#match-stress:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpWIYsDQ/incremental-state"
[2022-05-29T07:48:41Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-05-29T07:48:41Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpWIYsDQ#match-stress:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpWIYsDQ/incremental-state"
Executing benchmark token-stream-stress (7/8)
Preparing token-stream-stress
[2022-05-29T07:48:43Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-29T07:48:43Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2022-05-29T07:48:47Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpYqAyDa#tuple-stress:0.1.0" "--release" "--" "--skip-this-rustc"
Running tuple-stress: Check + [Full, IncrFull, IncrUnchanged, IncrPatched]
[2022-05-29T07:48:47Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-29T07:48:47Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-05-29T07:48:47Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCYTMXQ#tuple-stress:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-29T07:48:53Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-05-29T07:48:53Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCYTMXQ#tuple-stress:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpCYTMXQ/incremental-state"
[2022-05-29T07:49:00Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-05-29T07:49:00Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCYTMXQ#tuple-stress:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpCYTMXQ/incremental-state"
[2022-05-29T07:49:02Z DEBUG collector::execute] applying new row to "/tmp/.tmpCYTMXQ"
[2022-05-29T07:49:02Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-05-29T07:49:02Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-05-29T07:49:02Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCYTMXQ#tuple-stress:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpCYTMXQ/incremental-state"
[2022-05-29T07:49:09Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-29T07:49:09Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-29T07:49:09Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpxgGcSQ#tuple-stress:0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-29T07:49:15Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-29T07:49:15Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-29T07:49:15Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpxgGcSQ#tuple-stress:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpxgGcSQ/incremental-state"
[2022-05-29T07:49:22Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-05-29T07:49:22Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpxgGcSQ#tuple-stress:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpxgGcSQ/incremental-state"
[2022-05-29T07:49:24Z DEBUG collector::execute] applying new row to "/tmp/.tmpxgGcSQ"
[2022-05-29T07:49:24Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-05-29T07:49:24Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-05-29T07:49:24Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpxgGcSQ#tuple-stress:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpxgGcSQ/incremental-state"
[2022-05-29T07:49:31Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-29T07:49:31Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-29T07:49:31Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp46SmCS#tuple-stress:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-29T07:49:37Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
[RUSTC-TIMING] arrayvec test:false 0.293
   Compiling tinyvec v0.3.4
[RUSTC-TIMING] autocfg test:false 0.639
   Compiling regex-syntax v0.6.25
warning: rustc_fs_util.5e8fc7ac-cgu.2: no profile data available for function _RNvXs2_NtCsjptrfQZq4Em_4core7convertNtNtNtB7_5alloc6layout11LayoutErrorINtB5_4FromBy_E4fromCs8wBidqGmIJA_13rustc_fs_util Hash = 742261418966908927

warning: rustc_graphviz.cc7a4c19-cgu.10: no profile data available for function _RNvXs2_NtCsjptrfQZq4Em_4core7convertNtNtNtB7_5alloc6layout11LayoutErrorINtB5_4FromBy_E4fromCs3RawBqgILQz_14rustc_graphviz Hash = 742261418966908927
[RUSTC-TIMING] rustc_fs_util test:false 0.127
warning: `rustc_fs_util` (lib) generated 1 warning
   Compiling ansi_term v0.12.1
   Compiling ansi_term v0.12.1
warning: rustc_graphviz.cc7a4c19-cgu.4: no profile data available for function _RNvMNtNtNtCsjptrfQZq4Em_4core4iter8adapters3revINtB2_3RevINtNtNtB8_3ops5range5RangejEE3newCs3RawBqgILQz_14rustc_graphviz Hash = 742261418966908927
[RUSTC-TIMING] either test:false 0.170
   Compiling crc32fast v1.2.0
[RUSTC-TIMING] ppv_lite86 test:false 0.577
   Compiling snap v1.0.1
---
   Compiling measureme v10.0.0
[RUSTC-TIMING] flate2 test:false 1.211
[RUSTC-TIMING] jobserver test:false 1.534
   Compiling rand v0.8.5
warning: rustc_llvm.383416c9-cgu.2: no profile data available for function _RNvXs2_NtCsjptrfQZq4Em_4core7convertNtNtNtB7_5alloc6layout11LayoutErrorINtB5_4FromBy_E4fromCs5DUHknTMBHV_10rustc_llvm Hash = 742261418966908927
[RUSTC-TIMING] cstr test:false 0.696
   Compiling matchers v0.1.0
[RUSTC-TIMING] rustc_llvm test:false 0.264
warning: `rustc_llvm` (lib) generated 1 warning
---
   Compiling rustc_ast v0.0.0 (/checkout/compiler/rustc_ast)
   Compiling rustc_feature v0.0.0 (/checkout/compiler/rustc_feature)
   Compiling rustc_target v0.0.0 (/checkout/compiler/rustc_target)
   Compiling rustc_error_messages v0.0.0 (/checkout/compiler/rustc_error_messages)
warning: rustc_error_messages.42795cbf-cgu.11: no profile data available for function _RINvMsx_NtCs9LCLh1xCWLV_12tracing_core5fieldNtB6_8FieldSet9value_setATRNtB6_5FieldINtNtCsjptrfQZq4Em_4core6option6OptionRDNtB6_5ValueEL_EEj1_ECsj77h8Ju3xmZ_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.42795cbf-cgu.11: no profile data available for function _RINvMsx_NtCs9LCLh1xCWLV_12tracing_core5fieldNtB6_8FieldSet9value_setATRNtB6_5FieldINtNtCsjptrfQZq4Em_4core6option6OptionRDNtB6_5ValueEL_EEj2_ECsj77h8Ju3xmZ_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.42795cbf-cgu.11: no profile data available for function _RINvMsx_NtCs9LCLh1xCWLV_12tracing_core5fieldNtB6_8FieldSet9value_setATRNtB6_5FieldINtNtCsjptrfQZq4Em_4core6option6OptionRDNtB6_5ValueEL_EEj5_ECsj77h8Ju3xmZ_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.42795cbf-cgu.11: no profile data available for function _RINvNtCs9LCLh1xCWLV_12tracing_core5field5debugRINtNtCsaLBvu7ICNX8_5alloc3vec3VecNtNtCsa7WvuEjxA1e_3std4path7PathBufEECsj77h8Ju3xmZ_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.42795cbf-cgu.11: no profile data available for function _RINvNtCs9LCLh1xCWLV_12tracing_core5field5debugRINtNtCsjptrfQZq4Em_4core6option6OptionNtCs89ziMmiLBgy_16unic_langid_impl18LanguageIdentifierEECsj77h8Ju3xmZ_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.42795cbf-cgu.11: no profile data available for function _RINvNtCs9LCLh1xCWLV_12tracing_core5field5debugRINtNtCsjptrfQZq4Em_4core6option6OptionNtNtCsa7WvuEjxA1e_3std4path7PathBufEECsj77h8Ju3xmZ_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.42795cbf-cgu.11: no profile data available for function _RINvNtCs9LCLh1xCWLV_12tracing_core5field5debugRINtNtCsjptrfQZq4Em_4core6option6OptionRNtNtCsa7WvuEjxA1e_3std4path4PathEECsj77h8Ju3xmZ_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.42795cbf-cgu.11: no profile data available for function _RINvNtCs9LCLh1xCWLV_12tracing_core5field5debugRNtCs89ziMmiLBgy_16unic_langid_impl18LanguageIdentifierECsj77h8Ju3xmZ_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.42795cbf-cgu.11: no profile data available for function _RINvNtCs9LCLh1xCWLV_12tracing_core5field5debugRNtNtCsa7WvuEjxA1e_3std4path7PathBufECsj77h8Ju3xmZ_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.42795cbf-cgu.11: no profile data available for function _RINvNtCs9LCLh1xCWLV_12tracing_core5field5debugRNtNtCsfhE9ZFxOy7o_13fluent_bundle8resource14FluentResourceECsj77h8Ju3xmZ_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.42795cbf-cgu.11: no profile data available for function _RINvNtCs9LCLh1xCWLV_12tracing_core5field5debugRQNtNtCsa7WvuEjxA1e_3std4path7PathBufECsj77h8Ju3xmZ_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.42795cbf-cgu.11: no profile data available for function _RINvNtCs9LCLh1xCWLV_12tracing_core5field5debugRRSReECsj77h8Ju3xmZ_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.42795cbf-cgu.11: no profile data available for function _RINvNtCsjptrfQZq4Em_4core3ptr13drop_in_placeRINtNtB4_6option6OptionNtCs89ziMmiLBgy_16unic_langid_impl18LanguageIdentifierEECsj77h8Ju3xmZ_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.42795cbf-cgu.11: no profile data available for function _RINvNtCsjptrfQZq4Em_4core3ptr13drop_in_placeRINtNtB4_6option6OptionNtNtCsa7WvuEjxA1e_3std4path7PathBufEECsj77h8Ju3xmZ_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.42795cbf-cgu.11: no profile data available for function _RINvNtCsjptrfQZq4Em_4core3ptr13drop_in_placeRINtNtB4_6option6OptionRNtNtCsa7WvuEjxA1e_3std4path4PathEECsj77h8Ju3xmZ_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.42795cbf-cgu.11: no profile data available for function _RINvNtCsjptrfQZq4Em_4core3ptr13drop_in_placeRINtNtCsaLBvu7ICNX8_5alloc3vec3VecNtNtCsa7WvuEjxA1e_3std4path7PathBufEECsj77h8Ju3xmZ_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.42795cbf-cgu.11: no profile data available for function _RINvNtCsjptrfQZq4Em_4core3ptr13drop_in_placeRNtCs89ziMmiLBgy_16unic_langid_impl18LanguageIdentifierECsj77h8Ju3xmZ_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.42795cbf-cgu.11: no profile data available for function _RINvNtCsjptrfQZq4Em_4core3ptr13drop_in_placeRNtNtCsa7WvuEjxA1e_3std4path7PathBufECsj77h8Ju3xmZ_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.42795cbf-cgu.11: no profile data available for function _RINvNtCsjptrfQZq4Em_4core3ptr13drop_in_placeRNtNtCsfhE9ZFxOy7o_13fluent_bundle8resource14FluentResourceECsj77h8Ju3xmZ_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.42795cbf-cgu.11: no profile data available for function _RINvNtCsjptrfQZq4Em_4core3ptr13drop_in_placeRQNtNtCsa7WvuEjxA1e_3std4path7PathBufECsj77h8Ju3xmZ_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.42795cbf-cgu.11: no profile data available for function _RINvNtCsjptrfQZq4Em_4core3ptr13drop_in_placeRRSReECsj77h8Ju3xmZ_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.42795cbf-cgu.11: no profile data available for function _RNvXsk_NtCs9LCLh1xCWLV_12tracing_core5fieldINtB5_10DebugValueRINtNtCsaLBvu7ICNX8_5alloc3vec3VecNtNtCsa7WvuEjxA1e_3std4path7PathBufEENtB5_5Value6recordCsj77h8Ju3xmZ_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.42795cbf-cgu.11: no profile data available for function _RNvXsk_NtCs9LCLh1xCWLV_12tracing_core5fieldINtB5_10DebugValueRINtNtCsjptrfQZq4Em_4core6option6OptionNtCs89ziMmiLBgy_16unic_langid_impl18LanguageIdentifierEENtB5_5Value6recordCsj77h8Ju3xmZ_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.42795cbf-cgu.11: no profile data available for function _RNvXsk_NtCs9LCLh1xCWLV_12tracing_core5fieldINtB5_10DebugValueRINtNtCsjptrfQZq4Em_4core6option6OptionNtNtCsa7WvuEjxA1e_3std4path7PathBufEENtB5_5Value6recordCsj77h8Ju3xmZ_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.42795cbf-cgu.11: no profile data available for function _RNvXsk_NtCs9LCLh1xCWLV_12tracing_core5fieldINtB5_10DebugValueRINtNtCsjptrfQZq4Em_4core6option6OptionRNtNtCsa7WvuEjxA1e_3std4path4PathEENtB5_5Value6recordCsj77h8Ju3xmZ_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.42795cbf-cgu.11: no profile data available for function _RNvXsk_NtCs9LCLh1xCWLV_12tracing_core5fieldINtB5_10DebugValueRNtCs89ziMmiLBgy_16unic_langid_impl18LanguageIdentifierENtB5_5Value6recordCsj77h8Ju3xmZ_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.42795cbf-cgu.11: no profile data available for function _RNvXsk_NtCs9LCLh1xCWLV_12tracing_core5fieldINtB5_10DebugValueRNtNtCsa7WvuEjxA1e_3std4path7PathBufENtB5_5Value6recordCsj77h8Ju3xmZ_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.42795cbf-cgu.11: no profile data available for function _RNvXsk_NtCs9LCLh1xCWLV_12tracing_core5fieldINtB5_10DebugValueRNtNtCsfhE9ZFxOy7o_13fluent_bundle8resource14FluentResourceENtB5_5Value6recordCsj77h8Ju3xmZ_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.42795cbf-cgu.11: no profile data available for function _RNvXsk_NtCs9LCLh1xCWLV_12tracing_core5fieldINtB5_10DebugValueRQNtNtCsa7WvuEjxA1e_3std4path7PathBufENtB5_5Value6recordCsj77h8Ju3xmZ_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.42795cbf-cgu.11: no profile data available for function _RNvXsk_NtCs9LCLh1xCWLV_12tracing_core5fieldINtB5_10DebugValueRRSReENtB5_5Value6recordCsj77h8Ju3xmZ_20rustc_error_messages Hash = 170957022131388415
[RUSTC-TIMING] rustc_error_messages test:false 1.023
warning: `rustc_error_messages` (lib) generated 30 warnings
[RUSTC-TIMING] rustc_feature test:false 1.225
[RUSTC-TIMING] jemalloc_sys test:false 0.069
