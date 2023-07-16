plain
[2022-09-05T10:40:24Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-09-05T10:40:24Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-09-05T10:40:24Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpK9FybH#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-09-05T10:40:26Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-09-05T10:40:26Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpK9FybH#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpK9FybH/incremental-state"
[2022-09-05T10:40:29Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-09-05T10:40:29Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpK9FybH#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpK9FybH/incremental-state"
[2022-09-05T10:40:30Z DEBUG collector::execute] applying println to "/tmp/.tmpK9FybH"
[2022-09-05T10:40:30Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-09-05T10:40:30Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-09-05T10:40:30Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpK9FybH#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpK9FybH/incremental-state"
[2022-09-05T10:40:31Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-09-05T10:40:31Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-09-05T10:40:31Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpucGOtg#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-09-05T10:40:33Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-09-05T10:40:33Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-09-05T10:40:33Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpucGOtg#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpucGOtg/incremental-state"
[2022-09-05T10:40:36Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-09-05T10:40:36Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpucGOtg#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpucGOtg/incremental-state"
[2022-09-05T10:40:36Z DEBUG collector::execute] applying println to "/tmp/.tmpucGOtg"
[2022-09-05T10:40:36Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-09-05T10:40:36Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-09-05T10:40:36Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpucGOtg#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpucGOtg/incremental-state"
[2022-09-05T10:40:37Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-09-05T10:40:37Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-09-05T10:40:38Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpL0uWW2#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-09-05T10:40:40Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
[2022-09-05T10:42:38Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-09-05T10:42:38Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-09-05T10:42:38Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpW5njpj#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2022-09-05T10:43:28Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-09-05T10:43:28Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpW5njpj#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpW5njpj/incremental-state"
[2022-09-05T10:44:27Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-09-05T10:44:27Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpW5njpj#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpW5njpj/incremental-state"
[2022-09-05T10:44:37Z DEBUG collector::execute] applying println to "/tmp/.tmpW5njpj"
[2022-09-05T10:44:37Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-09-05T10:44:37Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-09-05T10:44:37Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpW5njpj#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpW5njpj/incremental-state"
[2022-09-05T10:44:59Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-09-05T10:45:00Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-09-05T10:45:00Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJMFgvV#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2022-09-05T10:45:58Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-09-05T10:45:58Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-09-05T10:45:58Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJMFgvV#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpJMFgvV/incremental-state"
[2022-09-05T10:46:56Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-09-05T10:46:56Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJMFgvV#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpJMFgvV/incremental-state"
[2022-09-05T10:47:06Z DEBUG collector::execute] applying println to "/tmp/.tmpJMFgvV"
[2022-09-05T10:47:06Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-09-05T10:47:06Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-09-05T10:47:06Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJMFgvV#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpJMFgvV/incremental-state"
Executing benchmark ctfe-stress-5 (3/8)
Preparing ctfe-stress-5
[2022-09-05T10:47:33Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-09-05T10:47:33Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
---
[2022-09-05T10:47:34Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-09-05T10:47:34Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-09-05T10:47:34Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDOOJMz#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-09-05T10:47:42Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-09-05T10:47:42Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDOOJMz#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpDOOJMz/incremental-state"
[2022-09-05T10:47:52Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-09-05T10:47:52Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDOOJMz#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpDOOJMz/incremental-state"
[2022-09-05T10:47:52Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-09-05T10:47:52Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-09-05T10:47:52Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp9Pc1cL#ctfe-stress-5@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-09-05T10:48:01Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
---
[2022-09-05T10:48:48Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-09-05T10:48:48Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-09-05T10:48:48Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpPooOXh#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-09-05T10:49:04Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-09-05T10:49:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpPooOXh#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpPooOXh/incremental-state"
[2022-09-05T10:49:23Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-09-05T10:49:23Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpPooOXh#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpPooOXh/incremental-state"
[2022-09-05T10:49:26Z DEBUG collector::execute] applying println to "/tmp/.tmpPooOXh"
[2022-09-05T10:49:26Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-09-05T10:49:26Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-09-05T10:49:26Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpPooOXh#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpPooOXh/incremental-state"
[2022-09-05T10:49:30Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-09-05T10:49:30Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-09-05T10:49:30Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpUQXrXm#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln"
[2022-09-05T10:49:49Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-09-05T10:49:49Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-09-05T10:49:49Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpUQXrXm#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpUQXrXm/incremental-state"
[2022-09-05T10:50:11Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-09-05T10:50:11Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpUQXrXm#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpUQXrXm/incremental-state"
[2022-09-05T10:50:14Z DEBUG collector::execute] applying println to "/tmp/.tmpUQXrXm"
[2022-09-05T10:50:14Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-09-05T10:50:14Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-09-05T10:50:14Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpUQXrXm#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpUQXrXm/incremental-state"
[2022-09-05T10:50:19Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-09-05T10:50:19Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-09-05T10:50:19Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpu22PlK#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-09-05T10:50:38Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
[2022-09-05T10:51:08Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-09-05T10:51:08Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-09-05T10:51:08Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwhjgQG#externs@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-09-05T10:51:09Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-09-05T10:51:09Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwhjgQG#externs@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpwhjgQG/incremental-state"
[2022-09-05T10:51:09Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-09-05T10:51:09Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwhjgQG#externs@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpwhjgQG/incremental-state"
[2022-09-05T10:51:10Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-09-05T10:51:10Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-09-05T10:51:10Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpv89a1J#externs@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-09-05T10:51:11Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
---
[2022-09-05T10:51:12Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-09-05T10:51:12Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-09-05T10:51:12Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpnPUPwH#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-09-05T10:51:13Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-09-05T10:51:13Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpnPUPwH#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpnPUPwH/incremental-state"
[2022-09-05T10:51:14Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-09-05T10:51:14Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpnPUPwH#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpnPUPwH/incremental-state"
Executing benchmark match-stress (6/8)
Preparing match-stress
[2022-09-05T10:51:15Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-09-05T10:51:15Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2022-09-05T10:51:15Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-09-05T10:51:15Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-09-05T10:51:15Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpxdxmKN#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-09-05T10:51:18Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-09-05T10:51:18Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpxdxmKN#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpxdxmKN/incremental-state"
[2022-09-05T10:51:21Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-09-05T10:51:21Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpxdxmKN#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpxdxmKN/incremental-state"
[2022-09-05T10:51:22Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-09-05T10:51:22Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-09-05T10:51:22Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKLPHX9#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-09-05T10:51:25Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
---
[2022-09-05T10:51:38Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-09-05T10:51:38Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-09-05T10:51:38Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpGGfkxA#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2022-09-05T10:51:39Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-09-05T10:51:39Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpGGfkxA#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpGGfkxA/incremental-state"
[2022-09-05T10:51:39Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-09-05T10:51:39Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpGGfkxA#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpGGfkxA/incremental-state"
Executing benchmark tuple-stress (8/8)
Preparing tuple-stress
[2022-09-05T10:51:39Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-09-05T10:51:39Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
---
   Compiling either v1.6.0
[RUSTC-TIMING] self_cell test:false 0.061
   Compiling datafrog v2.0.1
[RUSTC-TIMING] thin_vec test:false 0.278
warning: rustc_graphviz.ea1211c4-cgu.0: no profile data available for function _RNvMNtNtNtCskZvspLqNnVi_4core4iter8adapters3revINtB2_3RevINtNtNtB8_3ops5range5RangejEE3newCs3Oihf1CiN4m_14rustc_graphviz Hash = 742261418966908927 up to 0 count discarded
   Compiling memchr v2.5.0
[RUSTC-TIMING] lazy_static test:false 0.056
   Compiling tinyvec v0.3.4
[RUSTC-TIMING] build_script_build test:false 0.273
---
[RUSTC-TIMING] aho_corasick test:false 2.379
   Compiling object v0.29.0
[RUSTC-TIMING] regex_automata test:false 2.918
[RUSTC-TIMING] tempfile test:false 0.846
warning: rustc_serialize.93807e15-cgu.7: no profile data available for function _RINvNtCskZvspLqNnVi_4core3ptr13drop_in_placeRjECszymOWxUP75_15rustc_serialize Hash = 742261418966908927 up to 0 count discarded

warning: rustc_serialize.93807e15-cgu.7: no profile data available for function _RINvNtCskZvspLqNnVi_4core9panicking13assert_failedjjECszymOWxUP75_15rustc_serialize Hash = 742261418966908927 up to 0 count discarded

warning: rustc_serialize.93807e15-cgu.10: no profile data available for function _RNvXsV_NtCskZvspLqNnVi_4core3fmtRjNtB5_5Debug3fmtCszymOWxUP75_15rustc_serialize Hash = 1124680650125156080 up to 0 count discarded
[RUSTC-TIMING] rustc_serialize test:false 0.534
warning: `rustc_serialize` (lib) generated 3 warnings
[RUSTC-TIMING] sha2 test:false 1.318
[RUSTC-TIMING] regex_syntax test:false 6.193
---
 Documenting rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
error: unresolved link to `DefId`
   --> compiler/rustc_query_impl/src/plumbing.rs:515:17
    |
515 |             $($(#[$attr])*  $name: QueryState<<queries::$name<'tcx> as QueryConfig>::Key>,)*
    |
   ::: compiler/rustc_query_impl/src/lib.rs:57:1
    |
    |
57  | rustc_query_append! { define_queries! }
    |
    |
    = note: `-D rustdoc::broken-intra-doc-links` implied by `-D warnings`
    = note: the link appears in this line:
            
            Returns the [`Ty`][rustc_middle::ty::Ty] of the given `AnonConst`'s [`DefId`].
    = note: no item named `DefId` in scope
    = note: no item named `DefId` in scope
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
    = note: this error originates in the macro `define_queries_struct` which comes from the expansion of the macro `rustc_query_append` (in Nightly builds, run with -Z macro-backtrace for more info)
error: unresolved link to `DefId`
   --> compiler/rustc_query_impl/src/plumbing.rs:568:17
    |
    |
568 |             $($(#[$attr])*
    |
   ::: compiler/rustc_query_impl/src/lib.rs:57:1
    |
    |
57  | rustc_query_append! { define_queries! }
    |
    = note: the link appears in this line:
            
            
            Returns the [`Ty`][rustc_middle::ty::Ty] of the given `AnonConst`'s [`DefId`].
    = note: no item named `DefId` in scope
    = note: no item named `DefId` in scope
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
    = note: this error originates in the macro `define_queries_struct` which comes from the expansion of the macro `rustc_query_append` (in Nightly builds, run with -Z macro-backtrace for more info)
error: could not document `rustc_query_impl`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name rustc_query_impl compiler/rustc_query_impl/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/doc -Zunstable-options --check-cfg 'values(feature, "rustc-rayon-core", "rustc_use_parallel_compiler")' --check-cfg 'names()' --check-cfg 'values()' --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -C metadata=e32ea5cfe62a598b -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps --extern measureme=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libmeasureme-2fbce49d62b98776.rmeta --extern rustc_ast=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_ast-a07acca7efebf59b.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-565e278fc80840d2.rmeta --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-11448850605788e3.rmeta --extern rustc_hir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_hir-0660e98d79d66260.rmeta --extern rustc_index=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_index-986bed15c081e748.rmeta --extern rustc_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps/librustc_macros-d6b8922907fd8851.so --extern rustc_middle=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_middle-001b097eb8df1fa4.rmeta --extern rustc_query_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_query_system-e78c7b19f644772d.rmeta --extern rustc_serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_serialize-0ac38a19376a0cf6.rmeta --extern rustc_session=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_session-13b946e3fb8448b7.rmeta --extern rustc_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_span-469e9e56fc4b3beb.rmeta --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-94f1f7abe8ea2198.rmeta --extern thin_vec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libthin_vec-04fd0ba67f8a8f4d.rmeta --extern tracing=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libtracing-863fd2d56b359bad.rmeta --extern-html-root-url 'measureme=https://docs.rs/measureme/10.1.0/' --extern-html-root-url 'thin_vec=https://docs.rs/thin-vec/0.2.8/' --extern-html-root-url 'tracing=https://docs.rs/tracing/0.1.35/' -Zunstable-options -Csymbol-mangling-version=v0 -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(parallel_compiler)' '--check-cfg=values(no_btreemap_remove_entry)' '--check-cfg=values(crossbeam_loom)' '--check-cfg=values(span_locations)' -Clink-arg=-fuse-ld=lld -Clink-arg=-Wl,--threads=1 -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.65.0-nightly
  (c9b94d8c8
  2022-09-05)' --document-private-items '-Arustdoc::private-intra-doc-links' --enable-index-page -Zunstable-options -Znormalize-docs --show-type-layout --generate-link-to-definition --extern-html-root-url 'ena=https://docs.rs/ena/latest/'` (exit status: 1)
[RUSTC-TIMING] rustc_infer test:false 3.416
[RUSTC-TIMING] rustc_resolve test:false 2.611
[RUSTC-TIMING] rustc_query_impl test:false 5.618
Build completed unsuccessfully in 0:20:57
