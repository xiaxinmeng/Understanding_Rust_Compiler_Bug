plain
Executing benchmark regex-1.5.5 (4/7)
Preparing regex-1.5.5
[2022-07-06T16:10:55Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-07-06T16:10:55Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-07-06T16:10:55Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpTKpGfU#regex@1.5.5" "--" "--skip-this-rustc"
[2022-07-06T16:10:55Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp8edcM6#regex@1.5.5" "--release" "--" "--skip-this-rustc"
[2022-07-06T16:11:04Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-06T16:11:04Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-07-06T16:11:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp31l86p#regex@1.5.5" "--" "--wrap-rustc-with" "Eprintln"
Running regex-1.5.5: Opt + [Full]
---
[2022-07-06T16:11:45Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp3cLL2o#ripgrep@13.0.0" "--" "--wrap-rustc-with" "Eprintln"
Running ripgrep-13.0.0: Opt + [Full]
[2022-07-06T16:11:49Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-06T16:11:50Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-07-06T16:11:50Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpVlzYqe#ripgrep@13.0.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
Executing benchmark serde-1.0.136 (6/7)
Preparing serde-1.0.136
[2022-07-06T16:12:08Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-07-06T16:12:08Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
---
[2022-07-06T16:12:19Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp5dK76f#syn@1.0.89" "--" "--wrap-rustc-with" "Eprintln"
Running syn-1.0.89: Opt + [Full]
[2022-07-06T16:12:22Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-06T16:12:22Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-07-06T16:12:22Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpkiRpXp#syn@1.0.89" "--release" "--" "--wrap-rustc-with" "Eprintln"
+ cd /checkout/obj
+ LLVM_PROFILE_MERGED_FILE=/tmp/llvm-pgo.profdata
+ /rustroot/bin/llvm-profdata merge -o /tmp/llvm-pgo.profdata /tmp/llvm-pgo
+ echo 'LLVM PGO statistics'
---
[2022-07-06T16:24:05Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-06T16:24:05Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-07-06T16:24:05Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpihda4K#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-07-06T16:24:06Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-07-06T16:24:07Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpihda4K#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpihda4K/incremental-state"
[2022-07-06T16:24:10Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-07-06T16:24:10Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpihda4K#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpihda4K/incremental-state"
[2022-07-06T16:24:10Z DEBUG collector::execute] applying println to "/tmp/.tmpihda4K"
[2022-07-06T16:24:10Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-07-06T16:24:10Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-07-06T16:24:10Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpihda4K#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpihda4K/incremental-state"
[2022-07-06T16:24:11Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-06T16:24:11Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-07-06T16:24:11Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp6as4Yr#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-07-06T16:24:13Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
[2022-07-06T16:25:07Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-06T16:25:08Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-07-06T16:25:08Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpvstILe#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2022-07-06T16:25:25Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-07-06T16:25:25Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpvstILe#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpvstILe/incremental-state"
[2022-07-06T16:25:47Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-07-06T16:25:47Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpvstILe#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpvstILe/incremental-state"
[2022-07-06T16:25:51Z DEBUG collector::execute] applying println to "/tmp/.tmpvstILe"
[2022-07-06T16:25:51Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-07-06T16:25:51Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-07-06T16:25:51Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpvstILe#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpvstILe/incremental-state"
[2022-07-06T16:26:00Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-06T16:26:01Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-07-06T16:26:01Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpbwPEK9#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2022-07-06T16:26:50Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-07-06T16:26:50Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-07-06T16:26:50Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpbwPEK9#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpbwPEK9/incremental-state"
[2022-07-06T16:27:43Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-07-06T16:27:43Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpbwPEK9#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpbwPEK9/incremental-state"
[2022-07-06T16:27:52Z DEBUG collector::execute] applying println to "/tmp/.tmpbwPEK9"
[2022-07-06T16:27:52Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-07-06T16:27:52Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-07-06T16:27:52Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpbwPEK9#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpbwPEK9/incremental-state"
[2022-07-06T16:28:12Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-06T16:28:12Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-07-06T16:28:12Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpqAOs3H#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2022-07-06T16:29:07Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-07-06T16:29:07Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-07-06T16:29:07Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpqAOs3H#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpqAOs3H/incremental-state"
[2022-07-06T16:30:00Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-07-06T16:30:00Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpqAOs3H#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpqAOs3H/incremental-state"
[2022-07-06T16:30:08Z DEBUG collector::execute] applying println to "/tmp/.tmpqAOs3H"
[2022-07-06T16:30:08Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-07-06T16:30:08Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-07-06T16:30:08Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpqAOs3H#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpqAOs3H/incremental-state"
Executing benchmark ctfe-stress-5 (3/8)
Preparing ctfe-stress-5
[2022-07-06T16:30:33Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-07-06T16:30:33Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2022-07-06T16:30:34Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-06T16:30:34Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-07-06T16:30:34Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpm4azAm#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-07-06T16:30:42Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-07-06T16:30:42Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpm4azAm#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpm4azAm/incremental-state"
[2022-07-06T16:30:51Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-07-06T16:30:51Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpm4azAm#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpm4azAm/incremental-state"
[2022-07-06T16:30:52Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-06T16:30:52Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-07-06T16:30:52Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmphrF0qV#ctfe-stress-5@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-07-06T16:31:00Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
---
[2022-07-06T16:31:12Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-06T16:31:12Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-07-06T16:31:12Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp9dXJqM#ctfe-stress-5@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-07-06T16:31:19Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-07-06T16:31:19Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp9dXJqM#ctfe-stress-5@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp9dXJqM/incremental-state"
[2022-07-06T16:31:28Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-07-06T16:31:28Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp9dXJqM#ctfe-stress-5@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp9dXJqM/incremental-state"
Executing benchmark diesel-1.4.8 (4/8)
Preparing diesel-1.4.8
[2022-07-06T16:31:29Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-07-06T16:31:29Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-07-06T16:31:29Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-07-06T16:31:29Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-07-06T16:31:29Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp0BRGF7#diesel@1.4.8" "--profile" "check" "--" "--skip-this-rustc"
[2022-07-06T16:31:29Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpxlngsQ#diesel@1.4.8" "--release" "--" "--skip-this-rustc"
[2022-07-06T16:31:29Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmprGgOZE#diesel@1.4.8" "--" "--skip-this-rustc"
[2022-07-06T16:31:46Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-06T16:31:46Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-07-06T16:31:46Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp0dS8Li#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-07-06T16:32:01Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
---
[2022-07-06T16:32:24Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-06T16:32:24Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-07-06T16:32:24Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpE0Sfbp#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln"
[2022-07-06T16:32:40Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-07-06T16:32:40Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpE0Sfbp#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpE0Sfbp/incremental-state"
[2022-07-06T16:33:01Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-07-06T16:33:01Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpE0Sfbp#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpE0Sfbp/incremental-state"
[2022-07-06T16:33:04Z DEBUG collector::execute] applying println to "/tmp/.tmpE0Sfbp"
[2022-07-06T16:33:04Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-07-06T16:33:04Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-07-06T16:33:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpE0Sfbp#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpE0Sfbp/incremental-state"
[2022-07-06T16:33:08Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-06T16:33:08Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-07-06T16:33:08Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOUjj9Y#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-07-06T16:33:25Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-07-06T16:33:25Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-07-06T16:33:25Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOUjj9Y#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpOUjj9Y/incremental-state"
[2022-07-06T16:33:44Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-07-06T16:33:44Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOUjj9Y#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpOUjj9Y/incremental-state"
[2022-07-06T16:33:47Z DEBUG collector::execute] applying println to "/tmp/.tmpOUjj9Y"
[2022-07-06T16:33:47Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-07-06T16:33:47Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-07-06T16:33:47Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOUjj9Y#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpOUjj9Y/incremental-state"
Executing benchmark externs (5/8)
Preparing externs
[2022-07-06T16:33:51Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-07-06T16:33:51Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
---
[2022-07-06T16:33:51Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-06T16:33:51Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-07-06T16:33:51Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzYNvXb#externs@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-07-06T16:33:52Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-07-06T16:33:52Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzYNvXb#externs@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpzYNvXb/incremental-state"
[2022-07-06T16:33:53Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-07-06T16:33:53Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzYNvXb#externs@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpzYNvXb/incremental-state"
[2022-07-06T16:33:53Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-06T16:33:53Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-07-06T16:33:53Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjgZKTz#externs@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-07-06T16:33:54Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-07-06T16:33:54Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-07-06T16:33:54Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjgZKTz#externs@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpjgZKTz/incremental-state"
[2022-07-06T16:33:55Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-07-06T16:33:55Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjgZKTz#externs@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpjgZKTz/incremental-state"
[2022-07-06T16:33:55Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-06T16:33:55Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-07-06T16:33:55Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpkYy43W#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-07-06T16:33:56Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
[2022-07-06T16:34:05Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-06T16:34:05Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-07-06T16:34:05Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKAFwBM#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-07-06T16:34:07Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-07-06T16:34:07Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKAFwBM#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpKAFwBM/incremental-state"
[2022-07-06T16:34:11Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-07-06T16:34:11Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKAFwBM#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpKAFwBM/incremental-state"
[2022-07-06T16:34:12Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-06T16:34:12Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-07-06T16:34:12Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwCywvV#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-07-06T16:34:14Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-07-06T16:34:14Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-07-06T16:34:14Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwCywvV#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpwCywvV/incremental-state"
[2022-07-06T16:34:18Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-07-06T16:34:18Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwCywvV#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpwCywvV/incremental-state"
Executing benchmark token-stream-stress (7/8)
Preparing token-stream-stress
[2022-07-06T16:34:19Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-07-06T16:34:19Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2022-07-06T16:34:20Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-06T16:34:20Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-07-06T16:34:20Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpkeCuJh#token-stream-stress@0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2022-07-06T16:34:20Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-07-06T16:34:20Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpkeCuJh#token-stream-stress@0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpkeCuJh/incremental-state"
[2022-07-06T16:34:20Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-07-06T16:34:20Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpkeCuJh#token-stream-stress@0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpkeCuJh/incremental-state"
[2022-07-06T16:34:20Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-06T16:34:20Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-07-06T16:34:20Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpi3bwq5#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2022-07-06T16:34:20Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
---
[2022-07-06T16:34:21Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-06T16:34:21Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-07-06T16:34:21Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpgTQYil#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2022-07-06T16:34:21Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-07-06T16:34:21Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpgTQYil#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpgTQYil/incremental-state"
[2022-07-06T16:34:22Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-07-06T16:34:22Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpgTQYil#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpgTQYil/incremental-state"
Executing benchmark tuple-stress (8/8)
Preparing tuple-stress
[2022-07-06T16:34:22Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-07-06T16:34:22Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2022-07-06T16:34:41Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-06T16:34:41Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-07-06T16:34:41Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpnBhGUV#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-07-06T16:34:46Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-07-06T16:34:46Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpnBhGUV#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpnBhGUV/incremental-state"
[2022-07-06T16:34:52Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-07-06T16:34:52Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpnBhGUV#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpnBhGUV/incremental-state"
[2022-07-06T16:34:54Z DEBUG collector::execute] applying new row to "/tmp/.tmpnBhGUV"
[2022-07-06T16:34:54Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-07-06T16:34:54Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-07-06T16:34:54Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpnBhGUV#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpnBhGUV/incremental-state"
[2022-07-06T16:35:00Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-06T16:35:00Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-07-06T16:35:00Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpiQmSjE#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-07-06T16:35:05Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-07-06T16:35:05Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-07-06T16:35:05Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpiQmSjE#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpiQmSjE/incremental-state"
[2022-07-06T16:35:10Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-07-06T16:35:10Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpiQmSjE#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpiQmSjE/incremental-state"
[2022-07-06T16:35:12Z DEBUG collector::execute] applying new row to "/tmp/.tmpiQmSjE"
[2022-07-06T16:35:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-07-06T16:35:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-07-06T16:35:12Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpiQmSjE#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpiQmSjE/incremental-state"
+ cd /checkout/obj
+ RUSTC_PROFILE_MERGED_FILE=/tmp/rustc-pgo.profdata
+ ./build/x86_64-unknown-linux-gnu/llvm/bin/llvm-profdata merge -o /tmp/rustc-pgo.profdata /tmp/rustc-pgo
+ echo 'Rustc PGO statistics'
---
[RUSTC-TIMING] unic_char_range test:false 0.126
   Compiling either v1.6.0
[RUSTC-TIMING] unicode_xid test:false 0.093
   Compiling datafrog v2.0.1
warning: rustc_fs_util.eba7f75c-cgu.3: no profile data available for function _RNvXs2_NtCs1RiYDeDbftX_4core7convertNtNtNtB7_5alloc6layout11LayoutErrorINtB5_4FromBy_E4fromCslxyY8cvUtSy_13rustc_fs_util Hash = 742261418966908927
[RUSTC-TIMING] itoa test:false 0.115
   Compiling memchr v2.4.1
   Compiling memchr v2.4.1
warning: rustc_graphviz.533fbbd0-cgu.8: no profile data available for function _RNvXs2_NtCs1RiYDeDbftX_4core7convertNtNtNtB7_5alloc6layout11LayoutErrorINtB5_4FromBy_E4fromCsXBqYDOWXY_14rustc_graphviz Hash = 742261418966908927

warning: rustc_graphviz.533fbbd0-cgu.1: no profile data available for function _RNvMNtNtNtCs1RiYDeDbftX_4core4iter8adapters3revINtB2_3RevINtNtNtB8_3ops5range5RangejEE3newCsXBqYDOWXY_14rustc_graphviz Hash = 742261418966908927
[RUSTC-TIMING] ppv_lite86 test:false 0.423
   Compiling regex-syntax v0.6.25
[RUSTC-TIMING] rustc_fs_util test:false 0.107
warning: `rustc_fs_util` (lib) generated 1 warning
---
[RUSTC-TIMING] jobserver test:false 1.045
   Compiling block-buffer v0.10.2
[RUSTC-TIMING] rustc_parse_format test:false 0.911
   Compiling crypto-common v0.1.2
warning: rustc_llvm.4c6e1e7f-cgu.8: no profile data available for function _RNvXs2_NtCs1RiYDeDbftX_4core7convertNtNtNtB7_5alloc6layout11LayoutErrorINtB5_4FromBy_E4fromCsbTA0zitAzMx_10rustc_llvm Hash = 742261418966908927
[RUSTC-TIMING] block_buffer test:false 0.113
   Compiling rand v0.8.5
[RUSTC-TIMING] crypto_common test:false 0.108
   Compiling digest v0.10.2
---
[RUSTC-TIMING] regex_automata test:false 3.057
   Compiling object v0.28.4
[RUSTC-TIMING] rand test:false 1.363
   Compiling object v0.29.0
warning: rustc_serialize.f6e1cfa4-cgu.7: no profile data available for function _RINvNtCs1RiYDeDbftX_4core3ptr13drop_in_placeRjECsid0BaocMzwm_15rustc_serialize Hash = 742261418966908927

warning: rustc_serialize.f6e1cfa4-cgu.7: no profile data available for function _RINvNtCs1RiYDeDbftX_4core9panicking13assert_failedjjECsid0BaocMzwm_15rustc_serialize Hash = 742261418966908927

warning: rustc_serialize.f6e1cfa4-cgu.8: no profile data available for function _RNvXsV_NtCs1RiYDeDbftX_4core3fmtRjNtB5_5Debug3fmtCsid0BaocMzwm_15rustc_serialize Hash = 1124680650125156080
[RUSTC-TIMING] sha2 test:false 1.154
[RUSTC-TIMING] rustc_serialize test:false 0.595
warning: `rustc_serialize` (lib) generated 3 warnings
[RUSTC-TIMING] tempfile test:false 0.680
---
[RUSTC-TIMING] rustc_mir_build test:false 1.931
error: unresolved link to `Abi`
   --> compiler/rustc_const_eval/src/interpret/machine.rs:142:48
    |
142 |     /// Whether function calls should be [ABI](Abi)-checked.
    |
    |
    = note: `-D rustdoc::broken-intra-doc-links` implied by `-D warnings`
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: could not document `rustc_const_eval`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name rustc_const_eval compiler/rustc_const_eval/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/doc -Zunstable-options --check-cfg 'values(feature)' --check-cfg 'names()' --check-cfg 'values()' --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -C metadata=56430a280d89f337 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-a40e8f2b6ed34d4d.rmeta --extern rustc_ast=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_ast-11872d71b90facab.rmeta --extern rustc_attr=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_attr-c7b8675a5cf10d01.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-6cc9efdf702efa25.rmeta --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-c74330eea2a242b3.rmeta --extern rustc_hir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_hir-3ca6dfe3b03ab4c2.rmeta --extern rustc_index=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_index-f77118641bc8969b.rmeta --extern rustc_infer=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_infer-e9f7d29a6242b5cb.rmeta --extern rustc_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps/librustc_macros-3402b970e104daab.so --extern rustc_middle=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_middle-f2577e2fba08f8fe.rmeta --extern rustc_mir_dataflow=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_mir_dataflow-19b6e5c46ba8c693.rmeta --extern rustc_query_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_query_system-4348aa68faca38de.rmeta --extern rustc_session=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_session-2d2def363823d426.rmeta --extern rustc_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_span-11b12bd83e996fca.rmeta --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-a9e3cbc0d109f75c.rmeta --extern rustc_trait_selection=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_trait_selection-958786e589489a82.rmeta --extern rustc_type_ir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_type_ir-bc57348614b023cf.rmeta --extern tracing=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libtracing-eef4fab2e6aecb3e.rmeta --extern-html-root-url 'tracing=https://docs.rs/tracing/0.1.29/' -Zunstable-options -Csymbol-mangling-version=v0 -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(parallel_compiler)' '--check-cfg=values(no_btreemap_remove_entry)' '--check-cfg=values(crossbeam_loom)' '--check-cfg=values(span_locations)' -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.64.0-nightly
  (0063a509f
  2022-07-06)' --document-private-items '-Arustdoc::private-intra-doc-links' --enable-index-page -Zunstable-options -Znormalize-docs --show-type-layout --generate-link-to-definition --extern-html-root-url 'ena=https://docs.rs/ena/latest/'` (exit status: 1)
[RUSTC-TIMING] rustc_const_eval test:false 2.849
[RUSTC-TIMING] rustc_typeck test:false 7.517
Build completed unsuccessfully in 0:21:31
