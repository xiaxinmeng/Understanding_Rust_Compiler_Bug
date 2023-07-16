plain
Executing benchmark hyper-0.14.18 (3/7)
Preparing hyper-0.14.18
[2022-06-20T16:02:03Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-06-20T16:02:03Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-06-20T16:02:03Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpY417tB#hyper@0.14.18" "--features=client,http1,http2,server,stream" "--" "--skip-this-rustc"
[2022-06-20T16:02:03Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpLTrXWd#hyper@0.14.18" "--release" "--features=client,http1,http2,server,stream" "--" "--skip-this-rustc"
[2022-06-20T16:02:39Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-20T16:02:39Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-06-20T16:02:39Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpR8LPlz#hyper@0.14.18" "--features=client,http1,http2,server,stream" "--" "--wrap-rustc-with" "Eprintln"
Running hyper-0.14.18: Opt + [Full]
---
Executing benchmark ripgrep-13.0.0 (5/7)
Preparing ripgrep-13.0.0
[2022-06-20T16:03:28Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-06-20T16:03:28Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-06-20T16:03:28Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp2Ehbrr#ripgrep@13.0.0" "--" "--skip-this-rustc"
[2022-06-20T16:03:28Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmprKwzXz#ripgrep@13.0.0" "--release" "--" "--skip-this-rustc"
[2022-06-20T16:04:04Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-20T16:04:05Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-06-20T16:04:05Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpUZmuDZ#ripgrep@13.0.0" "--" "--wrap-rustc-with" "Eprintln"
Running ripgrep-13.0.0: Opt + [Full]
Running ripgrep-13.0.0: Opt + [Full]
[2022-06-20T16:04:10Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-20T16:04:11Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-06-20T16:04:11Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpkWCcbh#ripgrep@13.0.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
Executing benchmark serde-1.0.136 (6/7)
Preparing serde-1.0.136
[2022-06-20T16:04:33Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-06-20T16:04:33Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
---
Preparing bitmaps-3.1.0
[2022-06-20T16:20:19Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-06-20T16:20:19Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-06-20T16:20:19Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-06-20T16:20:19Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpLRxzDS#bitmaps@3.1.0" "--" "--skip-this-rustc"
[2022-06-20T16:20:19Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFUNUll#bitmaps@3.1.0" "--profile" "check" "--" "--skip-this-rustc"
[2022-06-20T16:20:19Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzG05vy#bitmaps@3.1.0" "--release" "--" "--skip-this-rustc"
[2022-06-20T16:20:20Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-20T16:20:20Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-06-20T16:20:20Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp4CtgIg#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-20T16:20:23Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-06-20T16:20:23Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-06-20T16:20:23Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp4CtgIg#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp4CtgIg/incremental-state"
[2022-06-20T16:20:26Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-06-20T16:20:26Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp4CtgIg#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp4CtgIg/incremental-state"
[2022-06-20T16:20:27Z DEBUG collector::execute] applying println to "/tmp/.tmp4CtgIg"
[2022-06-20T16:20:27Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-06-20T16:20:27Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-06-20T16:20:27Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp4CtgIg#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp4CtgIg/incremental-state"
[2022-06-20T16:20:28Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-20T16:20:28Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-06-20T16:20:28Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpGkJ0cg#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-20T16:20:31Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-06-20T16:20:31Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-06-20T16:20:31Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpGkJ0cg#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpGkJ0cg/incremental-state"
[2022-06-20T16:20:35Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-06-20T16:20:35Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpGkJ0cg#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpGkJ0cg/incremental-state"
[2022-06-20T16:20:36Z DEBUG collector::execute] applying println to "/tmp/.tmpGkJ0cg"
[2022-06-20T16:20:36Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-06-20T16:20:36Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-06-20T16:20:36Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpGkJ0cg#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpGkJ0cg/incremental-state"
[2022-06-20T16:20:37Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-20T16:20:37Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-06-20T16:20:37Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-06-20T16:20:37Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpZTRZMa#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-20T16:20:40Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-06-20T16:20:40Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpZTRZMa#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpZTRZMa/incremental-state"
[2022-06-20T16:20:44Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-06-20T16:20:44Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpZTRZMa#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpZTRZMa/incremental-state"
[2022-06-20T16:20:45Z DEBUG collector::execute] applying println to "/tmp/.tmpZTRZMa"
[2022-06-20T16:20:45Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-06-20T16:20:45Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-06-20T16:20:45Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpZTRZMa#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpZTRZMa/incremental-state"
Executing benchmark cargo-0.60.0 (2/8)
Preparing cargo-0.60.0
[2022-06-20T16:20:46Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-06-20T16:20:46Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
---
[2022-06-20T16:22:02Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-20T16:22:02Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-06-20T16:22:02Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpLrqnQr#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-20T16:22:28Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-06-20T16:22:28Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpLrqnQr#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpLrqnQr/incremental-state"
[2022-06-20T16:22:58Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-06-20T16:22:58Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpLrqnQr#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpLrqnQr/incremental-state"
[2022-06-20T16:23:04Z DEBUG collector::execute] applying println to "/tmp/.tmpLrqnQr"
[2022-06-20T16:23:04Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-06-20T16:23:04Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-06-20T16:23:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpLrqnQr#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpLrqnQr/incremental-state"
[2022-06-20T16:23:17Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-20T16:23:17Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-06-20T16:23:17Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpsCZFjA#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-20T16:24:20Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-06-20T16:24:20Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-06-20T16:24:20Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpsCZFjA#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpsCZFjA/incremental-state"
[2022-06-20T16:25:37Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-06-20T16:25:37Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpsCZFjA#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpsCZFjA/incremental-state"
[2022-06-20T16:25:50Z DEBUG collector::execute] applying println to "/tmp/.tmpsCZFjA"
[2022-06-20T16:25:50Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-06-20T16:25:50Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-06-20T16:25:50Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpsCZFjA#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpsCZFjA/incremental-state"
[2022-06-20T16:26:18Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-20T16:26:19Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-06-20T16:26:19Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp3zzdyg#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-20T16:27:37Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-06-20T16:27:37Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-06-20T16:27:37Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp3zzdyg#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp3zzdyg/incremental-state"
[2022-06-20T16:29:00Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-06-20T16:29:00Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp3zzdyg#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp3zzdyg/incremental-state"
[2022-06-20T16:29:13Z DEBUG collector::execute] applying println to "/tmp/.tmp3zzdyg"
[2022-06-20T16:29:13Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-06-20T16:29:13Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-06-20T16:29:13Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp3zzdyg#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp3zzdyg/incremental-state"
Executing benchmark ctfe-stress-5 (3/8)
Preparing ctfe-stress-5
[2022-06-20T16:29:49Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-06-20T16:29:49Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2022-06-20T16:30:14Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-20T16:30:14Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-06-20T16:30:14Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKDxObC#ctfe-stress-5@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-20T16:30:24Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-06-20T16:30:24Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKDxObC#ctfe-stress-5@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpKDxObC/incremental-state"
[2022-06-20T16:30:38Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-06-20T16:30:38Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKDxObC#ctfe-stress-5@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpKDxObC/incremental-state"
[2022-06-20T16:30:38Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-20T16:30:38Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-06-20T16:30:38Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpoQhNv8#ctfe-stress-5@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-20T16:30:49Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-06-20T16:30:49Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-06-20T16:30:49Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpoQhNv8#ctfe-stress-5@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpoQhNv8/incremental-state"
[2022-06-20T16:31:02Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-06-20T16:31:02Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpoQhNv8#ctfe-stress-5@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpoQhNv8/incremental-state"
Executing benchmark diesel-1.4.8 (4/8)
Preparing diesel-1.4.8
[2022-06-20T16:31:02Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-06-20T16:31:02Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2022-06-20T16:33:13Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmph4b0yO#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmph4b0yO/incremental-state"
Running diesel-1.4.8: Opt + [Full, IncrFull, IncrUnchanged, IncrPatched]
[2022-06-20T16:33:18Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-20T16:33:18Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-06-20T16:33:18Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmphdRMRc#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-20T16:33:40Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-06-20T16:33:40Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmphdRMRc#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmphdRMRc/incremental-state"
[2022-06-20T16:34:08Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-06-20T16:34:08Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmphdRMRc#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmphdRMRc/incremental-state"
[2022-06-20T16:34:12Z DEBUG collector::execute] applying println to "/tmp/.tmphdRMRc"
[2022-06-20T16:34:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-06-20T16:34:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-06-20T16:34:12Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmphdRMRc#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmphdRMRc/incremental-state"
Executing benchmark externs (5/8)
Preparing externs
[2022-06-20T16:34:17Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-06-20T16:34:17Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2022-06-20T16:34:17Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-20T16:34:17Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-06-20T16:34:17Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmplLe1Er#externs@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-20T16:34:18Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-06-20T16:34:18Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmplLe1Er#externs@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmplLe1Er/incremental-state"
[2022-06-20T16:34:19Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-06-20T16:34:19Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmplLe1Er#externs@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmplLe1Er/incremental-state"
[2022-06-20T16:34:20Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-20T16:34:20Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-06-20T16:34:20Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpGKSuzH#externs@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-20T16:34:21Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-06-20T16:34:21Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-06-20T16:34:21Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpGKSuzH#externs@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpGKSuzH/incremental-state"
[2022-06-20T16:34:22Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-06-20T16:34:22Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpGKSuzH#externs@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpGKSuzH/incremental-state"
[2022-06-20T16:34:23Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-20T16:34:23Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-06-20T16:34:23Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpSaMhig#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-20T16:34:24Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
[2022-06-20T16:34:36Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-20T16:34:36Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-06-20T16:34:36Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpThUl4V#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-20T16:34:39Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-06-20T16:34:39Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpThUl4V#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpThUl4V/incremental-state"
[2022-06-20T16:34:43Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-06-20T16:34:43Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpThUl4V#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpThUl4V/incremental-state"
[2022-06-20T16:34:45Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-20T16:34:45Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-06-20T16:34:45Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp0ksymx#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-20T16:34:48Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-06-20T16:34:48Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-06-20T16:34:48Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp0ksymx#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp0ksymx/incremental-state"
[2022-06-20T16:34:53Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-06-20T16:34:53Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp0ksymx#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp0ksymx/incremental-state"
Executing benchmark token-stream-stress (7/8)
Preparing token-stream-stress
[2022-06-20T16:34:54Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-06-20T16:34:54Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
---
[2022-06-20T16:34:55Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-20T16:34:55Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-06-20T16:34:55Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmprLG4np#token-stream-stress@0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-20T16:34:56Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-06-20T16:34:56Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmprLG4np#token-stream-stress@0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmprLG4np/incremental-state"
[2022-06-20T16:34:56Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-06-20T16:34:56Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmprLG4np#token-stream-stress@0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmprLG4np/incremental-state"
[2022-06-20T16:34:56Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-20T16:34:56Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-06-20T16:34:56Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpIDGsGM#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-20T16:34:56Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-06-20T16:34:56Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-06-20T16:34:56Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpIDGsGM#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpIDGsGM/incremental-state"
[2022-06-20T16:34:57Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-06-20T16:34:57Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpIDGsGM#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpIDGsGM/incremental-state"
[2022-06-20T16:34:57Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-20T16:34:57Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-06-20T16:34:57Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKju5Hb#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-20T16:34:57Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
[2022-06-20T16:34:58Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-20T16:34:58Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-06-20T16:34:58Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmplchCaV#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-20T16:35:04Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-06-20T16:35:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmplchCaV#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmplchCaV/incremental-state"
[2022-06-20T16:35:10Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-06-20T16:35:10Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmplchCaV#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmplchCaV/incremental-state"
[2022-06-20T16:35:12Z DEBUG collector::execute] applying new row to "/tmp/.tmplchCaV"
[2022-06-20T16:35:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-06-20T16:35:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-06-20T16:35:12Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmplchCaV#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmplchCaV/incremental-state"
[2022-06-20T16:35:19Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-20T16:35:19Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-06-20T16:35:19Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpbFlxL3#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-20T16:35:25Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-06-20T16:35:25Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-06-20T16:35:25Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpbFlxL3#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpbFlxL3/incremental-state"
[2022-06-20T16:35:31Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-06-20T16:35:31Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpbFlxL3#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpbFlxL3/incremental-state"
[2022-06-20T16:35:33Z DEBUG collector::execute] applying new row to "/tmp/.tmpbFlxL3"
[2022-06-20T16:35:33Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-06-20T16:35:33Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-06-20T16:35:33Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpbFlxL3#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpbFlxL3/incremental-state"
[2022-06-20T16:35:40Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-20T16:35:40Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-06-20T16:35:40Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpIFYBIW#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-20T16:35:46Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-06-20T16:35:46Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-06-20T16:35:46Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpIFYBIW#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpIFYBIW/incremental-state"
[2022-06-20T16:35:52Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-06-20T16:35:52Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpIFYBIW#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpIFYBIW/incremental-state"
[2022-06-20T16:35:54Z DEBUG collector::execute] applying new row to "/tmp/.tmpIFYBIW"
[2022-06-20T16:35:54Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-06-20T16:35:54Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-06-20T16:35:54Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpIFYBIW#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpIFYBIW/incremental-state"
+ cd /checkout/obj
+ RUSTC_PROFILE_MERGED_FILE=/tmp/rustc-pgo.profdata
+ ./build/x86_64-unknown-linux-gnu/llvm/bin/llvm-profdata merge -o /tmp/rustc-pgo.profdata /tmp/rustc-pgo
+ echo 'Rustc PGO statistics'
---
[RUSTC-TIMING] unic_char_range test:false 0.167
   Compiling either v1.6.0
[RUSTC-TIMING] self_cell test:false 0.071
   Compiling datafrog v2.0.1
warning: rustc_graphviz.eed2e141-cgu.1: no profile data available for function _RNvXs2_NtCshmVhAiSNDn2_4core7convertNtNtNtB7_5alloc6layout11LayoutErrorINtB5_4FromBy_E4fromCsbOVucredaoL_14rustc_graphviz Hash = 742261418966908927

warning: rustc_fs_util.e6ff12f2-cgu.4: no profile data available for function _RNvXs2_NtCshmVhAiSNDn2_4core7convertNtNtNtB7_5alloc6layout11LayoutErrorINtB5_4FromBy_E4fromCset1xSFjQmFW_13rustc_fs_util Hash = 742261418966908927
[RUSTC-TIMING] itoa test:false 0.131
   Compiling memchr v2.4.1
   Compiling memchr v2.4.1
warning: rustc_graphviz.eed2e141-cgu.9: no profile data available for function _RNvMNtNtNtCshmVhAiSNDn2_4core4iter8adapters3revINtB2_3RevINtNtNtB8_3ops5range5RangejEE3newCsbOVucredaoL_14rustc_graphviz Hash = 742261418966908927
[RUSTC-TIMING] rustc_fs_util test:false 0.144
warning: `rustc_fs_util` (lib) generated 1 warning
   Compiling regex-syntax v0.6.25
[RUSTC-TIMING] build_script_build test:false 0.479
---
   Compiling rand v0.8.5
[RUSTC-TIMING] generic_array test:false 1.066
[RUSTC-TIMING] hashbrown test:false 0.813
   Compiling crypto-common v0.1.2
warning: rustc_llvm.ad28d15d-cgu.6: no profile data available for function _RNvXs2_NtCshmVhAiSNDn2_4core7convertNtNtNtB7_5alloc6layout11LayoutErrorINtB5_4FromBy_E4fromCs9ECgU3EzyQP_10rustc_llvm Hash = 742261418966908927
[RUSTC-TIMING] crypto_common test:false 0.106
   Compiling block-buffer v0.10.2
[RUSTC-TIMING] rustc_llvm test:false 0.259
warning: `rustc_llvm` (lib) generated 1 warning
---
[RUSTC-TIMING] sha1 test:false 0.423
   Compiling gimli v0.26.1
[RUSTC-TIMING] rand test:false 1.742
[RUSTC-TIMING] aho_corasick test:false 3.037
warning: rustc_serialize.1993aa9f-cgu.5: no profile data available for function _RINvNtCshmVhAiSNDn2_4core3ptr13drop_in_placeRjECsahU9xAQfDL_15rustc_serialize Hash = 742261418966908927

warning: rustc_serialize.1993aa9f-cgu.5: no profile data available for function _RINvNtCshmVhAiSNDn2_4core9panicking13assert_failedjjECsahU9xAQfDL_15rustc_serialize Hash = 742261418966908927

warning: rustc_serialize.1993aa9f-cgu.6: no profile data available for function _RNvXsV_NtCshmVhAiSNDn2_4core3fmtRjNtB5_5Debug3fmtCsahU9xAQfDL_15rustc_serialize Hash = 1124680650125156080
[RUSTC-TIMING] rustc_serialize test:false 0.735
warning: `rustc_serialize` (lib) generated 3 warnings
[RUSTC-TIMING] tempfile test:false 1.148
[RUSTC-TIMING] regex_automata test:false 3.982
---
   Compiling rustc_ast v0.0.0 (/checkout/compiler/rustc_ast)
   Compiling rustc_target v0.0.0 (/checkout/compiler/rustc_target)
   Compiling rustc_error_messages v0.0.0 (/checkout/compiler/rustc_error_messages)
   Compiling rustc_feature v0.0.0 (/checkout/compiler/rustc_feature)
warning: rustc_error_messages.758e938a-cgu.11: no profile data available for function _RINvMsx_NtCsgwOiVo4MmOn_12tracing_core5fieldNtB6_8FieldSet9value_setATRNtB6_5FieldINtNtCshmVhAiSNDn2_4core6option6OptionRDNtB6_5ValueEL_EEj1_ECsea16vY5Hcla_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.758e938a-cgu.11: no profile data available for function _RINvMsx_NtCsgwOiVo4MmOn_12tracing_core5fieldNtB6_8FieldSet9value_setATRNtB6_5FieldINtNtCshmVhAiSNDn2_4core6option6OptionRDNtB6_5ValueEL_EEj2_ECsea16vY5Hcla_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.758e938a-cgu.11: no profile data available for function _RINvMsx_NtCsgwOiVo4MmOn_12tracing_core5fieldNtB6_8FieldSet9value_setATRNtB6_5FieldINtNtCshmVhAiSNDn2_4core6option6OptionRDNtB6_5ValueEL_EEj5_ECsea16vY5Hcla_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.758e938a-cgu.11: no profile data available for function _RINvNtCsgwOiVo4MmOn_12tracing_core5field5debugRINtNtCshmVhAiSNDn2_4core6option6OptionNtCsbG4HA8s131E_16unic_langid_impl18LanguageIdentifierEECsea16vY5Hcla_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.758e938a-cgu.11: no profile data available for function _RINvNtCsgwOiVo4MmOn_12tracing_core5field5debugRINtNtCshmVhAiSNDn2_4core6option6OptionNtNtCshpT4EJHIn4l_3std4path7PathBufEECsea16vY5Hcla_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.758e938a-cgu.11: no profile data available for function _RINvNtCsgwOiVo4MmOn_12tracing_core5field5debugRINtNtCshmVhAiSNDn2_4core6option6OptionRNtNtCshpT4EJHIn4l_3std4path4PathEECsea16vY5Hcla_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.758e938a-cgu.11: no profile data available for function _RINvNtCsgwOiVo4MmOn_12tracing_core5field5debugRINtNtCsj9BzbIBk3uF_5alloc3vec3VecNtNtCshpT4EJHIn4l_3std4path7PathBufEECsea16vY5Hcla_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.758e938a-cgu.11: no profile data available for function _RINvNtCsgwOiVo4MmOn_12tracing_core5field5debugRNtCsbG4HA8s131E_16unic_langid_impl18LanguageIdentifierECsea16vY5Hcla_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.758e938a-cgu.11: no profile data available for function _RINvNtCsgwOiVo4MmOn_12tracing_core5field5debugRNtNtCs6HTbTwjBRq6_13fluent_bundle8resource14FluentResourceECsea16vY5Hcla_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.758e938a-cgu.11: no profile data available for function _RINvNtCsgwOiVo4MmOn_12tracing_core5field5debugRNtNtCshpT4EJHIn4l_3std4path7PathBufECsea16vY5Hcla_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.758e938a-cgu.11: no profile data available for function _RINvNtCsgwOiVo4MmOn_12tracing_core5field5debugRQNtNtCshpT4EJHIn4l_3std4path7PathBufECsea16vY5Hcla_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.758e938a-cgu.11: no profile data available for function _RINvNtCsgwOiVo4MmOn_12tracing_core5field5debugRRSReECsea16vY5Hcla_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.758e938a-cgu.11: no profile data available for function _RINvNtCshmVhAiSNDn2_4core3ptr13drop_in_placeRINtNtB4_6option6OptionNtCsbG4HA8s131E_16unic_langid_impl18LanguageIdentifierEECsea16vY5Hcla_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.758e938a-cgu.11: no profile data available for function _RINvNtCshmVhAiSNDn2_4core3ptr13drop_in_placeRINtNtB4_6option6OptionNtNtCshpT4EJHIn4l_3std4path7PathBufEECsea16vY5Hcla_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.758e938a-cgu.11: no profile data available for function _RINvNtCshmVhAiSNDn2_4core3ptr13drop_in_placeRINtNtB4_6option6OptionRNtNtCshpT4EJHIn4l_3std4path4PathEECsea16vY5Hcla_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.758e938a-cgu.11: no profile data available for function _RINvNtCshmVhAiSNDn2_4core3ptr13drop_in_placeRINtNtCsj9BzbIBk3uF_5alloc3vec3VecNtNtCshpT4EJHIn4l_3std4path7PathBufEECsea16vY5Hcla_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.758e938a-cgu.11: no profile data available for function _RINvNtCshmVhAiSNDn2_4core3ptr13drop_in_placeRNtCsbG4HA8s131E_16unic_langid_impl18LanguageIdentifierECsea16vY5Hcla_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.758e938a-cgu.11: no profile data available for function _RINvNtCshmVhAiSNDn2_4core3ptr13drop_in_placeRNtNtCs6HTbTwjBRq6_13fluent_bundle8resource14FluentResourceECsea16vY5Hcla_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.758e938a-cgu.11: no profile data available for function _RINvNtCshmVhAiSNDn2_4core3ptr13drop_in_placeRNtNtCshpT4EJHIn4l_3std4path7PathBufECsea16vY5Hcla_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.758e938a-cgu.11: no profile data available for function _RINvNtCshmVhAiSNDn2_4core3ptr13drop_in_placeRQNtNtCshpT4EJHIn4l_3std4path7PathBufECsea16vY5Hcla_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.758e938a-cgu.11: no profile data available for function _RINvNtCshmVhAiSNDn2_4core3ptr13drop_in_placeRRSReECsea16vY5Hcla_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.758e938a-cgu.11: no profile data available for function _RNvXsk_NtCsgwOiVo4MmOn_12tracing_core5fieldINtB5_10DebugValueRINtNtCshmVhAiSNDn2_4core6option6OptionNtCsbG4HA8s131E_16unic_langid_impl18LanguageIdentifierEENtB5_5Value6recordCsea16vY5Hcla_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.758e938a-cgu.11: no profile data available for function _RNvXsk_NtCsgwOiVo4MmOn_12tracing_core5fieldINtB5_10DebugValueRINtNtCshmVhAiSNDn2_4core6option6OptionNtNtCshpT4EJHIn4l_3std4path7PathBufEENtB5_5Value6recordCsea16vY5Hcla_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.758e938a-cgu.11: no profile data available for function _RNvXsk_NtCsgwOiVo4MmOn_12tracing_core5fieldINtB5_10DebugValueRINtNtCshmVhAiSNDn2_4core6option6OptionRNtNtCshpT4EJHIn4l_3std4path4PathEENtB5_5Value6recordCsea16vY5Hcla_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.758e938a-cgu.11: no profile data available for function _RNvXsk_NtCsgwOiVo4MmOn_12tracing_core5fieldINtB5_10DebugValueRINtNtCsj9BzbIBk3uF_5alloc3vec3VecNtNtCshpT4EJHIn4l_3std4path7PathBufEENtB5_5Value6recordCsea16vY5Hcla_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.758e938a-cgu.11: no profile data available for function _RNvXsk_NtCsgwOiVo4MmOn_12tracing_core5fieldINtB5_10DebugValueRNtCsbG4HA8s131E_16unic_langid_impl18LanguageIdentifierENtB5_5Value6recordCsea16vY5Hcla_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.758e938a-cgu.11: no profile data available for function _RNvXsk_NtCsgwOiVo4MmOn_12tracing_core5fieldINtB5_10DebugValueRNtNtCs6HTbTwjBRq6_13fluent_bundle8resource14FluentResourceENtB5_5Value6recordCsea16vY5Hcla_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.758e938a-cgu.11: no profile data available for function _RNvXsk_NtCsgwOiVo4MmOn_12tracing_core5fieldINtB5_10DebugValueRNtNtCshpT4EJHIn4l_3std4path7PathBufENtB5_5Value6recordCsea16vY5Hcla_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.758e938a-cgu.11: no profile data available for function _RNvXsk_NtCsgwOiVo4MmOn_12tracing_core5fieldINtB5_10DebugValueRQNtNtCshpT4EJHIn4l_3std4path7PathBufENtB5_5Value6recordCsea16vY5Hcla_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.758e938a-cgu.11: no profile data available for function _RNvXsk_NtCsgwOiVo4MmOn_12tracing_core5fieldINtB5_10DebugValueRRSReENtB5_5Value6recordCsea16vY5Hcla_20rustc_error_messages Hash = 170957022131388415
[RUSTC-TIMING] rustc_error_messages test:false 1.041
warning: `rustc_error_messages` (lib) generated 30 warnings
[RUSTC-TIMING] jemalloc_sys test:false 0.087
[RUSTC-TIMING] rustc_feature test:false 1.181
---
 Documenting rustdoc v0.0.0 (/checkout/src/librustdoc)
error: unresolved link to `rustc_ast_lowering`
   --> src/librustdoc/clean/mod.rs:550:44
    |
550 | /// See [`lifetime_to_generic_param`] in [`rustc_ast_lowering`] for more information.
    |                                            ^^^^^^^^^^^^^^^^^^ no item named `rustc_ast_lowering` in scope
    |
    = note: `-D rustdoc::broken-intra-doc-links` implied by `-D warnings`
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `rustc_ast_lowering::LoweringContext::lifetime_to_generic_param`
   --> src/librustdoc/clean/mod.rs:552:36
    |
    |
552 | /// [`lifetime_to_generic_param`]: rustc_ast_lowering::LoweringContext::lifetime_to_generic_param
    |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no item named `rustc_ast_lowering` in scope
error: could not document `rustdoc`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name rustdoc src/librustdoc/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/doc -Zunstable-options --check-cfg 'values(feature, "jemalloc")' --check-cfg 'names()' --check-cfg 'values()' --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -C metadata=9bc1b42a245d5d24 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps --extern arrayvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libarrayvec-ace1ee1a1d07d40c.rmeta --extern askama=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libaskama-31578ae2ae7d2aed.rmeta --extern atty=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libatty-64b4a17853c00163.rmeta --extern itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-c7877d326fb91f59.rmeta --extern minifier=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libminifier-63a8604e84c1c8ef.rmeta --extern once_cell=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libonce_cell-92a2402ba4e449e9.rmeta --extern pulldown_cmark=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libpulldown_cmark-89f2d83f3e754b2a.rmeta --extern regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-0bf6b92da2839bc8.rmeta --extern rustdoc_json_types=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustdoc_json_types-56cee05fd9d327b9.rmeta --extern serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-1d44fdf25cf19218.rmeta --extern serde_json=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde_json-3f451d1220a81bf6.rmeta --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsmallvec-4a27a50a17ed7f75.rmeta --extern tempfile=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtempfile-ab473257af9b7367.rmeta --extern tracing=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtracing-c3eb623173c41f53.rmeta --extern tracing_subscriber=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtracing_subscriber-e440cedb61999a06.rmeta --extern tracing_tree=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtracing_tree-d95f5b6acf15f26e.rmeta -Csymbol-mangling-version=v0 -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(parallel_compiler)' '--check-cfg=values(release)' '--check-cfg=values(no_btreemap_remove_entry)' '--check-cfg=values(crossbeam_loom)' '--check-cfg=values(span_locations)' -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.63.0-nightly
  (01800f2df
  2022-06-20)' --document-private-items --enable-index-page --show-type-layout --generate-link-to-definition -Zunstable-options` (exit status: 1)
