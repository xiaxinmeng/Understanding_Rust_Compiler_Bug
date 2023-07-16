plain
Preparing cargo
[2021-09-12T06:04:35Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=None, patch=None
[2021-09-12T06:04:35Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2021-09-12T06:04:35Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2021-09-12T06:04:35Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmppvGcoz#cargo:0.29.0" "--profile" "check" "--lib" "--" "--skip-this-rustc"
[2021-09-12T06:04:35Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpVXgYKJ#cargo:0.29.0" "--release" "--lib" "--" "--skip-this-rustc"
[2021-09-12T06:04:35Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmps1uSvd#cargo:0.29.0" "--lib" "--" "--skip-this-rustc"
[2021-09-12T06:06:40Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-09-12T06:06:40Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2021-09-12T06:06:40Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2021-09-12T06:06:40Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp0yzvWH#cargo:0.29.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "eprintln"
[2021-09-12T06:06:55Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2021-09-12T06:06:55Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp0yzvWH#cargo:0.29.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmp0yzvWH/incremental-state"
[2021-09-12T06:07:13Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrUnchanged), patch=None
[2021-09-12T06:07:13Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp0yzvWH#cargo:0.29.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmp0yzvWH/incremental-state"
[2021-09-12T06:07:17Z DEBUG collector::execute] applying println to "/tmp/.tmp0yzvWH"
[2021-09-12T06:07:17Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2021-09-12T06:07:17Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2021-09-12T06:07:17Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp0yzvWH#cargo:0.29.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmp0yzvWH/incremental-state"
[2021-09-12T06:07:25Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-09-12T06:07:26Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2021-09-12T06:07:26Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2021-09-12T06:07:26Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpiLpgFl#cargo:0.29.0" "--lib" "--" "--wrap-rustc-with" "eprintln"
[2021-09-12T06:08:03Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2021-09-12T06:08:03Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpiLpgFl#cargo:0.29.0" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpiLpgFl/incremental-state"
[2021-09-12T06:08:49Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2021-09-12T06:08:49Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpiLpgFl#cargo:0.29.0" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpiLpgFl/incremental-state"
[2021-09-12T06:08:57Z DEBUG collector::execute] applying patch println
[2021-09-12T06:08:57Z DEBUG collector::execute] applying println to "/tmp/.tmpiLpgFl"
[2021-09-12T06:08:57Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2021-09-12T06:08:57Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpiLpgFl#cargo:0.29.0" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpiLpgFl/incremental-state"
[2021-09-12T06:09:14Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-09-12T06:09:15Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2021-09-12T06:09:15Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2021-09-12T06:09:15Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpog1TPl#cargo:0.29.0" "--release" "--lib" "--" "--wrap-rustc-with" "eprintln"
[2021-09-12T06:10:43Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2021-09-12T06:10:43Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpog1TPl#cargo:0.29.0" "--release" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpog1TPl/incremental-state"
[2021-09-12T06:12:13Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrUnchanged), patch=None
[2021-09-12T06:12:14Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpog1TPl#cargo:0.29.0" "--release" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpog1TPl/incremental-state"
[2021-09-12T06:12:21Z DEBUG collector::execute] applying println to "/tmp/.tmpog1TPl"
[2021-09-12T06:12:21Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2021-09-12T06:12:21Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2021-09-12T06:12:21Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpog1TPl#cargo:0.29.0" "--release" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpog1TPl/incremental-state"
Preparing ctfe-stress-4
[2021-09-12T06:13:12Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=None, patch=None
[2021-09-12T06:13:12Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2021-09-12T06:13:12Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2021-09-12T06:13:12Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2021-09-12T06:13:12Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpaRTjAw#ctfe-stress-4:0.1.0" "--release" "--" "--skip-this-rustc"
[2021-09-12T06:13:12Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQ38UnE#ctfe-stress-4:0.1.0" "--profile" "check" "--" "--skip-this-rustc"
[2021-09-12T06:13:12Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmposisDu#ctfe-stress-4:0.1.0" "--" "--skip-this-rustc"
[2021-09-12T06:13:13Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-09-12T06:13:13Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2021-09-12T06:13:13Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp8gzw3M#ctfe-stress-4:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln"
[2021-09-12T06:13:32Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2021-09-12T06:13:32Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2021-09-12T06:13:33Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp8gzw3M#ctfe-stress-4:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmp8gzw3M/incremental-state"
[2021-09-12T06:13:57Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrUnchanged), patch=None
[2021-09-12T06:13:57Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp8gzw3M#ctfe-stress-4:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmp8gzw3M/incremental-state"
[2021-09-12T06:13:58Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-09-12T06:13:58Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2021-09-12T06:13:58Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2021-09-12T06:13:58Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpIugCYS#ctfe-stress-4:0.1.0" "--" "--wrap-rustc-with" "eprintln"
[2021-09-12T06:14:18Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2021-09-12T06:14:18Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpIugCYS#ctfe-stress-4:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpIugCYS/incremental-state"
[2021-09-12T06:14:42Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2021-09-12T06:14:43Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpIugCYS#ctfe-stress-4:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpIugCYS/incremental-state"
[2021-09-12T06:14:43Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-09-12T06:14:43Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2021-09-12T06:14:43Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2021-09-12T06:14:43Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpudKbcF#ctfe-stress-4:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln"
[2021-09-12T06:15:02Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2021-09-12T06:15:02Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpudKbcF#ctfe-stress-4:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpudKbcF/incremental-state"
[2021-09-12T06:15:26Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrUnchanged), patch=None
[2021-09-12T06:15:26Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpudKbcF#ctfe-stress-4:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpudKbcF/incremental-state"
Preparing externs
[2021-09-12T06:15:26Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2021-09-12T06:15:26Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=None, patch=None
[2021-09-12T06:15:26Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2021-09-12T06:15:26Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2021-09-12T06:15:26Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpIjXaxF#externs:0.1.0" "--" "--skip-this-rustc"
[2021-09-12T06:15:26Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCx9aUm#externs:0.1.0" "--profile" "check" "--" "--skip-this-rustc"
[2021-09-12T06:15:26Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmppFFwM7#externs:0.1.0" "--release" "--" "--skip-this-rustc"
[2021-09-12T06:15:27Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-09-12T06:15:27Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2021-09-12T06:15:27Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2021-09-12T06:15:27Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpRNgVdu#externs:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln"
[2021-09-12T06:15:28Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2021-09-12T06:15:28Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpRNgVdu#externs:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpRNgVdu/incremental-state"
[2021-09-12T06:15:29Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrUnchanged), patch=None
[2021-09-12T06:15:29Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpRNgVdu#externs:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpRNgVdu/incremental-state"
[2021-09-12T06:15:30Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-09-12T06:15:30Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2021-09-12T06:15:30Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwo2LL9#externs:0.1.0" "--" "--wrap-rustc-with" "eprintln"
[2021-09-12T06:15:31Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2021-09-12T06:15:31Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2021-09-12T06:15:31Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwo2LL9#externs:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpwo2LL9/incremental-state"
[2021-09-12T06:15:32Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2021-09-12T06:15:32Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwo2LL9#externs:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpwo2LL9/incremental-state"
[2021-09-12T06:15:33Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-09-12T06:15:33Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2021-09-12T06:15:33Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2021-09-12T06:15:33Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpo8HgDW#externs:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln"
[2021-09-12T06:15:35Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2021-09-12T06:15:35Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpo8HgDW#externs:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpo8HgDW/incremental-state"
[2021-09-12T06:15:36Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrUnchanged), patch=None
[2021-09-12T06:15:36Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpo8HgDW#externs:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpo8HgDW/incremental-state"
Preparing inflate
[2021-09-12T06:15:37Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=None, patch=None
[2021-09-12T06:15:37Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2021-09-12T06:15:37Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
---
[2021-09-12T06:15:38Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-09-12T06:15:38Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2021-09-12T06:15:38Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQ0CTp2#inflate:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln"
[2021-09-12T06:15:40Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2021-09-12T06:15:40Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQ0CTp2#inflate:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpQ0CTp2/incremental-state"
[2021-09-12T06:15:43Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrUnchanged), patch=None
[2021-09-12T06:15:43Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQ0CTp2#inflate:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpQ0CTp2/incremental-state"
[2021-09-12T06:15:43Z DEBUG collector::execute] applying println to "/tmp/.tmpQ0CTp2"
[2021-09-12T06:15:43Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2021-09-12T06:15:43Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2021-09-12T06:15:43Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQ0CTp2#inflate:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpQ0CTp2/incremental-state"
[2021-09-12T06:15:46Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-09-12T06:15:46Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2021-09-12T06:15:46Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpc5m6XC#inflate:0.1.0" "--" "--wrap-rustc-with" "eprintln"
[2021-09-12T06:15:49Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2021-09-12T06:15:49Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2021-09-12T06:15:49Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpc5m6XC#inflate:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpc5m6XC/incremental-state"
[2021-09-12T06:15:53Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2021-09-12T06:15:53Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpc5m6XC#inflate:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpc5m6XC/incremental-state"
[2021-09-12T06:15:54Z DEBUG collector::execute] applying println to "/tmp/.tmpc5m6XC"
[2021-09-12T06:15:54Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2021-09-12T06:15:54Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2021-09-12T06:15:54Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpc5m6XC#inflate:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpc5m6XC/incremental-state"
[2021-09-12T06:15:58Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-09-12T06:15:58Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2021-09-12T06:15:58Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpH3C3mV#inflate:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln"
[2021-09-12T06:16:08Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2021-09-12T06:16:08Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2021-09-12T06:16:08Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpH3C3mV#inflate:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpH3C3mV/incremental-state"
[2021-09-12T06:16:19Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrUnchanged), patch=None
[2021-09-12T06:16:19Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpH3C3mV#inflate:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpH3C3mV/incremental-state"
[2021-09-12T06:16:19Z DEBUG collector::execute] applying println to "/tmp/.tmpH3C3mV"
[2021-09-12T06:16:19Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2021-09-12T06:16:19Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2021-09-12T06:16:19Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpH3C3mV#inflate:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpH3C3mV/incremental-state"
Preparing match-stress-enum
[2021-09-12T06:16:30Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=None, patch=None
[2021-09-12T06:16:30Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2021-09-12T06:16:30Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2021-09-12T06:16:30Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2021-09-12T06:16:30Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpelS1RK#match-stress-enum:0.1.0" "--profile" "check" "--" "--skip-this-rustc"
[2021-09-12T06:16:30Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpIzVnvX#match-stress-enum:0.1.0" "--release" "--" "--skip-this-rustc"
[2021-09-12T06:16:30Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpBBxt68#match-stress-enum:0.1.0" "--" "--skip-this-rustc"
[2021-09-12T06:16:31Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-09-12T06:16:31Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2021-09-12T06:16:31Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpk6ZNc4#match-stress-enum:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln"
[2021-09-12T06:16:33Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2021-09-12T06:16:33Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2021-09-12T06:16:33Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpk6ZNc4#match-stress-enum:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpk6ZNc4/incremental-state"
[2021-09-12T06:16:35Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrUnchanged), patch=None
[2021-09-12T06:16:35Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpk6ZNc4#match-stress-enum:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpk6ZNc4/incremental-state"
[2021-09-12T06:16:35Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-09-12T06:16:35Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2021-09-12T06:16:35Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpC90DZb#match-stress-enum:0.1.0" "--" "--wrap-rustc-with" "eprintln"
[2021-09-12T06:16:37Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2021-09-12T06:16:37Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2021-09-12T06:16:37Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpC90DZb#match-stress-enum:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpC90DZb/incremental-state"
[2021-09-12T06:16:39Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2021-09-12T06:16:39Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpC90DZb#match-stress-enum:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpC90DZb/incremental-state"
[2021-09-12T06:16:40Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-09-12T06:16:40Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2021-09-12T06:16:40Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2021-09-12T06:16:40Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp4GgTAs#match-stress-enum:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln"
[2021-09-12T06:16:42Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2021-09-12T06:16:42Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp4GgTAs#match-stress-enum:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmp4GgTAs/incremental-state"
[2021-09-12T06:16:44Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrUnchanged), patch=None
[2021-09-12T06:16:44Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp4GgTAs#match-stress-enum:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmp4GgTAs/incremental-state"
Preparing token-stream-stress
[2021-09-12T06:16:45Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2021-09-12T06:16:45Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=None, patch=None
[2021-09-12T06:16:45Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2021-09-12T06:16:45Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2021-09-12T06:16:45Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpWHVib4#token-stream-stress:0.0.0" "--bin" "token-stream-stress" "--" "--skip-this-rustc"
[2021-09-12T06:16:45Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpWBX14Y#token-stream-stress:0.0.0" "--release" "--bin" "token-stream-stress" "--" "--skip-this-rustc"
[2021-09-12T06:16:45Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpdliSIG#token-stream-stress:0.0.0" "--profile" "check" "--bin" "token-stream-stress" "--" "--skip-this-rustc"
[2021-09-12T06:16:46Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-09-12T06:16:46Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2021-09-12T06:16:46Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpL4m3kX#token-stream-stress:0.0.0" "--profile" "check" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln"
[2021-09-12T06:16:46Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2021-09-12T06:16:46Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2021-09-12T06:16:46Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpL4m3kX#token-stream-stress:0.0.0" "--profile" "check" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpL4m3kX/incremental-state"
[2021-09-12T06:16:47Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrUnchanged), patch=None
[2021-09-12T06:16:47Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpL4m3kX#token-stream-stress:0.0.0" "--profile" "check" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpL4m3kX/incremental-state"
[2021-09-12T06:16:47Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-09-12T06:16:47Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2021-09-12T06:16:47Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2021-09-12T06:16:47Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpTuJJFU#token-stream-stress:0.0.0" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln"
[2021-09-12T06:16:47Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2021-09-12T06:16:47Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpTuJJFU#token-stream-stress:0.0.0" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpTuJJFU/incremental-state"
[2021-09-12T06:16:48Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2021-09-12T06:16:48Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpTuJJFU#token-stream-stress:0.0.0" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpTuJJFU/incremental-state"
[2021-09-12T06:16:48Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-09-12T06:16:48Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2021-09-12T06:16:48Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2021-09-12T06:16:48Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmprBkycQ#token-stream-stress:0.0.0" "--release" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln"
[2021-09-12T06:16:48Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2021-09-12T06:16:48Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmprBkycQ#token-stream-stress:0.0.0" "--release" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmprBkycQ/incremental-state"
[2021-09-12T06:16:49Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrUnchanged), patch=None
[2021-09-12T06:16:49Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmprBkycQ#token-stream-stress:0.0.0" "--release" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmprBkycQ/incremental-state"
+ ./build/x86_64-unknown-linux-gnu/llvm/bin/llvm-profdata merge -o /tmp/rustc-pgo.profdata /tmp/rustc-pgo
+ /rustroot/bin/llvm-profdata merge -o /tmp/llvm-pgo.profdata ./build/x86_64-unknown-linux-gnu/llvm/build/profiles
+ rm -r ./build/x86_64-unknown-linux-gnu/llvm ./build/x86_64-unknown-linux-gnu/lld
+ python3 ../x.py dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu --include-default-paths src/tools/build-manifest --rust-profile-use=/tmp/rustc-pgo.profdata --llvm-profile-use=/tmp/llvm-pgo.profdata
---
For more information about this error, try `rustc --explain E0463`.
error: could not document `rustdoc`

Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type lib --crate-name rustdoc src/librustdoc/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/doc --error-format=json --json=diagnostic-rendered-ansi -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps --extern arrayvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libarrayvec-ace1ee1a1d07d40c.rmeta --extern itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-e62f3700972f969d.rmeta --extern minifier=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libminifier-f11b8f3f07d7c4a8.rmeta --extern pulldown_cmark=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libpulldown_cmark-adff8a52ae7c79b6.rmeta --extern regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-e81e93d9771314e9.rmeta --extern rayon=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_rayon-a2e5bff31f3e9141.rmeta --extern rustdoc_json_types=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustdoc_json_types-e9cb0d8e349af893.rmeta --extern serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-4f00e59b1d8a53ba.rmeta --extern serde_json=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde_json-f0532694270708df.rmeta --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsmallvec-cc460c8e897d6e30.rmeta --extern tempfile=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtempfile-1b265042af626c67.rmeta --extern tera=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtera-08f2517cd80414d8.rmeta --extern tracing=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtracing-488d62e00d117ca3.rmeta --extern tracing_subscriber=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtracing_subscriber-8a7a2815623afe36.rmeta --extern tracing_tree=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtracing_tree-7a33633184694dac.rmeta -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.57.0-nightly
  (c900274b7
  2021-09-12)' --document-private-items --enable-index-page --show-type-layout -Zunstable-options` (exit status: 1)


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rustdoc/Cargo.toml" "-Zskip-rustdoc-fingerprint" "--no-deps" "-p" "rustdoc" "-p" "rustdoc-json-types"


Build completed unsuccessfully in 0:25:25
