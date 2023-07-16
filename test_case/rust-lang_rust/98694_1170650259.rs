plain
Executing benchmark regex-1.5.5 (4/7)
Preparing regex-1.5.5
[2022-06-29T23:59:13Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-06-29T23:59:13Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-06-29T23:59:13Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpcvXosy#regex@1.5.5" "--release" "--" "--skip-this-rustc"
[2022-06-29T23:59:13Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpBpbTpm#regex@1.5.5" "--" "--skip-this-rustc"
[2022-06-29T23:59:28Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-29T23:59:28Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-06-29T23:59:28Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpn34Ra1#regex@1.5.5" "--" "--wrap-rustc-with" "Eprintln"
Running regex-1.5.5: Opt + [Full]
---
Executing benchmark ripgrep-13.0.0 (5/7)
Preparing ripgrep-13.0.0
[2022-06-29T23:59:53Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-06-29T23:59:53Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-06-29T23:59:53Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpsbriHH#ripgrep@13.0.0" "--release" "--" "--skip-this-rustc"
[2022-06-29T23:59:53Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpLMtc5H#ripgrep@13.0.0" "--" "--skip-this-rustc"
[2022-06-30T00:00:27Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-30T00:00:27Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-06-30T00:00:27Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpAKnhVs#ripgrep@13.0.0" "--" "--wrap-rustc-with" "Eprintln"
Running ripgrep-13.0.0: Opt + [Full]
---
Executing benchmark syn-1.0.89 (7/7)
Preparing syn-1.0.89
[2022-06-30T00:01:14Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-06-30T00:01:14Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-06-30T00:01:14Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwzAppQ#syn@1.0.89" "--release" "--" "--skip-this-rustc"
[2022-06-30T00:01:14Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpRAsR5q#syn@1.0.89" "--" "--skip-this-rustc"
[2022-06-30T00:01:19Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-30T00:01:19Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-06-30T00:01:19Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpleoTvB#syn@1.0.89" "--" "--wrap-rustc-with" "Eprintln"
Running syn-1.0.89: Opt + [Full]
---
[2022-06-30T00:16:59Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-30T00:16:59Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-06-30T00:16:59Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp9SJLjY#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-30T00:17:02Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-06-30T00:17:02Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp9SJLjY#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp9SJLjY/incremental-state"
[2022-06-30T00:17:06Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-06-30T00:17:06Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp9SJLjY#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp9SJLjY/incremental-state"
[2022-06-30T00:17:07Z DEBUG collector::execute] applying println to "/tmp/.tmp9SJLjY"
[2022-06-30T00:17:07Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-06-30T00:17:07Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-06-30T00:17:07Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp9SJLjY#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp9SJLjY/incremental-state"
[2022-06-30T00:17:08Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-30T00:17:08Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-06-30T00:17:08Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjaYD98#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-30T00:17:11Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
---
[2022-06-30T00:17:17Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-30T00:17:17Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-06-30T00:17:17Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp2bjFM4#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-30T00:17:20Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-06-30T00:17:20Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp2bjFM4#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp2bjFM4/incremental-state"
[2022-06-30T00:17:24Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-06-30T00:17:24Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp2bjFM4#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp2bjFM4/incremental-state"
[2022-06-30T00:17:25Z DEBUG collector::execute] applying println to "/tmp/.tmp2bjFM4"
[2022-06-30T00:17:25Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-06-30T00:17:25Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-06-30T00:17:25Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp2bjFM4#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp2bjFM4/incremental-state"
Executing benchmark cargo-0.60.0 (2/8)
Preparing cargo-0.60.0
[2022-06-30T00:17:27Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-06-30T00:17:27Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2022-06-30T00:19:57Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-30T00:19:58Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-06-30T00:19:58Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpnpjp5b#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-30T00:21:02Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-06-30T00:21:02Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpnpjp5b#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpnpjp5b/incremental-state"
[2022-06-30T00:22:19Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-06-30T00:22:19Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpnpjp5b#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpnpjp5b/incremental-state"
[2022-06-30T00:22:32Z DEBUG collector::execute] applying println to "/tmp/.tmpnpjp5b"
[2022-06-30T00:22:32Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-06-30T00:22:32Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-06-30T00:22:32Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpnpjp5b#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpnpjp5b/incremental-state"
[2022-06-30T00:23:02Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-30T00:23:02Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-06-30T00:23:02Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmps7Uj7J#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-30T00:24:21Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
[2022-06-30T00:27:01Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-30T00:27:01Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-06-30T00:27:01Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzII7Oi#ctfe-stress-5@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-30T00:27:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-06-30T00:27:12Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzII7Oi#ctfe-stress-5@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpzII7Oi/incremental-state"
[2022-06-30T00:27:25Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-06-30T00:27:25Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzII7Oi#ctfe-stress-5@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpzII7Oi/incremental-state"
[2022-06-30T00:27:26Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-30T00:27:26Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-06-30T00:27:26Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmptsNEP8#ctfe-stress-5@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-30T00:27:36Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-06-30T00:27:36Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-06-30T00:27:36Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmptsNEP8#ctfe-stress-5@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmptsNEP8/incremental-state"
[2022-06-30T00:27:49Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-06-30T00:27:49Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmptsNEP8#ctfe-stress-5@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmptsNEP8/incremental-state"
Executing benchmark diesel-1.4.8 (4/8)
Preparing diesel-1.4.8
[2022-06-30T00:27:50Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-06-30T00:27:50Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-06-30T00:27:50Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-06-30T00:27:50Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-06-30T00:27:50Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDtLiuO#diesel@1.4.8" "--profile" "check" "--" "--skip-this-rustc"
[2022-06-30T00:27:50Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmptu8Clk#diesel@1.4.8" "--release" "--" "--skip-this-rustc"
[2022-06-30T00:27:50Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpIuYbla#diesel@1.4.8" "--" "--skip-this-rustc"
[2022-06-30T00:28:14Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-30T00:28:14Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-06-30T00:28:14Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpLB02AB#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-30T00:28:34Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
---
[2022-06-30T00:29:08Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-30T00:29:08Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-06-30T00:29:08Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpie8gdW#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-30T00:29:31Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-06-30T00:29:31Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpie8gdW#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpie8gdW/incremental-state"
[2022-06-30T00:29:58Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-06-30T00:29:58Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpie8gdW#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpie8gdW/incremental-state"
[2022-06-30T00:30:02Z DEBUG collector::execute] applying println to "/tmp/.tmpie8gdW"
[2022-06-30T00:30:02Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-06-30T00:30:02Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-06-30T00:30:02Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpie8gdW#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpie8gdW/incremental-state"
[2022-06-30T00:30:07Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-30T00:30:07Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-06-30T00:30:07Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-06-30T00:30:07Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpeMTpRy#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-30T00:30:30Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-06-30T00:30:31Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpeMTpRy#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpeMTpRy/incremental-state"
[2022-06-30T00:30:58Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-06-30T00:30:58Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpeMTpRy#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpeMTpRy/incremental-state"
[2022-06-30T00:31:03Z DEBUG collector::execute] applying println to "/tmp/.tmpeMTpRy"
[2022-06-30T00:31:03Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-06-30T00:31:03Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-06-30T00:31:03Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpeMTpRy#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpeMTpRy/incremental-state"
Executing benchmark externs (5/8)
Preparing externs
[2022-06-30T00:31:08Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-06-30T00:31:08Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2022-06-30T00:31:11Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-30T00:31:11Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-06-30T00:31:11Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJDSsXa#externs@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-30T00:31:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-06-30T00:31:12Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJDSsXa#externs@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpJDSsXa/incremental-state"
[2022-06-30T00:31:13Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-06-30T00:31:13Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJDSsXa#externs@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpJDSsXa/incremental-state"
[2022-06-30T00:31:14Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-30T00:31:14Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-06-30T00:31:14Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpyBL1Zt#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-30T00:31:15Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-06-30T00:31:15Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-06-30T00:31:15Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpyBL1Zt#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpyBL1Zt/incremental-state"
[2022-06-30T00:31:17Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-06-30T00:31:17Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpyBL1Zt#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpyBL1Zt/incremental-state"
Executing benchmark match-stress (6/8)
Preparing match-stress
[2022-06-30T00:31:17Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-06-30T00:31:17Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2022-06-30T00:31:18Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-30T00:31:18Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-06-30T00:31:18Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpSwhHI3#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-30T00:31:21Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-06-30T00:31:21Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpSwhHI3#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpSwhHI3/incremental-state"
[2022-06-30T00:31:26Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-06-30T00:31:26Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpSwhHI3#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpSwhHI3/incremental-state"
[2022-06-30T00:31:27Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-30T00:31:27Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-06-30T00:31:27Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCkY4Fh#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-30T00:31:30Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-06-30T00:31:30Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-06-30T00:31:31Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCkY4Fh#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpCkY4Fh/incremental-state"
[2022-06-30T00:31:35Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-06-30T00:31:35Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCkY4Fh#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpCkY4Fh/incremental-state"
[2022-06-30T00:31:36Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-30T00:31:36Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-06-30T00:31:36Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzajpHV#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-30T00:31:40Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-06-30T00:31:40Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-06-30T00:31:40Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzajpHV#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpzajpHV/incremental-state"
[2022-06-30T00:31:45Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-06-30T00:31:45Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzajpHV#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpzajpHV/incremental-state"
Executing benchmark token-stream-stress (7/8)
Preparing token-stream-stress
[2022-06-30T00:31:46Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-06-30T00:31:46Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-06-30T00:31:46Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-06-30T00:31:46Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-06-30T00:31:46Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpNrhIPl#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--skip-this-rustc"
[2022-06-30T00:31:46Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpfoxqBm#token-stream-stress@0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--skip-this-rustc"
[2022-06-30T00:31:46Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpVuc6U2#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--skip-this-rustc"
[2022-06-30T00:31:47Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-30T00:31:47Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-06-30T00:31:47Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpZkwLcd#token-stream-stress@0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-30T00:31:47Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-06-30T00:31:47Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-06-30T00:31:47Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpZkwLcd#token-stream-stress@0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpZkwLcd/incremental-state"
[2022-06-30T00:31:48Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-06-30T00:31:48Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpZkwLcd#token-stream-stress@0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpZkwLcd/incremental-state"
[2022-06-30T00:31:48Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-30T00:31:48Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-06-30T00:31:48Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp6C8cO1#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-30T00:31:48Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
---
Preparing tuple-stress
[2022-06-30T00:31:50Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-06-30T00:31:50Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-06-30T00:31:50Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-06-30T00:31:50Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpYTrYiI#tuple-stress@0.1.0" "--" "--skip-this-rustc"
[2022-06-30T00:31:50Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpkuaHMT#tuple-stress@0.1.0" "--release" "--" "--skip-this-rustc"
[2022-06-30T00:31:50Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpw6y1wS#tuple-stress@0.1.0" "--profile" "check" "--" "--skip-this-rustc"
[2022-06-30T00:31:50Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-30T00:31:50Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-06-30T00:31:50Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpmJZobc#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-30T00:31:57Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-06-30T00:31:57Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-06-30T00:31:57Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpmJZobc#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpmJZobc/incremental-state"
[2022-06-30T00:32:04Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-06-30T00:32:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpmJZobc#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpmJZobc/incremental-state"
[2022-06-30T00:32:06Z DEBUG collector::execute] applying new row to "/tmp/.tmpmJZobc"
[2022-06-30T00:32:06Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-06-30T00:32:06Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-06-30T00:32:06Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpmJZobc#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpmJZobc/incremental-state"
[2022-06-30T00:32:13Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-30T00:32:13Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-06-30T00:32:13Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpPyJOiv#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-30T00:32:19Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-06-30T00:32:19Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-06-30T00:32:19Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpPyJOiv#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpPyJOiv/incremental-state"
[2022-06-30T00:32:27Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-06-30T00:32:27Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpPyJOiv#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpPyJOiv/incremental-state"
[2022-06-30T00:32:29Z DEBUG collector::execute] applying new row to "/tmp/.tmpPyJOiv"
[2022-06-30T00:32:29Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-06-30T00:32:29Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-06-30T00:32:29Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpPyJOiv#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpPyJOiv/incremental-state"
[2022-06-30T00:32:36Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-30T00:32:36Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-06-30T00:32:36Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpLr9TGS#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-30T00:32:43Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-06-30T00:32:43Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-06-30T00:32:43Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpLr9TGS#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpLr9TGS/incremental-state"
[2022-06-30T00:32:50Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-06-30T00:32:50Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpLr9TGS#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpLr9TGS/incremental-state"
[2022-06-30T00:32:52Z DEBUG collector::execute] applying new row to "/tmp/.tmpLr9TGS"
[2022-06-30T00:32:52Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-06-30T00:32:52Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-06-30T00:32:52Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpLr9TGS#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpLr9TGS/incremental-state"
+ cd /checkout/obj
+ RUSTC_PROFILE_MERGED_FILE=/tmp/rustc-pgo.profdata
+ ./build/x86_64-unknown-linux-gnu/llvm/bin/llvm-profdata merge -o /tmp/rustc-pgo.profdata /tmp/rustc-pgo
+ echo 'Rustc PGO statistics'
---
[RUSTC-TIMING] unic_common test:false 0.093
   Compiling itoa v0.4.6
[RUSTC-TIMING] arrayvec test:false 0.313
   Compiling termcolor v1.1.2
warning: rustc_graphviz.e603138b-cgu.10: no profile data available for function _RNvXs2_NtCshJUuoHaRvLu_4core7convertNtNtNtB7_5alloc6layout11LayoutErrorINtB5_4FromBy_E4fromCsbB3UoS4RkCJ_14rustc_graphviz Hash = 742261418966908927
[RUSTC-TIMING] scoped_tls test:false 0.110
[RUSTC-TIMING] self_cell test:false 0.073
   Compiling rustc_fs_util v0.0.0 (/checkout/compiler/rustc_fs_util)
[RUSTC-TIMING] unic_char_range test:false 0.179
[RUSTC-TIMING] unic_char_range test:false 0.179
   Compiling either v1.6.0
[RUSTC-TIMING] unicode_xid test:false 0.111
   Compiling datafrog v2.0.1
warning: rustc_graphviz.e603138b-cgu.4: no profile data available for function _RNvMNtNtNtCshJUuoHaRvLu_4core4iter8adapters3revINtB2_3RevINtNtNtB8_3ops5range5RangejEE3newCsbB3UoS4RkCJ_14rustc_graphviz Hash = 742261418966908927

warning: rustc_fs_util.878729e4-cgu.1: no profile data available for function _RNvXs2_NtCshJUuoHaRvLu_4core7convertNtNtNtB7_5alloc6layout11LayoutErrorINtB5_4FromBy_E4fromCsiC8PZgYKRFg_13rustc_fs_util Hash = 742261418966908927
[RUSTC-TIMING] itoa test:false 0.152
   Compiling memchr v2.4.1
[RUSTC-TIMING] rustc_fs_util test:false 0.114
warning: `rustc_fs_util` (lib) generated 1 warning
---
[RUSTC-TIMING] rustc_parse_format test:false 1.267
   Compiling matchers v0.1.0
[RUSTC-TIMING] matchers test:false 0.146
[RUSTC-TIMING] hashbrown test:false 0.770
warning: rustc_llvm.9419fabb-cgu.6: no profile data available for function _RNvXs2_NtCshJUuoHaRvLu_4core7convertNtNtNtB7_5alloc6layout11LayoutErrorINtB5_4FromBy_E4fromCs9vL99zyPuhX_10rustc_llvm Hash = 742261418966908927
[RUSTC-TIMING] generic_array test:false 1.085
   Compiling crypto-common v0.1.2
[RUSTC-TIMING] rand_chacha test:false 1.022
   Compiling block-buffer v0.10.2
---
   Compiling object v0.29.0
[RUSTC-TIMING] sha1 test:false 0.417
   Compiling tempfile v3.2.0
[RUSTC-TIMING] rand test:false 1.696
warning: rustc_serialize.6e7efbdd-cgu.5: no profile data available for function _RINvNtCshJUuoHaRvLu_4core3ptr13drop_in_placeRjECs5Nf2XQXbW1t_15rustc_serialize Hash = 742261418966908927

warning: rustc_serialize.6e7efbdd-cgu.5: no profile data available for function _RINvNtCshJUuoHaRvLu_4core9panicking13assert_failedjjECs5Nf2XQXbW1t_15rustc_serialize Hash = 742261418966908927

warning: rustc_serialize.6e7efbdd-cgu.7: no profile data available for function _RNvXsV_NtCshJUuoHaRvLu_4core3fmtRjNtB5_5Debug3fmtCs5Nf2XQXbW1t_15rustc_serialize Hash = 1124680650125156080
[RUSTC-TIMING] rustc_serialize test:false 0.865
warning: `rustc_serialize` (lib) generated 3 warnings
[RUSTC-TIMING] aho_corasick test:false 3.537
[RUSTC-TIMING] sha2 test:false 1.625
---
   Compiling rustc_ast v0.0.0 (/checkout/compiler/rustc_ast)
   Compiling rustc_target v0.0.0 (/checkout/compiler/rustc_target)
   Compiling rustc_error_messages v0.0.0 (/checkout/compiler/rustc_error_messages)
   Compiling rustc_feature v0.0.0 (/checkout/compiler/rustc_feature)
warning: rustc_error_messages.3552a0b8-cgu.11: no profile data available for function _RINvMsx_NtCs2NDwSn78v9v_12tracing_core5fieldNtB6_8FieldSet9value_setATRNtB6_5FieldINtNtCshJUuoHaRvLu_4core6option6OptionRDNtB6_5ValueEL_EEj1_ECshxOtgCX3h0q_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.3552a0b8-cgu.11: no profile data available for function _RINvMsx_NtCs2NDwSn78v9v_12tracing_core5fieldNtB6_8FieldSet9value_setATRNtB6_5FieldINtNtCshJUuoHaRvLu_4core6option6OptionRDNtB6_5ValueEL_EEj2_ECshxOtgCX3h0q_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.3552a0b8-cgu.11: no profile data available for function _RINvMsx_NtCs2NDwSn78v9v_12tracing_core5fieldNtB6_8FieldSet9value_setATRNtB6_5FieldINtNtCshJUuoHaRvLu_4core6option6OptionRDNtB6_5ValueEL_EEj5_ECshxOtgCX3h0q_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.3552a0b8-cgu.11: no profile data available for function _RINvNtCs2NDwSn78v9v_12tracing_core5field5debugRINtNtCs2yifCDJXqxT_5alloc3vec3VecNtNtCsa7zhOS5cG9S_3std4path7PathBufEECshxOtgCX3h0q_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.3552a0b8-cgu.11: no profile data available for function _RINvNtCs2NDwSn78v9v_12tracing_core5field5debugRINtNtCshJUuoHaRvLu_4core6option6OptionNtCsaLZdf3U3Ut_16unic_langid_impl18LanguageIdentifierEECshxOtgCX3h0q_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.3552a0b8-cgu.11: no profile data available for function _RINvNtCs2NDwSn78v9v_12tracing_core5field5debugRINtNtCshJUuoHaRvLu_4core6option6OptionNtNtCsa7zhOS5cG9S_3std4path7PathBufEECshxOtgCX3h0q_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.3552a0b8-cgu.11: no profile data available for function _RINvNtCs2NDwSn78v9v_12tracing_core5field5debugRINtNtCshJUuoHaRvLu_4core6option6OptionRNtNtCsa7zhOS5cG9S_3std4path4PathEECshxOtgCX3h0q_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.3552a0b8-cgu.11: no profile data available for function _RINvNtCs2NDwSn78v9v_12tracing_core5field5debugRNtCsaLZdf3U3Ut_16unic_langid_impl18LanguageIdentifierECshxOtgCX3h0q_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.3552a0b8-cgu.11: no profile data available for function _RINvNtCs2NDwSn78v9v_12tracing_core5field5debugRNtNtCsa7zhOS5cG9S_3std4path7PathBufECshxOtgCX3h0q_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.3552a0b8-cgu.11: no profile data available for function _RINvNtCs2NDwSn78v9v_12tracing_core5field5debugRNtNtCsf12Ut7MUxPH_13fluent_bundle8resource14FluentResourceECshxOtgCX3h0q_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.3552a0b8-cgu.11: no profile data available for function _RINvNtCs2NDwSn78v9v_12tracing_core5field5debugRQNtNtCsa7zhOS5cG9S_3std4path7PathBufECshxOtgCX3h0q_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.3552a0b8-cgu.11: no profile data available for function _RINvNtCs2NDwSn78v9v_12tracing_core5field5debugRRSReECshxOtgCX3h0q_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.3552a0b8-cgu.11: no profile data available for function _RINvNtCshJUuoHaRvLu_4core3ptr13drop_in_placeRINtNtB4_6option6OptionNtCsaLZdf3U3Ut_16unic_langid_impl18LanguageIdentifierEECshxOtgCX3h0q_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.3552a0b8-cgu.11: no profile data available for function _RINvNtCshJUuoHaRvLu_4core3ptr13drop_in_placeRINtNtB4_6option6OptionNtNtCsa7zhOS5cG9S_3std4path7PathBufEECshxOtgCX3h0q_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.3552a0b8-cgu.11: no profile data available for function _RINvNtCshJUuoHaRvLu_4core3ptr13drop_in_placeRINtNtB4_6option6OptionRNtNtCsa7zhOS5cG9S_3std4path4PathEECshxOtgCX3h0q_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.3552a0b8-cgu.11: no profile data available for function _RINvNtCshJUuoHaRvLu_4core3ptr13drop_in_placeRINtNtCs2yifCDJXqxT_5alloc3vec3VecNtNtCsa7zhOS5cG9S_3std4path7PathBufEECshxOtgCX3h0q_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.3552a0b8-cgu.11: no profile data available for function _RINvNtCshJUuoHaRvLu_4core3ptr13drop_in_placeRNtCsaLZdf3U3Ut_16unic_langid_impl18LanguageIdentifierECshxOtgCX3h0q_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.3552a0b8-cgu.11: no profile data available for function _RINvNtCshJUuoHaRvLu_4core3ptr13drop_in_placeRNtNtCsa7zhOS5cG9S_3std4path7PathBufECshxOtgCX3h0q_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.3552a0b8-cgu.11: no profile data available for function _RINvNtCshJUuoHaRvLu_4core3ptr13drop_in_placeRNtNtCsf12Ut7MUxPH_13fluent_bundle8resource14FluentResourceECshxOtgCX3h0q_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.3552a0b8-cgu.11: no profile data available for function _RINvNtCshJUuoHaRvLu_4core3ptr13drop_in_placeRQNtNtCsa7zhOS5cG9S_3std4path7PathBufECshxOtgCX3h0q_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.3552a0b8-cgu.11: no profile data available for function _RINvNtCshJUuoHaRvLu_4core3ptr13drop_in_placeRRSReECshxOtgCX3h0q_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.3552a0b8-cgu.11: no profile data available for function _RNvXsk_NtCs2NDwSn78v9v_12tracing_core5fieldINtB5_10DebugValueRINtNtCs2yifCDJXqxT_5alloc3vec3VecNtNtCsa7zhOS5cG9S_3std4path7PathBufEENtB5_5Value6recordCshxOtgCX3h0q_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.3552a0b8-cgu.11: no profile data available for function _RNvXsk_NtCs2NDwSn78v9v_12tracing_core5fieldINtB5_10DebugValueRINtNtCshJUuoHaRvLu_4core6option6OptionNtCsaLZdf3U3Ut_16unic_langid_impl18LanguageIdentifierEENtB5_5Value6recordCshxOtgCX3h0q_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.3552a0b8-cgu.11: no profile data available for function _RNvXsk_NtCs2NDwSn78v9v_12tracing_core5fieldINtB5_10DebugValueRINtNtCshJUuoHaRvLu_4core6option6OptionNtNtCsa7zhOS5cG9S_3std4path7PathBufEENtB5_5Value6recordCshxOtgCX3h0q_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.3552a0b8-cgu.11: no profile data available for function _RNvXsk_NtCs2NDwSn78v9v_12tracing_core5fieldINtB5_10DebugValueRINtNtCshJUuoHaRvLu_4core6option6OptionRNtNtCsa7zhOS5cG9S_3std4path4PathEENtB5_5Value6recordCshxOtgCX3h0q_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.3552a0b8-cgu.11: no profile data available for function _RNvXsk_NtCs2NDwSn78v9v_12tracing_core5fieldINtB5_10DebugValueRNtCsaLZdf3U3Ut_16unic_langid_impl18LanguageIdentifierENtB5_5Value6recordCshxOtgCX3h0q_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.3552a0b8-cgu.11: no profile data available for function _RNvXsk_NtCs2NDwSn78v9v_12tracing_core5fieldINtB5_10DebugValueRNtNtCsa7zhOS5cG9S_3std4path7PathBufENtB5_5Value6recordCshxOtgCX3h0q_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.3552a0b8-cgu.11: no profile data available for function _RNvXsk_NtCs2NDwSn78v9v_12tracing_core5fieldINtB5_10DebugValueRNtNtCsf12Ut7MUxPH_13fluent_bundle8resource14FluentResourceENtB5_5Value6recordCshxOtgCX3h0q_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.3552a0b8-cgu.11: no profile data available for function _RNvXsk_NtCs2NDwSn78v9v_12tracing_core5fieldINtB5_10DebugValueRQNtNtCsa7zhOS5cG9S_3std4path7PathBufENtB5_5Value6recordCshxOtgCX3h0q_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.3552a0b8-cgu.11: no profile data available for function _RNvXsk_NtCs2NDwSn78v9v_12tracing_core5fieldINtB5_10DebugValueRRSReENtB5_5Value6recordCshxOtgCX3h0q_20rustc_error_messages Hash = 170957022131388415
[RUSTC-TIMING] jemalloc_sys test:false 0.073
[RUSTC-TIMING] rustc_error_messages test:false 0.972
warning: `rustc_error_messages` (lib) generated 30 warnings
[RUSTC-TIMING] rustc_feature test:false 1.124
---
error[E0308]: mismatched types
    |
   ::: src/librustdoc/clean/types.rs:126:1
    |
126 |   rustc_data_structures::static_assert_size!(Crate, 72);

[RUSTC-TIMING] askama_shared test:false 6.245
error[E0308]: mismatched types
     |
