plain
Preparing cargo
[2022-01-19T22:20:00Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=None, patch=None
[2022-01-19T22:20:00Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2022-01-19T22:20:00Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2022-01-19T22:20:00Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpqusa6t#cargo:0.29.0" "--release" "--lib" "--" "--skip-this-rustc"
[2022-01-19T22:20:00Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpXrxBU2#cargo:0.29.0" "--profile" "check" "--lib" "--" "--skip-this-rustc"
[2022-01-19T22:20:00Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzdXoJx#cargo:0.29.0" "--lib" "--" "--skip-this-rustc"
[2022-01-19T22:21:24Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-01-19T22:21:24Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2022-01-19T22:21:24Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp99kRzn#cargo:0.29.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "eprintln"
[2022-01-19T22:21:38Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
---
[2022-01-19T22:22:06Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-01-19T22:22:07Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-01-19T22:22:07Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpvzS4ce#cargo:0.29.0" "--lib" "--" "--wrap-rustc-with" "eprintln"
[2022-01-19T22:22:36Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-01-19T22:22:36Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpvzS4ce#cargo:0.29.0" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpvzS4ce/incremental-state"
[2022-01-19T22:23:14Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2022-01-19T22:23:14Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpvzS4ce#cargo:0.29.0" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpvzS4ce/incremental-state"
[2022-01-19T22:23:20Z DEBUG collector::execute] applying println to "/tmp/.tmpvzS4ce"
[2022-01-19T22:23:20Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-01-19T22:23:20Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-01-19T22:23:20Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpvzS4ce#cargo:0.29.0" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpvzS4ce/incremental-state"
[2022-01-19T22:23:34Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-01-19T22:23:35Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-01-19T22:23:35Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-01-19T22:23:35Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpVtgzsJ#cargo:0.29.0" "--release" "--lib" "--" "--wrap-rustc-with" "eprintln"
[2022-01-19T22:24:09Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2022-01-19T22:24:09Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpVtgzsJ#cargo:0.29.0" "--release" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpVtgzsJ/incremental-state"
[2022-01-19T22:24:48Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrUnchanged), patch=None
[2022-01-19T22:24:48Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpVtgzsJ#cargo:0.29.0" "--release" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpVtgzsJ/incremental-state"
[2022-01-19T22:24:55Z DEBUG collector::execute] applying println to "/tmp/.tmpVtgzsJ"
[2022-01-19T22:24:55Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-01-19T22:24:55Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-01-19T22:24:55Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpVtgzsJ#cargo:0.29.0" "--release" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpVtgzsJ/incremental-state"
Preparing ctfe-stress-4
[2022-01-19T22:25:13Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=None, patch=None
[2022-01-19T22:25:13Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2022-01-19T22:25:13Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
---
[2022-01-19T22:26:11Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-01-19T22:26:11Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-01-19T22:26:11Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpMpdald#ctfe-stress-4:0.1.0" "--" "--wrap-rustc-with" "eprintln"
[2022-01-19T22:26:37Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-01-19T22:26:37Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpMpdald#ctfe-stress-4:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpMpdald/incremental-state"
[2022-01-19T22:27:07Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2022-01-19T22:27:07Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpMpdald#ctfe-stress-4:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpMpdald/incremental-state"
[2022-01-19T22:27:08Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-01-19T22:27:08Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-01-19T22:27:08Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpUzTeSN#ctfe-stress-4:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln"
[2022-01-19T22:27:33Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2022-01-19T22:27:33Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2022-01-19T22:27:33Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpUzTeSN#ctfe-stress-4:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpUzTeSN/incremental-state"
[2022-01-19T22:28:01Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrUnchanged), patch=None
[2022-01-19T22:28:01Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpUzTeSN#ctfe-stress-4:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpUzTeSN/incremental-state"
Preparing externs
[2022-01-19T22:28:02Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=None, patch=None
[2022-01-19T22:28:02Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2022-01-19T22:28:02Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
---
[2022-01-19T22:28:02Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-01-19T22:28:02Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2022-01-19T22:28:02Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpMgpipA#externs:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln"
[2022-01-19T22:28:03Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2022-01-19T22:28:03Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpMgpipA#externs:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpMgpipA/incremental-state"
[2022-01-19T22:28:04Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrUnchanged), patch=None
[2022-01-19T22:28:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpMgpipA#externs:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpMgpipA/incremental-state"
[2022-01-19T22:28:04Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-01-19T22:28:04Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-01-19T22:28:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKDGzR9#externs:0.1.0" "--" "--wrap-rustc-with" "eprintln"
[2022-01-19T22:28:05Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-01-19T22:28:05Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-01-19T22:28:05Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKDGzR9#externs:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpKDGzR9/incremental-state"
[2022-01-19T22:28:07Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2022-01-19T22:28:07Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKDGzR9#externs:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpKDGzR9/incremental-state"
[2022-01-19T22:28:08Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-01-19T22:28:08Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-01-19T22:28:08Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp5wty9Q#externs:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln"
[2022-01-19T22:28:09Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2022-01-19T22:28:09Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2022-01-19T22:28:09Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp5wty9Q#externs:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmp5wty9Q/incremental-state"
[2022-01-19T22:28:10Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrUnchanged), patch=None
[2022-01-19T22:28:10Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp5wty9Q#externs:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmp5wty9Q/incremental-state"
Preparing inflate
[2022-01-19T22:28:11Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=None, patch=None
[2022-01-19T22:28:11Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2022-01-19T22:28:11Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
---
[2022-01-19T22:28:17Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpg31jzt#inflate:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpg31jzt/incremental-state"
Running inflate: Debug + [Full, IncrFull, IncrUnchanged, IncrPatched]
[2022-01-19T22:28:19Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-01-19T22:28:19Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-01-19T22:28:19Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpacEAXX#inflate:0.1.0" "--" "--wrap-rustc-with" "eprintln"
[2022-01-19T22:28:22Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-01-19T22:28:22Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpacEAXX#inflate:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpacEAXX/incremental-state"
[2022-01-19T22:28:26Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2022-01-19T22:28:26Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpacEAXX#inflate:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpacEAXX/incremental-state"
[2022-01-19T22:28:26Z DEBUG collector::execute] applying println to "/tmp/.tmpacEAXX"
[2022-01-19T22:28:26Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-01-19T22:28:26Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-01-19T22:28:26Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpacEAXX#inflate:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpacEAXX/incremental-state"
[2022-01-19T22:28:30Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-01-19T22:28:30Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-01-19T22:28:30Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmptNaTTe#inflate:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln"
[2022-01-19T22:28:36Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2022-01-19T22:28:36Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2022-01-19T22:28:36Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmptNaTTe#inflate:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmptNaTTe/incremental-state"
[2022-01-19T22:28:41Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrUnchanged), patch=None
[2022-01-19T22:28:41Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmptNaTTe#inflate:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmptNaTTe/incremental-state"
[2022-01-19T22:28:41Z DEBUG collector::execute] applying println to "/tmp/.tmptNaTTe"
[2022-01-19T22:28:41Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-01-19T22:28:41Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-01-19T22:28:41Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmptNaTTe#inflate:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmptNaTTe/incremental-state"
Preparing match-stress-enum
[2022-01-19T22:28:47Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=None, patch=None
[2022-01-19T22:28:47Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2022-01-19T22:28:47Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2022-01-19T22:28:47Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2022-01-19T22:28:47Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpA2McpU#match-stress-enum:0.1.0" "--release" "--" "--skip-this-rustc"
[2022-01-19T22:28:47Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpuQdRFe#match-stress-enum:0.1.0" "--" "--skip-this-rustc"
[2022-01-19T22:28:47Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpynCp2I#match-stress-enum:0.1.0" "--profile" "check" "--" "--skip-this-rustc"
[2022-01-19T22:28:47Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-01-19T22:28:47Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2022-01-19T22:28:47Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpN8UBih#match-stress-enum:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln"
[2022-01-19T22:28:49Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2022-01-19T22:28:49Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2022-01-19T22:28:49Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpN8UBih#match-stress-enum:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpN8UBih/incremental-state"
[2022-01-19T22:28:51Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrUnchanged), patch=None
[2022-01-19T22:28:51Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpN8UBih#match-stress-enum:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpN8UBih/incremental-state"
[2022-01-19T22:28:52Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-01-19T22:28:52Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-01-19T22:28:52Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpLtXjbE#match-stress-enum:0.1.0" "--" "--wrap-rustc-with" "eprintln"
[2022-01-19T22:28:54Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-01-19T22:28:54Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-01-19T22:28:54Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpLtXjbE#match-stress-enum:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpLtXjbE/incremental-state"
[2022-01-19T22:28:56Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2022-01-19T22:28:56Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpLtXjbE#match-stress-enum:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpLtXjbE/incremental-state"
[2022-01-19T22:28:57Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-01-19T22:28:57Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-01-19T22:28:57Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpGhsbHk#match-stress-enum:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln"
[2022-01-19T22:28:59Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2022-01-19T22:28:59Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2022-01-19T22:28:59Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpGhsbHk#match-stress-enum:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpGhsbHk/incremental-state"
[2022-01-19T22:29:01Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrUnchanged), patch=None
[2022-01-19T22:29:01Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpGhsbHk#match-stress-enum:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpGhsbHk/incremental-state"
Preparing token-stream-stress
[2022-01-19T22:29:02Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=None, patch=None
[2022-01-19T22:29:02Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2022-01-19T22:29:02Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2022-01-19T22:29:02Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2022-01-19T22:29:02Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpk0P72C#token-stream-stress:0.0.0" "--profile" "check" "--bin" "token-stream-stress" "--" "--skip-this-rustc"
[2022-01-19T22:29:02Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpspxH6X#token-stream-stress:0.0.0" "--bin" "token-stream-stress" "--" "--skip-this-rustc"
[2022-01-19T22:29:02Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpqjuIeD#token-stream-stress:0.0.0" "--release" "--bin" "token-stream-stress" "--" "--skip-this-rustc"
[2022-01-19T22:29:03Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-01-19T22:29:03Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2022-01-19T22:29:03Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwxwmrU#token-stream-stress:0.0.0" "--profile" "check" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln"
[2022-01-19T22:29:03Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2022-01-19T22:29:03Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2022-01-19T22:29:03Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwxwmrU#token-stream-stress:0.0.0" "--profile" "check" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpwxwmrU/incremental-state"
[2022-01-19T22:29:03Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrUnchanged), patch=None
[2022-01-19T22:29:03Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwxwmrU#token-stream-stress:0.0.0" "--profile" "check" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpwxwmrU/incremental-state"
[2022-01-19T22:29:03Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-01-19T22:29:03Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-01-19T22:29:03Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-01-19T22:29:03Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDocVxI#token-stream-stress:0.0.0" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln"
[2022-01-19T22:29:04Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-01-19T22:29:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDocVxI#token-stream-stress:0.0.0" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpDocVxI/incremental-state"
[2022-01-19T22:29:04Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2022-01-19T22:29:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDocVxI#token-stream-stress:0.0.0" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpDocVxI/incremental-state"
[2022-01-19T22:29:04Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-01-19T22:29:04Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-01-19T22:29:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpTFfQ0Q#token-stream-stress:0.0.0" "--release" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln"
[2022-01-19T22:29:05Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
---
[RUSTC-TIMING] serde test:false 3.567
[RUSTC-TIMING] serde_json test:false 1.517
   Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
[RUSTC-TIMING] rustdoc_json_types test:false 2.395
error[E0615]: attempted to take value of method `ident` on type `&AssocItem`
    --> src/librustdoc/clean/types.rs:2225:72
     |
2225 |             .map(|did| tcx.provided_trait_methods(did).map(|meth| meth.ident.name).collect())
     |
help: use parentheses to call the method
     |
     |
2225 |             .map(|did| tcx.provided_trait_methods(did).map(|meth| meth.ident(_).name).collect())


error[E0615]: attempted to take value of method `ident` on type `&AssocItem`
   --> src/librustdoc/clean/mod.rs:390:60
    |
390 |             name: cx.tcx.associated_item(self.item_def_id).ident.name,
    |
help: use parentheses to call the method
    |
    |
390 |             name: cx.tcx.associated_item(self.item_def_id).ident(_).name,


error[E0615]: attempted to take value of method `ident` on type `&AssocItem`
    --> src/librustdoc/clean/mod.rs:1134:36
     |
1134 |                 let my_name = self.ident.name;
     |
help: use parentheses to call the method
     |
     |
1134 |                 let my_name = self.ident(_).name;


error[E0615]: attempted to take value of method `ident` on type `&AssocItem`
    --> src/librustdoc/clean/mod.rs:1200:60
     |
1200 |         Item::from_def_id_and_parts(self.def_id, Some(self.ident.name), kind, cx)
     |
help: use parentheses to call the method
     |
     |
1200 |         Item::from_def_id_and_parts(self.def_id, Some(self.ident(_).name), kind, cx)


error[E0615]: attempted to take value of method `ident` on type `&AssocItem`
    --> src/librustdoc/clean/mod.rs:1524:72
     |
1524 |                         name: cx.tcx.associated_item(pb.item_def_id()).ident.name,
     |
help: use parentheses to call the method
     |
     |
1524 |                         name: cx.tcx.associated_item(pb.item_def_id()).ident(_).name,


error[E0615]: attempted to take value of method `ident` on type `&AssocItem`
    --> src/librustdoc/clean/mod.rs:1595:50
1595 | ...                   .ident
     |                        ^^^^^ method, not a field
     |
help: use parentheses to call the method
help: use parentheses to call the method
     |
1595 |                                                 .ident(_)


error[E0615]: attempted to take value of method `ident` on type `&AssocItem`
    |
    |
431 |                     .any(|item| item.ident.name == variant_name)
    |
help: use parentheses to call the method
    |
    |
431 |                     .any(|item| item.ident(_).name == variant_name)

For more information about this error, try `rustc --explain E0615`.
[RUSTC-TIMING] rustdoc test:false 2.822
error: could not compile `rustdoc` due to 7 previous errors
