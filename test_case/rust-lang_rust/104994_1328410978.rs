plain
[2022-11-27T23:35:39Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-11-27T23:35:39Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-11-27T23:35:39Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpYd5GAn#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-11-27T23:35:41Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-11-27T23:35:41Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpYd5GAn#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpYd5GAn/incremental-state"
[2022-11-27T23:35:44Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-11-27T23:35:44Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpYd5GAn#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpYd5GAn/incremental-state"
[2022-11-27T23:35:45Z DEBUG collector::execute] applying println to "/tmp/.tmpYd5GAn"
[2022-11-27T23:35:45Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-11-27T23:35:45Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-11-27T23:35:45Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpYd5GAn#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpYd5GAn/incremental-state"
[2022-11-27T23:35:46Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-11-27T23:35:46Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-11-27T23:35:46Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjbjsQv#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-11-27T23:35:48Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-11-27T23:35:48Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-11-27T23:35:48Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjbjsQv#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpjbjsQv/incremental-state"
[2022-11-27T23:35:52Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-11-27T23:35:52Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjbjsQv#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpjbjsQv/incremental-state"
[2022-11-27T23:35:53Z DEBUG collector::execute] applying println to "/tmp/.tmpjbjsQv"
[2022-11-27T23:35:53Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-11-27T23:35:53Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-11-27T23:35:53Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjbjsQv#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpjbjsQv/incremental-state"
[2022-11-27T23:35:54Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-11-27T23:35:54Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-11-27T23:35:54Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpVHL1oH#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-11-27T23:35:57Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-11-27T23:35:57Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-11-27T23:35:57Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpVHL1oH#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpVHL1oH/incremental-state"
[2022-11-27T23:36:00Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-11-27T23:36:00Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpVHL1oH#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpVHL1oH/incremental-state"
[2022-11-27T23:36:01Z DEBUG collector::execute] applying println to "/tmp/.tmpVHL1oH"
[2022-11-27T23:36:01Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-11-27T23:36:01Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-11-27T23:36:01Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpVHL1oH#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpVHL1oH/incremental-state"
Executing benchmark cargo-0.60.0 (2/8)
Preparing cargo-0.60.0
[2022-11-27T23:36:03Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-11-27T23:36:03Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
---
[2022-11-27T23:37:15Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-11-27T23:37:16Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-11-27T23:37:16Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpSQNsdc#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2022-11-27T23:37:39Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-11-27T23:37:39Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpSQNsdc#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpSQNsdc/incremental-state"
[2022-11-27T23:38:07Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-11-27T23:38:07Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpSQNsdc#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpSQNsdc/incremental-state"
[2022-11-27T23:38:11Z DEBUG collector::execute] applying println to "/tmp/.tmpSQNsdc"
[2022-11-27T23:38:11Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-11-27T23:38:11Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-11-27T23:38:11Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpSQNsdc#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpSQNsdc/incremental-state"
[2022-11-27T23:38:17Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-11-27T23:38:18Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-11-27T23:38:18Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpdcuJ7i#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2022-11-27T23:39:24Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-11-27T23:39:24Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-11-27T23:39:24Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpdcuJ7i#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpdcuJ7i/incremental-state"
[2022-11-27T23:40:36Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-11-27T23:40:36Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpdcuJ7i#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpdcuJ7i/incremental-state"
[2022-11-27T23:40:47Z DEBUG collector::execute] applying println to "/tmp/.tmpdcuJ7i"
[2022-11-27T23:40:47Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-11-27T23:40:47Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-11-27T23:40:47Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpdcuJ7i#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpdcuJ7i/incremental-state"
[2022-11-27T23:41:06Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-11-27T23:41:07Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-11-27T23:41:07Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp98tFlf#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2022-11-27T23:42:17Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
[2022-11-27T23:44:07Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-11-27T23:44:07Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-11-27T23:44:07Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpiU1Gwq#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-11-27T23:44:17Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-11-27T23:44:17Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpiU1Gwq#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpiU1Gwq/incremental-state"
[2022-11-27T23:44:27Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-11-27T23:44:27Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpiU1Gwq#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpiU1Gwq/incremental-state"
[2022-11-27T23:44:27Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-11-27T23:44:27Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-11-27T23:44:27Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpNsboVu#ctfe-stress-5@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-11-27T23:44:36Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
---
[2022-11-27T23:44:47Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-11-27T23:44:47Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-11-27T23:44:47Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwmL7ii#ctfe-stress-5@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-11-27T23:44:56Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-11-27T23:44:56Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwmL7ii#ctfe-stress-5@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpwmL7ii/incremental-state"
[2022-11-27T23:45:06Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-11-27T23:45:06Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwmL7ii#ctfe-stress-5@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpwmL7ii/incremental-state"
Executing benchmark diesel-1.4.8 (4/8)
Preparing diesel-1.4.8
[2022-11-27T23:45:06Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-11-27T23:45:06Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2022-11-27T23:45:30Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-11-27T23:45:30Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-11-27T23:45:30Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpfZOvKR#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-11-27T23:45:49Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-11-27T23:45:49Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpfZOvKR#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpfZOvKR/incremental-state"
[2022-11-27T23:46:10Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-11-27T23:46:10Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpfZOvKR#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpfZOvKR/incremental-state"
[2022-11-27T23:46:14Z DEBUG collector::execute] applying println to "/tmp/.tmpfZOvKR"
[2022-11-27T23:46:14Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-11-27T23:46:14Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-11-27T23:46:14Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpfZOvKR#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpfZOvKR/incremental-state"
[2022-11-27T23:46:17Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-11-27T23:46:17Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-11-27T23:46:17Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpLhXlzr#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln"
[2022-11-27T23:46:37Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
---
[2022-11-27T23:47:11Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-11-27T23:47:11Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-11-27T23:47:11Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpD5pHYO#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-11-27T23:47:33Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-11-27T23:47:33Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpD5pHYO#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpD5pHYO/incremental-state"
[2022-11-27T23:47:59Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-11-27T23:47:59Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpD5pHYO#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpD5pHYO/incremental-state"
[2022-11-27T23:48:03Z DEBUG collector::execute] applying println to "/tmp/.tmpD5pHYO"
[2022-11-27T23:48:03Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-11-27T23:48:03Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-11-27T23:48:03Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpD5pHYO#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpD5pHYO/incremental-state"
Executing benchmark externs (5/8)
Preparing externs
[2022-11-27T23:48:07Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-11-27T23:48:07Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2022-11-27T23:48:31Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-11-27T23:48:31Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-11-27T23:48:31Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpglfDgA#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-11-27T23:48:34Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-11-27T23:48:34Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpglfDgA#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpglfDgA/incremental-state"
[2022-11-27T23:48:38Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-11-27T23:48:38Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpglfDgA#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpglfDgA/incremental-state"
Executing benchmark token-stream-stress (7/8)
Preparing token-stream-stress
[2022-11-27T23:48:39Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-11-27T23:48:39Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2022-11-27T23:48:41Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-11-27T23:48:41Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-11-27T23:48:41Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpz9tKeG#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2022-11-27T23:48:41Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-11-27T23:48:41Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpz9tKeG#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpz9tKeG/incremental-state"
[2022-11-27T23:48:41Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-11-27T23:48:41Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpz9tKeG#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpz9tKeG/incremental-state"
[2022-11-27T23:48:42Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-11-27T23:48:42Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-11-27T23:48:42Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJzvYeV#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2022-11-27T23:48:42Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-11-27T23:48:42Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-11-27T23:48:42Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJzvYeV#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpJzvYeV/incremental-state"
[2022-11-27T23:48:42Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-11-27T23:48:42Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJzvYeV#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpJzvYeV/incremental-state"
Executing benchmark tuple-stress (8/8)
Preparing tuple-stress
[2022-11-27T23:48:42Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-11-27T23:48:42Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
---
[2022-11-27T23:48:43Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-11-27T23:48:43Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-11-27T23:48:43Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKciMLy#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-11-27T23:48:48Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-11-27T23:48:48Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKciMLy#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpKciMLy/incremental-state"
[2022-11-27T23:48:54Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-11-27T23:48:54Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKciMLy#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpKciMLy/incremental-state"
[2022-11-27T23:48:55Z DEBUG collector::execute] applying new row to "/tmp/.tmpKciMLy"
[2022-11-27T23:48:55Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-11-27T23:48:55Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-11-27T23:48:55Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKciMLy#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpKciMLy/incremental-state"
[2022-11-27T23:49:01Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-11-27T23:49:01Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-11-27T23:49:01Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpp6Qkrs#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-11-27T23:49:07Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-11-27T23:49:07Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-11-27T23:49:07Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpp6Qkrs#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpp6Qkrs/incremental-state"
[2022-11-27T23:49:13Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-11-27T23:49:13Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpp6Qkrs#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpp6Qkrs/incremental-state"
[2022-11-27T23:49:14Z DEBUG collector::execute] applying new row to "/tmp/.tmpp6Qkrs"
[2022-11-27T23:49:14Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-11-27T23:49:14Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-11-27T23:49:14Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpp6Qkrs#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpp6Qkrs/incremental-state"
[2022-11-27T23:49:20Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-11-27T23:49:20Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-11-27T23:49:20Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpyPtCEu#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-11-27T23:49:26Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-11-27T23:49:26Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-11-27T23:49:26Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpyPtCEu#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpyPtCEu/incremental-state"
[2022-11-27T23:49:32Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-11-27T23:49:32Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpyPtCEu#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpyPtCEu/incremental-state"
[2022-11-27T23:49:33Z DEBUG collector::execute] applying new row to "/tmp/.tmpyPtCEu"
[2022-11-27T23:49:33Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-11-27T23:49:33Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-11-27T23:49:33Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpyPtCEu#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpyPtCEu/incremental-state"
+ cd /checkout/obj
+ RUSTC_PROFILE_MERGED_FILE=/tmp/tmp-pgo/rustc-pgo.profdata
+ /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-profdata merge -o /tmp/tmp-pgo/rustc-pgo.profdata /tmp/tmp-pgo/rustc-pgo
+ echo 'Rustc PGO statistics'
---
   Compiling tracing-subscriber v0.3.3
   Compiling tracing-tree v0.2.0
[RUSTC-TIMING] cssparser test:false 2.089
[RUSTC-TIMING] tracing_tree test:false 0.555
   Compiling lol_html v0.2.0
[RUSTC-TIMING] selectors test:false 1.424
[RUSTC-TIMING] tracing_subscriber test:false 3.540
[RUSTC-TIMING] syn test:false 12.826
[RUSTC-TIMING] syn test:false 12.826
[RUSTC-TIMING] lol_html test:false 3.175
[RUSTC-TIMING] serde test:false 4.451
[RUSTC-TIMING] serde test:false 4.451
   Compiling cargo-deadlinks v0.8.0 (https://github.com/jyn514/cargo-deadlinks/?branch=http-feature#6eb4eb1a)
[RUSTC-TIMING] serde test:false 4.729
   Compiling askama_shared v0.12.0
[RUSTC-TIMING] cargo_deadlinks test:false 1.459
[RUSTC-TIMING] toml test:false 1.735
---
Documenting standalone (x86_64-unknown-linux-gnu)
[TIMING] doc::Standalone { compiler: Compiler { stage: 2, host: x86_64-unknown-linux-gnu }, target: x86_64-unknown-linux-gnu } -- 0.546
Documenting stage2 std (x86_64-unknown-linux-gnu) in HTML format
 Documenting core v0.0.0 (/checkout/library/core)
error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/prelude/v1/macro.derive.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/reference/attributes/derive.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/prelude/v1/macro.alloc_error_handler.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/alloc/fn.handle_alloc_error.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/marker/trait.Unsize.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/rc/struct.Rc.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/nomicon/coercions.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/iter/trait.Iterator.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/collections/struct.VecDeque.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/ffi/struct.OsStr.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/string/struct.String.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/book/ch03-05-control-flow.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/result/index.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/io/trait.Write.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/io/trait.Write.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/io/type.Result.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/io/index.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/io/struct.Error.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/intrinsics/fn.ceilf64.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/primitive.f64.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/intrinsics/fn.log10f64.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/primitive.f64.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/intrinsics/fn.transmute.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/reference/behavior-considered-undefined.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/nomicon/what-unsafe-does.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/nomicon/transmutes.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/intrinsics/fn.copy_nonoverlapping.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/vec/struct.Vec.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/intrinsics/fn.log2f64.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/primitive.f64.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/intrinsics/fn.logf32.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/primitive.f32.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/intrinsics/fn.fmaf64.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/primitive.f64.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/primitive.fn.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/nomicon/ffi.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/nomicon/ffi.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/fmt/trait.UpperExp.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/fmt/index.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/fmt/struct.Error.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/error/trait.Error.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/io/struct.Error.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/primitive.char.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/string/struct.String.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/book/ch06-02-match.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/keyword.as.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/string/trait.ToString.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/string/struct.String.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/marker/trait.Send.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/nomicon/send-and-sync.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/sync/struct.Arc.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/reference/behavior-considered-undefined.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/rc/struct.Rc.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/intrinsics/fn.log10f32.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/primitive.f32.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/intrinsics/fn.powf64.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/primitive.f64.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/intrinsics/fn.truncf32.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/primitive.f32.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/intrinsics/fn.sinf64.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/primitive.f64.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/intrinsics/fn.copysignf32.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/primitive.f32.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/cmp/struct.Reverse.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/vec/struct.Vec.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/macro.cfg.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/reference/conditional-compilation.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/future/index.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/keyword.await.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/keyword.async.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/fmt/trait.Octal.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/fmt/index.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/marker/trait.Sized.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/book/ch17-02-trait-objects.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/marker/trait.Sync.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/boxed/struct.Box.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/nomicon/send-and-sync.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/sync/struct.Mutex.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/sync/struct.Arc.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/reference/behavior-considered-undefined.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/rc/struct.Rc.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/sync/struct.RwLock.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/vec/struct.Vec.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/index.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/embedded-book/index.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/rust-by-example/index.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/book/index.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/index.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/error_codes/index.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/cargo/index.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/reference/index.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/unstable-book/index.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/edition-guide/index.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/rustdoc/index.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/rustc/index.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/clippy/index.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/style-guide/index.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/nomicon/index.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/ops/enum.Bound.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/collections/btree_map/struct.BTreeMap.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/alloc/trait.Allocator.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/alloc/alloc/fn.handle_alloc_error.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/alloc/struct.Layout.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/book/ch17-02-trait-objects.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/unstable-book/language-features/extern-types.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/alloc/trait.GlobalAlloc.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/alloc/alloc/fn.handle_alloc_error.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/hash/index.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/collections/struct.HashMap.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/collections/struct.HashSet.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/marker/trait.Copy.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/vec/struct.Vec.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/string/struct.String.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/error_codes/E0204.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/intrinsics/fn.exp2f64.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/primitive.f64.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/intrinsics/fn.expf64.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/primitive.f64.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/intrinsics/fn.cosf32.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/primitive.f32.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/intrinsics/fn.roundf64.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/primitive.f64.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/intrinsics/fn.sqrtf32.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/primitive.f32.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/intrinsics/fn.fabsf32.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/primitive.f32.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/intrinsics/fn.cosf64.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/primitive.f64.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/ops/trait.FnMut.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/nomicon/hrtb.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/book/ch13-01-closures.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/convert/trait.Into.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/string/struct.String.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/vec/struct.Vec.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/convert/trait.AsRef.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/string/struct.String.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/ptr/fn.copy_nonoverlapping.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/vec/struct.Vec.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/ops/trait.Drop.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/error_codes/E0040.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/ops/trait.FnOnce.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/book/ch13-01-closures.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/nomicon/hrtb.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/ops/trait.DerefMut.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/reference/expressions/operator-expr.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/reference/type-coercions.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/reference/expressions/method-call-expr.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/book/ch15-02-deref.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/borrow/trait.Borrow.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/rc/struct.Rc.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/boxed/struct.Box.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/collections/struct.HashMap.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/string/struct.String.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/ops/trait.CoerceUnsized.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/nomicon/coercions.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/fmt/trait.Pointer.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/fmt/index.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/primitive.never.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/fs/struct.File.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/process/fn.exit.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/string/struct.String.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/mem/fn.size_of_val_raw.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/book/ch17-02-trait-objects.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/unstable-book/language-features/extern-types.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/intrinsics/fn.truncf64.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/primitive.f64.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/intrinsics/fn.ceilf32.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/primitive.f32.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/intrinsics/fn.floorf64.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/primitive.f64.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/intrinsics/fn.roundf32.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/primitive.f32.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/macro.env.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/env/fn.var.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/ops/trait.Generator.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/unstable-book/language-features/generators.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/fmt/trait.Display.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/string/trait.ToString.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/string/trait.ToString.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/fmt/index.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/hash/trait.BuildHasher.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/collections/struct.HashMap.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/macro.write.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/io/trait.Write.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/io/type.Result.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/fmt/index.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/panic/trait.UnwindSafe.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/panic/fn.catch_unwind.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/iter/fn.from_fn.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/keyword.move.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/intrinsics/fn.floorf32.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/primitive.f32.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/intrinsics/fn.sqrtf64.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/primitive.f64.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/intrinsics/fn.powif64.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/primitive.f64.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/fmt/struct.Arguments.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/fmt/fn.format.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/prelude/v1/macro.derive_const.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/reference/attributes/derive.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/prelude/v1/macro.global_allocator.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/alloc/trait.GlobalAlloc.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/prelude/v1/macro.test.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/reference/attributes/testing.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/mem/fn.size_of_val.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/book/ch17-02-trait-objects.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/mem/fn.transmute_copy.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/reference/behavior-considered-undefined.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/str/struct.Utf8Chunks.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/string/struct.String.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/pin/macro.pin.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/boxed/struct.Box.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/intrinsics/fn.copysignf64.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/primitive.f64.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/intrinsics/fn.fabsf64.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/primitive.f64.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/ops/index.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/reference/expressions.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/ops/trait.Deref.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/reference/expressions/operator-expr.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/reference/type-coercions.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/reference/expressions/method-call-expr.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/book/ch15-02-deref.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/fmt/trait.Write.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/io/trait.Write.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/io/trait.Write.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/fmt/trait.UpperHex.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/fmt/index.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/fmt/trait.Binary.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/fmt/index.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/macro.option_env.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/env/fn.var.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/macro.format_args.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/macro.format.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/macro.println.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/fmt/index.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/slice/struct.ArrayChunksMut.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/slice/struct.ArrayChunksMut.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/ptr/index.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/nomicon/exotic-sizes.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/reference/behavior-considered-undefined.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/book/ch19-01-unsafe-rust.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/nomicon/aliasing.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/macro.assert.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/fmt/index.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/time/struct.Duration.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/time/struct.Instant.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/time/struct.SystemTime.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/primitive.tuple.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/book/ch03-02-data-types.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/ops/trait.Fn.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/book/ch13-01-closures.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/nomicon/hrtb.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/marker/struct.PhantomData.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/nomicon/dropck.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/nomicon/phantom-data.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/book/ch19-01-unsafe-rust.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/panic/struct.AssertUnwindSafe.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/panic/fn.catch_unwind.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/slice/index.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/slice/index.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/mem/fn.transmute.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/nomicon/transmutes.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/reference/behavior-considered-undefined.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/nomicon/what-unsafe-does.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/mem/union.MaybeUninit.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/io/trait.Read.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/vec/struct.Vec.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/reference/behavior-considered-undefined.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/primitive.slice.html:
           Fragment #method.to_ascii_lowercase at /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/primitive.slice.html does not exist!
           Fragment #method.sort_by_cached_key at /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/primitive.slice.html does not exist!
           Fragment #method.to_ascii_uppercase at /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/primitive.slice.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/reference/dynamically-sized-types.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/fmt/trait.LowerHex.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/fmt/index.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/iter/index.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/sync/mpsc/struct.TryIter.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/collections/struct.HashSet.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/convert/trait.From.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/book/ch09-00-error-handling.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/string/struct.String.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/result/enum.Result.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/error/index.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/error/index.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/option/index.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/boxed/struct.Box.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/primitive.array.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/reference/patterns.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/intrinsics/fn.expf32.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/primitive.f32.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/intrinsics/fn.log2f32.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/primitive.f32.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/intrinsics/fn.powf32.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/primitive.f32.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/ops/trait.Add.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/time/struct.SystemTime.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/str/fn.from_utf8.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/string/struct.String.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/str/struct.Utf8Error.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/char/constant.REPLACEMENT_CHARACTER.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/string/struct.String.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/sync/atomic/index.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/nomicon/atomics.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/thread/index.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/sync/struct.Arc.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/sync/atomic/enum.Ordering.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/nomicon/atomics.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/fmt/trait.LowerExp.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/fmt/index.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/error/index.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/panic/fn.take_hook.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/panic/fn.catch_unwind.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/panic/fn.resume_unwind.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/keyword.match.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/panic/fn.panic_any.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/panic/fn.set_hook.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/process/trait.Termination.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/result/index.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/ffi/struct.CStr.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/ffi/struct.CString.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/string/struct.String.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/hash/struct.BuildHasherDefault.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/collections/struct.HashMap.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/collections/struct.HashSet.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/cell/index.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/rc/struct.Rc.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/sync/struct.RwLock.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/sync/struct.Arc.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/sync/struct.Mutex.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/pin/index.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/rc/struct.Rc.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/boxed/struct.Box.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/collections/struct.VecDeque.html does not exist!
           Fragment #1 at /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/pin/index.html does not exist!
           Fragment #2 at /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/pin/index.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/vec/struct.Vec.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/vec/struct.Vec.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/vec/struct.Vec.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/vec/struct.Vec.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/keyword.unsafe.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/macro.panic.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/panic/fn.set_hook.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/book/ch09-00-error-handling.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/result/index.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/panic/fn.panic_any.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/macro.format.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/task/enum.Poll.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/string/struct.String.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/any/index.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/boxed/struct.Box.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/hint/fn.spin_loop.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/thread/fn.yield_now.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/future/trait.Future.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/keyword.async.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/ops/trait.Sub.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/time/struct.SystemTime.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/fmt/trait.Debug.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/fmt/index.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/cell/struct.LazyCell.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/sync/struct.LazyLock.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/str/index.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/str/index.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/option/enum.Option.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/error/index.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/string/struct.String.html does not exist!
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/error/index.html does not exist!

error: Found invalid urls in /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/core/intrinsics/fn.abort.html:
           Linked file at path /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc/std/process/fn.abort.html does not exist!
