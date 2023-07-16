plain
[2022-03-11T18:15:29Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-11T18:15:30Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-03-11T18:15:30Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpyIMNKP#cargo:0.29.0" "--lib" "--" "--wrap-rustc-with" "eprintln"
[2022-03-11T18:16:06Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-03-11T18:16:06Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpyIMNKP#cargo:0.29.0" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpyIMNKP/incremental-state"
[2022-03-11T18:16:47Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2022-03-11T18:16:47Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpyIMNKP#cargo:0.29.0" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpyIMNKP/incremental-state"
[2022-03-11T18:16:54Z DEBUG collector::execute] applying println to "/tmp/.tmpyIMNKP"
[2022-03-11T18:16:54Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-03-11T18:16:54Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-03-11T18:16:54Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpyIMNKP#cargo:0.29.0" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpyIMNKP/incremental-state"
[2022-03-11T18:17:11Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-11T18:17:12Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-03-11T18:17:12Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpuoCvKL#cargo:0.29.0" "--release" "--lib" "--" "--wrap-rustc-with" "eprintln"
[2022-03-11T18:17:54Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2022-03-11T18:17:54Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2022-03-11T18:17:54Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpuoCvKL#cargo:0.29.0" "--release" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpuoCvKL/incremental-state"
[2022-03-11T18:18:39Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrUnchanged), patch=None
[2022-03-11T18:18:39Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpuoCvKL#cargo:0.29.0" "--release" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpuoCvKL/incremental-state"
[2022-03-11T18:18:46Z DEBUG collector::execute] applying println to "/tmp/.tmpuoCvKL"
[2022-03-11T18:18:46Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-03-11T18:18:46Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-03-11T18:18:46Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpuoCvKL#cargo:0.29.0" "--release" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpuoCvKL/incremental-state"
Preparing ctfe-stress-4
[2022-03-11T18:19:07Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=None, patch=None
[2022-03-11T18:19:07Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2022-03-11T18:19:07Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
---
[2022-03-11T18:19:08Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-11T18:19:08Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2022-03-11T18:19:08Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpbq7zds#ctfe-stress-4:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln"
[2022-03-11T18:19:31Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2022-03-11T18:19:31Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpbq7zds#ctfe-stress-4:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpbq7zds/incremental-state"
[2022-03-11T18:19:58Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrUnchanged), patch=None
[2022-03-11T18:19:58Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpbq7zds#ctfe-stress-4:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpbq7zds/incremental-state"
[2022-03-11T18:19:58Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-11T18:19:58Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-03-11T18:19:58Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmphPj097#ctfe-stress-4:0.1.0" "--" "--wrap-rustc-with" "eprintln"
[2022-03-11T18:20:21Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
---
[2022-03-11T18:21:35Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpXi2cXq#externs:0.1.0" "--" "--skip-this-rustc"
Running externs: Check + [Full, IncrFull, IncrUnchanged, IncrPatched]
[2022-03-11T18:21:36Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-11T18:21:36Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2022-03-11T18:21:36Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFIqMum#externs:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln"
[2022-03-11T18:21:37Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2022-03-11T18:21:37Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFIqMum#externs:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpFIqMum/incremental-state"
[2022-03-11T18:21:38Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrUnchanged), patch=None
[2022-03-11T18:21:38Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFIqMum#externs:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpFIqMum/incremental-state"
[2022-03-11T18:21:38Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-11T18:21:38Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-03-11T18:21:38Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpj3NRWt#externs:0.1.0" "--" "--wrap-rustc-with" "eprintln"
[2022-03-11T18:21:39Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-03-11T18:21:39Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-03-11T18:21:39Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpj3NRWt#externs:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpj3NRWt/incremental-state"
[2022-03-11T18:21:41Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2022-03-11T18:21:41Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpj3NRWt#externs:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpj3NRWt/incremental-state"
[2022-03-11T18:21:42Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-11T18:21:42Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-03-11T18:21:42Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp63ZMPq#externs:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln"
[2022-03-11T18:21:43Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
---
[2022-03-11T18:21:54Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-11T18:21:54Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-03-11T18:21:54Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmptLzqPg#inflate:0.1.0" "--" "--wrap-rustc-with" "eprintln"
[2022-03-11T18:21:57Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-03-11T18:21:57Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmptLzqPg#inflate:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmptLzqPg/incremental-state"
[2022-03-11T18:22:01Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2022-03-11T18:22:01Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmptLzqPg#inflate:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmptLzqPg/incremental-state"
[2022-03-11T18:22:02Z DEBUG collector::execute] applying println to "/tmp/.tmptLzqPg"
[2022-03-11T18:22:02Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-03-11T18:22:02Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-03-11T18:22:02Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmptLzqPg#inflate:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmptLzqPg/incremental-state"
[2022-03-11T18:22:06Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-11T18:22:06Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-03-11T18:22:06Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpq530vZ#inflate:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln"
[2022-03-11T18:22:11Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
---
[2022-03-11T18:22:24Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-11T18:22:24Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2022-03-11T18:22:24Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmps1QRVa#match-stress-enum:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln"
[2022-03-11T18:22:26Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2022-03-11T18:22:26Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmps1QRVa#match-stress-enum:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmps1QRVa/incremental-state"
[2022-03-11T18:22:28Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrUnchanged), patch=None
[2022-03-11T18:22:28Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmps1QRVa#match-stress-enum:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmps1QRVa/incremental-state"
[2022-03-11T18:22:28Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-11T18:22:28Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-03-11T18:22:28Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp4K6PXR#match-stress-enum:0.1.0" "--" "--wrap-rustc-with" "eprintln"
[2022-03-11T18:22:30Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
---
[2022-03-11T18:22:33Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-11T18:22:33Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-03-11T18:22:33Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmplGuKhk#match-stress-enum:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln"
[2022-03-11T18:22:35Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2022-03-11T18:22:35Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmplGuKhk#match-stress-enum:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmplGuKhk/incremental-state"
[2022-03-11T18:22:37Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrUnchanged), patch=None
[2022-03-11T18:22:37Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmplGuKhk#match-stress-enum:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmplGuKhk/incremental-state"
Preparing token-stream-stress
[2022-03-11T18:22:38Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=None, patch=None
[2022-03-11T18:22:38Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2022-03-11T18:22:38Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
---
[2022-03-11T18:22:40Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-11T18:22:40Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-03-11T18:22:40Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpkWPT0G#token-stream-stress:0.0.0" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln"
[2022-03-11T18:22:40Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-03-11T18:22:40Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpkWPT0G#token-stream-stress:0.0.0" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpkWPT0G/incremental-state"
[2022-03-11T18:22:41Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2022-03-11T18:22:41Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpkWPT0G#token-stream-stress:0.0.0" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpkWPT0G/incremental-state"
[2022-03-11T18:22:41Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-11T18:22:41Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-03-11T18:22:41Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQgQB4N#token-stream-stress:0.0.0" "--release" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln"
[2022-03-11T18:22:41Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2022-03-11T18:22:41Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2022-03-11T18:22:41Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQgQB4N#token-stream-stress:0.0.0" "--release" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpQgQB4N/incremental-state"
[2022-03-11T18:22:42Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrUnchanged), patch=None
[2022-03-11T18:22:42Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQgQB4N#token-stream-stress:0.0.0" "--release" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpQgQB4N/incremental-state"
+ ./build/x86_64-unknown-linux-gnu/llvm/bin/llvm-profdata merge -o /tmp/rustc-pgo.profdata /tmp/rustc-pgo
+ rm -r ./build/x86_64-unknown-linux-gnu/llvm ./build/x86_64-unknown-linux-gnu/lld
+ python3 ../x.py dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu --include-default-paths build-manifest --rust-profile-use=/tmp/rustc-pgo.profdata --llvm-profile-use=/tmp/llvm-pgo.profdata
warning: sqlite3 not available in python, skipping build directory lock
---
Copying stage2 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[TIMING] StdLink { compiler: Compiler { stage: 1, host: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } }, target_compiler: Compiler { stage: 2, host: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } }, target: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } } -- 0.001
[TIMING] Std { target: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None }, compiler: Compiler { stage: 2, host: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } } } -- 0.000
 Documenting core v0.0.0 (/checkout/library/core)
no resolution for "self" MacroNS DefId(0:1199 ~ core[bdef]::prelude)
no resolution for "self" TypeNS DefId(0:1199 ~ core[bdef]::prelude)
no resolution for "self" ValueNS DefId(0:1199 ~ core[bdef]::prelude)
error: unresolved link to `self`
   |
   |
13 | /// See the [module-level documentation](self) for more.
   |                                          ^^^^ no item named `self` in scope
   |
   = note: `-D rustdoc::broken-intra-doc-links` implied by `-D warnings`
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

no resolution for "self" MacroNS DefId(0:1199 ~ core[bdef]::prelude)
no resolution for "self" TypeNS DefId(0:1199 ~ core[bdef]::prelude)
no resolution for "self" ValueNS DefId(0:1199 ~ core[bdef]::prelude)
error: unresolved link to `self`
   |
   |
23 | /// See the [module-level documentation](self) for more.
   |                                          ^^^^ no item named `self` in scope
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

no resolution for "self" MacroNS DefId(0:1199 ~ core[bdef]::prelude)
no resolution for "self" TypeNS DefId(0:1199 ~ core[bdef]::prelude)
no resolution for "self" ValueNS DefId(0:1199 ~ core[bdef]::prelude)
error: unresolved link to `self`
   |
   |
33 | /// See the [module-level documentation](self) for more.
   |                                          ^^^^ no item named `self` in scope
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

no resolution for "core" TypeNS DefId(0:4780 ~ core[bdef]::char::methods)
no resolution for "super" TypeNS DefId(0:7796 ~ core[bdef]::iter::traits::exact_size)
no resolution for "ops::BitAnd" MacroNS DefId(0:0 ~ core[bdef])
no resolution for "ops::BitAnd" TypeNS DefId(0:0 ~ core[bdef])
no resolution for "ops" TypeNS DefId(0:0 ~ core[bdef])
no resolution for "ops::BitAnd" ValueNS DefId(0:0 ~ core[bdef])
no resolution for "ops" TypeNS DefId(0:0 ~ core[bdef])
no resolution for "ops" TypeNS DefId(0:0 ~ core[bdef])
no resolution for "ops" ValueNS DefId(0:0 ~ core[bdef])
no resolution for "ops" MacroNS DefId(0:0 ~ core[bdef])
error: unresolved link to `ops::BitAnd`
   |
   |
28 | /// [`BitAnd`]: ops::BitAnd
   |                 ^^^^^^^^^^^ no item named `ops` in scope

no resolution for "ops::BitOr" MacroNS DefId(0:0 ~ core[bdef])
no resolution for "ops::BitOr" TypeNS DefId(0:0 ~ core[bdef])
no resolution for "ops" TypeNS DefId(0:0 ~ core[bdef])
no resolution for "ops::BitOr" ValueNS DefId(0:0 ~ core[bdef])
no resolution for "ops" TypeNS DefId(0:0 ~ core[bdef])
no resolution for "ops" TypeNS DefId(0:0 ~ core[bdef])
no resolution for "ops" ValueNS DefId(0:0 ~ core[bdef])
no resolution for "ops" MacroNS DefId(0:0 ~ core[bdef])
error: unresolved link to `ops::BitOr`
   |
   |
29 | /// [`BitOr`]: ops::BitOr
   |                ^^^^^^^^^^ no item named `ops` in scope

no resolution for "ops::Not" MacroNS DefId(0:0 ~ core[bdef])
no resolution for "ops::Not" TypeNS DefId(0:0 ~ core[bdef])
no resolution for "ops" TypeNS DefId(0:0 ~ core[bdef])
no resolution for "ops::Not" ValueNS DefId(0:0 ~ core[bdef])
no resolution for "ops" TypeNS DefId(0:0 ~ core[bdef])
no resolution for "ops" TypeNS DefId(0:0 ~ core[bdef])
no resolution for "ops" ValueNS DefId(0:0 ~ core[bdef])
no resolution for "ops" MacroNS DefId(0:0 ~ core[bdef])
error: unresolved link to `ops::Not`
   |
   |
30 | /// [`Not`]: ops::Not
   |              ^^^^^^^^ no item named `ops` in scope

no resolution for "assert" MacroNS DefId(0:0 ~ core[bdef])
no resolution for "Copy" MacroNS DefId(0:0 ~ core[bdef])
no resolution for "Copy" TypeNS DefId(0:0 ~ core[bdef])
no resolution for "Copy" ValueNS DefId(0:0 ~ core[bdef])
error: unresolved link to `Copy`
   |
   |
54 | /// Also, since `bool` implements the [`Copy`] trait, we don't
   |                                         ^^^^ no item named `Copy` in scope
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

no resolution for "u32" TypeNS DefId(0:0 ~ core[bdef])
no resolution for "str::FromStr" MacroNS DefId(0:0 ~ core[bdef])
no resolution for "str::FromStr" TypeNS DefId(0:0 ~ core[bdef])
no resolution for "str::FromStr" ValueNS DefId(0:0 ~ core[bdef])
no resolution for "str" TypeNS DefId(0:0 ~ core[bdef])
error: unresolved link to `str::FromStr`
    |
    |
69  | / /// The `!` type, also called "never".
70  | | ///
71  | | /// `!` represents the type of computations which never resolve to any value at all. For example,
72  | | /// the [`exit`] function `fn exit(code: i32) -> !` exits the process without ever returning, and
...   |
272 | | /// [`default()`]: Default::default
    | |___^
    |
    = note: the link appears in this line:
            
            
            [`FromStr`]: str::FromStr
                         ^^^^^^^^^^^^
    = note: the builtin type `str` has no associated item named `FromStr`

no resolution for "Err" MacroNS DefId(0:0 ~ core[bdef])
no resolution for "Err" TypeNS DefId(0:0 ~ core[bdef])
no resolution for "Err" ValueNS DefId(0:0 ~ core[bdef])
error: unresolved link to `Err`
    |
    |
69  | / /// The `!` type, also called "never".
70  | | ///
71  | | /// `!` represents the type of computations which never resolve to any value at all. For example,
72  | | /// the [`exit`] function `fn exit(code: i32) -> !` exits the process without ever returning, and
...   |
272 | | /// [`default()`]: Default::default
    | |___^
    |
    = note: the link appears in this line:
            
            
            When implementing this trait for [`String`] we need to pick a type for [`Err`]. And since
    = note: no item named `Err` in scope
    = note: no item named `Err` in scope
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

no resolution for "Err" MacroNS DefId(0:0 ~ core[bdef])
no resolution for "Err" TypeNS DefId(0:0 ~ core[bdef])
no resolution for "Err" ValueNS DefId(0:0 ~ core[bdef])
no resolution for "Result" MacroNS DefId(0:0 ~ core[bdef])
no resolution for "Result" TypeNS DefId(0:0 ~ core[bdef])
no resolution for "Result" ValueNS DefId(0:0 ~ core[bdef])
error: unresolved link to `Result`
    |
    |
69  | / /// The `!` type, also called "never".
70  | | ///
71  | | /// `!` represents the type of computations which never resolve to any value at all. For example,
72  | | /// the [`exit`] function `fn exit(code: i32) -> !` exits the process without ever returning, and
...   |
272 | | /// [`default()`]: Default::default
    | |___^
    |
    = note: the link appears in this line:
            
            
            [`Result<String, !>`] which we can unpack like this:
    = note: no item named `Result` in scope
    = note: no item named `Result` in scope
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

no resolution for "str::FromStr::from_str" MacroNS DefId(0:0 ~ core[bdef])
no resolution for "str::FromStr::from_str" TypeNS DefId(0:0 ~ core[bdef])
no resolution for "str::FromStr" TypeNS DefId(0:0 ~ core[bdef])
no resolution for "str::FromStr::from_str" ValueNS DefId(0:0 ~ core[bdef])
no resolution for "str::FromStr" TypeNS DefId(0:0 ~ core[bdef])
no resolution for "str" TypeNS DefId(0:0 ~ core[bdef])
no resolution for "str::FromStr" TypeNS DefId(0:0 ~ core[bdef])
no resolution for "str::FromStr" ValueNS DefId(0:0 ~ core[bdef])
no resolution for "str::FromStr" MacroNS DefId(0:0 ~ core[bdef])
no resolution for "str" TypeNS DefId(0:0 ~ core[bdef])
error: unresolved link to `str::FromStr::from_str`
    |
    |
69  | / /// The `!` type, also called "never".
70  | | ///
71  | | /// `!` represents the type of computations which never resolve to any value at all. For example,
72  | | /// the [`exit`] function `fn exit(code: i32) -> !` exits the process without ever returning, and
...   |
272 | | /// [`default()`]: Default::default
    | |___^
    |
    = note: the link appears in this line:
            
            
            [`String::from_str`]: str::FromStr::from_str
                                  ^^^^^^^^^^^^^^^^^^^^^^
    = note: the builtin type `str` has no associated item named `FromStr`

no resolution for "Err" MacroNS DefId(0:0 ~ core[bdef])
no resolution for "Err" TypeNS DefId(0:0 ~ core[bdef])
no resolution for "Err" ValueNS DefId(0:0 ~ core[bdef])
no resolution for "Result" MacroNS DefId(0:0 ~ core[bdef])
no resolution for "Result" TypeNS DefId(0:0 ~ core[bdef])
no resolution for "Result" ValueNS DefId(0:0 ~ core[bdef])
no resolution for "Ok" MacroNS DefId(0:0 ~ core[bdef])
no resolution for "Ok" TypeNS DefId(0:0 ~ core[bdef])
no resolution for "Ok" ValueNS DefId(0:0 ~ core[bdef])
error: unresolved link to `Ok`
    |
    |
69  | / /// The `!` type, also called "never".
70  | | ///
71  | | /// `!` represents the type of computations which never resolve to any value at all. For example,
72  | | /// the [`exit`] function `fn exit(code: i32) -> !` exits the process without ever returning, and
...   |
272 | | /// [`default()`]: Default::default
    | |___^
    |
    = note: the link appears in this line:
            
            
            [`Ok`] variant. This illustrates another behaviour of `!` - it can be used to "delete" certain
             ^^^^
    = note: no item named `Ok` in scope
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

no resolution for "Result" MacroNS DefId(0:0 ~ core[bdef])
no resolution for "Result" TypeNS DefId(0:0 ~ core[bdef])
no resolution for "Result" ValueNS DefId(0:0 ~ core[bdef])
no resolution for "Result" MacroNS DefId(0:0 ~ core[bdef])
no resolution for "Result" TypeNS DefId(0:0 ~ core[bdef])
no resolution for "Result" ValueNS DefId(0:0 ~ core[bdef])
no resolution for "Result" MacroNS DefId(0:0 ~ core[bdef])
no resolution for "Result" TypeNS DefId(0:0 ~ core[bdef])
no resolution for "Result" ValueNS DefId(0:0 ~ core[bdef])
no resolution for "Result" MacroNS DefId(0:0 ~ core[bdef])
no resolution for "Result" TypeNS DefId(0:0 ~ core[bdef])
no resolution for "Result" ValueNS DefId(0:0 ~ core[bdef])
no resolution for "fmt::Debug" MacroNS DefId(0:0 ~ core[bdef])
no resolution for "fmt::Debug" TypeNS DefId(0:0 ~ core[bdef])
no resolution for "fmt" TypeNS DefId(0:0 ~ core[bdef])
no resolution for "fmt::Debug" ValueNS DefId(0:0 ~ core[bdef])
no resolution for "fmt" TypeNS DefId(0:0 ~ core[bdef])
no resolution for "fmt" TypeNS DefId(0:0 ~ core[bdef])
no resolution for "fmt" ValueNS DefId(0:0 ~ core[bdef])
no resolution for "fmt" MacroNS DefId(0:0 ~ core[bdef])
error: unresolved link to `fmt::Debug`
    |
    |
69  | / /// The `!` type, also called "never".
70  | | ///
71  | | /// `!` represents the type of computations which never resolve to any value at all. For example,
72  | | /// the [`exit`] function `fn exit(code: i32) -> !` exits the process without ever returning, and
...   |
272 | | /// [`default()`]: Default::default
    | |___^
    |
    = note: the link appears in this line:
            
            
            [`Debug`]: fmt::Debug
    = note: no item named `fmt` in scope


no resolution for "fmt::Result" MacroNS DefId(0:0 ~ core[bdef])
no resolution for "fmt::Result" TypeNS DefId(0:0 ~ core[bdef])
no resolution for "fmt" TypeNS DefId(0:0 ~ core[bdef])
no resolution for "fmt::Result" ValueNS DefId(0:0 ~ core[bdef])
no resolution for "fmt" TypeNS DefId(0:0 ~ core[bdef])
no resolution for "fmt" TypeNS DefId(0:0 ~ core[bdef])
no resolution for "fmt" ValueNS DefId(0:0 ~ core[bdef])
no resolution for "fmt" MacroNS DefId(0:0 ~ core[bdef])
error: unresolved link to `fmt::Result`
    |
    |
69  | / /// The `!` type, also called "never".
70  | | ///
71  | | /// `!` represents the type of computations which never resolve to any value at all. For example,
72  | | /// the [`exit`] function `fn exit(code: i32) -> !` exits the process without ever returning, and
...   |
272 | | /// [`default()`]: Default::default
    | |___^
    |
    = note: the link appears in this line:
            
            
            [`fmt::Result`]. Since this method takes a `&!` as an argument we know that it can never be
    = note: no item named `fmt` in scope


no resolution for "fmt::Result" MacroNS DefId(0:0 ~ core[bdef])
no resolution for "fmt::Result" TypeNS DefId(0:0 ~ core[bdef])
no resolution for "fmt" TypeNS DefId(0:0 ~ core[bdef])
no resolution for "fmt::Result" ValueNS DefId(0:0 ~ core[bdef])
no resolution for "fmt" TypeNS DefId(0:0 ~ core[bdef])
no resolution for "fmt" TypeNS DefId(0:0 ~ core[bdef])
no resolution for "fmt" ValueNS DefId(0:0 ~ core[bdef])
no resolution for "fmt" MacroNS DefId(0:0 ~ core[bdef])
no resolution for "Default" MacroNS DefId(0:0 ~ core[bdef])
no resolution for "Default" TypeNS DefId(0:0 ~ core[bdef])
no resolution for "Default" ValueNS DefId(0:0 ~ core[bdef])
error: unresolved link to `Default`
    |
    |
69  | / /// The `!` type, also called "never".
70  | | ///
71  | | /// `!` represents the type of computations which never resolve to any value at all. For example,
72  | | /// the [`exit`] function `fn exit(code: i32) -> !` exits the process without ever returning, and
...   |
272 | | /// [`default()`]: Default::default
    | |___^
    |
    = note: the link appears in this line:
            
            
            On the other hand, one trait which would not be appropriate to implement is [`Default`]:
    = note: no item named `Default` in scope
    = note: no item named `Default` in scope
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

no resolution for "Default::default" MacroNS DefId(0:0 ~ core[bdef])
no resolution for "Default::default" TypeNS DefId(0:0 ~ core[bdef])
no resolution for "Default" TypeNS DefId(0:0 ~ core[bdef])
no resolution for "Default::default" ValueNS DefId(0:0 ~ core[bdef])
no resolution for "Default" TypeNS DefId(0:0 ~ core[bdef])
no resolution for "Default" TypeNS DefId(0:0 ~ core[bdef])
no resolution for "Default" ValueNS DefId(0:0 ~ core[bdef])
no resolution for "Default" MacroNS DefId(0:0 ~ core[bdef])
error: unresolved link to `Default::default`
    |
    |
69  | / /// The `!` type, also called "never".
70  | | ///
71  | | /// `!` represents the type of computations which never resolve to any value at all. For example,
72  | | /// the [`exit`] function `fn exit(code: i32) -> !` exits the process without ever returning, and
...   |
272 | | /// [`default()`]: Default::default
    | |___^
    |
    = note: the link appears in this line:
            
            
            [`default()`]: Default::default
    = note: no item named `Default` in scope


no resolution for "ptr" MacroNS DefId(0:0 ~ core[bdef])
no resolution for "ptr" TypeNS DefId(0:0 ~ core[bdef])
no resolution for "ptr" ValueNS DefId(0:0 ~ core[bdef])
error: unresolved link to `ptr`
    |
    |
448 | / /// Raw, unsafe pointers, `*const T`, and `*mut T`.
449 | | ///
450 | | /// *[See also the `std::ptr` module](ptr).*
...   |
...   |
554 | | /// [`drop`]: mem::drop
555 | | /// [`write`]: ptr::write
    |
    = note: the link appears in this line:
            
            
            *[See also the `std::ptr` module](ptr).*
    = note: no item named `ptr` in scope
    = note: no item named `ptr` in scope
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

no resolution for "ptr::null" MacroNS DefId(0:0 ~ core[bdef])
no resolution for "ptr::null" TypeNS DefId(0:0 ~ core[bdef])
no resolution for "ptr" TypeNS DefId(0:0 ~ core[bdef])
no resolution for "ptr::null" ValueNS DefId(0:0 ~ core[bdef])
no resolution for "ptr" TypeNS DefId(0:0 ~ core[bdef])
no resolution for "ptr" TypeNS DefId(0:0 ~ core[bdef])
no resolution for "ptr" ValueNS DefId(0:0 ~ core[bdef])
no resolution for "ptr" MacroNS DefId(0:0 ~ core[bdef])
error: unresolved link to `ptr::null`
    |
    |
448 | / /// Raw, unsafe pointers, `*const T`, and `*mut T`.
449 | | ///
450 | | /// *[See also the `std::ptr` module](ptr).*
...   |
...   |
554 | | /// [`drop`]: mem::drop
555 | | /// [`write`]: ptr::write
    |
    = note: the link appears in this line:
            
            
            [`null`]: ptr::null
    = note: no item named `ptr` in scope


no resolution for "ptr::write" MacroNS DefId(0:0 ~ core[bdef])
no resolution for "ptr::write" TypeNS DefId(0:0 ~ core[bdef])
no resolution for "ptr" TypeNS DefId(0:0 ~ core[bdef])
no resolution for "ptr::write" ValueNS DefId(0:0 ~ core[bdef])
no resolution for "ptr" TypeNS DefId(0:0 ~ core[bdef])
no resolution for "ptr" TypeNS DefId(0:0 ~ core[bdef])
no resolution for "ptr" ValueNS DefId(0:0 ~ core[bdef])
no resolution for "ptr" MacroNS DefId(0:0 ~ core[bdef])
error: unresolved link to `ptr::write`
    |
    |
448 | / /// Raw, unsafe pointers, `*const T`, and `*mut T`.
449 | | ///
450 | | /// *[See also the `std::ptr` module](ptr).*
...   |
...   |
554 | | /// [`drop`]: mem::drop
555 | | /// [`write`]: ptr::write
    |
    = note: the link appears in this line:
            
            
            [`write`]: ptr::write
    = note: no item named `ptr` in scope


no resolution for "ptr::null_mut" MacroNS DefId(0:0 ~ core[bdef])
no resolution for "ptr::null_mut" TypeNS DefId(0:0 ~ core[bdef])
no resolution for "ptr" TypeNS DefId(0:0 ~ core[bdef])
no resolution for "ptr::null_mut" ValueNS DefId(0:0 ~ core[bdef])
no resolution for "ptr" TypeNS DefId(0:0 ~ core[bdef])
no resolution for "ptr" TypeNS DefId(0:0 ~ core[bdef])
no resolution for "ptr" ValueNS DefId(0:0 ~ core[bdef])
no resolution for "ptr" MacroNS DefId(0:0 ~ core[bdef])
error: unresolved link to `ptr::null_mut`
    |
    |
448 | / /// Raw, unsafe pointers, `*const T`, and `*mut T`.
449 | | ///
450 | | /// *[See also the `std::ptr` module](ptr).*
...   |
...   |
554 | | /// [`drop`]: mem::drop
555 | | /// [`write`]: ptr::write
    |
    = note: the link appears in this line:
            
            
            [`null_mut`]: ptr::null_mut
    = note: no item named `ptr` in scope


no resolution for "pointer::is_null" MacroNS DefId(0:0 ~ core[bdef])
no resolution for "pointer::is_null" TypeNS DefId(0:0 ~ core[bdef])
no resolution for "pointer::is_null" ValueNS DefId(0:0 ~ core[bdef])
no resolution for "pointer::offset" MacroNS DefId(0:0 ~ core[bdef])
no resolution for "pointer::offset" TypeNS DefId(0:0 ~ core[bdef])
no resolution for "pointer::offset" ValueNS DefId(0:0 ~ core[bdef])
no resolution for "mem::drop" MacroNS DefId(0:0 ~ core[bdef])
no resolution for "mem::drop" TypeNS DefId(0:0 ~ core[bdef])
no resolution for "mem" TypeNS DefId(0:0 ~ core[bdef])
no resolution for "mem::drop" ValueNS DefId(0:0 ~ core[bdef])
no resolution for "mem" TypeNS DefId(0:0 ~ core[bdef])
no resolution for "mem" TypeNS DefId(0:0 ~ core[bdef])
no resolution for "mem" ValueNS DefId(0:0 ~ core[bdef])
no resolution for "mem" MacroNS DefId(0:0 ~ core[bdef])
error: unresolved link to `mem::drop`
    |
    |
448 | / /// Raw, unsafe pointers, `*const T`, and `*mut T`.
449 | | ///
450 | | /// *[See also the `std::ptr` module](ptr).*
...   |
...   |
554 | | /// [`drop`]: mem::drop
555 | | /// [`write`]: ptr::write
    |
    = note: the link appears in this line:
            
            
            [`drop`]: mem::drop
    = note: no item named `mem` in scope


no resolution for "ptr::addr_of" MacroNS DefId(0:0 ~ core[bdef])
no resolution for "ptr::addr_of" TypeNS DefId(0:0 ~ core[bdef])
no resolution for "ptr" TypeNS DefId(0:0 ~ core[bdef])
no resolution for "ptr::addr_of" ValueNS DefId(0:0 ~ core[bdef])
no resolution for "ptr" TypeNS DefId(0:0 ~ core[bdef])
no resolution for "ptr" TypeNS DefId(0:0 ~ core[bdef])
no resolution for "ptr" ValueNS DefId(0:0 ~ core[bdef])
no resolution for "ptr" MacroNS DefId(0:0 ~ core[bdef])
error: unresolved link to `ptr::addr_of`
    |
    |
448 | / /// Raw, unsafe pointers, `*const T`, and `*mut T`.
449 | | ///
450 | | /// *[See also the `std::ptr` module](ptr).*
...   |
...   |
554 | | /// [`drop`]: mem::drop
555 | | /// [`write`]: ptr::write
    |
    = note: the link appears in this line:
            
            
            [`ptr::addr_of!`] (for `*const T`) and [`ptr::addr_of_mut!`] (for `*mut T`).
    = note: no item named `ptr` in scope


no resolution for "ptr::addr_of_mut" MacroNS DefId(0:0 ~ core[bdef])
no resolution for "ptr::addr_of_mut" TypeNS DefId(0:0 ~ core[bdef])
