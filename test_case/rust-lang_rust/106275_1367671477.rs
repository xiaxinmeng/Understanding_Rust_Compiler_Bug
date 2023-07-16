plain
[2022-12-29T22:58:18Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-12-29T22:58:18Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-12-29T22:58:18Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKfcbtQ#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-12-29T22:58:20Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-12-29T22:58:20Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKfcbtQ#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpKfcbtQ/incremental-state"
[2022-12-29T22:58:24Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-12-29T22:58:24Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKfcbtQ#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpKfcbtQ/incremental-state"
[2022-12-29T22:58:24Z DEBUG collector::execute] applying println to "/tmp/.tmpKfcbtQ"
[2022-12-29T22:58:24Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-12-29T22:58:24Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-12-29T22:58:24Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKfcbtQ#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpKfcbtQ/incremental-state"
[2022-12-29T22:58:26Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-12-29T22:58:26Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-12-29T22:58:26Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpx9UkLt#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-12-29T22:58:28Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
---
[2022-12-29T23:00:52Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-12-29T23:00:53Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-12-29T23:00:53Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpRC1hJH#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2022-12-29T23:01:51Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-12-29T23:01:51Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpRC1hJH#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpRC1hJH/incremental-state"
[2022-12-29T23:03:00Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-12-29T23:03:00Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpRC1hJH#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpRC1hJH/incremental-state"
[2022-12-29T23:03:12Z DEBUG collector::execute] applying println to "/tmp/.tmpRC1hJH"
[2022-12-29T23:03:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-12-29T23:03:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-12-29T23:03:12Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpRC1hJH#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpRC1hJH/incremental-state"
[2022-12-29T23:03:31Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-12-29T23:03:32Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-12-29T23:03:32Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpH3mQBi#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2022-12-29T23:04:42Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-12-29T23:04:42Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-12-29T23:04:42Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpH3mQBi#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpH3mQBi/incremental-state"
[2022-12-29T23:05:52Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-12-29T23:05:52Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpH3mQBi#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpH3mQBi/incremental-state"
[2022-12-29T23:06:04Z DEBUG collector::execute] applying println to "/tmp/.tmpH3mQBi"
[2022-12-29T23:06:04Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-12-29T23:06:04Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-12-29T23:06:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpH3mQBi#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpH3mQBi/incremental-state"
Executing benchmark ctfe-stress-5 (3/8)
Preparing ctfe-stress-5
[2022-12-29T23:06:28Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-12-29T23:06:28Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
---
[2022-12-29T23:10:43Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-12-29T23:10:43Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-12-29T23:10:43Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDSKQDV#externs@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-12-29T23:10:43Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-12-29T23:10:43Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDSKQDV#externs@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpDSKQDV/incremental-state"
[2022-12-29T23:10:45Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-12-29T23:10:45Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDSKQDV#externs@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpDSKQDV/incremental-state"
[2022-12-29T23:10:45Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-12-29T23:10:45Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-12-29T23:10:45Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp7MJGKS#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-12-29T23:10:46Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
[2022-12-29T23:10:58Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-12-29T23:10:58Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-12-29T23:10:58Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzvAPq5#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-12-29T23:11:01Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-12-29T23:11:02Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzvAPq5#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpzvAPq5/incremental-state"
[2022-12-29T23:11:06Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-12-29T23:11:06Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzvAPq5#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpzvAPq5/incremental-state"
[2022-12-29T23:11:07Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-12-29T23:11:07Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-12-29T23:11:07Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpZ6dTNv#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-12-29T23:11:11Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-12-29T23:11:11Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-12-29T23:11:11Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpZ6dTNv#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpZ6dTNv/incremental-state"
[2022-12-29T23:11:16Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-12-29T23:11:16Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpZ6dTNv#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpZ6dTNv/incremental-state"
Executing benchmark token-stream-stress (7/8)
Preparing token-stream-stress
[2022-12-29T23:11:17Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-12-29T23:11:17Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
---
[2022-12-29T23:11:18Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-12-29T23:11:18Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-12-29T23:11:18Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpHCaPFt#token-stream-stress@0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2022-12-29T23:11:18Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-12-29T23:11:18Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpHCaPFt#token-stream-stress@0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpHCaPFt/incremental-state"
[2022-12-29T23:11:18Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-12-29T23:11:18Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpHCaPFt#token-stream-stress@0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpHCaPFt/incremental-state"
[2022-12-29T23:11:18Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-12-29T23:11:18Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-12-29T23:11:18Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpF5Es9Z#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2022-12-29T23:11:19Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
---
[2022-12-29T23:11:21Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-12-29T23:11:21Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-12-29T23:11:21Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpRAP8Qg#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-12-29T23:11:27Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-12-29T23:11:27Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpRAP8Qg#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpRAP8Qg/incremental-state"
[2022-12-29T23:11:34Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-12-29T23:11:34Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpRAP8Qg#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpRAP8Qg/incremental-state"
[2022-12-29T23:11:36Z DEBUG collector::execute] applying new row to "/tmp/.tmpRAP8Qg"
[2022-12-29T23:11:36Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-12-29T23:11:36Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-12-29T23:11:36Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpRAP8Qg#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpRAP8Qg/incremental-state"
[2022-12-29T23:11:42Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-12-29T23:11:42Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-12-29T23:11:43Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpdfGoUl#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-12-29T23:11:48Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-12-29T23:11:48Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-12-29T23:11:48Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpdfGoUl#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpdfGoUl/incremental-state"
[2022-12-29T23:11:56Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-12-29T23:11:56Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpdfGoUl#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpdfGoUl/incremental-state"
[2022-12-29T23:11:58Z DEBUG collector::execute] applying new row to "/tmp/.tmpdfGoUl"
[2022-12-29T23:11:58Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-12-29T23:11:58Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-12-29T23:11:58Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpdfGoUl#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpdfGoUl/incremental-state"
[2022-12-29T23:12:05Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-12-29T23:12:05Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-12-29T23:12:05Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpGIRvxA#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-12-29T23:12:11Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-12-29T23:12:11Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-12-29T23:12:11Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpGIRvxA#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpGIRvxA/incremental-state"
[2022-12-29T23:12:18Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-12-29T23:12:18Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpGIRvxA#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpGIRvxA/incremental-state"
[2022-12-29T23:12:20Z DEBUG collector::execute] applying new row to "/tmp/.tmpGIRvxA"
[2022-12-29T23:12:20Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-12-29T23:12:20Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-12-29T23:12:20Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpGIRvxA#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpGIRvxA/incremental-state"
+ cd /checkout/obj
+ RUSTC_PROFILE_MERGED_FILE=/tmp/tmp-pgo/rustc-pgo.profdata
+ /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-profdata merge -o /tmp/tmp-pgo/rustc-pgo.profdata /tmp/tmp-pgo/rustc-pgo
+ echo 'Rustc PGO statistics'
---
[RUSTC-TIMING] rustc_mir_transform test:false 88.082
[RUSTC-TIMING] rustc_borrowck test:false 93.108
[RUSTC-TIMING] rustc_query_impl test:false 162.492
   Compiling rustc_driver v0.0.0 (/checkout/compiler/rustc_driver)
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-351ee7202cade707.so(+0x13887e3)[0x7fa9a8ed27e3]
/lib64/libpthread.so.0(+0xf630)[0x7fa9a7531630]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libLLVM-15-rust-1.68.0-nightly.so(+0x7b5e345)[0x7fa9a1e46345]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libLLVM-15-rust-1.68.0-nightly.so(+0x7b5d760)[0x7fa9a1e45760]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libLLVM-15-rust-1.68.0-nightly.so(_ZN4llvm26LoopVectorizationCostModel22getInterleaveGroupCostEPNS_11InstructionENS_12ElementCountE+0x4dc)[0x7fa99fc1431c]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libLLVM-15-rust-1.68.0-nightly.so(_ZN4llvm26LoopVectorizationCostModel28setCostBasedWideningDecisionENS_12ElementCountE+0x4db)[0x7fa99fc0cedb]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libLLVM-15-rust-1.68.0-nightly.so(+0x5915d51)[0x7fa99fbfdd51]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libLLVM-15-rust-1.68.0-nightly.so(_ZN4llvm24LoopVectorizationPlanner4planENS_12ElementCountEj+0x23a)[0x7fa99fbfd600]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libLLVM-15-rust-1.68.0-nightly.so(_ZN4llvm17LoopVectorizePass11processLoopEPNS_4LoopE+0x1284)[0x7fa99fbdaac4]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libLLVM-15-rust-1.68.0-nightly.so(_ZN4llvm17LoopVectorizePass7runImplERNS_8FunctionERNS_15ScalarEvolutionERNS_8LoopInfoERNS_19TargetTransformInfoERNS_13DominatorTreeERNS_18BlockFrequencyInfoEPNS_17TargetLibraryInfoERNS_12DemandedBitsERNS_9AAResultsERNS_15AssumptionCacheERSt8functionIFRKNS_14LoopAccessInfoERNS_4LoopEEERNS_25OptimizationRemarkEmitterEPNS_18ProfileSummaryInfoE+0x20f)[0x7fa99fbd8f93]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libLLVM-15-rust-1.68.0-nightly.so(_ZN4llvm17LoopVectorizePass3runERNS_8FunctionERNS_15AnalysisManagerIS1_JEEE+0x3a7)[0x7fa99fbd88a7]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libLLVM-15-rust-1.68.0-nightly.so(+0x58f04fb)[0x7fa99fbd84fb]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libLLVM-15-rust-1.68.0-nightly.so(_ZN4llvm11PassManagerINS_8FunctionENS_15AnalysisManagerIS1_JEEEJEE3runERS1_RS3_+0x729)[0x7fa99ff9aaf1]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libLLVM-15-rust-1.68.0-nightly.so(_ZN4llvm27ModuleToFunctionPassAdaptor3runERNS_6ModuleERNS_15AnalysisManagerIS1_JEEE+0xc32)[0x7fa9a14256f2]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libLLVM-15-rust-1.68.0-nightly.so(+0x713caab)[0x7fa9a1424aab]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libLLVM-15-rust-1.68.0-nightly.so(_ZN4llvm11PassManagerINS_6ModuleENS_15AnalysisManagerIS1_JEEEJEE3runERS1_RS3_+0x1b1)[0x7fa9a19555c7]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-351ee7202cade707.so(+0x1683b21)[0x7fa9a91cdb21]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-351ee7202cade707.so(+0x155e0d3)[0x7fa9a90a80d3]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-351ee7202cade707.so(+0x1520d2a)[0x7fa9a906ad2a]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-351ee7202cade707.so(+0x161efad)[0x7fa9a9168fad]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-351ee7202cade707.so(+0x157d2bf)[0x7fa9a90c72bf]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-351ee7202cade707.so(+0x1619426)[0x7fa9a9163426]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-351ee7202cade707.so(+0x15d8f99)[0x7fa9a9122f99]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libstd-3ff12aaffc3ee0b1.so(rust_metadata_std_63a5edb545e2cdaa+0x114b23)[0x7fa9ac04bb23]
/lib64/libpthread.so.0(+0x7ea5)[0x7fa9a7529ea5]
/lib64/libc.so.6(clone+0x6d)[0x7fa9a7252b0d]
[RUSTC-TIMING] rustc_driver test:false 113.852
rustc exited with signal: 11 (SIGSEGV) (core dumped)

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_driver --edition=2021 compiler/rustc_driver/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 --cfg 'feature="llvm"' --cfg 'feature="max_level_info"' -Zunstable-options --check-cfg 'values(feature, "llvm", "max_level_info", "rustc_use_parallel_compiler")' --check-cfg 'names()' --check-cfg 'values()' -C metadata=ce965b63303a743e -C extra-filename=-ce965b63303a743e --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -C linker=clang -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblibc-4636db88ffbdb5d0.rlib --extern rustc_ast=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_ast-2146c7b2e46f919d.rlib --extern rustc_ast_pretty=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_ast_pretty-8167c6414135bf66.rlib --extern rustc_codegen_ssa=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_codegen_ssa-57a15cca4285e051.rlib --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-a1b5e9ca82f30953.rlib --extern rustc_error_codes=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_error_codes-626a768a67da3436.rlib --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-1e95a6587de14c34.rlib --extern rustc_feature=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_feature-f3b10b67d9eba753.rlib --extern rustc_hir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_hir-2c79463f23cc5151.rlib --extern rustc_hir_analysis=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_hir_analysis-cf80757deca2698b.rlib --extern rustc_hir_pretty=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_hir_pretty-3c3751cc004706a2.rlib --extern rustc_interface=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_interface-01e13bdb88daf2cc.rlib --extern rustc_lint=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_lint-b334e57aaafe6740.rlib --extern rustc_log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_log-57a255fbd7e347bf.rlib --extern rustc_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/librustc_macros-23e117c1973272ed.so --extern rustc_metadata=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_metadata-e992139259504242.rlib --extern rustc_middle=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_middle-aad22d25cf2423e6.rlib --extern rustc_parse=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_parse-9e2a5a05c979498b.rlib --extern rustc_plugin_impl=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_plugin_impl-e11f3aa45cd0562b.rlib --extern rustc_save_analysis=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_save_analysis-ed396cab65060b6d.rlib --extern rustc_session=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_session-8e2a690479f4d052.rlib --extern rustc_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_span-60d0ef299a9ec18c.rlib --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-a88b0926e932a3fe.rlib --extern serde_json=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libserde_json-bcece10381e5dde9.rlib --extern tracing=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libtracing-6e726c84fde876f7.rlib -Csymbol-mangling-version=v0 -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(parallel_compiler)' '--check-cfg=values(no_btreemap_remove_entry)' '--check-cfg=values(crossbeam_loom)' '--check-cfg=values(span_locations)' '--check-cfg=values(rustix_use_libc)' -Zmacro-backtrace -Clink-args=-Wl,-z,origin '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Clink-args=-fuse-ld=lld -Csplit-debuginfo=off -Zunstable-options '-Wrustc::internal' -Cprefer-dynamic -Clink-args=-Wl,--icf=all -Zdylib-lto -Clto=thin -Cembed-bitcode=yes -Z binary-dep-depinfo -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/psm-81554199d5567f44/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/rustc_llvm-039a8de5fd642836/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lib -L native=/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/7.5.0/../../../../lib64` (exit status: 254)
