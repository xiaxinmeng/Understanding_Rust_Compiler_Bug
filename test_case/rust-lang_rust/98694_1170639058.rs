plain
[2022-06-29T23:37:23Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpctuZ7o#serde@1.0.136" "--" "--wrap-rustc-with" "Eprintln"
Running serde-1.0.136: Opt + [Full]
[2022-06-29T23:37:27Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-29T23:37:27Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-06-29T23:37:27Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpdsMuZL#serde@1.0.136" "--release" "--" "--wrap-rustc-with" "Eprintln"
Executing benchmark syn-1.0.89 (7/7)
Preparing syn-1.0.89
[2022-06-29T23:37:31Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-06-29T23:37:31Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
Preparing bitmaps-3.1.0
[2022-06-29T23:50:10Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-06-29T23:50:10Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-06-29T23:50:10Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-06-29T23:50:10Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpropNYQ#bitmaps@3.1.0" "--profile" "check" "--" "--skip-this-rustc"
[2022-06-29T23:50:10Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpEX7ApE#bitmaps@3.1.0" "--" "--skip-this-rustc"
[2022-06-29T23:50:10Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpZ0abeo#bitmaps@3.1.0" "--release" "--" "--skip-this-rustc"
[2022-06-29T23:50:11Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-29T23:50:11Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-06-29T23:50:11Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp2JrXvt#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-29T23:50:13Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-06-29T23:50:13Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-06-29T23:50:13Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp2JrXvt#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp2JrXvt/incremental-state"
[2022-06-29T23:50:16Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-06-29T23:50:16Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp2JrXvt#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp2JrXvt/incremental-state"
[2022-06-29T23:50:16Z DEBUG collector::execute] applying println to "/tmp/.tmp2JrXvt"
[2022-06-29T23:50:16Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-06-29T23:50:16Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-06-29T23:50:16Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp2JrXvt#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp2JrXvt/incremental-state"
[2022-06-29T23:50:17Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-29T23:50:17Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-06-29T23:50:17Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpuNSp3X#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-29T23:50:19Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-06-29T23:50:19Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-06-29T23:50:19Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpuNSp3X#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpuNSp3X/incremental-state"
[2022-06-29T23:50:22Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-06-29T23:50:22Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpuNSp3X#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpuNSp3X/incremental-state"
[2022-06-29T23:50:23Z DEBUG collector::execute] applying println to "/tmp/.tmpuNSp3X"
[2022-06-29T23:50:23Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-06-29T23:50:23Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-06-29T23:50:23Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpuNSp3X#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpuNSp3X/incremental-state"
[2022-06-29T23:50:24Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-29T23:50:24Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-06-29T23:50:24Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpdwpRTd#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-29T23:50:26Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-06-29T23:50:26Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-06-29T23:50:26Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpdwpRTd#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpdwpRTd/incremental-state"
[2022-06-29T23:50:29Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-06-29T23:50:29Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpdwpRTd#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpdwpRTd/incremental-state"
[2022-06-29T23:50:30Z DEBUG collector::execute] applying println to "/tmp/.tmpdwpRTd"
[2022-06-29T23:50:30Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-06-29T23:50:30Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-06-29T23:50:30Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpdwpRTd#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpdwpRTd/incremental-state"
Executing benchmark cargo-0.60.0 (2/8)
Preparing cargo-0.60.0
[2022-06-29T23:50:31Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-06-29T23:50:31Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2022-06-29T23:51:28Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-29T23:51:28Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-06-29T23:51:28Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpeEvDEU#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-29T23:51:49Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-06-29T23:51:49Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpeEvDEU#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpeEvDEU/incremental-state"
[2022-06-29T23:52:13Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-06-29T23:52:13Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpeEvDEU#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpeEvDEU/incremental-state"
[2022-06-29T23:52:18Z DEBUG collector::execute] applying println to "/tmp/.tmpeEvDEU"
[2022-06-29T23:52:18Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-06-29T23:52:18Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-06-29T23:52:18Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpeEvDEU#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpeEvDEU/incremental-state"
[2022-06-29T23:52:28Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-29T23:52:29Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-06-29T23:52:29Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzz6y58#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-29T23:53:17Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
---
[2022-06-29T23:54:49Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-29T23:54:50Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-06-29T23:54:50Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpS3UmTK#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-29T23:55:51Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-06-29T23:55:51Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpS3UmTK#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpS3UmTK/incremental-state"
[2022-06-29T23:56:54Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-06-29T23:56:54Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpS3UmTK#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpS3UmTK/incremental-state"
[2022-06-29T23:57:05Z DEBUG collector::execute] applying println to "/tmp/.tmpS3UmTK"
[2022-06-29T23:57:05Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-06-29T23:57:05Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-06-29T23:57:05Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpS3UmTK#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpS3UmTK/incremental-state"
Executing benchmark ctfe-stress-5 (3/8)
Preparing ctfe-stress-5
[2022-06-29T23:57:34Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-06-29T23:57:34Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
---
[2022-06-29T23:58:17Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-29T23:58:17Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-06-29T23:58:17Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpHArLz3#ctfe-stress-5@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-29T23:58:25Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-06-29T23:58:26Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpHArLz3#ctfe-stress-5@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpHArLz3/incremental-state"
[2022-06-29T23:58:37Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-06-29T23:58:37Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpHArLz3#ctfe-stress-5@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpHArLz3/incremental-state"
Executing benchmark diesel-1.4.8 (4/8)
Preparing diesel-1.4.8
[2022-06-29T23:58:38Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-06-29T23:58:38Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2022-06-29T23:58:56Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-29T23:58:57Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-06-29T23:58:57Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpMrY4iA#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-29T23:59:13Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-06-29T23:59:13Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpMrY4iA#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpMrY4iA/incremental-state"
[2022-06-29T23:59:32Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-06-29T23:59:32Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpMrY4iA#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpMrY4iA/incremental-state"
[2022-06-29T23:59:35Z DEBUG collector::execute] applying println to "/tmp/.tmpMrY4iA"
[2022-06-29T23:59:35Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-06-29T23:59:35Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-06-29T23:59:35Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpMrY4iA#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpMrY4iA/incremental-state"
[2022-06-29T23:59:39Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-29T23:59:39Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-06-29T23:59:39Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp6iy4DM#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-29T23:59:57Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
---
[2022-06-30T00:01:23Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-30T00:01:23Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-06-30T00:01:23Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpcKKW3t#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-30T00:01:26Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-06-30T00:01:26Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpcKKW3t#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpcKKW3t/incremental-state"
[2022-06-30T00:01:29Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-06-30T00:01:29Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpcKKW3t#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpcKKW3t/incremental-state"
[2022-06-30T00:01:30Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-30T00:01:30Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-06-30T00:01:30Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjV9gzr#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-30T00:01:34Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-06-30T00:01:34Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-06-30T00:01:34Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjV9gzr#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpjV9gzr/incremental-state"
[2022-06-30T00:01:37Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-06-30T00:01:37Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjV9gzr#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpjV9gzr/incremental-state"
[2022-06-30T00:01:38Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-30T00:01:38Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-06-30T00:01:38Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmph7rA1i#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-30T00:01:42Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-06-30T00:01:42Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-06-30T00:01:42Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmph7rA1i#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmph7rA1i/incremental-state"
[2022-06-30T00:01:45Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-06-30T00:01:45Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmph7rA1i#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmph7rA1i/incremental-state"
Executing benchmark token-stream-stress (7/8)
Preparing token-stream-stress
[2022-06-30T00:01:47Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-06-30T00:01:47Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2022-06-30T00:01:47Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-30T00:01:47Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-06-30T00:01:47Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpGGEIfN#token-stream-stress@0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-30T00:01:48Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-06-30T00:01:48Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpGGEIfN#token-stream-stress@0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpGGEIfN/incremental-state"
[2022-06-30T00:01:48Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-06-30T00:01:48Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpGGEIfN#token-stream-stress@0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpGGEIfN/incremental-state"
[2022-06-30T00:01:48Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-30T00:01:48Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-06-30T00:01:48Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp9Dowlq#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-30T00:01:48Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-06-30T00:01:48Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-06-30T00:01:48Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp9Dowlq#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp9Dowlq/incremental-state"
[2022-06-30T00:01:49Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-06-30T00:01:49Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp9Dowlq#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp9Dowlq/incremental-state"
[2022-06-30T00:01:49Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-30T00:01:49Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-06-30T00:01:49Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwpZk5c#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-30T00:01:49Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-06-30T00:01:49Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-06-30T00:01:49Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwpZk5c#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpwpZk5c/incremental-state"
[2022-06-30T00:01:50Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-06-30T00:01:50Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwpZk5c#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpwpZk5c/incremental-state"
Executing benchmark tuple-stress (8/8)
Preparing tuple-stress
[2022-06-30T00:01:50Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-06-30T00:01:50Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2022-06-30T00:02:31Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-06-30T00:02:31Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-06-30T00:02:31Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpnXo7gg#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-06-30T00:02:36Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-06-30T00:02:36Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpnXo7gg#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpnXo7gg/incremental-state"
[2022-06-30T00:02:43Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-06-30T00:02:43Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpnXo7gg#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpnXo7gg/incremental-state"
[2022-06-30T00:02:45Z DEBUG collector::execute] applying new row to "/tmp/.tmpnXo7gg"
[2022-06-30T00:02:45Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-06-30T00:02:45Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-06-30T00:02:45Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpnXo7gg#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpnXo7gg/incremental-state"
+ cd /checkout/obj
+ RUSTC_PROFILE_MERGED_FILE=/tmp/rustc-pgo.profdata
+ ./build/x86_64-unknown-linux-gnu/llvm/bin/llvm-profdata merge -o /tmp/rustc-pgo.profdata /tmp/rustc-pgo
+ echo 'Rustc PGO statistics'
---
   Compiling itoa v0.4.6
[RUSTC-TIMING] unic_common test:false 0.096
[RUSTC-TIMING] arrayvec test:false 0.243
[RUSTC-TIMING] scoped_tls test:false 0.117
warning: rustc_graphviz.d70860d4-cgu.1: no profile data available for function _RNvXs2_NtCs9J8scJQPwgM_4core7convertNtNtNtB7_5alloc6layout11LayoutErrorINtB5_4FromBy_E4fromCsdIj4FGAg0kW_14rustc_graphviz Hash = 742261418966908927
   Compiling termcolor v1.1.2
[RUSTC-TIMING] unicode_xid test:false 0.091
   Compiling rustc_fs_util v0.0.0 (/checkout/compiler/rustc_fs_util)
[RUSTC-TIMING] unic_char_range test:false 0.150
[RUSTC-TIMING] unic_char_range test:false 0.150
   Compiling either v1.6.0
warning: rustc_graphviz.d70860d4-cgu.2: no profile data available for function _RNvMNtNtNtCs9J8scJQPwgM_4core4iter8adapters3revINtB2_3RevINtNtNtB8_3ops5range5RangejEE3newCsdIj4FGAg0kW_14rustc_graphviz Hash = 742261418966908927
[RUSTC-TIMING] self_cell test:false 0.090
   Compiling datafrog v2.0.1
[RUSTC-TIMING] itoa test:false 0.116
   Compiling memchr v2.4.1
   Compiling memchr v2.4.1
warning: rustc_fs_util.0dfac017-cgu.5: no profile data available for function _RNvXs2_NtCs9J8scJQPwgM_4core7convertNtNtNtB7_5alloc6layout11LayoutErrorINtB5_4FromBy_E4fromCsi4LyHHnApkF_13rustc_fs_util Hash = 742261418966908927
[RUSTC-TIMING] rustc_fs_util test:false 0.106
warning: `rustc_fs_util` (lib) generated 1 warning
   Compiling tinyvec v0.3.4
[RUSTC-TIMING] ppv_lite86 test:false 0.470
---
[RUSTC-TIMING] crypto_common test:false 0.083
   Compiling rand v0.8.5
[RUSTC-TIMING] block_buffer test:false 0.121
   Compiling digest v0.10.2
warning: rustc_llvm.62c9b899-cgu.2: no profile data available for function _RNvXs2_NtCs9J8scJQPwgM_4core7convertNtNtNtB7_5alloc6layout11LayoutErrorINtB5_4FromBy_E4fromCsiOiNP0dW70X_10rustc_llvm Hash = 742261418966908927
[RUSTC-TIMING] rustc_llvm test:false 0.246
warning: `rustc_llvm` (lib) generated 1 warning
[RUSTC-TIMING] hashbrown test:false 0.660
   Compiling md-5 v0.10.0
---
[RUSTC-TIMING] rand_xoshiro test:false 1.110
   Compiling object v0.29.0
[RUSTC-TIMING] measureme test:false 1.280
   Compiling tempfile v3.2.0
warning: rustc_serialize.da5944de-cgu.5: no profile data available for function _RINvNtCs9J8scJQPwgM_4core3ptr13drop_in_placeRjECs3BnJQTbX4OU_15rustc_serialize Hash = 742261418966908927

warning: rustc_serialize.da5944de-cgu.5: no profile data available for function _RINvNtCs9J8scJQPwgM_4core9panicking13assert_failedjjECs3BnJQTbX4OU_15rustc_serialize Hash = 742261418966908927
[RUSTC-TIMING] aho_corasick test:false 2.986
[RUSTC-TIMING] aho_corasick test:false 2.986
warning: rustc_serialize.da5944de-cgu.6: no profile data available for function _RNvXsV_NtCs9J8scJQPwgM_4core3fmtRjNtB5_5Debug3fmtCs3BnJQTbX4OU_15rustc_serialize Hash = 1124680650125156080
[RUSTC-TIMING] rand test:false 1.301
[RUSTC-TIMING] rustc_serialize test:false 0.627
warning: `rustc_serialize` (lib) generated 3 warnings
[RUSTC-TIMING] sha2 test:false 1.361
---
   Compiling rustc_ast v0.0.0 (/checkout/compiler/rustc_ast)
   Compiling rustc_error_messages v0.0.0 (/checkout/compiler/rustc_error_messages)
   Compiling rustc_target v0.0.0 (/checkout/compiler/rustc_target)
   Compiling rustc_feature v0.0.0 (/checkout/compiler/rustc_feature)
warning: rustc_error_messages.965eb93b-cgu.11: no profile data available for function _RINvMsx_NtCs7U4p3AdycwC_12tracing_core5fieldNtB6_8FieldSet9value_setATRNtB6_5FieldINtNtCs9J8scJQPwgM_4core6option6OptionRDNtB6_5ValueEL_EEj1_ECs5HBKtRtzoXT_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.965eb93b-cgu.11: no profile data available for function _RINvMsx_NtCs7U4p3AdycwC_12tracing_core5fieldNtB6_8FieldSet9value_setATRNtB6_5FieldINtNtCs9J8scJQPwgM_4core6option6OptionRDNtB6_5ValueEL_EEj2_ECs5HBKtRtzoXT_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.965eb93b-cgu.11: no profile data available for function _RINvMsx_NtCs7U4p3AdycwC_12tracing_core5fieldNtB6_8FieldSet9value_setATRNtB6_5FieldINtNtCs9J8scJQPwgM_4core6option6OptionRDNtB6_5ValueEL_EEj5_ECs5HBKtRtzoXT_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.965eb93b-cgu.11: no profile data available for function _RINvNtCs7U4p3AdycwC_12tracing_core5field5debugRINtNtCs9J8scJQPwgM_4core6option6OptionNtCsegVWy4bq351_16unic_langid_impl18LanguageIdentifierEECs5HBKtRtzoXT_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.965eb93b-cgu.11: no profile data available for function _RINvNtCs7U4p3AdycwC_12tracing_core5field5debugRINtNtCs9J8scJQPwgM_4core6option6OptionNtNtCshSBNp2kA8Pe_3std4path7PathBufEECs5HBKtRtzoXT_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.965eb93b-cgu.11: no profile data available for function _RINvNtCs7U4p3AdycwC_12tracing_core5field5debugRINtNtCs9J8scJQPwgM_4core6option6OptionRNtNtCshSBNp2kA8Pe_3std4path4PathEECs5HBKtRtzoXT_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.965eb93b-cgu.11: no profile data available for function _RINvNtCs7U4p3AdycwC_12tracing_core5field5debugRINtNtCsiwe2CTJV7s7_5alloc3vec3VecNtNtCshSBNp2kA8Pe_3std4path7PathBufEECs5HBKtRtzoXT_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.965eb93b-cgu.11: no profile data available for function _RINvNtCs7U4p3AdycwC_12tracing_core5field5debugRNtCsegVWy4bq351_16unic_langid_impl18LanguageIdentifierECs5HBKtRtzoXT_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.965eb93b-cgu.11: no profile data available for function _RINvNtCs7U4p3AdycwC_12tracing_core5field5debugRNtNtCsbUDO7twQdUc_13fluent_bundle8resource14FluentResourceECs5HBKtRtzoXT_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.965eb93b-cgu.11: no profile data available for function _RINvNtCs7U4p3AdycwC_12tracing_core5field5debugRNtNtCshSBNp2kA8Pe_3std4path7PathBufECs5HBKtRtzoXT_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.965eb93b-cgu.11: no profile data available for function _RINvNtCs7U4p3AdycwC_12tracing_core5field5debugRQNtNtCshSBNp2kA8Pe_3std4path7PathBufECs5HBKtRtzoXT_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.965eb93b-cgu.11: no profile data available for function _RINvNtCs7U4p3AdycwC_12tracing_core5field5debugRRSReECs5HBKtRtzoXT_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.965eb93b-cgu.11: no profile data available for function _RINvNtCs9J8scJQPwgM_4core3ptr13drop_in_placeRINtNtB4_6option6OptionNtCsegVWy4bq351_16unic_langid_impl18LanguageIdentifierEECs5HBKtRtzoXT_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.965eb93b-cgu.11: no profile data available for function _RINvNtCs9J8scJQPwgM_4core3ptr13drop_in_placeRINtNtB4_6option6OptionNtNtCshSBNp2kA8Pe_3std4path7PathBufEECs5HBKtRtzoXT_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.965eb93b-cgu.11: no profile data available for function _RINvNtCs9J8scJQPwgM_4core3ptr13drop_in_placeRINtNtB4_6option6OptionRNtNtCshSBNp2kA8Pe_3std4path4PathEECs5HBKtRtzoXT_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.965eb93b-cgu.11: no profile data available for function _RINvNtCs9J8scJQPwgM_4core3ptr13drop_in_placeRINtNtCsiwe2CTJV7s7_5alloc3vec3VecNtNtCshSBNp2kA8Pe_3std4path7PathBufEECs5HBKtRtzoXT_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.965eb93b-cgu.11: no profile data available for function _RINvNtCs9J8scJQPwgM_4core3ptr13drop_in_placeRNtCsegVWy4bq351_16unic_langid_impl18LanguageIdentifierECs5HBKtRtzoXT_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.965eb93b-cgu.11: no profile data available for function _RINvNtCs9J8scJQPwgM_4core3ptr13drop_in_placeRNtNtCsbUDO7twQdUc_13fluent_bundle8resource14FluentResourceECs5HBKtRtzoXT_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.965eb93b-cgu.11: no profile data available for function _RINvNtCs9J8scJQPwgM_4core3ptr13drop_in_placeRNtNtCshSBNp2kA8Pe_3std4path7PathBufECs5HBKtRtzoXT_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.965eb93b-cgu.11: no profile data available for function _RINvNtCs9J8scJQPwgM_4core3ptr13drop_in_placeRQNtNtCshSBNp2kA8Pe_3std4path7PathBufECs5HBKtRtzoXT_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.965eb93b-cgu.11: no profile data available for function _RINvNtCs9J8scJQPwgM_4core3ptr13drop_in_placeRRSReECs5HBKtRtzoXT_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.965eb93b-cgu.11: no profile data available for function _RNvXsk_NtCs7U4p3AdycwC_12tracing_core5fieldINtB5_10DebugValueRINtNtCs9J8scJQPwgM_4core6option6OptionNtCsegVWy4bq351_16unic_langid_impl18LanguageIdentifierEENtB5_5Value6recordCs5HBKtRtzoXT_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.965eb93b-cgu.11: no profile data available for function _RNvXsk_NtCs7U4p3AdycwC_12tracing_core5fieldINtB5_10DebugValueRINtNtCs9J8scJQPwgM_4core6option6OptionNtNtCshSBNp2kA8Pe_3std4path7PathBufEENtB5_5Value6recordCs5HBKtRtzoXT_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.965eb93b-cgu.11: no profile data available for function _RNvXsk_NtCs7U4p3AdycwC_12tracing_core5fieldINtB5_10DebugValueRINtNtCs9J8scJQPwgM_4core6option6OptionRNtNtCshSBNp2kA8Pe_3std4path4PathEENtB5_5Value6recordCs5HBKtRtzoXT_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.965eb93b-cgu.11: no profile data available for function _RNvXsk_NtCs7U4p3AdycwC_12tracing_core5fieldINtB5_10DebugValueRINtNtCsiwe2CTJV7s7_5alloc3vec3VecNtNtCshSBNp2kA8Pe_3std4path7PathBufEENtB5_5Value6recordCs5HBKtRtzoXT_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.965eb93b-cgu.11: no profile data available for function _RNvXsk_NtCs7U4p3AdycwC_12tracing_core5fieldINtB5_10DebugValueRNtCsegVWy4bq351_16unic_langid_impl18LanguageIdentifierENtB5_5Value6recordCs5HBKtRtzoXT_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.965eb93b-cgu.11: no profile data available for function _RNvXsk_NtCs7U4p3AdycwC_12tracing_core5fieldINtB5_10DebugValueRNtNtCsbUDO7twQdUc_13fluent_bundle8resource14FluentResourceENtB5_5Value6recordCs5HBKtRtzoXT_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.965eb93b-cgu.11: no profile data available for function _RNvXsk_NtCs7U4p3AdycwC_12tracing_core5fieldINtB5_10DebugValueRNtNtCshSBNp2kA8Pe_3std4path7PathBufENtB5_5Value6recordCs5HBKtRtzoXT_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.965eb93b-cgu.11: no profile data available for function _RNvXsk_NtCs7U4p3AdycwC_12tracing_core5fieldINtB5_10DebugValueRQNtNtCshSBNp2kA8Pe_3std4path7PathBufENtB5_5Value6recordCs5HBKtRtzoXT_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.965eb93b-cgu.11: no profile data available for function _RNvXsk_NtCs7U4p3AdycwC_12tracing_core5fieldINtB5_10DebugValueRRSReENtB5_5Value6recordCs5HBKtRtzoXT_20rustc_error_messages Hash = 170957022131388415
[RUSTC-TIMING] rustc_error_messages test:false 0.859
warning: `rustc_error_messages` (lib) generated 30 warnings
[RUSTC-TIMING] rustc_feature test:false 1.033
[RUSTC-TIMING] jemalloc_sys test:false 0.064
---
error[E0308]: mismatched types
    |
   ::: src/librustdoc/clean/types.rs:126:1
    |
126 |   rustc_data_structures::static_assert_size!(Crate, 72);

[RUSTC-TIMING] askama_shared test:false 5.490
For more information about this error, try `rustc --explain E0308`.
[RUSTC-TIMING] rustdoc test:false 3.430
