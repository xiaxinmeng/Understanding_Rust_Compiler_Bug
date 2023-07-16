plain
Executing benchmark hyper-0.14.18 (3/7)
Preparing hyper-0.14.18
[2022-05-26T17:41:50Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-26T17:41:50Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-26T17:41:50Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpvCG91c#hyper:0.14.18" "--release" "--features=client,http1,http2,server,stream" "--" "--skip-this-rustc"
[2022-05-26T17:41:50Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjTIssY#hyper:0.14.18" "--features=client,http1,http2,server,stream" "--" "--skip-this-rustc"
[2022-05-26T17:42:19Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T17:42:20Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-26T17:42:20Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmprzsON3#hyper:0.14.18" "--features=client,http1,http2,server,stream" "--" "--wrap-rustc-with" "Eprintln"
Running hyper-0.14.18: Opt + [Full]
---
[2022-05-26T17:42:36Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpD7k1KY#regex:1.5.5" "--" "--skip-this-rustc"
Running regex-1.5.5: Debug + [Full]
[2022-05-26T17:42:46Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T17:42:46Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-26T17:42:46Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCZsHlL#regex:1.5.5" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T17:42:49Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T17:42:49Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-26T17:42:49Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp0uVDFR#regex:1.5.5" "--release" "--" "--wrap-rustc-with" "Eprintln"
Finished benchmark regex-1.5.5 (4/7)
---
Executing benchmark serde-1.0.136 (6/7)
Preparing serde-1.0.136
[2022-05-26T17:43:58Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-26T17:43:58Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-26T17:43:58Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpcmYcIC#serde:1.0.136" "--" "--skip-this-rustc"
[2022-05-26T17:43:58Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpg3HHZP#serde:1.0.136" "--release" "--" "--skip-this-rustc"
[2022-05-26T17:43:59Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T17:43:59Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-26T17:43:59Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-26T17:43:59Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpghAgtA#serde:1.0.136" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T17:44:02Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T17:44:02Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-26T17:44:02Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpNHQn2I#serde:1.0.136" "--release" "--" "--wrap-rustc-with" "Eprintln"
Finished benchmark serde-1.0.136 (6/7)
Finished benchmark serde-1.0.136 (6/7)
Executing benchmark syn-1.0.89 (7/7)
Preparing syn-1.0.89
[2022-05-26T17:44:06Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-26T17:44:06Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-26T17:44:06Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJAuNTj#syn:1.0.89" "--release" "--" "--skip-this-rustc"
[2022-05-26T17:44:06Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpNV4KuW#syn:1.0.89" "--" "--skip-this-rustc"
[2022-05-26T17:44:10Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T17:44:10Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-26T17:44:10Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKwgerY#syn:1.0.89" "--" "--wrap-rustc-with" "Eprintln"
Running syn-1.0.89: Opt + [Full]
---
Preparing bitmaps-3.1.0
[2022-05-26T17:55:58Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-05-26T17:55:58Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-26T17:55:58Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-26T17:55:58Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpq7BXX2#bitmaps:3.1.0" "--release" "--" "--skip-this-rustc"
[2022-05-26T17:55:58Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpiCsfF0#bitmaps:3.1.0" "--profile" "check" "--" "--skip-this-rustc"
[2022-05-26T17:55:58Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKs1a5H#bitmaps:3.1.0" "--" "--skip-this-rustc"
[2022-05-26T17:55:58Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T17:55:58Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-05-26T17:55:58Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-05-26T17:55:58Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDzolVW#bitmaps:3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T17:56:00Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-05-26T17:56:00Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDzolVW#bitmaps:3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpDzolVW/incremental-state"
[2022-05-26T17:56:03Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-05-26T17:56:03Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDzolVW#bitmaps:3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpDzolVW/incremental-state"
[2022-05-26T17:56:04Z DEBUG collector::execute] applying println to "/tmp/.tmpDzolVW"
[2022-05-26T17:56:04Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-26T17:56:04Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-26T17:56:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDzolVW#bitmaps:3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpDzolVW/incremental-state"
[2022-05-26T17:56:04Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T17:56:04Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-26T17:56:04Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-26T17:56:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpLULVwM#bitmaps:3.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T17:56:06Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-26T17:56:06Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpLULVwM#bitmaps:3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpLULVwM/incremental-state"
[2022-05-26T17:56:10Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-05-26T17:56:10Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpLULVwM#bitmaps:3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpLULVwM/incremental-state"
[2022-05-26T17:56:10Z DEBUG collector::execute] applying println to "/tmp/.tmpLULVwM"
[2022-05-26T17:56:10Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-26T17:56:10Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-26T17:56:10Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpLULVwM#bitmaps:3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpLULVwM/incremental-state"
[2022-05-26T17:56:11Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T17:56:11Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-26T17:56:11Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpaal5wB#bitmaps:3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T17:56:14Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-26T17:56:14Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-26T17:56:14Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpaal5wB#bitmaps:3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpaal5wB/incremental-state"
[2022-05-26T17:56:17Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-05-26T17:56:17Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpaal5wB#bitmaps:3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpaal5wB/incremental-state"
[2022-05-26T17:56:18Z DEBUG collector::execute] applying println to "/tmp/.tmpaal5wB"
[2022-05-26T17:56:18Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-26T17:56:18Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-26T17:56:18Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpaal5wB#bitmaps:3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpaal5wB/incremental-state"
Executing benchmark cargo-0.60.0 (2/8)
Preparing cargo-0.60.0
[2022-05-26T17:56:19Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-26T17:56:19Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
---
[2022-05-26T17:57:13Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T17:57:14Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-05-26T17:57:14Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpHvOyxV#cargo:0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T17:57:35Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-05-26T17:57:35Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpHvOyxV#cargo:0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpHvOyxV/incremental-state"
[2022-05-26T17:57:58Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-05-26T17:57:58Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpHvOyxV#cargo:0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpHvOyxV/incremental-state"
[2022-05-26T17:58:03Z DEBUG collector::execute] applying println to "/tmp/.tmpHvOyxV"
[2022-05-26T17:58:03Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-26T17:58:03Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-26T17:58:03Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpHvOyxV#cargo:0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpHvOyxV/incremental-state"
[2022-05-26T17:58:13Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T17:58:14Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-26T17:58:14Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpMfAHyG#cargo:0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T17:59:02Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-26T17:59:02Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-26T17:59:02Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpMfAHyG#cargo:0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpMfAHyG/incremental-state"
[2022-05-26T18:00:02Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-05-26T18:00:02Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpMfAHyG#cargo:0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpMfAHyG/incremental-state"
[2022-05-26T18:00:13Z DEBUG collector::execute] applying println to "/tmp/.tmpMfAHyG"
[2022-05-26T18:00:13Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-26T18:00:13Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-26T18:00:13Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpMfAHyG#cargo:0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpMfAHyG/incremental-state"
[2022-05-26T18:00:36Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T18:00:36Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-26T18:00:36Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpax62h2#cargo:0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T18:01:38Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
[2022-05-26T18:03:25Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T18:03:25Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-05-26T18:03:25Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpvfPxWZ#ctfe-stress-5:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T18:03:34Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-05-26T18:03:34Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpvfPxWZ#ctfe-stress-5:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpvfPxWZ/incremental-state"
[2022-05-26T18:03:44Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-05-26T18:03:44Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpvfPxWZ#ctfe-stress-5:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpvfPxWZ/incremental-state"
[2022-05-26T18:03:45Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T18:03:45Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-26T18:03:45Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpTXm62n#ctfe-stress-5:0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T18:03:54Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
---
Preparing diesel-1.4.8
[2022-05-26T18:04:24Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-26T18:04:24Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-05-26T18:04:24Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-26T18:04:24Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp87Izjm#diesel:1.4.8" "--release" "--" "--skip-this-rustc"
[2022-05-26T18:04:24Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmptowx8N#diesel:1.4.8" "--" "--skip-this-rustc"
[2022-05-26T18:04:24Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpfNcRAR#diesel:1.4.8" "--profile" "check" "--" "--skip-this-rustc"
[2022-05-26T18:04:43Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T18:04:43Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-05-26T18:04:43Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpoXQvp5#diesel:1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T18:05:00Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-05-26T18:05:00Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-05-26T18:05:00Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpoXQvp5#diesel:1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpoXQvp5/incremental-state"
[2022-05-26T18:05:18Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-05-26T18:05:18Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpoXQvp5#diesel:1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpoXQvp5/incremental-state"
[2022-05-26T18:05:22Z DEBUG collector::execute] applying println to "/tmp/.tmpoXQvp5"
[2022-05-26T18:05:22Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-26T18:05:22Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-26T18:05:22Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpoXQvp5#diesel:1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpoXQvp5/incremental-state"
[2022-05-26T18:05:25Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T18:05:26Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-26T18:05:26Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpb7nFJk#diesel:1.4.8" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T18:05:45Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-26T18:05:45Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-26T18:05:45Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpb7nFJk#diesel:1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpb7nFJk/incremental-state"
[2022-05-26T18:06:08Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-05-26T18:06:08Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpb7nFJk#diesel:1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpb7nFJk/incremental-state"
[2022-05-26T18:06:12Z DEBUG collector::execute] applying println to "/tmp/.tmpb7nFJk"
[2022-05-26T18:06:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-26T18:06:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-26T18:06:12Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpb7nFJk#diesel:1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpb7nFJk/incremental-state"
[2022-05-26T18:06:16Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T18:06:16Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-26T18:06:16Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpmGp9Nh#diesel:1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T18:06:35Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-26T18:06:35Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-26T18:06:35Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpmGp9Nh#diesel:1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpmGp9Nh/incremental-state"
[2022-05-26T18:06:58Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-05-26T18:06:58Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpmGp9Nh#diesel:1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpmGp9Nh/incremental-state"
[2022-05-26T18:07:02Z DEBUG collector::execute] applying println to "/tmp/.tmpmGp9Nh"
[2022-05-26T18:07:02Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-26T18:07:02Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-26T18:07:02Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpmGp9Nh#diesel:1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpmGp9Nh/incremental-state"
Executing benchmark externs (5/8)
Preparing externs
[2022-05-26T18:07:06Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-05-26T18:07:06Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2022-05-26T18:07:09Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T18:07:09Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-26T18:07:09Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpA66HSG#externs:0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T18:07:10Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-26T18:07:10Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpA66HSG#externs:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpA66HSG/incremental-state"
[2022-05-26T18:07:11Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-05-26T18:07:11Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpA66HSG#externs:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpA66HSG/incremental-state"
[2022-05-26T18:07:12Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T18:07:12Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-26T18:07:12Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpn9qIjI#externs:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T18:07:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-26T18:07:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-26T18:07:12Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpn9qIjI#externs:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpn9qIjI/incremental-state"
[2022-05-26T18:07:13Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-05-26T18:07:13Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpn9qIjI#externs:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpn9qIjI/incremental-state"
Executing benchmark match-stress (6/8)
Preparing match-stress
[2022-05-26T18:07:14Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-05-26T18:07:14Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2022-05-26T18:07:30Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T18:07:30Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-26T18:07:30Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpew6UXx#match-stress:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T18:07:33Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-26T18:07:33Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpew6UXx#match-stress:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpew6UXx/incremental-state"
[2022-05-26T18:07:36Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-05-26T18:07:36Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpew6UXx#match-stress:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpew6UXx/incremental-state"
Executing benchmark token-stream-stress (7/8)
Preparing token-stream-stress
[2022-05-26T18:07:37Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-05-26T18:07:37Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
---
[2022-05-26T18:07:39Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T18:07:39Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-26T18:07:39Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpulHp2g#token-stream-stress:0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T18:07:39Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-26T18:07:39Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpulHp2g#token-stream-stress:0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpulHp2g/incremental-state"
[2022-05-26T18:07:40Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-05-26T18:07:40Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpulHp2g#token-stream-stress:0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpulHp2g/incremental-state"
[2022-05-26T18:07:40Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T18:07:40Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-26T18:07:40Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp7XHTP4#token-stream-stress:0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T18:07:40Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
Preparing tuple-stress
[2022-05-26T18:07:41Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-05-26T18:07:41Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-26T18:07:41Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-26T18:07:41Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQgA3Lt#tuple-stress:0.1.0" "--profile" "check" "--" "--skip-this-rustc"
[2022-05-26T18:07:41Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp9VqvOK#tuple-stress:0.1.0" "--release" "--" "--skip-this-rustc"
[2022-05-26T18:07:41Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpgBzNVK#tuple-stress:0.1.0" "--" "--skip-this-rustc"
[2022-05-26T18:07:42Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T18:07:42Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-05-26T18:07:42Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpgPv9lV#tuple-stress:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T18:07:47Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-05-26T18:07:47Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-05-26T18:07:47Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpgPv9lV#tuple-stress:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpgPv9lV/incremental-state"
[2022-05-26T18:07:53Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-05-26T18:07:53Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpgPv9lV#tuple-stress:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpgPv9lV/incremental-state"
[2022-05-26T18:07:55Z DEBUG collector::execute] applying new row to "/tmp/.tmpgPv9lV"
[2022-05-26T18:07:55Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-05-26T18:07:55Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-05-26T18:07:55Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpgPv9lV#tuple-stress:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpgPv9lV/incremental-state"
[2022-05-26T18:08:01Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T18:08:01Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-26T18:08:01Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpA1RnRR#tuple-stress:0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T18:08:07Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-26T18:08:07Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-26T18:08:07Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpA1RnRR#tuple-stress:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpA1RnRR/incremental-state"
[2022-05-26T18:08:14Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-05-26T18:08:14Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpA1RnRR#tuple-stress:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpA1RnRR/incremental-state"
[2022-05-26T18:08:16Z DEBUG collector::execute] applying new row to "/tmp/.tmpA1RnRR"
[2022-05-26T18:08:16Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-05-26T18:08:16Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-05-26T18:08:16Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpA1RnRR#tuple-stress:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpA1RnRR/incremental-state"
[2022-05-26T18:08:22Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T18:08:22Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-26T18:08:22Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpETqvlO#tuple-stress:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T18:08:28Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-26T18:08:28Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-26T18:08:28Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpETqvlO#tuple-stress:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpETqvlO/incremental-state"
[2022-05-26T18:08:35Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-05-26T18:08:35Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpETqvlO#tuple-stress:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpETqvlO/incremental-state"
[2022-05-26T18:08:37Z DEBUG collector::execute] applying new row to "/tmp/.tmpETqvlO"
[2022-05-26T18:08:37Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-05-26T18:08:37Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-05-26T18:08:37Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpETqvlO#tuple-stress:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpETqvlO/incremental-state"
+ cd /checkout/obj
+ ./build/x86_64-unknown-linux-gnu/llvm/bin/llvm-profdata merge -o /tmp/rustc-pgo.profdata /tmp/rustc-pgo
+ rm -r ./build/x86_64-unknown-linux-gnu/llvm ./build/x86_64-unknown-linux-gnu/lld
+ python3 ../x.py dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu --include-default-paths build-manifest --rust-profile-use=/tmp/rustc-pgo.profdata --llvm-profile-use=/tmp/llvm-pgo.profdata
---
[RUSTC-TIMING] unic_char_range test:false 0.156
   Compiling regex-syntax v0.6.25
[RUSTC-TIMING] unicode_xid test:false 0.090
   Compiling crc32fast v1.2.0
warning: rustc_fs_util.fd2263fa-cgu.3: no profile data available for function _RNvXs2_NtCsdrNQTddxpmJ_4core7convertNtNtNtB7_5alloc6layout11LayoutErrorINtB5_4FromBy_E4fromCsdxHP2m9LDRK_13rustc_fs_util Hash = 742261418966908927
[RUSTC-TIMING] ppv_lite86 test:false 0.462
   Compiling snap v1.0.1
   Compiling snap v1.0.1
warning: rustc_graphviz.9f06ca7a-cgu.13: no profile data available for function _RNvXs2_NtCsdrNQTddxpmJ_4core7convertNtNtNtB7_5alloc6layout11LayoutErrorINtB5_4FromBy_E4fromCs1QZkyyZTmnS_14rustc_graphviz Hash = 742261418966908927
[RUSTC-TIMING] rustc_fs_util test:false 0.121
warning: `rustc_fs_util` (lib) generated 1 warning
   Compiling ansi_term v0.12.1
   Compiling ansi_term v0.12.1
warning: rustc_graphviz.9f06ca7a-cgu.6: no profile data available for function _RNvMNtNtNtCsdrNQTddxpmJ_4core4iter8adapters3revINtB2_3RevINtNtNtB8_3ops5range5RangejEE3newCs1QZkyyZTmnS_14rustc_graphviz Hash = 742261418966908927
[RUSTC-TIMING] either test:false 0.136
   Compiling adler v0.2.3
[RUSTC-TIMING] tinystr test:false 0.382
   Compiling fixedbitset v0.2.0
---
[RUSTC-TIMING] cstr test:false 0.630
   Compiling digest v0.10.2
[RUSTC-TIMING] matchers test:false 0.123
[RUSTC-TIMING] hashbrown test:false 0.602
warning: rustc_llvm.32c30684-cgu.5: no profile data available for function _RNvXs2_NtCsdrNQTddxpmJ_4core7convertNtNtNtB7_5alloc6layout11LayoutErrorINtB5_4FromBy_E4fromCs6dDi3oLtxOq_10rustc_llvm Hash = 742261418966908927
[RUSTC-TIMING] digest test:false 0.197
   Compiling sha2 v0.10.1
[RUSTC-TIMING] rustc_llvm test:false 0.267
warning: `rustc_llvm` (lib) generated 1 warning
---
   Compiling rustc_target v0.0.0 (/checkout/compiler/rustc_target)
   Compiling rustc_error_messages v0.0.0 (/checkout/compiler/rustc_error_messages)
   Compiling rustc_feature v0.0.0 (/checkout/compiler/rustc_feature)
[RUSTC-TIMING] tikv_jemalloc_sys test:false 0.059
warning: rustc_error_messages.7e787bd5-cgu.11: no profile data available for function _RINvMsx_NtCs26F55phn3Xk_12tracing_core5fieldNtB6_8FieldSet9value_setATRNtB6_5FieldINtNtCsdrNQTddxpmJ_4core6option6OptionRDNtB6_5ValueEL_EEj1_ECshrAstYCSVZ_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.7e787bd5-cgu.11: no profile data available for function _RINvMsx_NtCs26F55phn3Xk_12tracing_core5fieldNtB6_8FieldSet9value_setATRNtB6_5FieldINtNtCsdrNQTddxpmJ_4core6option6OptionRDNtB6_5ValueEL_EEj2_ECshrAstYCSVZ_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.7e787bd5-cgu.11: no profile data available for function _RINvMsx_NtCs26F55phn3Xk_12tracing_core5fieldNtB6_8FieldSet9value_setATRNtB6_5FieldINtNtCsdrNQTddxpmJ_4core6option6OptionRDNtB6_5ValueEL_EEj5_ECshrAstYCSVZ_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.7e787bd5-cgu.11: no profile data available for function _RINvNtCs26F55phn3Xk_12tracing_core5field5debugRINtNtCsc6QY0izxJj7_5alloc3vec3VecNtNtCshHeXuD1NJO9_3std4path7PathBufEECshrAstYCSVZ_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.7e787bd5-cgu.11: no profile data available for function _RINvNtCs26F55phn3Xk_12tracing_core5field5debugRINtNtCsdrNQTddxpmJ_4core6option6OptionNtCsjWAHwbrQ53G_16unic_langid_impl18LanguageIdentifierEECshrAstYCSVZ_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.7e787bd5-cgu.11: no profile data available for function _RINvNtCs26F55phn3Xk_12tracing_core5field5debugRINtNtCsdrNQTddxpmJ_4core6option6OptionNtNtCshHeXuD1NJO9_3std4path7PathBufEECshrAstYCSVZ_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.7e787bd5-cgu.11: no profile data available for function _RINvNtCs26F55phn3Xk_12tracing_core5field5debugRINtNtCsdrNQTddxpmJ_4core6option6OptionRNtNtCshHeXuD1NJO9_3std4path4PathEECshrAstYCSVZ_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.7e787bd5-cgu.11: no profile data available for function _RINvNtCs26F55phn3Xk_12tracing_core5field5debugRNtCsjWAHwbrQ53G_16unic_langid_impl18LanguageIdentifierECshrAstYCSVZ_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.7e787bd5-cgu.11: no profile data available for function _RINvNtCs26F55phn3Xk_12tracing_core5field5debugRNtNtCs5vCmAwQuMCm_13fluent_bundle8resource14FluentResourceECshrAstYCSVZ_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.7e787bd5-cgu.11: no profile data available for function _RINvNtCs26F55phn3Xk_12tracing_core5field5debugRNtNtCshHeXuD1NJO9_3std4path7PathBufECshrAstYCSVZ_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.7e787bd5-cgu.11: no profile data available for function _RINvNtCs26F55phn3Xk_12tracing_core5field5debugRQNtNtCshHeXuD1NJO9_3std4path7PathBufECshrAstYCSVZ_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.7e787bd5-cgu.11: no profile data available for function _RINvNtCs26F55phn3Xk_12tracing_core5field5debugRRSReECshrAstYCSVZ_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.7e787bd5-cgu.11: no profile data available for function _RINvNtCsdrNQTddxpmJ_4core3ptr13drop_in_placeRINtNtB4_6option6OptionNtCsjWAHwbrQ53G_16unic_langid_impl18LanguageIdentifierEECshrAstYCSVZ_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.7e787bd5-cgu.11: no profile data available for function _RINvNtCsdrNQTddxpmJ_4core3ptr13drop_in_placeRINtNtB4_6option6OptionNtNtCshHeXuD1NJO9_3std4path7PathBufEECshrAstYCSVZ_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.7e787bd5-cgu.11: no profile data available for function _RINvNtCsdrNQTddxpmJ_4core3ptr13drop_in_placeRINtNtB4_6option6OptionRNtNtCshHeXuD1NJO9_3std4path4PathEECshrAstYCSVZ_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.7e787bd5-cgu.11: no profile data available for function _RINvNtCsdrNQTddxpmJ_4core3ptr13drop_in_placeRINtNtCsc6QY0izxJj7_5alloc3vec3VecNtNtCshHeXuD1NJO9_3std4path7PathBufEECshrAstYCSVZ_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.7e787bd5-cgu.11: no profile data available for function _RINvNtCsdrNQTddxpmJ_4core3ptr13drop_in_placeRNtCsjWAHwbrQ53G_16unic_langid_impl18LanguageIdentifierECshrAstYCSVZ_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.7e787bd5-cgu.11: no profile data available for function _RINvNtCsdrNQTddxpmJ_4core3ptr13drop_in_placeRNtNtCs5vCmAwQuMCm_13fluent_bundle8resource14FluentResourceECshrAstYCSVZ_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.7e787bd5-cgu.11: no profile data available for function _RINvNtCsdrNQTddxpmJ_4core3ptr13drop_in_placeRNtNtCshHeXuD1NJO9_3std4path7PathBufECshrAstYCSVZ_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.7e787bd5-cgu.11: no profile data available for function _RINvNtCsdrNQTddxpmJ_4core3ptr13drop_in_placeRQNtNtCshHeXuD1NJO9_3std4path7PathBufECshrAstYCSVZ_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.7e787bd5-cgu.11: no profile data available for function _RINvNtCsdrNQTddxpmJ_4core3ptr13drop_in_placeRRSReECshrAstYCSVZ_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.7e787bd5-cgu.11: no profile data available for function _RNvXsk_NtCs26F55phn3Xk_12tracing_core5fieldINtB5_10DebugValueRINtNtCsc6QY0izxJj7_5alloc3vec3VecNtNtCshHeXuD1NJO9_3std4path7PathBufEENtB5_5Value6recordCshrAstYCSVZ_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.7e787bd5-cgu.11: no profile data available for function _RNvXsk_NtCs26F55phn3Xk_12tracing_core5fieldINtB5_10DebugValueRINtNtCsdrNQTddxpmJ_4core6option6OptionNtCsjWAHwbrQ53G_16unic_langid_impl18LanguageIdentifierEENtB5_5Value6recordCshrAstYCSVZ_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.7e787bd5-cgu.11: no profile data available for function _RNvXsk_NtCs26F55phn3Xk_12tracing_core5fieldINtB5_10DebugValueRINtNtCsdrNQTddxpmJ_4core6option6OptionNtNtCshHeXuD1NJO9_3std4path7PathBufEENtB5_5Value6recordCshrAstYCSVZ_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.7e787bd5-cgu.11: no profile data available for function _RNvXsk_NtCs26F55phn3Xk_12tracing_core5fieldINtB5_10DebugValueRINtNtCsdrNQTddxpmJ_4core6option6OptionRNtNtCshHeXuD1NJO9_3std4path4PathEENtB5_5Value6recordCshrAstYCSVZ_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.7e787bd5-cgu.11: no profile data available for function _RNvXsk_NtCs26F55phn3Xk_12tracing_core5fieldINtB5_10DebugValueRNtCsjWAHwbrQ53G_16unic_langid_impl18LanguageIdentifierENtB5_5Value6recordCshrAstYCSVZ_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.7e787bd5-cgu.11: no profile data available for function _RNvXsk_NtCs26F55phn3Xk_12tracing_core5fieldINtB5_10DebugValueRNtNtCs5vCmAwQuMCm_13fluent_bundle8resource14FluentResourceENtB5_5Value6recordCshrAstYCSVZ_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.7e787bd5-cgu.11: no profile data available for function _RNvXsk_NtCs26F55phn3Xk_12tracing_core5fieldINtB5_10DebugValueRNtNtCshHeXuD1NJO9_3std4path7PathBufENtB5_5Value6recordCshrAstYCSVZ_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.7e787bd5-cgu.11: no profile data available for function _RNvXsk_NtCs26F55phn3Xk_12tracing_core5fieldINtB5_10DebugValueRQNtNtCshHeXuD1NJO9_3std4path7PathBufENtB5_5Value6recordCshrAstYCSVZ_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.7e787bd5-cgu.11: no profile data available for function _RNvXsk_NtCs26F55phn3Xk_12tracing_core5fieldINtB5_10DebugValueRRSReENtB5_5Value6recordCshrAstYCSVZ_20rustc_error_messages Hash = 170957022131388415
[RUSTC-TIMING] chalk_engine test:false 0.980
[RUSTC-TIMING] rustc_error_messages test:false 0.782
warning: `rustc_error_messages` (lib) generated 30 warnings
   Compiling gsgdt v0.1.2
---
    Checking rustc_type_ir v0.0.0 (/checkout/compiler/rustc_type_ir)
 Documenting rustc_type_ir v0.0.0 (/checkout/compiler/rustc_type_ir)
 Documenting rustc_span v0.0.0 (/checkout/compiler/rustc_span)
[RUSTC-TIMING] rustc_type_ir test:false 0.367
error: unresolved link to `ClosureSubsts`
   --> compiler/rustc_type_ir/src/sty.rs:103:10
    |
103 |     /// [ClosureSubsts] for more details.
    |          ^^^^^^^^^^^^^ no item named `ClosureSubsts` in scope
    |
    = note: `-D rustdoc::broken-intra-doc-links` implied by `-D warnings`
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

error: unresolved link to `GeneratorSubsts`
   --> compiler/rustc_type_ir/src/sty.rs:110:10
    |
110 |     /// [GeneratorSubsts].
    |          ^^^^^^^^^^^^^^^ no item named `GeneratorSubsts` in scope
    |
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

error: unresolved link to `GeneratorSubsts`
   --> compiler/rustc_type_ir/src/sty.rs:114:49
    |
114 |     /// This should only appear as part of the [GeneratorSubsts].
    |                                                 ^^^^^^^^^^^^^^^ no item named `GeneratorSubsts` in scope
    |
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: could not document `rustc_type_ir`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name rustc_type_ir compiler/rustc_type_ir/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/doc --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -C metadata=6d581c1b7631f9e2 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-8db9ec982a978a4d.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-650bfd0594a5fe32.rmeta --extern rustc_index=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_index-351f3773c639cba7.rmeta --extern rustc_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps/librustc_macros-5d871d1088043d1f.so --extern rustc_serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_serialize-fdb4862e2b55fc87.rmeta --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-3876c03f3e7b9b20.rmeta --extern-html-root-url 'bitflags=https://docs.rs/bitflags/1.2.1/' --extern-html-root-url 'smallvec=https://docs.rs/smallvec/1.7.0/' -Zunstable-options -Csymbol-mangling-version=v0 -Zunstable-options '--check-cfg=names()' '--check-cfg=values()' '--check-cfg=values(bootstrap)' '--check-cfg=values(parallel_compiler)' '--check-cfg=values(no_btreemap_remove_entry)' '--check-cfg=values(crossbeam_loom)' '--check-cfg=values(span_locations)' -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.63.0-nightly
  (55657ee07
  2022-05-26)' --document-private-items '-Arustdoc::private-intra-doc-links' --enable-index-page -Zunstable-options -Znormalize-docs --show-type-layout --generate-link-to-definition --extern-html-root-url 'ena=https://docs.rs/ena/latest/'` (exit status: 1)
[RUSTC-TIMING] rustc_span test:false 1.306
[RUSTC-TIMING] chalk_solve test:false 2.113
[RUSTC-TIMING] serde test:false 2.446
Build completed unsuccessfully in 0:21:17
