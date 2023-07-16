plain
5 normal benchmarks remaining
Preparing hyper-0.14.18
[2022-04-10T19:09:27Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-04-10T19:09:27Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-04-10T19:09:27Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp2smEfx#hyper:0.14.18" "--features" "client,http1,http2,server,stream" "--" "--skip-this-rustc"
[2022-04-10T19:09:27Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCCCDxo#hyper:0.14.18" "--release" "--features" "client,http1,http2,server,stream" "--" "--skip-this-rustc"
[2022-04-10T19:10:03Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-04-10T19:10:03Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-04-10T19:10:03Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpHPmnd5#hyper:0.14.18" "--features" "client,http1,http2,server,stream" "--" "--wrap-rustc-with" "Eprintln"
Running hyper-0.14.18: Opt + [Full]
Running hyper-0.14.18: Opt + [Full]
[2022-04-10T19:10:08Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-04-10T19:10:09Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-04-10T19:10:09Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpP93Wvk#hyper:0.14.18" "--release" "--features" "client,http1,http2,server,stream" "--" "--wrap-rustc-with" "Eprintln"
4 normal benchmarks remaining
Preparing regex-1.5.5
[2022-04-10T19:10:22Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-04-10T19:10:22Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-04-10T19:10:22Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpoMH2p7#regex:1.5.5" "--release" "--" "--skip-this-rustc"
[2022-04-10T19:10:22Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmphwssfO#regex:1.5.5" "--" "--skip-this-rustc"
[2022-04-10T19:10:35Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-04-10T19:10:35Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-04-10T19:10:35Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-04-10T19:10:35Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpgwfoej#regex:1.5.5" "--" "--wrap-rustc-with" "Eprintln"
[2022-04-10T19:10:39Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-04-10T19:10:39Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-04-10T19:10:39Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmphH8klB#regex:1.5.5" "--release" "--" "--wrap-rustc-with" "Eprintln"
3 normal benchmarks remaining
3 normal benchmarks remaining
Preparing ripgrep-13.0.0
[2022-04-10T19:10:56Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-04-10T19:10:56Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-04-10T19:10:56Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOFbgAi#ripgrep:13.0.0" "--release" "--" "--skip-this-rustc"
[2022-04-10T19:10:56Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpZlXinV#ripgrep:13.0.0" "--" "--skip-this-rustc"
[2022-04-10T19:11:33Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-04-10T19:11:34Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-04-10T19:11:34Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-04-10T19:11:34Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpMiJkQb#ripgrep:13.0.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-04-10T19:11:40Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-04-10T19:11:41Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-04-10T19:11:41Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp2VAUe3#ripgrep:13.0.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
2 normal benchmarks remaining
2 normal benchmarks remaining
Preparing serde-1.0.136
[2022-04-10T19:12:05Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-04-10T19:12:05Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-04-10T19:12:05Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmptLRlOi#serde:1.0.136" "--" "--skip-this-rustc"
[2022-04-10T19:12:05Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp1d1nEh#serde:1.0.136" "--release" "--" "--skip-this-rustc"
[2022-04-10T19:12:06Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-04-10T19:12:06Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-04-10T19:12:06Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-04-10T19:12:06Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpk9vk5m#serde:1.0.136" "--" "--wrap-rustc-with" "Eprintln"
[2022-04-10T19:12:10Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-04-10T19:12:10Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-04-10T19:12:10Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-04-10T19:12:10Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpGZeUbS#serde:1.0.136" "--release" "--" "--wrap-rustc-with" "Eprintln"
Preparing syn-1.0.89
[2022-04-10T19:12:15Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-04-10T19:12:15Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-04-10T19:12:15Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpEiyHIh#syn:1.0.89" "--release" "--" "--skip-this-rustc"
---
[2022-04-10T19:27:23Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-04-10T19:27:23Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-04-10T19:27:23Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpqnSn9X#cargo:0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2022-04-10T19:27:49Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-04-10T19:27:49Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpqnSn9X#cargo:0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpqnSn9X/incremental-state"
[2022-04-10T19:28:18Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-04-10T19:28:18Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpqnSn9X#cargo:0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpqnSn9X/incremental-state"
[2022-04-10T19:28:24Z DEBUG collector::execute] applying println to "/tmp/.tmpqnSn9X"
[2022-04-10T19:28:24Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-04-10T19:28:24Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-04-10T19:28:24Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpqnSn9X#cargo:0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpqnSn9X/incremental-state"
[2022-04-10T19:28:36Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-04-10T19:28:37Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-04-10T19:28:37Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpmYXgnw#cargo:0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2022-04-10T19:29:37Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-04-10T19:29:37Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-04-10T19:29:37Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpmYXgnw#cargo:0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpmYXgnw/incremental-state"
[2022-04-10T19:30:45Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-04-10T19:30:45Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpmYXgnw#cargo:0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpmYXgnw/incremental-state"
[2022-04-10T19:30:59Z DEBUG collector::execute] applying println to "/tmp/.tmpmYXgnw"
[2022-04-10T19:30:59Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-04-10T19:30:59Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-04-10T19:30:59Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpmYXgnw#cargo:0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpmYXgnw/incremental-state"
[2022-04-10T19:31:28Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-04-10T19:31:29Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-04-10T19:31:29Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpMFOGir#cargo:0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2022-04-10T19:32:45Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-04-10T19:32:45Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-04-10T19:32:45Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpMFOGir#cargo:0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpMFOGir/incremental-state"
[2022-04-10T19:33:59Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-04-10T19:33:59Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpMFOGir#cargo:0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpMFOGir/incremental-state"
[2022-04-10T19:34:13Z DEBUG collector::execute] applying println to "/tmp/.tmpMFOGir"
[2022-04-10T19:34:13Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-04-10T19:34:13Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-04-10T19:34:13Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpMFOGir#cargo:0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpMFOGir/incremental-state"
Preparing ctfe-stress-4
[2022-04-10T19:34:48Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-04-10T19:34:48Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-04-10T19:34:48Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
---
[2022-04-10T19:34:48Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-04-10T19:34:48Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-04-10T19:34:48Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpxfpsgw#ctfe-stress-4:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-04-10T19:35:11Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-04-10T19:35:11Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpxfpsgw#ctfe-stress-4:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpxfpsgw/incremental-state"
[2022-04-10T19:35:39Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-04-10T19:35:39Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpxfpsgw#ctfe-stress-4:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpxfpsgw/incremental-state"
[2022-04-10T19:35:40Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-04-10T19:35:40Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-04-10T19:35:40Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpz8fXZX#ctfe-stress-4:0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-04-10T19:36:04Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
---
[2022-04-10T19:36:31Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-04-10T19:36:31Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-04-10T19:36:31Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJlGg1U#ctfe-stress-4:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-04-10T19:36:54Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-04-10T19:36:54Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJlGg1U#ctfe-stress-4:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpJlGg1U/incremental-state"
[2022-04-10T19:37:19Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-04-10T19:37:19Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJlGg1U#ctfe-stress-4:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpJlGg1U/incremental-state"
Preparing externs
[2022-04-10T19:37:20Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-04-10T19:37:20Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-04-10T19:37:20Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2022-04-10T19:37:21Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-04-10T19:37:21Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-04-10T19:37:21Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpeW3TQV#externs:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-04-10T19:37:21Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-04-10T19:37:21Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpeW3TQV#externs:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpeW3TQV/incremental-state"
[2022-04-10T19:37:23Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-04-10T19:37:23Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpeW3TQV#externs:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpeW3TQV/incremental-state"
[2022-04-10T19:37:23Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-04-10T19:37:23Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-04-10T19:37:23Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOfsjlk#externs:0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-04-10T19:37:25Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-04-10T19:37:25Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-04-10T19:37:25Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOfsjlk#externs:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpOfsjlk/incremental-state"
[2022-04-10T19:37:26Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-04-10T19:37:26Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOfsjlk#externs:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpOfsjlk/incremental-state"
[2022-04-10T19:37:27Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-04-10T19:37:27Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-04-10T19:37:27Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpG3WVPw#externs:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-04-10T19:37:28Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-04-10T19:37:28Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-04-10T19:37:28Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpG3WVPw#externs:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpG3WVPw/incremental-state"
[2022-04-10T19:37:29Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-04-10T19:37:29Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpG3WVPw#externs:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpG3WVPw/incremental-state"
Preparing match-stress
[2022-04-10T19:37:31Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-04-10T19:37:31Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-04-10T19:37:31Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
---
[2022-04-10T19:37:31Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-04-10T19:37:31Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-04-10T19:37:31Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpqYfoeT#match-stress:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-04-10T19:37:35Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-04-10T19:37:35Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpqYfoeT#match-stress:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpqYfoeT/incremental-state"
[2022-04-10T19:37:39Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-04-10T19:37:39Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpqYfoeT#match-stress:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpqYfoeT/incremental-state"
[2022-04-10T19:37:40Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-04-10T19:37:40Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-04-10T19:37:40Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpgkPMAK#match-stress:0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-04-10T19:37:44Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-04-10T19:37:44Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-04-10T19:37:44Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpgkPMAK#match-stress:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpgkPMAK/incremental-state"
[2022-04-10T19:37:48Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-04-10T19:37:48Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpgkPMAK#match-stress:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpgkPMAK/incremental-state"
[2022-04-10T19:37:50Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-04-10T19:37:50Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-04-10T19:37:50Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp3AG0nB#match-stress:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-04-10T19:37:54Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-04-10T19:37:54Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-04-10T19:37:54Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp3AG0nB#match-stress:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp3AG0nB/incremental-state"
[2022-04-10T19:37:58Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-04-10T19:37:58Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp3AG0nB#match-stress:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp3AG0nB/incremental-state"
Preparing token-stream-stress
[2022-04-10T19:38:00Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-04-10T19:38:00Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-04-10T19:38:00Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
---
[2022-04-10T19:38:03Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-04-10T19:38:03Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-04-10T19:38:03Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpV7EIvm#token-stream-stress:0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2022-04-10T19:38:04Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-04-10T19:38:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpV7EIvm#token-stream-stress:0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpV7EIvm/incremental-state"
[2022-04-10T19:38:04Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-04-10T19:38:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpV7EIvm#token-stream-stress:0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpV7EIvm/incremental-state"
Preparing tuple-stress
[2022-04-10T19:38:05Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-04-10T19:38:05Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-04-10T19:38:05Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2022-04-10T19:38:05Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-04-10T19:38:05Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-04-10T19:38:05Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpimhv0C#tuple-stress:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-04-10T19:38:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-04-10T19:38:12Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpimhv0C#tuple-stress:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpimhv0C/incremental-state"
[2022-04-10T19:38:19Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-04-10T19:38:19Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpimhv0C#tuple-stress:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpimhv0C/incremental-state"
[2022-04-10T19:38:21Z DEBUG collector::execute] applying new row to "/tmp/.tmpimhv0C"
[2022-04-10T19:38:21Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-04-10T19:38:21Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-04-10T19:38:21Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpimhv0C#tuple-stress:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpimhv0C/incremental-state"
[2022-04-10T19:38:28Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-04-10T19:38:28Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-04-10T19:38:28Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-04-10T19:38:28Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpfmUIjC#tuple-stress:0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-04-10T19:38:34Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-04-10T19:38:34Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpfmUIjC#tuple-stress:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpfmUIjC/incremental-state"
[2022-04-10T19:38:41Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-04-10T19:38:41Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpfmUIjC#tuple-stress:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpfmUIjC/incremental-state"
[2022-04-10T19:38:44Z DEBUG collector::execute] applying new row to "/tmp/.tmpfmUIjC"
[2022-04-10T19:38:44Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-04-10T19:38:44Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-04-10T19:38:44Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpfmUIjC#tuple-stress:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpfmUIjC/incremental-state"
[2022-04-10T19:38:51Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-04-10T19:38:51Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-04-10T19:38:51Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpdXJz1D#tuple-stress:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-04-10T19:38:57Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-04-10T19:38:57Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-04-10T19:38:57Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpdXJz1D#tuple-stress:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpdXJz1D/incremental-state"
[2022-04-10T19:39:04Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-04-10T19:39:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpdXJz1D#tuple-stress:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpdXJz1D/incremental-state"
[2022-04-10T19:39:06Z DEBUG collector::execute] applying new row to "/tmp/.tmpdXJz1D"
[2022-04-10T19:39:06Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-04-10T19:39:06Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-04-10T19:39:06Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpdXJz1D#tuple-stress:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpdXJz1D/incremental-state"
+ ./build/x86_64-unknown-linux-gnu/llvm/bin/llvm-profdata merge -o /tmp/rustc-pgo.profdata /tmp/rustc-pgo
+ rm -r ./build/x86_64-unknown-linux-gnu/llvm ./build/x86_64-unknown-linux-gnu/lld
+ python3 ../x.py dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu --include-default-paths build-manifest --rust-profile-use=/tmp/rustc-pgo.profdata --llvm-profile-use=/tmp/llvm-pgo.profdata
warning: sqlite3 not available in python, skipping build directory lock
---
    Checking addr2line v0.16.0
[RUSTC-TIMING] addr2line test:false 0.252
[RUSTC-TIMING] object test:false 3.057
 Documenting std v0.0.0 (/checkout/library/std)
thread 'rustc' panicked at 'no entry found for key', src/librustdoc/passes/collect_intra_doc_links.rs:976:16
stack backtrace:
   0:     0x7f320844402d - std::backtrace_rs::backtrace::libunwind::trace::h50c1c945a5f2cb1e
                               at /rustc/26744b574ebb507bdfea3eb38d0a841573c7a124/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7f320844402d - std::backtrace_rs::backtrace::trace_unsynchronized::h5a59cb7cdf32485c
                               at /rustc/26744b574ebb507bdfea3eb38d0a841573c7a124/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f320844402d - std::sys_common::backtrace::_print_fmt::h2fae622a49f7aee2
                               at /rustc/26744b574ebb507bdfea3eb38d0a841573c7a124/library/std/src/sys_common/backtrace.rs:66:5
   3:     0x7f320844402d - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h0d8b01f441c5b0bd
                               at /rustc/26744b574ebb507bdfea3eb38d0a841573c7a124/library/std/src/sys_common/backtrace.rs:45:22
   4:     0x7f320849dd4c - core::fmt::write::h679ee2faa281f816
                               at /rustc/26744b574ebb507bdfea3eb38d0a841573c7a124/library/core/src/fmt/mod.rs:1194:17
   5:     0x7f32084358f1 - std::io::Write::write_fmt::he48f8018b7b72d02
                               at /rustc/26744b574ebb507bdfea3eb38d0a841573c7a124/library/std/src/io/mod.rs:1655:15
   6:     0x7f3208446f45 - std::sys_common::backtrace::_print::h6bb536a2fdcfd7db
                               at /rustc/26744b574ebb507bdfea3eb38d0a841573c7a124/library/std/src/sys_common/backtrace.rs:48:5
   7:     0x7f3208446f45 - std::sys_common::backtrace::print::h85499501407e4757
                               at /rustc/26744b574ebb507bdfea3eb38d0a841573c7a124/library/std/src/sys_common/backtrace.rs:35:9
   8:     0x7f3208446f45 - std::panicking::default_hook::{{closure}}::hbe4a9ff08828bf78
                               at /rustc/26744b574ebb507bdfea3eb38d0a841573c7a124/library/std/src/panicking.rs:295:22
   9:     0x7f3208446bb9 - std::panicking::default_hook::h98b58af309d6351e
                               at /rustc/26744b574ebb507bdfea3eb38d0a841573c7a124/library/std/src/panicking.rs:314:9
  10:     0x7f3208be6f91 - rustc_driver[8602ed13697243da]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7f32084476e0 - std::panicking::rust_panic_with_hook::h609fa8a184ab6c3f
                               at /rustc/26744b574ebb507bdfea3eb38d0a841573c7a124/library/std/src/panicking.rs:702:17
  12:     0x7f3208447517 - std::panicking::begin_panic_handler::{{closure}}::h13fab5a6272ecbf8
                               at /rustc/26744b574ebb507bdfea3eb38d0a841573c7a124/library/std/src/panicking.rs:588:13
  13:     0x7f32084444e4 - std::sys_common::backtrace::__rust_end_short_backtrace::hc5a4787010425993
                               at /rustc/26744b574ebb507bdfea3eb38d0a841573c7a124/library/std/src/sys_common/backtrace.rs:138:18
  14:     0x7f3208447249 - rust_begin_unwind
                               at /rustc/26744b574ebb507bdfea3eb38d0a841573c7a124/library/std/src/panicking.rs:584:5
  15:     0x7f320840b793 - core::panicking::panic_fmt::hd8e5ede81a55cefd
                               at /rustc/26744b574ebb507bdfea3eb38d0a841573c7a124/library/core/src/panicking.rs:142:14
  16:     0x7f320849ab61 - core::panicking::panic_display::h0d47e6813e124086
                               at /rustc/26744b574ebb507bdfea3eb38d0a841573c7a124/library/core/src/panicking.rs:72:5
  17:     0x7f320849ab0b - core::panicking::panic_str::h16e8a8112e8b91fb
                               at /rustc/26744b574ebb507bdfea3eb38d0a841573c7a124/library/core/src/panicking.rs:56:5
  18:     0x7f320840b606 - core::option::expect_failed::h8e7e35e058036248
                               at /rustc/26744b574ebb507bdfea3eb38d0a841573c7a124/library/core/src/option.rs:1855:5
  19:     0x55fa1ce914f9 - rustdoc[326657f5c31177d8]::passes::collect_intra_doc_links::resolve_associated_trait_item
  20:     0x55fa1ce8fc9e - <rustdoc[326657f5c31177d8]::passes::collect_intra_doc_links::LinkCollector>::resolve_associated_item
  21:     0x55fa1ce8ce85 - <rustdoc[326657f5c31177d8]::passes::collect_intra_doc_links::LinkCollector>::resolve
  22:     0x55fa1ce92e09 - <rustdoc[326657f5c31177d8]::passes::collect_intra_doc_links::LinkCollector as rustdoc[326657f5c31177d8]::visit::DocVisitor>::visit_item
  23:     0x55fa1cea004a - <rustdoc[326657f5c31177d8]::passes::collect_intra_doc_links::LinkCollector as rustdoc[326657f5c31177d8]::visit::DocVisitor>::visit_inner_recur
  24:     0x55fa1ce96588 - <rustdoc[326657f5c31177d8]::passes::collect_intra_doc_links::LinkCollector as rustdoc[326657f5c31177d8]::visit::DocVisitor>::visit_item
  25:     0x55fa1ce9ffca - <rustdoc[326657f5c31177d8]::passes::collect_intra_doc_links::LinkCollector as rustdoc[326657f5c31177d8]::visit::DocVisitor>::visit_inner_recur
  26:     0x55fa1ce9656d - <rustdoc[326657f5c31177d8]::passes::collect_intra_doc_links::LinkCollector as rustdoc[326657f5c31177d8]::visit::DocVisitor>::visit_item
  27:     0x55fa1ce9ffca - <rustdoc[326657f5c31177d8]::passes::collect_intra_doc_links::LinkCollector as rustdoc[326657f5c31177d8]::visit::DocVisitor>::visit_inner_recur
  28:     0x55fa1ce9656d - <rustdoc[326657f5c31177d8]::passes::collect_intra_doc_links::LinkCollector as rustdoc[326657f5c31177d8]::visit::DocVisitor>::visit_item
  29:     0x55fa1ce8b43c - rustdoc[326657f5c31177d8]::passes::collect_intra_doc_links::collect_intra_doc_links
  30:     0x55fa1cea5ed9 - <rustc_session[66bf4275014b286d]::session::Session>::time::<(rustdoc[326657f5c31177d8]::clean::types::Crate, rustdoc[326657f5c31177d8]::config::RenderOptions, rustdoc[326657f5c31177d8]::formats::cache::Cache), rustdoc[326657f5c31177d8]::main_options::{closure#0}::{closure#0}::{closure#1}::{closure#0}>
  31:     0x55fa1cef0d56 - <rustc_interface[ab46fd22a7254c2f]::passes::QueryContext>::enter::<rustdoc[326657f5c31177d8]::main_options::{closure#0}::{closure#0}::{closure#1}, core[99f397898898b5ec]::result::Result<(), rustc_errors[873d486d09f0e01c]::ErrorGuaranteed>>
  32:     0x55fa1ce3a629 - <rustc_interface[ab46fd22a7254c2f]::interface::Compiler>::enter::<rustdoc[326657f5c31177d8]::main_options::{closure#0}::{closure#0}, core[99f397898898b5ec]::result::Result<(), rustc_errors[873d486d09f0e01c]::ErrorGuaranteed>>
  33:     0x55fa1ccc9fda - rustc_span[d0ec9ff954d36ced]::with_source_map::<core[99f397898898b5ec]::result::Result<(), rustc_errors[873d486d09f0e01c]::ErrorGuaranteed>, rustc_interface[ab46fd22a7254c2f]::interface::create_compiler_and_run<core[99f397898898b5ec]::result::Result<(), rustc_errors[873d486d09f0e01c]::ErrorGuaranteed>, rustdoc[326657f5c31177d8]::main_options::{closure#0}>::{closure#1}>
  34:     0x55fa1ce5f8f4 - rustc_interface[ab46fd22a7254c2f]::interface::create_compiler_and_run::<core[99f397898898b5ec]::result::Result<(), rustc_errors[873d486d09f0e01c]::ErrorGuaranteed>, rustdoc[326657f5c31177d8]::main_options::{closure#0}>
  35:     0x55fa1cccdba7 - <scoped_tls[890143a7618b618f]::ScopedKey<rustc_span[d0ec9ff954d36ced]::SessionGlobals>>::set::<rustdoc[326657f5c31177d8]::main_args::{closure#0}, core[99f397898898b5ec]::result::Result<(), rustc_errors[873d486d09f0e01c]::ErrorGuaranteed>>
  36:     0x55fa1ce7245f - std[c49257cf7b1c9f3f]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[ab46fd22a7254c2f]::util::run_in_thread_pool_with_globals<rustdoc[326657f5c31177d8]::main_args::{closure#0}, core[99f397898898b5ec]::result::Result<(), rustc_errors[873d486d09f0e01c]::ErrorGuaranteed>>::{closure#0}, core[99f397898898b5ec]::result::Result<(), rustc_errors[873d486d09f0e01c]::ErrorGuaranteed>>
  37:     0x55fa1cf12949 - <<std[c49257cf7b1c9f3f]::thread::Builder>::spawn_unchecked_<rustc_interface[ab46fd22a7254c2f]::util::run_in_thread_pool_with_globals<rustdoc[326657f5c31177d8]::main_args::{closure#0}, core[99f397898898b5ec]::result::Result<(), rustc_errors[873d486d09f0e01c]::ErrorGuaranteed>>::{closure#0}, core[99f397898898b5ec]::result::Result<(), rustc_errors[873d486d09f0e01c]::ErrorGuaranteed>>::{closure#1} as core[99f397898898b5ec]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  38:     0x7f3208451843 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h161a4b888726eedd
                               at /rustc/26744b574ebb507bdfea3eb38d0a841573c7a124/library/alloc/src/boxed.rs:1858:9
  39:     0x7f3208451843 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h0b9fc73cce32602a
                               at /rustc/26744b574ebb507bdfea3eb38d0a841573c7a124/library/alloc/src/boxed.rs:1858:9
  40:     0x7f3208451843 - std::sys::unix::thread::Thread::new::thread_start::h7e88d053a47460d9
                               at /rustc/26744b574ebb507bdfea3eb38d0a841573c7a124/library/std/src/sys/unix/thread.rs:108:17
  41:     0x7f32081908ca - start_thread
  42:     0x7f3207653abd - clone
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.62.0-nightly (26744b574 2022-04-10) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type dylib --crate-type rlib -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
end of query stack
error: could not document `std`

Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type dylib --crate-type rlib --crate-name std library/std/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc --cfg 'feature="addr2line"' --cfg 'feature="backtrace"' --cfg 'feature="compiler-builtins-c"' --cfg 'feature="gimli-symbolize"' --cfg 'feature="miniz_oxide"' --cfg 'feature="object"' --cfg 'feature="panic_unwind"' --cfg 'feature="profiler"' --cfg 'feature="profiler_builtins"' --cfg 'feature="std_detect_dlsym_getauxval"' --cfg 'feature="std_detect_file_io"' --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --markdown-css rust.css --markdown-no-toc -Z unstable-options --resource-suffix 1.62.0 --index-page /checkout/src/doc/index.md -C metadata=12e62b6e57670362 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/deps --extern addr2line=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libaddr2line-6fe1572305185de5.rmeta --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/liballoc-9174162b35fbaaf3.rmeta --extern cfg_if=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libcfg_if-9ab53fc62ccce455.rmeta --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-1bffb59ea5de34a6.rmeta --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libcore-28281d62e40d685b.rmeta --extern hashbrown=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libhashbrown-28bdae850cfca5ed.rmeta --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/liblibc-4532b0396a25d6b3.rmeta --extern miniz_oxide=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libminiz_oxide-7484054d6d74bbdd.rmeta --extern object=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libobject-46c98921a5b83733.rmeta --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libpanic_abort-2988b8f652d24777.rmeta --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libpanic_unwind-196f861107df76ba.rmeta --extern profiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libprofiler_builtins-106d331fd2e5c041.rmeta --extern rustc_demangle=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/librustc_demangle-76470237729ffcd2.rmeta --extern std_detect=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libstd_detect-97a8175f6da23874.rmeta --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libunwind-99acc80e620bf3e9.rmeta -Csymbol-mangling-version=legacy -Zunstable-options -Zunstable-options '--check-cfg=names()' '--check-cfg=values()' '--check-cfg=values(bootstrap)' '--check-cfg=values(stdarch_intel_sde)' '--check-cfg=values(no_fp_fmt_parse)' '--check-cfg=values(no_global_oom_handling)' '--check-cfg=values(freebsd12)' '--check-cfg=values(backtrace_in_libstd)' '--check-cfg=values(target_env,"libnx")' '--check-cfg=values(target_os,"watchos")' '--check-cfg=values(target_arch,"asmjs","spirv","nvptx","nvptx64","le32","xtensa")' '--check-cfg=values(dont_compile_me)' -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.62.0-nightly
  (26744b574
  2022-04-10)' '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")' --cfg backtrace_in_libstd` (exit status: 101)
