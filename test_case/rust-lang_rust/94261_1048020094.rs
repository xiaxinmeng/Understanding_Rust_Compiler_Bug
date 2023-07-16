plain
Preparing cargo
[2022-02-22T16:33:27Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=None, patch=None
[2022-02-22T16:33:27Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2022-02-22T16:33:27Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2022-02-22T16:33:27Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpN4aZeP#cargo:0.29.0" "--release" "--lib" "--" "--skip-this-rustc"
[2022-02-22T16:33:27Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpIJTKaC#cargo:0.29.0" "--profile" "check" "--lib" "--" "--skip-this-rustc"
[2022-02-22T16:33:27Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpPN44md#cargo:0.29.0" "--lib" "--" "--skip-this-rustc"
[2022-02-22T16:35:10Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-02-22T16:35:10Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2022-02-22T16:35:10Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpBM1vHp#cargo:0.29.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "eprintln"
[2022-02-22T16:35:25Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
---
[2022-02-22T16:37:31Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-02-22T16:37:31Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-02-22T16:37:31Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpXID0ab#cargo:0.29.0" "--release" "--lib" "--" "--wrap-rustc-with" "eprintln"
[2022-02-22T16:38:12Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2022-02-22T16:38:12Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpXID0ab#cargo:0.29.0" "--release" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpXID0ab/incremental-state"
[2022-02-22T16:38:57Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrUnchanged), patch=None
[2022-02-22T16:38:57Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpXID0ab#cargo:0.29.0" "--release" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpXID0ab/incremental-state"
[2022-02-22T16:39:04Z DEBUG collector::execute] applying println to "/tmp/.tmpXID0ab"
[2022-02-22T16:39:04Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-02-22T16:39:04Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-02-22T16:39:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpXID0ab#cargo:0.29.0" "--release" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpXID0ab/incremental-state"
Preparing ctfe-stress-4
[2022-02-22T16:39:26Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2022-02-22T16:39:26Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2022-02-22T16:39:26Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=None, patch=None
---
[2022-02-22T16:39:27Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-02-22T16:39:27Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2022-02-22T16:39:27Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp5lTOTV#ctfe-stress-4:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln"
[2022-02-22T16:39:51Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2022-02-22T16:39:51Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp5lTOTV#ctfe-stress-4:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmp5lTOTV/incremental-state"
[2022-02-22T16:40:22Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrUnchanged), patch=None
[2022-02-22T16:40:22Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp5lTOTV#ctfe-stress-4:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmp5lTOTV/incremental-state"
[2022-02-22T16:40:23Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-02-22T16:40:23Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-02-22T16:40:23Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpu88Tzc#ctfe-stress-4:0.1.0" "--" "--wrap-rustc-with" "eprintln"
[2022-02-22T16:40:48Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
---
[2022-02-22T16:41:17Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-02-22T16:41:17Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-02-22T16:41:17Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpMPjCuo#ctfe-stress-4:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln"
[2022-02-22T16:41:40Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2022-02-22T16:41:41Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpMPjCuo#ctfe-stress-4:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpMPjCuo/incremental-state"
[2022-02-22T16:42:06Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrUnchanged), patch=None
[2022-02-22T16:42:06Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpMPjCuo#ctfe-stress-4:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpMPjCuo/incremental-state"
Preparing externs
[2022-02-22T16:42:07Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2022-02-22T16:42:07Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=None, patch=None
[2022-02-22T16:42:07Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2022-02-22T16:42:07Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2022-02-22T16:42:07Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFytfCQ#externs:0.1.0" "--" "--skip-this-rustc"
[2022-02-22T16:42:07Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpvuYMyl#externs:0.1.0" "--release" "--" "--skip-this-rustc"
[2022-02-22T16:42:07Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpdMIn2H#externs:0.1.0" "--profile" "check" "--" "--skip-this-rustc"
[2022-02-22T16:42:08Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-02-22T16:42:08Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2022-02-22T16:42:08Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp0mQJD4#externs:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln"
[2022-02-22T16:42:08Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
---
[2022-02-22T16:42:10Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-02-22T16:42:10Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-02-22T16:42:10Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpMqbeol#externs:0.1.0" "--" "--wrap-rustc-with" "eprintln"
[2022-02-22T16:42:11Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-02-22T16:42:11Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpMqbeol#externs:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpMqbeol/incremental-state"
[2022-02-22T16:42:13Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2022-02-22T16:42:13Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpMqbeol#externs:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpMqbeol/incremental-state"
[2022-02-22T16:42:14Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-02-22T16:42:14Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-02-22T16:42:14Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKr1L0o#externs:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln"
[2022-02-22T16:42:15Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2022-02-22T16:42:15Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2022-02-22T16:42:15Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKr1L0o#externs:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpKr1L0o/incremental-state"
[2022-02-22T16:42:16Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrUnchanged), patch=None
[2022-02-22T16:42:16Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKr1L0o#externs:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpKr1L0o/incremental-state"
Preparing inflate
[2022-02-22T16:42:17Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=None, patch=None
[2022-02-22T16:42:17Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2022-02-22T16:42:17Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
---
[2022-02-22T16:42:18Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-02-22T16:42:18Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2022-02-22T16:42:18Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmptvpjNl#inflate:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln"
[2022-02-22T16:42:20Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2022-02-22T16:42:20Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmptvpjNl#inflate:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmptvpjNl/incremental-state"
[2022-02-22T16:42:23Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrUnchanged), patch=None
[2022-02-22T16:42:23Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmptvpjNl#inflate:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmptvpjNl/incremental-state"
[2022-02-22T16:42:23Z DEBUG collector::execute] applying println to "/tmp/.tmptvpjNl"
[2022-02-22T16:42:23Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-02-22T16:42:23Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-02-22T16:42:23Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmptvpjNl#inflate:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmptvpjNl/incremental-state"
[2022-02-22T16:42:26Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-02-22T16:42:26Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-02-22T16:42:26Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpNG87lj#inflate:0.1.0" "--" "--wrap-rustc-with" "eprintln"
[2022-02-22T16:42:30Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
---
[2022-02-22T16:42:59Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-02-22T16:42:59Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2022-02-22T16:42:59Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwxeNN6#match-stress-enum:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln"
[2022-02-22T16:43:01Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2022-02-22T16:43:01Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwxeNN6#match-stress-enum:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpwxeNN6/incremental-state"
[2022-02-22T16:43:04Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrUnchanged), patch=None
[2022-02-22T16:43:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwxeNN6#match-stress-enum:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpwxeNN6/incremental-state"
[2022-02-22T16:43:04Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-02-22T16:43:04Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-02-22T16:43:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOGkChQ#match-stress-enum:0.1.0" "--" "--wrap-rustc-with" "eprintln"
[2022-02-22T16:43:06Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-02-22T16:43:06Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-02-22T16:43:06Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOGkChQ#match-stress-enum:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpOGkChQ/incremental-state"
[2022-02-22T16:43:09Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2022-02-22T16:43:09Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOGkChQ#match-stress-enum:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpOGkChQ/incremental-state"
[2022-02-22T16:43:09Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-02-22T16:43:09Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-02-22T16:43:09Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpmPOfqw#match-stress-enum:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln"
[2022-02-22T16:43:12Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2022-02-22T16:43:12Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2022-02-22T16:43:12Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpmPOfqw#match-stress-enum:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpmPOfqw/incremental-state"
[2022-02-22T16:43:14Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrUnchanged), patch=None
[2022-02-22T16:43:14Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpmPOfqw#match-stress-enum:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpmPOfqw/incremental-state"
Preparing token-stream-stress
[2022-02-22T16:43:15Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=None, patch=None
[2022-02-22T16:43:15Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2022-02-22T16:43:15Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
---
[2022-02-22T16:43:16Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-02-22T16:43:16Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2022-02-22T16:43:16Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpvUzr4B#token-stream-stress:0.0.0" "--profile" "check" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln"
[2022-02-22T16:43:17Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2022-02-22T16:43:17Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpvUzr4B#token-stream-stress:0.0.0" "--profile" "check" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpvUzr4B/incremental-state"
[2022-02-22T16:43:17Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrUnchanged), patch=None
[2022-02-22T16:43:17Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpvUzr4B#token-stream-stress:0.0.0" "--profile" "check" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpvUzr4B/incremental-state"
[2022-02-22T16:43:17Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-02-22T16:43:17Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-02-22T16:43:17Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJEiPKm#token-stream-stress:0.0.0" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln"
[2022-02-22T16:43:17Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-02-22T16:43:17Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-02-22T16:43:17Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJEiPKm#token-stream-stress:0.0.0" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpJEiPKm/incremental-state"
[2022-02-22T16:43:18Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2022-02-22T16:43:18Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJEiPKm#token-stream-stress:0.0.0" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpJEiPKm/incremental-state"
[2022-02-22T16:43:18Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-02-22T16:43:18Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-02-22T16:43:18Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-02-22T16:43:18Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpsnkkxA#token-stream-stress:0.0.0" "--release" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln"
[2022-02-22T16:43:19Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2022-02-22T16:43:19Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpsnkkxA#token-stream-stress:0.0.0" "--release" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpsnkkxA/incremental-state"
[2022-02-22T16:43:19Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrUnchanged), patch=None
[2022-02-22T16:43:19Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpsnkkxA#token-stream-stress:0.0.0" "--release" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpsnkkxA/incremental-state"
+ ./build/x86_64-unknown-linux-gnu/llvm/bin/llvm-profdata merge -o /tmp/rustc-pgo.profdata /tmp/rustc-pgo
+ rm -r ./build/x86_64-unknown-linux-gnu/llvm ./build/x86_64-unknown-linux-gnu/lld
+ python3 ../x.py dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu --include-default-paths build-manifest --rust-profile-use=/tmp/rustc-pgo.profdata --llvm-profile-use=/tmp/llvm-pgo.profdata
warning: sqlite3 not available in python, skipping build directory lock
---
[RUSTC-TIMING] rustc_query_impl test:false 5.383
error: unresolved link to `slice_type_metadata`
   --> compiler/rustc_codegen_llvm/src/debuginfo/metadata.rs:120:57
    |
120 | /// For slices (that is, "arrays" of unknown size) use [slice_type_metadata].
    |                                                         ^^^^^^^^^^^^^^^^^^^ no item named `slice_type_metadata` in scope
    |
    = note: `-D rustdoc::broken-intra-doc-links` implied by `-D warnings`
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: could not document `rustc_codegen_llvm`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name rustc_codegen_llvm compiler/rustc_codegen_llvm/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/doc --error-format=json --json=diagnostic-rendered-ansi,future-incompat -C metadata=2933c49ab767c206 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-8db9ec982a978a4d.rmeta --extern cstr=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps/libcstr-547a61b22dae33cd.so --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/liblibc-861edb7a3ad1a878.rmeta --extern libloading=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/liblibloading-0febe9c65d498348.rmeta --extern measureme=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libmeasureme-49adc4ca68a82697.rmeta --extern rustc_demangle=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_demangle-1d591788bbefd190.rmeta --extern rustc_arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_arena-b199ac8ed776ac79.rmeta --extern rustc_ast=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_ast-70a8c94b70c0e624.rmeta --extern rustc_attr=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_attr-fe61a03873f5315d.rmeta --extern rustc_codegen_ssa=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_codegen_ssa-6d0be7afd641a7f8.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-e00cab6ece707050.rmeta --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-554587abf5ad9abf.rmeta --extern rustc_fs_util=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_fs_util-e31927bb8762da62.rmeta --extern rustc_hir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_hir-61ba8f6a9e0172b7.rmeta --extern rustc_index=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_index-2d63746787217bce.rmeta --extern rustc_llvm=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_llvm-113b2ad6817f666a.rmeta --extern rustc_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps/librustc_macros-0da9b871c4e20306.so --extern rustc_metadata=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_metadata-2cd680a1bab76a03.rmeta --extern rustc_middle=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_middle-024e5835456e966e.rmeta --extern rustc_query_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_query_system-97353130c36a3de9.rmeta --extern rustc_serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_serialize-34d93fb476367033.rmeta --extern rustc_session=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_session-043ab862504b7030.rmeta --extern rustc_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_span-555a25f64a791389.rmeta --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-19f0bf54390e06f0.rmeta --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-1dd1cc997c3ff990.rmeta --extern tracing=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libtracing-46fbcf379c7d7b7b.rmeta --extern-html-root-url 'bitflags=https://docs.rs/bitflags/1.2.1/' --extern-html-root-url 'cstr=https://docs.rs/cstr/0.2.8/' --extern-html-root-url 'libc=https://docs.rs/libc/0.2.116/' --extern-html-root-url 'libloading=https://docs.rs/libloading/0.7.1/' --extern-html-root-url 'measureme=https://docs.rs/measureme/10.0.0/' --extern-html-root-url 'rustc_demangle=https://docs.rs/rustc-demangle/0.1.21/' --extern-html-root-url 'smallvec=https://docs.rs/smallvec/1.7.0/' --extern-html-root-url 'tracing=https://docs.rs/tracing/0.1.29/' -Zunstable-options -Csymbol-mangling-version=v0 -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.61.0-nightly
  (8ba23b046
  2022-02-22)' --document-private-items '-Arustdoc::private-intra-doc-links' --enable-index-page -Zunstable-options -Znormalize-docs --show-type-layout --generate-link-to-definition --extern-html-root-url 'ena=https://docs.rs/ena/latest/'` (exit status: 1)
[RUSTC-TIMING] rustc_codegen_llvm test:false 2.858
[RUSTC-TIMING] rustc_trait_selection test:false 4.296
error: build failed
Build completed unsuccessfully in 0:26:13
