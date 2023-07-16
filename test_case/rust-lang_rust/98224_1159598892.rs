plain
Executing benchmark regex-1.5.5 (4/7)
Preparing regex-1.5.5
[2022-06-19T00:47:34Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-06-19T00:47:34Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-06-19T00:47:34Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFtVWLW#regex@1.5.5" "--" "--skip-this-rustc"
[2022-06-19T00:47:34Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp6WohBn#regex@1.5.5" "--release" "--" "--skip-this-rustc"
[2022-06-19T00:47:47Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-19T00:47:47Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-06-19T00:47:47Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpEvhmRm#regex@1.5.5" "--" "--wrap-rustc-with" "Eprintln"
Running regex-1.5.5: Opt + [Full]
---
[2022-06-19T01:04:56Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOL32E1#bitmaps@3.1.0" "--" "--skip-this-rustc"
Running bitmaps-3.1.0: Check + [Full, IncrFull, IncrUnchanged, IncrPatched]
[2022-06-19T01:04:56Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-19T01:04:56Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-06-19T01:04:56Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpZdmYju#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-19T01:04:59Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-06-19T01:04:59Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpZdmYju#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpZdmYju/incremental-state"
[2022-06-19T01:05:02Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-06-19T01:05:02Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpZdmYju#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpZdmYju/incremental-state"
[2022-06-19T01:05:03Z DEBUG collector::execute] applying println to "/tmp/.tmpZdmYju"
[2022-06-19T01:05:03Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-06-19T01:05:03Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-06-19T01:05:03Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpZdmYju#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpZdmYju/incremental-state"
[2022-06-19T01:05:04Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-19T01:05:04Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-06-19T01:05:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpSpTKsF#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-19T01:05:07Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-06-19T01:05:07Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-06-19T01:05:07Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpSpTKsF#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpSpTKsF/incremental-state"
[2022-06-19T01:05:11Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-06-19T01:05:11Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpSpTKsF#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpSpTKsF/incremental-state"
[2022-06-19T01:05:12Z DEBUG collector::execute] applying println to "/tmp/.tmpSpTKsF"
[2022-06-19T01:05:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-06-19T01:05:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-06-19T01:05:12Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpSpTKsF#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpSpTKsF/incremental-state"
[2022-06-19T01:05:13Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-19T01:05:13Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-06-19T01:05:13Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpO6Lfuu#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-19T01:05:16Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-06-19T01:05:16Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-06-19T01:05:16Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpO6Lfuu#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpO6Lfuu/incremental-state"
[2022-06-19T01:05:20Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-06-19T01:05:20Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpO6Lfuu#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpO6Lfuu/incremental-state"
[2022-06-19T01:05:21Z DEBUG collector::execute] applying println to "/tmp/.tmpO6Lfuu"
[2022-06-19T01:05:21Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-06-19T01:05:21Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-06-19T01:05:21Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpO6Lfuu#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpO6Lfuu/incremental-state"
Executing benchmark cargo-0.60.0 (2/8)
Preparing cargo-0.60.0
[2022-06-19T01:05:22Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-06-19T01:05:22Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2022-06-19T01:06:32Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-19T01:06:33Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-06-19T01:06:33Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpeZQ9we#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-19T01:06:59Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-06-19T01:06:59Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpeZQ9we#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpeZQ9we/incremental-state"
[2022-06-19T01:07:30Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-06-19T01:07:30Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpeZQ9we#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpeZQ9we/incremental-state"
[2022-06-19T01:07:36Z DEBUG collector::execute] applying println to "/tmp/.tmpeZQ9we"
[2022-06-19T01:07:36Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-06-19T01:07:36Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-06-19T01:07:36Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpeZQ9we#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpeZQ9we/incremental-state"
[2022-06-19T01:07:49Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-19T01:07:50Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-06-19T01:07:50Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp7cmDon#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-19T01:08:51Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-06-19T01:08:51Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-06-19T01:08:51Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp7cmDon#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp7cmDon/incremental-state"
[2022-06-19T01:10:06Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-06-19T01:10:06Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp7cmDon#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp7cmDon/incremental-state"
[2022-06-19T01:10:20Z DEBUG collector::execute] applying println to "/tmp/.tmp7cmDon"
[2022-06-19T01:10:20Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-06-19T01:10:20Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-06-19T01:10:20Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp7cmDon#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp7cmDon/incremental-state"
[2022-06-19T01:10:49Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-19T01:10:50Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-06-19T01:10:50Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCx2dPu#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-19T01:12:08Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-06-19T01:12:08Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-06-19T01:12:08Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCx2dPu#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpCx2dPu/incremental-state"
[2022-06-19T01:13:29Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-06-19T01:13:29Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCx2dPu#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpCx2dPu/incremental-state"
[2022-06-19T01:13:43Z DEBUG collector::execute] applying println to "/tmp/.tmpCx2dPu"
[2022-06-19T01:13:43Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-06-19T01:13:43Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-06-19T01:13:43Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCx2dPu#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpCx2dPu/incremental-state"
Executing benchmark ctfe-stress-5 (3/8)
Preparing ctfe-stress-5
[2022-06-19T01:14:20Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-06-19T01:14:20Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2022-06-19T01:14:20Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-19T01:14:20Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-06-19T01:14:20Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpoqJHSy#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-19T01:14:33Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-06-19T01:14:33Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpoqJHSy#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpoqJHSy/incremental-state"
[2022-06-19T01:14:48Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-06-19T01:14:48Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpoqJHSy#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpoqJHSy/incremental-state"
[2022-06-19T01:14:49Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-19T01:14:49Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-06-19T01:14:49Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpW28f9n#ctfe-stress-5@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-19T01:15:01Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
---
Preparing diesel-1.4.8
[2022-06-19T01:15:45Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-06-19T01:15:45Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-06-19T01:15:45Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-06-19T01:15:45Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpryvALA#diesel@1.4.8" "--" "--skip-this-rustc"
[2022-06-19T01:15:45Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmphyljRV#diesel@1.4.8" "--profile" "check" "--" "--skip-this-rustc"
[2022-06-19T01:15:45Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpnkJkGW#diesel@1.4.8" "--release" "--" "--skip-this-rustc"
[2022-06-19T01:16:09Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-19T01:16:09Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-06-19T01:16:09Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFdNk7T#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-19T01:16:29Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-06-19T01:16:29Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-06-19T01:16:29Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFdNk7T#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpFdNk7T/incremental-state"
[2022-06-19T01:16:53Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-06-19T01:16:53Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFdNk7T#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpFdNk7T/incremental-state"
[2022-06-19T01:16:57Z DEBUG collector::execute] applying println to "/tmp/.tmpFdNk7T"
[2022-06-19T01:16:57Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-06-19T01:16:57Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-06-19T01:16:57Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFdNk7T#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpFdNk7T/incremental-state"
[2022-06-19T01:17:02Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-19T01:17:02Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-06-19T01:17:02Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpZgn9f8#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-19T01:17:25Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-06-19T01:17:25Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-06-19T01:17:25Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpZgn9f8#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpZgn9f8/incremental-state"
[2022-06-19T01:17:52Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-06-19T01:17:52Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpZgn9f8#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpZgn9f8/incremental-state"
[2022-06-19T01:17:57Z DEBUG collector::execute] applying println to "/tmp/.tmpZgn9f8"
[2022-06-19T01:17:57Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-06-19T01:17:57Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-06-19T01:17:57Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpZgn9f8#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpZgn9f8/incremental-state"
[2022-06-19T01:18:02Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-19T01:18:02Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-06-19T01:18:02Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp6zJ8TJ#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-19T01:18:25Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-06-19T01:18:25Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-06-19T01:18:25Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp6zJ8TJ#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp6zJ8TJ/incremental-state"
[2022-06-19T01:18:52Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-06-19T01:18:52Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp6zJ8TJ#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp6zJ8TJ/incremental-state"
[2022-06-19T01:18:57Z DEBUG collector::execute] applying println to "/tmp/.tmp6zJ8TJ"
[2022-06-19T01:18:57Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-06-19T01:18:57Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-06-19T01:18:57Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp6zJ8TJ#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp6zJ8TJ/incremental-state"
Executing benchmark externs (5/8)
Preparing externs
[2022-06-19T01:19:02Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-06-19T01:19:02Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2022-06-19T01:19:05Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-19T01:19:05Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-06-19T01:19:05Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpeYHiOw#externs@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-19T01:19:06Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-06-19T01:19:06Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpeYHiOw#externs@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpeYHiOw/incremental-state"
[2022-06-19T01:19:08Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-06-19T01:19:08Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpeYHiOw#externs@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpeYHiOw/incremental-state"
[2022-06-19T01:19:08Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-19T01:19:08Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-06-19T01:19:09Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpZDETA5#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-19T01:19:09Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-06-19T01:19:09Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-06-19T01:19:09Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpZDETA5#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpZDETA5/incremental-state"
[2022-06-19T01:19:11Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-06-19T01:19:11Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpZDETA5#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpZDETA5/incremental-state"
Executing benchmark match-stress (6/8)
Preparing match-stress
[2022-06-19T01:19:12Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-06-19T01:19:12Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
---
[2022-06-19T01:19:12Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-19T01:19:12Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-06-19T01:19:12Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmps2RkeX#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-19T01:19:16Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-06-19T01:19:16Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmps2RkeX#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmps2RkeX/incremental-state"
[2022-06-19T01:19:20Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-06-19T01:19:20Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmps2RkeX#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmps2RkeX/incremental-state"
[2022-06-19T01:19:22Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-19T01:19:22Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-06-19T01:19:22Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpb68YPM#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-19T01:19:26Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
---
[2022-06-19T01:19:44Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-19T01:19:44Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-06-19T01:19:44Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpq0aO8u#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-19T01:19:45Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-06-19T01:19:45Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpq0aO8u#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpq0aO8u/incremental-state"
[2022-06-19T01:19:45Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-06-19T01:19:45Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpq0aO8u#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpq0aO8u/incremental-state"
[2022-06-19T01:19:45Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-19T01:19:45Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-06-19T01:19:45Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpGkyvLY#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-19T01:19:46Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-06-19T01:19:46Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-06-19T01:19:46Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpGkyvLY#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpGkyvLY/incremental-state"
[2022-06-19T01:19:46Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-06-19T01:19:46Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpGkyvLY#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpGkyvLY/incremental-state"
Executing benchmark tuple-stress (8/8)
Preparing tuple-stress
[2022-06-19T01:19:47Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-06-19T01:19:47Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-06-19T01:19:47Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-06-19T01:19:47Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-06-19T01:19:47Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpfo8Rge#tuple-stress@0.1.0" "--profile" "check" "--" "--skip-this-rustc"
[2022-06-19T01:19:47Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpgjperZ#tuple-stress@0.1.0" "--release" "--" "--skip-this-rustc"
[2022-06-19T01:19:47Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpZN5YbI#tuple-stress@0.1.0" "--" "--skip-this-rustc"
[2022-06-19T01:19:47Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-19T01:19:47Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-06-19T01:19:47Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpP9fEuL#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-19T01:19:54Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-06-19T01:19:54Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-06-19T01:19:54Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpP9fEuL#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpP9fEuL/incremental-state"
[2022-06-19T01:20:01Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-06-19T01:20:01Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpP9fEuL#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpP9fEuL/incremental-state"
[2022-06-19T01:20:03Z DEBUG collector::execute] applying new row to "/tmp/.tmpP9fEuL"
[2022-06-19T01:20:03Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-06-19T01:20:03Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-06-19T01:20:03Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpP9fEuL#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpP9fEuL/incremental-state"
[2022-06-19T01:20:11Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-19T01:20:11Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-06-19T01:20:11Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpIR98t2#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-19T01:20:18Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
---
[RUSTC-TIMING] self_cell test:false 0.070
   Compiling either v1.6.0
[RUSTC-TIMING] unic_char_range test:false 0.196
   Compiling datafrog v2.0.1
warning: rustc_graphviz.9d80b030-cgu.4: no profile data available for function _RNvXs2_NtCsbEx53AbmnmZ_4core7convertNtNtNtB7_5alloc6layout11LayoutErrorINtB5_4FromBy_E4fromCs7xN3hwtpTzg_14rustc_graphviz Hash = 742261418966908927

warning: rustc_graphviz.9d80b030-cgu.2: no profile data available for function _RNvMNtNtNtCsbEx53AbmnmZ_4core4iter8adapters3revINtB2_3RevINtNtNtB8_3ops5range5RangejEE3newCs7xN3hwtpTzg_14rustc_graphviz Hash = 742261418966908927

warning: rustc_fs_util.e02b8e2b-cgu.0: no profile data available for function _RNvXs2_NtCsbEx53AbmnmZ_4core7convertNtNtNtB7_5alloc6layout11LayoutErrorINtB5_4FromBy_E4fromCsdx1jroOwoqt_13rustc_fs_util Hash = 742261418966908927
[RUSTC-TIMING] itoa test:false 0.130
   Compiling memchr v2.4.1
[RUSTC-TIMING] rustc_fs_util test:false 0.163
warning: `rustc_fs_util` (lib) generated 1 warning
---
   Compiling regex v1.5.5
[RUSTC-TIMING] ahash test:false 0.382
[RUSTC-TIMING] rand_core test:false 0.409
   Compiling rand_chacha v0.3.0
warning: rustc_llvm.398c8ca9-cgu.4: no profile data available for function _RNvXs2_NtCsbEx53AbmnmZ_4core7convertNtNtNtB7_5alloc6layout11LayoutErrorINtB5_4FromBy_E4fromCs5idtY3NCSQT_10rustc_llvm Hash = 742261418966908927
[RUSTC-TIMING] generic_array test:false 0.905
   Compiling rand_xoshiro v0.6.0
[RUSTC-TIMING] rustc_llvm test:false 0.279
warning: `rustc_llvm` (lib) generated 1 warning
---
   Compiling object v0.28.4
[RUSTC-TIMING] aho_corasick test:false 3.329
   Compiling gimli v0.26.1
[RUSTC-TIMING] rand test:false 1.505
warning: rustc_serialize.84c878fa-cgu.5: no profile data available for function _RINvNtCsbEx53AbmnmZ_4core3ptr13drop_in_placeRjECs5v24YBSRnHq_15rustc_serialize Hash = 742261418966908927

warning: rustc_serialize.84c878fa-cgu.5: no profile data available for function _RINvNtCsbEx53AbmnmZ_4core9panicking13assert_failedjjECs5v24YBSRnHq_15rustc_serialize Hash = 742261418966908927

warning: rustc_serialize.84c878fa-cgu.6: no profile data available for function _RNvXsV_NtCsbEx53AbmnmZ_4core3fmtRjNtB5_5Debug3fmtCs5v24YBSRnHq_15rustc_serialize Hash = 1124680650125156080
[RUSTC-TIMING] sha2 test:false 1.388
[RUSTC-TIMING] rustc_serialize test:false 0.745
warning: `rustc_serialize` (lib) generated 3 warnings
[RUSTC-TIMING] regex_automata test:false 3.544
---
   Compiling rustc_ast v0.0.0 (/checkout/compiler/rustc_ast)
   Compiling rustc_error_messages v0.0.0 (/checkout/compiler/rustc_error_messages)
   Compiling rustc_target v0.0.0 (/checkout/compiler/rustc_target)
   Compiling rustc_feature v0.0.0 (/checkout/compiler/rustc_feature)
warning: rustc_error_messages.2b0e6283-cgu.11: no profile data available for function _RINvMsx_NtCsij3fuQu9MVJ_12tracing_core5fieldNtB6_8FieldSet9value_setATRNtB6_5FieldINtNtCsbEx53AbmnmZ_4core6option6OptionRDNtB6_5ValueEL_EEj1_ECsawTty2dn1nj_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.2b0e6283-cgu.11: no profile data available for function _RINvMsx_NtCsij3fuQu9MVJ_12tracing_core5fieldNtB6_8FieldSet9value_setATRNtB6_5FieldINtNtCsbEx53AbmnmZ_4core6option6OptionRDNtB6_5ValueEL_EEj2_ECsawTty2dn1nj_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.2b0e6283-cgu.11: no profile data available for function _RINvMsx_NtCsij3fuQu9MVJ_12tracing_core5fieldNtB6_8FieldSet9value_setATRNtB6_5FieldINtNtCsbEx53AbmnmZ_4core6option6OptionRDNtB6_5ValueEL_EEj5_ECsawTty2dn1nj_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.2b0e6283-cgu.11: no profile data available for function _RINvNtCsbEx53AbmnmZ_4core3ptr13drop_in_placeRINtNtB4_6option6OptionNtCs17lbAI5aIvO_16unic_langid_impl18LanguageIdentifierEECsawTty2dn1nj_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.2b0e6283-cgu.11: no profile data available for function _RINvNtCsbEx53AbmnmZ_4core3ptr13drop_in_placeRINtNtB4_6option6OptionNtNtCs7mnjM6LZoBI_3std4path7PathBufEECsawTty2dn1nj_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.2b0e6283-cgu.11: no profile data available for function _RINvNtCsbEx53AbmnmZ_4core3ptr13drop_in_placeRINtNtB4_6option6OptionRNtNtCs7mnjM6LZoBI_3std4path4PathEECsawTty2dn1nj_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.2b0e6283-cgu.11: no profile data available for function _RINvNtCsbEx53AbmnmZ_4core3ptr13drop_in_placeRINtNtCs3NlKXAsLxam_5alloc3vec3VecNtNtCs7mnjM6LZoBI_3std4path7PathBufEECsawTty2dn1nj_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.2b0e6283-cgu.11: no profile data available for function _RINvNtCsbEx53AbmnmZ_4core3ptr13drop_in_placeRNtCs17lbAI5aIvO_16unic_langid_impl18LanguageIdentifierECsawTty2dn1nj_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.2b0e6283-cgu.11: no profile data available for function _RINvNtCsbEx53AbmnmZ_4core3ptr13drop_in_placeRNtNtCs7mnjM6LZoBI_3std4path7PathBufECsawTty2dn1nj_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.2b0e6283-cgu.11: no profile data available for function _RINvNtCsbEx53AbmnmZ_4core3ptr13drop_in_placeRNtNtCsiq2eXsEnkiW_13fluent_bundle8resource14FluentResourceECsawTty2dn1nj_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.2b0e6283-cgu.11: no profile data available for function _RINvNtCsbEx53AbmnmZ_4core3ptr13drop_in_placeRQNtNtCs7mnjM6LZoBI_3std4path7PathBufECsawTty2dn1nj_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.2b0e6283-cgu.11: no profile data available for function _RINvNtCsbEx53AbmnmZ_4core3ptr13drop_in_placeRRSReECsawTty2dn1nj_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.2b0e6283-cgu.11: no profile data available for function _RINvNtCsij3fuQu9MVJ_12tracing_core5field5debugRINtNtCs3NlKXAsLxam_5alloc3vec3VecNtNtCs7mnjM6LZoBI_3std4path7PathBufEECsawTty2dn1nj_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.2b0e6283-cgu.11: no profile data available for function _RINvNtCsij3fuQu9MVJ_12tracing_core5field5debugRINtNtCsbEx53AbmnmZ_4core6option6OptionNtCs17lbAI5aIvO_16unic_langid_impl18LanguageIdentifierEECsawTty2dn1nj_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.2b0e6283-cgu.11: no profile data available for function _RINvNtCsij3fuQu9MVJ_12tracing_core5field5debugRINtNtCsbEx53AbmnmZ_4core6option6OptionNtNtCs7mnjM6LZoBI_3std4path7PathBufEECsawTty2dn1nj_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.2b0e6283-cgu.11: no profile data available for function _RINvNtCsij3fuQu9MVJ_12tracing_core5field5debugRINtNtCsbEx53AbmnmZ_4core6option6OptionRNtNtCs7mnjM6LZoBI_3std4path4PathEECsawTty2dn1nj_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.2b0e6283-cgu.11: no profile data available for function _RINvNtCsij3fuQu9MVJ_12tracing_core5field5debugRNtCs17lbAI5aIvO_16unic_langid_impl18LanguageIdentifierECsawTty2dn1nj_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.2b0e6283-cgu.11: no profile data available for function _RINvNtCsij3fuQu9MVJ_12tracing_core5field5debugRNtNtCs7mnjM6LZoBI_3std4path7PathBufECsawTty2dn1nj_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.2b0e6283-cgu.11: no profile data available for function _RINvNtCsij3fuQu9MVJ_12tracing_core5field5debugRNtNtCsiq2eXsEnkiW_13fluent_bundle8resource14FluentResourceECsawTty2dn1nj_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.2b0e6283-cgu.11: no profile data available for function _RINvNtCsij3fuQu9MVJ_12tracing_core5field5debugRQNtNtCs7mnjM6LZoBI_3std4path7PathBufECsawTty2dn1nj_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.2b0e6283-cgu.11: no profile data available for function _RINvNtCsij3fuQu9MVJ_12tracing_core5field5debugRRSReECsawTty2dn1nj_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.2b0e6283-cgu.11: no profile data available for function _RNvXsk_NtCsij3fuQu9MVJ_12tracing_core5fieldINtB5_10DebugValueRINtNtCs3NlKXAsLxam_5alloc3vec3VecNtNtCs7mnjM6LZoBI_3std4path7PathBufEENtB5_5Value6recordCsawTty2dn1nj_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.2b0e6283-cgu.11: no profile data available for function _RNvXsk_NtCsij3fuQu9MVJ_12tracing_core5fieldINtB5_10DebugValueRINtNtCsbEx53AbmnmZ_4core6option6OptionNtCs17lbAI5aIvO_16unic_langid_impl18LanguageIdentifierEENtB5_5Value6recordCsawTty2dn1nj_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.2b0e6283-cgu.11: no profile data available for function _RNvXsk_NtCsij3fuQu9MVJ_12tracing_core5fieldINtB5_10DebugValueRINtNtCsbEx53AbmnmZ_4core6option6OptionNtNtCs7mnjM6LZoBI_3std4path7PathBufEENtB5_5Value6recordCsawTty2dn1nj_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.2b0e6283-cgu.11: no profile data available for function _RNvXsk_NtCsij3fuQu9MVJ_12tracing_core5fieldINtB5_10DebugValueRINtNtCsbEx53AbmnmZ_4core6option6OptionRNtNtCs7mnjM6LZoBI_3std4path4PathEENtB5_5Value6recordCsawTty2dn1nj_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.2b0e6283-cgu.11: no profile data available for function _RNvXsk_NtCsij3fuQu9MVJ_12tracing_core5fieldINtB5_10DebugValueRNtCs17lbAI5aIvO_16unic_langid_impl18LanguageIdentifierENtB5_5Value6recordCsawTty2dn1nj_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.2b0e6283-cgu.11: no profile data available for function _RNvXsk_NtCsij3fuQu9MVJ_12tracing_core5fieldINtB5_10DebugValueRNtNtCs7mnjM6LZoBI_3std4path7PathBufENtB5_5Value6recordCsawTty2dn1nj_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.2b0e6283-cgu.11: no profile data available for function _RNvXsk_NtCsij3fuQu9MVJ_12tracing_core5fieldINtB5_10DebugValueRNtNtCsiq2eXsEnkiW_13fluent_bundle8resource14FluentResourceENtB5_5Value6recordCsawTty2dn1nj_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.2b0e6283-cgu.11: no profile data available for function _RNvXsk_NtCsij3fuQu9MVJ_12tracing_core5fieldINtB5_10DebugValueRQNtNtCs7mnjM6LZoBI_3std4path7PathBufENtB5_5Value6recordCsawTty2dn1nj_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.2b0e6283-cgu.11: no profile data available for function _RNvXsk_NtCsij3fuQu9MVJ_12tracing_core5fieldINtB5_10DebugValueRRSReENtB5_5Value6recordCsawTty2dn1nj_20rustc_error_messages Hash = 170957022131388415
[RUSTC-TIMING] rustc_error_messages test:false 1.220
warning: `rustc_error_messages` (lib) generated 30 warnings
[RUSTC-TIMING] rustc_feature test:false 1.352
[RUSTC-TIMING] jemalloc_sys test:false 0.080
