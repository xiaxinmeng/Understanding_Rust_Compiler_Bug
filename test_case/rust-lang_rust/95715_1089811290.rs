plain
[2022-04-06T03:47:01Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp0xv7wM#deeply-nested-async:0.1.0" "--" "--skip-this-rustc"
Running deeply-nested-async: Debug + [Full]
[2022-04-06T03:47:02Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-04-06T03:47:02Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-04-06T03:47:02Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpNU1FHC#deeply-nested-async:0.1.0" "--" "--wrap-rustc-with" "eprintln"
[2022-04-06T03:47:02Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-04-06T03:47:02Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-04-06T03:47:02Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpbMOpxy#deeply-nested-async:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln"
5 benchmarks remaining
---
3 benchmarks remaining
Preparing ripgrep
[2022-04-06T03:47:53Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2022-04-06T03:47:53Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2022-04-06T03:47:53Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpaoKVDB#ripgrep:0.8.1" "--release" "--" "--skip-this-rustc"
[2022-04-06T03:47:53Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJBjL2m#ripgrep:0.8.1" "--" "--skip-this-rustc"
[2022-04-06T03:48:19Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-04-06T03:48:20Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-04-06T03:48:20Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-04-06T03:48:20Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpRreUYv#ripgrep:0.8.1" "--" "--wrap-rustc-with" "eprintln"
[2022-04-06T03:48:24Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-04-06T03:48:24Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-04-06T03:48:24Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-04-06T03:48:24Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpYtRprF#ripgrep:0.8.1" "--release" "--" "--wrap-rustc-with" "eprintln"
Preparing serde
[2022-04-06T03:48:42Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2022-04-06T03:48:42Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2022-04-06T03:48:42Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2022-04-06T03:48:42Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "serde/Cargo.toml" "-p" "file:///tmp/.tmpRPPaIH/serde#1.0.37" "--" "--skip-this-rustc"
[2022-04-06T03:48:42Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "serde/Cargo.toml" "-p" "file:///tmp/.tmp58331L/serde#1.0.37" "--release" "--" "--skip-this-rustc"
[2022-04-06T03:48:42Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-04-06T03:48:42Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-04-06T03:48:42Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "serde/Cargo.toml" "-p" "file:///tmp/.tmp5lM2Bg/serde#1.0.37" "--" "--wrap-rustc-with" "eprintln"
Running serde: Opt + [Full]
Running serde: Opt + [Full]
[2022-04-06T03:48:46Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-04-06T03:48:46Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-04-06T03:48:46Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "serde/Cargo.toml" "-p" "file:///tmp/.tmpdWqwWo/serde#1.0.37" "--release" "--" "--wrap-rustc-with" "eprintln"
Preparing syn
[2022-04-06T03:48:49Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2022-04-06T03:48:49Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2022-04-06T03:48:49Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpRpkGjR#syn:0.11.11" "--release" "--" "--skip-this-rustc"
[2022-04-06T03:48:49Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpRpkGjR#syn:0.11.11" "--release" "--" "--skip-this-rustc"
[2022-04-06T03:48:49Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQGzMhJ#syn:0.11.11" "--" "--skip-this-rustc"
[2022-04-06T03:48:53Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-04-06T03:48:53Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-04-06T03:48:53Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-04-06T03:48:53Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmph3zfQY#syn:0.11.11" "--" "--wrap-rustc-with" "eprintln"
[2022-04-06T03:48:55Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-04-06T03:48:55Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-04-06T03:48:55Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-04-06T03:48:55Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpyyveyF#syn:0.11.11" "--release" "--" "--wrap-rustc-with" "eprintln"
+ /rustroot/bin/llvm-profdata merge -o /tmp/llvm-pgo.profdata ./build/x86_64-unknown-linux-gnu/llvm/build/profiles
+ rm -r ./build/x86_64-unknown-linux-gnu/llvm ./build/x86_64-unknown-linux-gnu/lld
+ python3 ../x.py build --target=x86_64-unknown-linux-gnu --host=x86_64-unknown-linux-gnu --stage 2 library/std --rust-profile-generate=/tmp/rustc-pgo
warning: sqlite3 not available in python, skipping build directory lock
---
[2022-04-06T04:02:14Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-04-06T04:02:15Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2022-04-06T04:02:15Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpRzrgho#cargo:0.29.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "eprintln"
[2022-04-06T04:02:27Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2022-04-06T04:02:27Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpRzrgho#cargo:0.29.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpRzrgho/incremental-state"
[2022-04-06T04:02:42Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrUnchanged), patch=None
[2022-04-06T04:02:42Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpRzrgho#cargo:0.29.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpRzrgho/incremental-state"
[2022-04-06T04:02:45Z DEBUG collector::execute] applying println to "/tmp/.tmpRzrgho"
[2022-04-06T04:02:45Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-04-06T04:02:45Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-04-06T04:02:45Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpRzrgho#cargo:0.29.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpRzrgho/incremental-state"
[2022-04-06T04:02:52Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-04-06T04:02:53Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-04-06T04:02:53Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDk1E2T#cargo:0.29.0" "--lib" "--" "--wrap-rustc-with" "eprintln"
[2022-04-06T04:03:19Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
---
[2022-04-06T04:04:10Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-04-06T04:04:11Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-04-06T04:04:11Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKBPtRD#cargo:0.29.0" "--release" "--lib" "--" "--wrap-rustc-with" "eprintln"
[2022-04-06T04:04:44Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2022-04-06T04:04:44Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKBPtRD#cargo:0.29.0" "--release" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpKBPtRD/incremental-state"
[2022-04-06T04:05:19Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrUnchanged), patch=None
[2022-04-06T04:05:19Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKBPtRD#cargo:0.29.0" "--release" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpKBPtRD/incremental-state"
[2022-04-06T04:05:25Z DEBUG collector::execute] applying println to "/tmp/.tmpKBPtRD"
[2022-04-06T04:05:25Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-04-06T04:05:25Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-04-06T04:05:25Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKBPtRD#cargo:0.29.0" "--release" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpKBPtRD/incremental-state"
Preparing ctfe-stress-4
[2022-04-06T04:05:43Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=None, patch=None
[2022-04-06T04:05:43Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2022-04-06T04:05:43Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
---
[2022-04-06T04:07:12Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-04-06T04:07:12Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-04-06T04:07:12Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpeuDAVm#ctfe-stress-4:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln"
[2022-04-06T04:07:31Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2022-04-06T04:07:31Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpeuDAVm#ctfe-stress-4:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpeuDAVm/incremental-state"
[2022-04-06T04:07:53Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrUnchanged), patch=None
[2022-04-06T04:07:53Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpeuDAVm#ctfe-stress-4:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpeuDAVm/incremental-state"
Preparing externs
[2022-04-06T04:07:54Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=None, patch=None
[2022-04-06T04:07:54Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2022-04-06T04:07:54Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2022-04-06T04:07:54Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2022-04-06T04:07:54Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpThtvuy#externs:0.1.0" "--profile" "check" "--" "--skip-this-rustc"
[2022-04-06T04:07:54Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmptsbxv1#externs:0.1.0" "--" "--skip-this-rustc"
[2022-04-06T04:07:54Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQb97lj#externs:0.1.0" "--release" "--" "--skip-this-rustc"
[2022-04-06T04:07:54Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-04-06T04:07:54Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2022-04-06T04:07:54Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmph7ZOgK#externs:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln"
[2022-04-06T04:07:55Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2022-04-06T04:07:55Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2022-04-06T04:07:55Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmph7ZOgK#externs:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmph7ZOgK/incremental-state"
[2022-04-06T04:07:56Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrUnchanged), patch=None
[2022-04-06T04:07:56Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmph7ZOgK#externs:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmph7ZOgK/incremental-state"
[2022-04-06T04:07:56Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-04-06T04:07:56Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-04-06T04:07:56Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpAjAyJg#externs:0.1.0" "--" "--wrap-rustc-with" "eprintln"
[2022-04-06T04:07:57Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-04-06T04:07:57Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-04-06T04:07:57Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpAjAyJg#externs:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpAjAyJg/incremental-state"
[2022-04-06T04:07:58Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2022-04-06T04:07:58Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpAjAyJg#externs:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpAjAyJg/incremental-state"
[2022-04-06T04:07:59Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-04-06T04:07:59Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-04-06T04:07:59Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOs4SX6#externs:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln"
[2022-04-06T04:08:00Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2022-04-06T04:08:00Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2022-04-06T04:08:00Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOs4SX6#externs:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpOs4SX6/incremental-state"
[2022-04-06T04:08:01Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrUnchanged), patch=None
[2022-04-06T04:08:01Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOs4SX6#externs:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpOs4SX6/incremental-state"
Preparing inflate
[2022-04-06T04:08:02Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=None, patch=None
[2022-04-06T04:08:02Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2022-04-06T04:08:02Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
---
[2022-04-06T04:08:10Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-04-06T04:08:10Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-04-06T04:08:10Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzeIbIf#inflate:0.1.0" "--" "--wrap-rustc-with" "eprintln"
[2022-04-06T04:08:13Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-04-06T04:08:13Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzeIbIf#inflate:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpzeIbIf/incremental-state"
[2022-04-06T04:08:17Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2022-04-06T04:08:17Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzeIbIf#inflate:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpzeIbIf/incremental-state"
[2022-04-06T04:08:17Z DEBUG collector::execute] applying println to "/tmp/.tmpzeIbIf"
[2022-04-06T04:08:17Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-04-06T04:08:17Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-04-06T04:08:17Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzeIbIf#inflate:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpzeIbIf/incremental-state"
[2022-04-06T04:08:20Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-04-06T04:08:20Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-04-06T04:08:20Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp4uWXfD#inflate:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln"
[2022-04-06T04:08:26Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2022-04-06T04:08:26Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2022-04-06T04:08:26Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp4uWXfD#inflate:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmp4uWXfD/incremental-state"
[2022-04-06T04:08:31Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrUnchanged), patch=None
[2022-04-06T04:08:31Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp4uWXfD#inflate:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmp4uWXfD/incremental-state"
[2022-04-06T04:08:32Z DEBUG collector::execute] applying println to "/tmp/.tmp4uWXfD"
[2022-04-06T04:08:32Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-04-06T04:08:32Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-04-06T04:08:32Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp4uWXfD#inflate:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmp4uWXfD/incremental-state"
Preparing match-stress-enum
[2022-04-06T04:08:37Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=None, patch=None
[2022-04-06T04:08:37Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2022-04-06T04:08:37Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
---
[2022-04-06T04:08:38Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-04-06T04:08:38Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2022-04-06T04:08:38Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpdN5uaW#match-stress-enum:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln"
[2022-04-06T04:08:39Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2022-04-06T04:08:39Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpdN5uaW#match-stress-enum:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpdN5uaW/incremental-state"
[2022-04-06T04:08:41Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrUnchanged), patch=None
[2022-04-06T04:08:41Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpdN5uaW#match-stress-enum:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpdN5uaW/incremental-state"
[2022-04-06T04:08:42Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-04-06T04:08:42Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-04-06T04:08:42Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpBjqhGM#match-stress-enum:0.1.0" "--" "--wrap-rustc-with" "eprintln"
[2022-04-06T04:08:44Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-04-06T04:08:44Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-04-06T04:08:44Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpBjqhGM#match-stress-enum:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpBjqhGM/incremental-state"
[2022-04-06T04:08:46Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2022-04-06T04:08:46Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpBjqhGM#match-stress-enum:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpBjqhGM/incremental-state"
[2022-04-06T04:08:46Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-04-06T04:08:46Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-04-06T04:08:46Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpA5KAH4#match-stress-enum:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln"
[2022-04-06T04:08:48Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
---
[2022-04-06T04:08:52Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-04-06T04:08:52Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2022-04-06T04:08:52Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpSWqFGr#token-stream-stress:0.0.0" "--profile" "check" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln"
[2022-04-06T04:08:52Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2022-04-06T04:08:52Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpSWqFGr#token-stream-stress:0.0.0" "--profile" "check" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpSWqFGr/incremental-state"
[2022-04-06T04:08:52Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrUnchanged), patch=None
[2022-04-06T04:08:52Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpSWqFGr#token-stream-stress:0.0.0" "--profile" "check" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpSWqFGr/incremental-state"
[2022-04-06T04:08:53Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-04-06T04:08:53Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-04-06T04:08:53Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpgaJUcm#token-stream-stress:0.0.0" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln"
[2022-04-06T04:08:53Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-04-06T04:08:53Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-04-06T04:08:53Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpgaJUcm#token-stream-stress:0.0.0" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpgaJUcm/incremental-state"
[2022-04-06T04:08:53Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2022-04-06T04:08:53Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpgaJUcm#token-stream-stress:0.0.0" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpgaJUcm/incremental-state"
[2022-04-06T04:08:54Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-04-06T04:08:54Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-04-06T04:08:54Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-04-06T04:08:54Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpuvGccd#token-stream-stress:0.0.0" "--release" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln"
[2022-04-06T04:08:54Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2022-04-06T04:08:54Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpuvGccd#token-stream-stress:0.0.0" "--release" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpuvGccd/incremental-state"
[2022-04-06T04:08:55Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrUnchanged), patch=None
[2022-04-06T04:08:55Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpuvGccd#token-stream-stress:0.0.0" "--release" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpuvGccd/incremental-state"
+ ./build/x86_64-unknown-linux-gnu/llvm/bin/llvm-profdata merge -o /tmp/rustc-pgo.profdata /tmp/rustc-pgo
+ rm -r ./build/x86_64-unknown-linux-gnu/llvm ./build/x86_64-unknown-linux-gnu/lld
+ python3 ../x.py dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu --include-default-paths build-manifest --rust-profile-use=/tmp/rustc-pgo.profdata --llvm-profile-use=/tmp/llvm-pgo.profdata
warning: sqlite3 not available in python, skipping build directory lock
---
 Documenting rustfmt-nightly v1.4.38 (/checkout/src/tools/rustfmt)
error[E0631]: type mismatch in function arguments
    --> src/tools/rustfmt/src/closures.rs:97:62
     |
97   |             if let Some(expr) = block.stmts.first().and_then(stmt_expr) {
     |                                                     -------- ^^^^^^^^^ expected signature of `fn(&P<rustc_ast::Stmt>) -> _`
     |                                                     required by a bound introduced by this call
     |
    ::: src/tools/rustfmt/src/utils.rs:322:1
     |
     |
322  | pub(crate) fn stmt_expr(stmt: &ast::Stmt) -> Option<&ast::Expr> {
     | --------------------------------------------------------------- found signature of `for<'r> fn(&'r rustc_ast::Stmt) -> _`
note: required by a bound in `std::option::Option::<T>::and_then`

error[E0308]: mismatched types
   --> src/tools/rustfmt/src/closures.rs:150:21
   --> src/tools/rustfmt/src/closures.rs:150:21
    |
150 |           stmts: vec![ast::Stmt {
    |  _____________________^
151 | |             id: ast::NodeId::root(),
152 | |             kind: ast::StmtKind::Expr(ptr::P(body.clone())),
153 | |             span: body.span,
154 | |         }],
    | |_________^ expected struct `P`, found struct `rustc_ast::Stmt`
    |
    = note: expected struct `P<rustc_ast::Stmt>`
               found struct `rustc_ast::Stmt`

error[E0271]: type mismatch resolving `<std::slice::Iter<'_, P<rustc_ast::Stmt>> as Iterator>::Item == &rustc_ast::Stmt`
    |
    |
898 |         self.walk_stmts(&Stmt::from_ast_nodes(b.stmts.iter()), false)
    |                          ^^^^^^^^^^^^^^^^^^^^ expected struct `rustc_ast::Stmt`, found struct `P`
    = note: expected reference `&rustc_ast::Stmt`
    = note: expected reference `&rustc_ast::Stmt`
               found reference `&P<rustc_ast::Stmt>`
note: required by a bound in `stmt::Stmt::<'a>::from_ast_nodes`
    |
    |
40  |     pub(crate) fn from_ast_nodes<I>(iter: I) -> Vec<Self>
    |                   -------------- required by a bound in this
41  |     where
42  |         I: Iterator<Item = &'a ast::Stmt>,
    |                     ^^^^^^^^^^^^^^^^^^^^ required by this bound in `stmt::Stmt::<'a>::from_ast_nodes`
error[E0631]: type mismatch in function arguments
    --> src/tools/rustfmt/src/closures.rs:97:62
     |
     |
97   |             if let Some(expr) = block.stmts.first().and_then(stmt_expr) {
     |                                                     -------- ^^^^^^^^^ expected signature of `fn(&rustc_ast::ptr::P<rustc_ast::Stmt>) -> _`
     |                                                     required by a bound introduced by this call
     |
    ::: src/tools/rustfmt/src/utils.rs:322:1
     |
     |
322  | pub(crate) fn stmt_expr(stmt: &ast::Stmt) -> Option<&ast::Expr> {
     | --------------------------------------------------------------- found signature of `for<'r> fn(&'r rustc_ast::Stmt) -> _`
note: required by a bound in `std::option::Option::<T>::and_then`

error[E0308]: mismatched types
   --> src/tools/rustfmt/src/closures.rs:150:21
   --> src/tools/rustfmt/src/closures.rs:150:21
    |
150 |           stmts: vec![ast::Stmt {
    |  _____________________^
151 | |             id: ast::NodeId::root(),
152 | |             kind: ast::StmtKind::Expr(ptr::P(body.clone())),
153 | |             span: body.span,
154 | |         }],
    | |_________^ expected struct `rustc_ast::ptr::P`, found struct `rustc_ast::Stmt`
    |
    = note: expected struct `rustc_ast::ptr::P<rustc_ast::Stmt>`
               found struct `rustc_ast::Stmt`
Some errors have detailed explanations: E0271, E0308, E0631.
For more information about an error, try `rustc --explain E0271`.
[RUSTC-TIMING] rustfmt_nightly test:false 2.335
error: could not compile `rustfmt-nightly` due to 3 previous errors
error: could not compile `rustfmt-nightly` due to 3 previous errors
warning: build failed, waiting for other jobs to finish...
error[E0271]: type mismatch resolving `<std::slice::Iter<'_, rustc_ast::ptr::P<rustc_ast::Stmt>> as std::iter::Iterator>::Item == &rustc_ast::Stmt`
    |
    |
898 |         self.walk_stmts(&Stmt::from_ast_nodes(b.stmts.iter()), false)
    |                          ^^^^^^^^^^^^^^^^^^^^ expected struct `rustc_ast::Stmt`, found struct `rustc_ast::ptr::P`
    = note: expected reference `&rustc_ast::Stmt`
    = note: expected reference `&rustc_ast::Stmt`
               found reference `&rustc_ast::ptr::P<rustc_ast::Stmt>`
note: required by a bound in `stmt::Stmt::<'a>::from_ast_nodes`
    |
    |
40  |     pub(crate) fn from_ast_nodes<I>(iter: I) -> Vec<Self>
    |                   -------------- required by a bound in this
41  |     where
42  |         I: Iterator<Item = &'a ast::Stmt>,
    |                     ^^^^^^^^^^^^^^^^^^^^ required by this bound in `stmt::Stmt::<'a>::from_ast_nodes`
error: build failed
Build completed unsuccessfully in 0:23:32
