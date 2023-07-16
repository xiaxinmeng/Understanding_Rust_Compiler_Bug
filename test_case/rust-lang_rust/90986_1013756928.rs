plain
[2022-01-15T20:42:43Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-01-15T20:42:44Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2022-01-15T20:42:44Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpS6ogHf#cargo:0.29.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "eprintln"
[2022-01-15T20:43:00Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2022-01-15T20:43:00Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpS6ogHf#cargo:0.29.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpS6ogHf/incremental-state"
[2022-01-15T20:43:19Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrUnchanged), patch=None
[2022-01-15T20:43:19Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpS6ogHf#cargo:0.29.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpS6ogHf/incremental-state"
[2022-01-15T20:43:23Z DEBUG collector::execute] applying println to "/tmp/.tmpS6ogHf"
[2022-01-15T20:43:23Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-01-15T20:43:23Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-01-15T20:43:23Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpS6ogHf#cargo:0.29.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpS6ogHf/incremental-state"
[2022-01-15T20:43:31Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-01-15T20:43:32Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-01-15T20:43:32Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp5d6yYi#cargo:0.29.0" "--lib" "--" "--wrap-rustc-with" "eprintln"
[2022-01-15T20:44:07Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
---
[2022-01-15T20:45:21Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-01-15T20:45:21Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-01-15T20:45:21Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpTps2lz#cargo:0.29.0" "--release" "--lib" "--" "--wrap-rustc-with" "eprintln"
[2022-01-15T20:46:09Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2022-01-15T20:46:09Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpTps2lz#cargo:0.29.0" "--release" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpTps2lz/incremental-state"
[2022-01-15T20:46:57Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrUnchanged), patch=None
[2022-01-15T20:46:57Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpTps2lz#cargo:0.29.0" "--release" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpTps2lz/incremental-state"
[2022-01-15T20:47:05Z DEBUG collector::execute] applying println to "/tmp/.tmpTps2lz"
[2022-01-15T20:47:05Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-01-15T20:47:05Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-01-15T20:47:05Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpTps2lz#cargo:0.29.0" "--release" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpTps2lz/incremental-state"
Preparing ctfe-stress-4
[2022-01-15T20:47:28Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=None, patch=None
[2022-01-15T20:47:28Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2022-01-15T20:47:28Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
---
[2022-01-15T20:47:29Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-01-15T20:47:29Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2022-01-15T20:47:29Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOUYfxh#ctfe-stress-4:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln"
[2022-01-15T20:47:56Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2022-01-15T20:47:56Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOUYfxh#ctfe-stress-4:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpOUYfxh/incremental-state"
[2022-01-15T20:48:28Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrUnchanged), patch=None
[2022-01-15T20:48:28Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOUYfxh#ctfe-stress-4:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpOUYfxh/incremental-state"
[2022-01-15T20:48:28Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-01-15T20:48:28Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-01-15T20:48:28Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp6YrUJj#ctfe-stress-4:0.1.0" "--" "--wrap-rustc-with" "eprintln"
[2022-01-15T20:48:55Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-01-15T20:48:55Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-01-15T20:48:55Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp6YrUJj#ctfe-stress-4:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmp6YrUJj/incremental-state"
[2022-01-15T20:49:27Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2022-01-15T20:49:27Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp6YrUJj#ctfe-stress-4:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmp6YrUJj/incremental-state"
[2022-01-15T20:49:28Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-01-15T20:49:28Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-01-15T20:49:28Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpL1S7dg#ctfe-stress-4:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln"
[2022-01-15T20:49:53Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2022-01-15T20:49:53Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2022-01-15T20:49:53Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpL1S7dg#ctfe-stress-4:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpL1S7dg/incremental-state"
[2022-01-15T20:50:24Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrUnchanged), patch=None
[2022-01-15T20:50:24Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpL1S7dg#ctfe-stress-4:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpL1S7dg/incremental-state"
Preparing externs
[2022-01-15T20:50:25Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=None, patch=None
[2022-01-15T20:50:25Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2022-01-15T20:50:25Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
---
[2022-01-15T20:50:28Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-01-15T20:50:28Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-01-15T20:50:28Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp7s4wgo#externs:0.1.0" "--" "--wrap-rustc-with" "eprintln"
[2022-01-15T20:50:29Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-01-15T20:50:29Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp7s4wgo#externs:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmp7s4wgo/incremental-state"
[2022-01-15T20:50:31Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2022-01-15T20:50:31Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp7s4wgo#externs:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmp7s4wgo/incremental-state"
[2022-01-15T20:50:32Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-01-15T20:50:32Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-01-15T20:50:32Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp6xBdmJ#externs:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln"
[2022-01-15T20:50:33Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2022-01-15T20:50:33Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2022-01-15T20:50:33Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp6xBdmJ#externs:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmp6xBdmJ/incremental-state"
[2022-01-15T20:50:34Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrUnchanged), patch=None
[2022-01-15T20:50:34Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp6xBdmJ#externs:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmp6xBdmJ/incremental-state"
Preparing inflate
[2022-01-15T20:50:36Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=None, patch=None
[2022-01-15T20:50:36Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2022-01-15T20:50:36Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
---
[2022-01-15T20:50:36Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-01-15T20:50:36Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2022-01-15T20:50:36Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpsDk7Hb#inflate:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln"
[2022-01-15T20:50:39Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2022-01-15T20:50:39Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpsDk7Hb#inflate:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpsDk7Hb/incremental-state"
[2022-01-15T20:50:41Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrUnchanged), patch=None
[2022-01-15T20:50:41Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpsDk7Hb#inflate:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpsDk7Hb/incremental-state"
[2022-01-15T20:50:42Z DEBUG collector::execute] applying println to "/tmp/.tmpsDk7Hb"
[2022-01-15T20:50:42Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-01-15T20:50:42Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-01-15T20:50:42Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpsDk7Hb#inflate:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpsDk7Hb/incremental-state"
[2022-01-15T20:50:45Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-01-15T20:50:45Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-01-15T20:50:45Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp62rYKQ#inflate:0.1.0" "--" "--wrap-rustc-with" "eprintln"
[2022-01-15T20:50:48Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-01-15T20:50:48Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-01-15T20:50:48Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp62rYKQ#inflate:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmp62rYKQ/incremental-state"
[2022-01-15T20:50:52Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2022-01-15T20:50:52Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp62rYKQ#inflate:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmp62rYKQ/incremental-state"
[2022-01-15T20:50:53Z DEBUG collector::execute] applying println to "/tmp/.tmp62rYKQ"
[2022-01-15T20:50:53Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-01-15T20:50:53Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-01-15T20:50:53Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp62rYKQ#inflate:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmp62rYKQ/incremental-state"
[2022-01-15T20:50:57Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-01-15T20:50:57Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-01-15T20:50:57Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpBT879T#inflate:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln"
[2022-01-15T20:51:02Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
---
Preparing match-stress-enum
[2022-01-15T20:51:15Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2022-01-15T20:51:15Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=None, patch=None
[2022-01-15T20:51:15Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2022-01-15T20:51:15Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpYSsTQB#match-stress-enum:0.1.0" "--release" "--" "--skip-this-rustc"
[2022-01-15T20:51:15Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmphxxHM9#match-stress-enum:0.1.0" "--" "--skip-this-rustc"
[2022-01-15T20:51:15Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpmNFJlN#match-stress-enum:0.1.0" "--profile" "check" "--" "--skip-this-rustc"
[2022-01-15T20:51:15Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-01-15T20:51:15Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2022-01-15T20:51:15Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpNTS6gy#match-stress-enum:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln"
[2022-01-15T20:51:17Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2022-01-15T20:51:17Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2022-01-15T20:51:17Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpNTS6gy#match-stress-enum:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpNTS6gy/incremental-state"
[2022-01-15T20:51:19Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrUnchanged), patch=None
[2022-01-15T20:51:19Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpNTS6gy#match-stress-enum:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpNTS6gy/incremental-state"
[2022-01-15T20:51:19Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-01-15T20:51:19Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-01-15T20:51:19Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpC0Frqr#match-stress-enum:0.1.0" "--" "--wrap-rustc-with" "eprintln"
[2022-01-15T20:51:22Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-01-15T20:51:22Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-01-15T20:51:22Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpC0Frqr#match-stress-enum:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpC0Frqr/incremental-state"
[2022-01-15T20:51:24Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2022-01-15T20:51:24Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpC0Frqr#match-stress-enum:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpC0Frqr/incremental-state"
[2022-01-15T20:51:24Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-01-15T20:51:24Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-01-15T20:51:24Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpl9rqYt#match-stress-enum:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln"
[2022-01-15T20:51:26Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2022-01-15T20:51:26Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2022-01-15T20:51:26Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpl9rqYt#match-stress-enum:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpl9rqYt/incremental-state"
[2022-01-15T20:51:29Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrUnchanged), patch=None
[2022-01-15T20:51:29Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpl9rqYt#match-stress-enum:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpl9rqYt/incremental-state"
Preparing token-stream-stress
[2022-01-15T20:51:29Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2022-01-15T20:51:29Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=None, patch=None
[2022-01-15T20:51:29Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
---
[2022-01-15T20:51:30Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-01-15T20:51:30Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2022-01-15T20:51:30Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp9uRgqi#token-stream-stress:0.0.0" "--profile" "check" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln"
[2022-01-15T20:51:31Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2022-01-15T20:51:31Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp9uRgqi#token-stream-stress:0.0.0" "--profile" "check" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmp9uRgqi/incremental-state"
[2022-01-15T20:51:31Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrUnchanged), patch=None
[2022-01-15T20:51:31Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp9uRgqi#token-stream-stress:0.0.0" "--profile" "check" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmp9uRgqi/incremental-state"
[2022-01-15T20:51:31Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-01-15T20:51:31Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-01-15T20:51:31Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp2wOmpe#token-stream-stress:0.0.0" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln"
[2022-01-15T20:51:31Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-01-15T20:51:31Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-01-15T20:51:31Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp2wOmpe#token-stream-stress:0.0.0" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmp2wOmpe/incremental-state"
[2022-01-15T20:51:32Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2022-01-15T20:51:32Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp2wOmpe#token-stream-stress:0.0.0" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmp2wOmpe/incremental-state"
[2022-01-15T20:51:32Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-01-15T20:51:32Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-01-15T20:51:32Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp5hksjd#token-stream-stress:0.0.0" "--release" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln"
[2022-01-15T20:51:32Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2022-01-15T20:51:32Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2022-01-15T20:51:32Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp5hksjd#token-stream-stress:0.0.0" "--release" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmp5hksjd/incremental-state"
[2022-01-15T20:51:33Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrUnchanged), patch=None
[2022-01-15T20:51:33Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp5hksjd#token-stream-stress:0.0.0" "--release" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmp5hksjd/incremental-state"
+ ./build/x86_64-unknown-linux-gnu/llvm/bin/llvm-profdata merge -o /tmp/rustc-pgo.profdata /tmp/rustc-pgo
+ rm -r ./build/x86_64-unknown-linux-gnu/llvm ./build/x86_64-unknown-linux-gnu/lld
+ python3 ../x.py dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu --include-default-paths build-manifest --rust-profile-use=/tmp/rustc-pgo.profdata --llvm-profile-use=/tmp/llvm-pgo.profdata
warning: sqlite3 not available in python, skipping build directory lock
---
[RUSTC-TIMING] rustc_tools_util test:false 0.187
   Compiling clippy v0.1.60 (/checkout/src/tools/clippy)
[RUSTC-TIMING] rustc_semver test:false 0.294
[RUSTC-TIMING] build_script_build test:false 0.318
error[E0432]: unresolved imports `rustc_hir::intravisit::ErasedMap`, `rustc_hir::intravisit::NestedVisitorMap`
  --> src/tools/clippy/clippy_utils/src/eager_or_lazy.rs:15:40
   |
15 | use rustc_hir::intravisit::{walk_expr, ErasedMap, NestedVisitorMap, Visitor};
   |                                        ^^^^^^^^^  ^^^^^^^^^^^^^^^^ no `NestedVisitorMap` in `intravisit`
   |                                        |
   |                                        no `ErasedMap` in `intravisit`

error[E0432]: unresolved import `rustc_hir::intravisit::NestedVisitorMap`
  |
  |
4 | use rustc_hir::intravisit::{self, walk_block, walk_expr, NestedVisitorMap, Visitor};
  |                                                          ^^^^^^^^^^^^^^^^ no `NestedVisitorMap` in `intravisit`

error[E0432]: unresolved imports `rustc_hir::intravisit::ErasedMap`, `rustc_hir::intravisit::NestedVisitorMap`
   |
   |
75 | use rustc_hir::intravisit::{walk_expr, ErasedMap, FnKind, NestedVisitorMap, Visitor};
   |                                        ^^^^^^^^^          ^^^^^^^^^^^^^^^^ no `NestedVisitorMap` in `intravisit`
   |                                        |
   |                                        no `ErasedMap` in `intravisit`
[RUSTC-TIMING] rustc_tools_util test:false 0.387
[RUSTC-TIMING] rustc_tools_util test:false 0.387
error[E0433]: failed to resolve: could not find `NestedVisitorMap` in `intravisit`
    |
    |
109 |         intravisit::NestedVisitorMap::None
    |                     ^^^^^^^^^^^^^^^^ could not find `NestedVisitorMap` in `intravisit`

error[E0433]: failed to resolve: could not find `NestedVisitorMap` in `intravisit`
    |
    |
147 |         intravisit::NestedVisitorMap::OnlyBodies(self.cx.tcx.hir())
    |                     ^^^^^^^^^^^^^^^^ could not find `NestedVisitorMap` in `intravisit`

error[E0433]: failed to resolve: could not find `NestedVisitorMap` in `intravisit`
    |
    |
119 |             intravisit::NestedVisitorMap::None
    |                         ^^^^^^^^^^^^^^^^ could not find `NestedVisitorMap` in `intravisit`

error[E0412]: cannot find type `NestedVisitorMap` in module `intravisit`
    |
    |
108 |     fn nested_visit_map(&mut self) -> intravisit::NestedVisitorMap<Self::Map> {
    |                                                   ^^^^^^^^^^^^^^^^ not found in `intravisit`

error[E0412]: cannot find type `NestedVisitorMap` in module `intravisit`
    |
    |
146 |     fn nested_visit_map(&mut self) -> intravisit::NestedVisitorMap<Self::Map> {
    |                                                   ^^^^^^^^^^^^^^^^ not found in `intravisit`

error[E0412]: cannot find type `ErasedMap` in module `intravisit`
   |
   |
43 |         type Map = intravisit::ErasedMap<'tcx>;
   |                                ^^^^^^^^^ not found in `intravisit`

error[E0412]: cannot find type `NestedVisitorMap` in module `intravisit`
    |
    |
118 |         fn nested_visit_map(&mut self) -> intravisit::NestedVisitorMap<Self::Map> {
    |                                                       ^^^^^^^^^^^^^^^^ not found in `intravisit`
[RUSTC-TIMING] build_script_build test:false 0.264
[RUSTC-TIMING] cargo_platform test:false 0.651
   Compiling cargo_metadata v0.14.0
[RUSTC-TIMING] unicode_script test:false 0.793
---
   Compiling humantime v1.3.0
[RUSTC-TIMING] rustc_tools_util test:false 0.796
[RUSTC-TIMING] vte test:false 0.132
   Compiling pest v2.1.3
error[E0432]: unresolved imports `rustc_hir::intravisit::ErasedMap`, `rustc_hir::intravisit::NestedVisitorMap`
  --> src/tools/clippy/clippy_utils/src/eager_or_lazy.rs:15:40
   |
15 | use rustc_hir::intravisit::{walk_expr, ErasedMap, NestedVisitorMap, Visitor};
   |                                        ^^^^^^^^^  ^^^^^^^^^^^^^^^^ no `NestedVisitorMap` in `intravisit`
   |                                        |
   |                                        no `ErasedMap` in `intravisit`

error[E0432]: unresolved import `rustc_hir::intravisit::NestedVisitorMap`
  |
  |
4 | use rustc_hir::intravisit::{self, walk_block, walk_expr, NestedVisitorMap, Visitor};
  |                                                          ^^^^^^^^^^^^^^^^ no `NestedVisitorMap` in `intravisit`

error[E0432]: unresolved imports `rustc_hir::intravisit::ErasedMap`, `rustc_hir::intravisit::NestedVisitorMap`
   |
   |
75 | use rustc_hir::intravisit::{walk_expr, ErasedMap, FnKind, NestedVisitorMap, Visitor};
   |                                        ^^^^^^^^^          ^^^^^^^^^^^^^^^^ no `NestedVisitorMap` in `intravisit`
   |                                        |
   |                                        no `ErasedMap` in `intravisit`
[RUSTC-TIMING] unicode_segmentation test:false 0.996
[RUSTC-TIMING] ucd_trie test:false 1.168
   Compiling rls v1.41.0 (/checkout/src/tools/rls)
[RUSTC-TIMING] libc test:false 1.130
[RUSTC-TIMING] libc test:false 1.130
[RUSTC-TIMING] libc test:false 1.009
[RUSTC-TIMING] proc_macro2 test:false 1.194
[RUSTC-TIMING] log test:false 0.477
[RUSTC-TIMING] memchr test:false 2.192
[RUSTC-TIMING] bitflags test:false 0.046
   Compiling heck v0.3.1
error[E0433]: failed to resolve: could not find `NestedVisitorMap` in `intravisit`
    |
    |
109 |         intravisit::NestedVisitorMap::None
    |                     ^^^^^^^^^^^^^^^^ could not find `NestedVisitorMap` in `intravisit`

error[E0433]: failed to resolve: could not find `NestedVisitorMap` in `intravisit`
    |
    |
147 |         intravisit::NestedVisitorMap::OnlyBodies(self.cx.tcx.hir())
    |                     ^^^^^^^^^^^^^^^^ could not find `NestedVisitorMap` in `intravisit`

error[E0433]: failed to resolve: could not find `NestedVisitorMap` in `intravisit`
    |
    |
119 |             intravisit::NestedVisitorMap::None
    |                         ^^^^^^^^^^^^^^^^ could not find `NestedVisitorMap` in `intravisit`

error[E0412]: cannot find type `NestedVisitorMap` in module `intravisit`
    |
    |
108 |     fn nested_visit_map(&mut self) -> intravisit::NestedVisitorMap<Self::Map> {
    |                                                   ^^^^^^^^^^^^^^^^ not found in `intravisit`

error[E0412]: cannot find type `NestedVisitorMap` in module `intravisit`
    |
    |
146 |     fn nested_visit_map(&mut self) -> intravisit::NestedVisitorMap<Self::Map> {
    |                                                   ^^^^^^^^^^^^^^^^ not found in `intravisit`

error[E0412]: cannot find type `ErasedMap` in module `intravisit`
   |
   |
43 |         type Map = intravisit::ErasedMap<'tcx>;
   |                                ^^^^^^^^^ not found in `intravisit`

error[E0412]: cannot find type `NestedVisitorMap` in module `intravisit`
    |
    |
118 |         fn nested_visit_map(&mut self) -> intravisit::NestedVisitorMap<Self::Map> {
    |                                                       ^^^^^^^^^^^^^^^^ not found in `intravisit`
[RUSTC-TIMING] futures_channel test:false 0.373
[RUSTC-TIMING] proc_macro2 test:false 3.673
[RUSTC-TIMING] typenum test:false 1.025
[RUSTC-TIMING] maybe_uninit test:false 0.045
---
Dist rust-dev-nightly-x86_64-unknown-linux-gnu
 finished in 14.944 seconds
[TIMING] RustDev { target: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } } -- 15.115
Dist extended stage1 (x86_64-unknown-linux-gnu)
thread 'main' panicked at 'clippy expected to build - essential tool', src/bootstrap/dist.rs:1148:14
Build completed unsuccessfully in 0:37:45
