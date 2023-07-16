plain
Preparing cargo
[2021-10-20T18:41:14Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=None, patch=None
[2021-10-20T18:41:14Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2021-10-20T18:41:14Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2021-10-20T18:41:14Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKbbtKl#cargo:0.29.0" "--lib" "--" "--skip-this-rustc"
[2021-10-20T18:41:14Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpYqf86Y#cargo:0.29.0" "--profile" "check" "--lib" "--" "--skip-this-rustc"
[2021-10-20T18:41:14Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpsF8AUO#cargo:0.29.0" "--release" "--lib" "--" "--skip-this-rustc"
[2021-10-20T18:43:03Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-10-20T18:43:04Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2021-10-20T18:43:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpeqz2Me#cargo:0.29.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "eprintln"
[2021-10-20T18:43:22Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2021-10-20T18:43:22Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2021-10-20T18:43:22Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpeqz2Me#cargo:0.29.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpeqz2Me/incremental-state"
[2021-10-20T18:43:44Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrUnchanged), patch=None
[2021-10-20T18:43:44Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpeqz2Me#cargo:0.29.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpeqz2Me/incremental-state"
[2021-10-20T18:43:48Z DEBUG collector::execute] applying println to "/tmp/.tmpeqz2Me"
[2021-10-20T18:43:48Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2021-10-20T18:43:48Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2021-10-20T18:43:48Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpeqz2Me#cargo:0.29.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpeqz2Me/incremental-state"
[2021-10-20T18:43:59Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-10-20T18:44:00Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2021-10-20T18:44:00Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmphh6zYs#cargo:0.29.0" "--lib" "--" "--wrap-rustc-with" "eprintln"
[2021-10-20T18:44:38Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2021-10-20T18:44:38Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2021-10-20T18:44:38Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmphh6zYs#cargo:0.29.0" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmphh6zYs/incremental-state"
[2021-10-20T18:45:26Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2021-10-20T18:45:26Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmphh6zYs#cargo:0.29.0" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmphh6zYs/incremental-state"
[2021-10-20T18:45:35Z DEBUG collector::execute] applying println to "/tmp/.tmphh6zYs"
[2021-10-20T18:45:35Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2021-10-20T18:45:35Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2021-10-20T18:45:35Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmphh6zYs#cargo:0.29.0" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmphh6zYs/incremental-state"
[2021-10-20T18:45:55Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-10-20T18:45:55Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2021-10-20T18:45:55Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2021-10-20T18:45:55Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDsjdMl#cargo:0.29.0" "--release" "--lib" "--" "--wrap-rustc-with" "eprintln"
[2021-10-20T18:46:44Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2021-10-20T18:46:44Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDsjdMl#cargo:0.29.0" "--release" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpDsjdMl/incremental-state"
[2021-10-20T18:47:37Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrUnchanged), patch=None
[2021-10-20T18:47:37Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDsjdMl#cargo:0.29.0" "--release" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpDsjdMl/incremental-state"
[2021-10-20T18:47:47Z DEBUG collector::execute] applying println to "/tmp/.tmpDsjdMl"
[2021-10-20T18:47:47Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2021-10-20T18:47:47Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2021-10-20T18:47:47Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDsjdMl#cargo:0.29.0" "--release" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpDsjdMl/incremental-state"
Preparing ctfe-stress-4
[2021-10-20T18:48:14Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2021-10-20T18:48:14Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=None, patch=None
[2021-10-20T18:48:14Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2021-10-20T18:48:14Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2021-10-20T18:48:14Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpypBHkP#ctfe-stress-4:0.1.0" "--" "--skip-this-rustc"
[2021-10-20T18:48:14Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpRYdJka#ctfe-stress-4:0.1.0" "--release" "--" "--skip-this-rustc"
[2021-10-20T18:48:14Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpIDjvWv#ctfe-stress-4:0.1.0" "--profile" "check" "--" "--skip-this-rustc"
[2021-10-20T18:48:14Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-10-20T18:48:14Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2021-10-20T18:48:14Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2021-10-20T18:48:14Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpuwGmhN#ctfe-stress-4:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln"
[2021-10-20T18:48:37Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2021-10-20T18:48:37Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpuwGmhN#ctfe-stress-4:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpuwGmhN/incremental-state"
[2021-10-20T18:49:04Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrUnchanged), patch=None
[2021-10-20T18:49:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpuwGmhN#ctfe-stress-4:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpuwGmhN/incremental-state"
[2021-10-20T18:49:05Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-10-20T18:49:05Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2021-10-20T18:49:05Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp2iwMjK#ctfe-stress-4:0.1.0" "--" "--wrap-rustc-with" "eprintln"
[2021-10-20T18:49:27Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2021-10-20T18:49:27Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2021-10-20T18:49:27Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp2iwMjK#ctfe-stress-4:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmp2iwMjK/incremental-state"
[2021-10-20T18:49:55Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2021-10-20T18:49:55Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp2iwMjK#ctfe-stress-4:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmp2iwMjK/incremental-state"
[2021-10-20T18:49:56Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-10-20T18:49:56Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2021-10-20T18:49:56Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpNOyp61#ctfe-stress-4:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln"
[2021-10-20T18:50:18Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2021-10-20T18:50:18Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2021-10-20T18:50:18Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpNOyp61#ctfe-stress-4:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpNOyp61/incremental-state"
[2021-10-20T18:50:44Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrUnchanged), patch=None
[2021-10-20T18:50:44Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpNOyp61#ctfe-stress-4:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpNOyp61/incremental-state"
Preparing externs
[2021-10-20T18:50:45Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=None, patch=None
[2021-10-20T18:50:45Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2021-10-20T18:50:45Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2021-10-20T18:50:45Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2021-10-20T18:50:45Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpiiqLUb#externs:0.1.0" "--profile" "check" "--" "--skip-this-rustc"
[2021-10-20T18:50:45Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpAIyKoD#externs:0.1.0" "--release" "--" "--skip-this-rustc"
[2021-10-20T18:50:45Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjIDukW#externs:0.1.0" "--" "--skip-this-rustc"
[2021-10-20T18:50:46Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-10-20T18:50:46Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2021-10-20T18:50:46Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2021-10-20T18:50:46Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmplTw8Zh#externs:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln"
[2021-10-20T18:50:47Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2021-10-20T18:50:47Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmplTw8Zh#externs:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmplTw8Zh/incremental-state"
[2021-10-20T18:50:48Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrUnchanged), patch=None
[2021-10-20T18:50:48Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmplTw8Zh#externs:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmplTw8Zh/incremental-state"
[2021-10-20T18:50:49Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-10-20T18:50:49Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2021-10-20T18:50:49Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpWx1JGe#externs:0.1.0" "--" "--wrap-rustc-with" "eprintln"
[2021-10-20T18:50:50Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2021-10-20T18:50:50Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2021-10-20T18:50:50Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpWx1JGe#externs:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpWx1JGe/incremental-state"
[2021-10-20T18:50:52Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2021-10-20T18:50:52Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpWx1JGe#externs:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpWx1JGe/incremental-state"
[2021-10-20T18:50:53Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-10-20T18:50:53Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2021-10-20T18:50:53Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2021-10-20T18:50:53Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpfwWXsA#externs:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln"
[2021-10-20T18:50:54Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2021-10-20T18:50:54Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpfwWXsA#externs:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpfwWXsA/incremental-state"
[2021-10-20T18:50:56Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrUnchanged), patch=None
[2021-10-20T18:50:56Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpfwWXsA#externs:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpfwWXsA/incremental-state"
Preparing inflate
[2021-10-20T18:50:58Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2021-10-20T18:50:58Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=None, patch=None
[2021-10-20T18:50:58Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2021-10-20T18:50:58Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2021-10-20T18:50:58Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpnG0ipn#inflate:0.1.0" "--release" "--" "--skip-this-rustc"
[2021-10-20T18:50:58Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpyYyGYs#inflate:0.1.0" "--" "--skip-this-rustc"
[2021-10-20T18:50:58Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpfeQfqx#inflate:0.1.0" "--profile" "check" "--" "--skip-this-rustc"
[2021-10-20T18:50:58Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-10-20T18:50:58Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2021-10-20T18:50:58Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmprxsnfF#inflate:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln"
[2021-10-20T18:51:01Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2021-10-20T18:51:01Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2021-10-20T18:51:01Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmprxsnfF#inflate:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmprxsnfF/incremental-state"
[2021-10-20T18:51:04Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrUnchanged), patch=None
[2021-10-20T18:51:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmprxsnfF#inflate:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmprxsnfF/incremental-state"
[2021-10-20T18:51:04Z DEBUG collector::execute] applying println to "/tmp/.tmprxsnfF"
[2021-10-20T18:51:04Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2021-10-20T18:51:04Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2021-10-20T18:51:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmprxsnfF#inflate:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmprxsnfF/incremental-state"
[2021-10-20T18:51:07Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-10-20T18:51:07Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2021-10-20T18:51:07Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp8Q0LCg#inflate:0.1.0" "--" "--wrap-rustc-with" "eprintln"
[2021-10-20T18:51:11Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2021-10-20T18:51:11Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2021-10-20T18:51:11Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp8Q0LCg#inflate:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmp8Q0LCg/incremental-state"
[2021-10-20T18:51:16Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2021-10-20T18:51:16Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp8Q0LCg#inflate:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmp8Q0LCg/incremental-state"
[2021-10-20T18:51:16Z DEBUG collector::execute] applying println to "/tmp/.tmp8Q0LCg"
[2021-10-20T18:51:16Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2021-10-20T18:51:16Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2021-10-20T18:51:16Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp8Q0LCg#inflate:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmp8Q0LCg/incremental-state"
[2021-10-20T18:51:20Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-10-20T18:51:20Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2021-10-20T18:51:20Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2021-10-20T18:51:20Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpsaeSRP#inflate:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln"
[2021-10-20T18:51:27Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2021-10-20T18:51:27Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpsaeSRP#inflate:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpsaeSRP/incremental-state"
[2021-10-20T18:51:34Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrUnchanged), patch=None
[2021-10-20T18:51:34Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpsaeSRP#inflate:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpsaeSRP/incremental-state"
[2021-10-20T18:51:34Z DEBUG collector::execute] applying println to "/tmp/.tmpsaeSRP"
[2021-10-20T18:51:34Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2021-10-20T18:51:34Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2021-10-20T18:51:34Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpsaeSRP#inflate:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpsaeSRP/incremental-state"
Preparing match-stress-enum
[2021-10-20T18:51:41Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=None, patch=None
[2021-10-20T18:51:41Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2021-10-20T18:51:41Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2021-10-20T18:51:41Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2021-10-20T18:51:41Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOA75Nz#match-stress-enum:0.1.0" "--release" "--" "--skip-this-rustc"
[2021-10-20T18:51:41Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpurosb3#match-stress-enum:0.1.0" "--" "--skip-this-rustc"
[2021-10-20T18:51:41Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpftSABn#match-stress-enum:0.1.0" "--profile" "check" "--" "--skip-this-rustc"
[2021-10-20T18:51:42Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-10-20T18:51:42Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2021-10-20T18:51:42Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2021-10-20T18:51:42Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpvCJxzH#match-stress-enum:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln"
[2021-10-20T18:51:44Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2021-10-20T18:51:44Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpvCJxzH#match-stress-enum:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpvCJxzH/incremental-state"
[2021-10-20T18:51:46Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrUnchanged), patch=None
[2021-10-20T18:51:46Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpvCJxzH#match-stress-enum:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpvCJxzH/incremental-state"
[2021-10-20T18:51:47Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-10-20T18:51:47Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2021-10-20T18:51:47Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2021-10-20T18:51:47Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOYqDAt#match-stress-enum:0.1.0" "--" "--wrap-rustc-with" "eprintln"
[2021-10-20T18:51:49Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2021-10-20T18:51:49Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOYqDAt#match-stress-enum:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpOYqDAt/incremental-state"
[2021-10-20T18:51:52Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2021-10-20T18:51:52Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOYqDAt#match-stress-enum:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpOYqDAt/incremental-state"
[2021-10-20T18:51:53Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-10-20T18:51:53Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2021-10-20T18:51:53Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp19by3x#match-stress-enum:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln"
[2021-10-20T18:51:55Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
---
[2021-10-20T18:52:00Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-10-20T18:52:00Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2021-10-20T18:52:00Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjc6NuX#token-stream-stress:0.0.0" "--profile" "check" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln"
[2021-10-20T18:52:00Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2021-10-20T18:52:00Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjc6NuX#token-stream-stress:0.0.0" "--profile" "check" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpjc6NuX/incremental-state"
[2021-10-20T18:52:00Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrUnchanged), patch=None
[2021-10-20T18:52:00Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjc6NuX#token-stream-stress:0.0.0" "--profile" "check" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpjc6NuX/incremental-state"
[2021-10-20T18:52:01Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-10-20T18:52:01Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2021-10-20T18:52:01Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2021-10-20T18:52:01Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmphnczPi#token-stream-stress:0.0.0" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln"
[2021-10-20T18:52:01Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2021-10-20T18:52:01Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmphnczPi#token-stream-stress:0.0.0" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmphnczPi/incremental-state"
[2021-10-20T18:52:02Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2021-10-20T18:52:02Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmphnczPi#token-stream-stress:0.0.0" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmphnczPi/incremental-state"
[2021-10-20T18:52:02Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-10-20T18:52:02Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2021-10-20T18:52:02Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2021-10-20T18:52:02Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpLgKZKo#token-stream-stress:0.0.0" "--release" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln"
[2021-10-20T18:52:02Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2021-10-20T18:52:02Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpLgKZKo#token-stream-stress:0.0.0" "--release" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpLgKZKo/incremental-state"
[2021-10-20T18:52:03Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrUnchanged), patch=None
[2021-10-20T18:52:03Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpLgKZKo#token-stream-stress:0.0.0" "--release" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpLgKZKo/incremental-state"
+ ./build/x86_64-unknown-linux-gnu/llvm/bin/llvm-profdata merge -o /tmp/rustc-pgo.profdata /tmp/rustc-pgo
+ rm -r ./build/x86_64-unknown-linux-gnu/llvm ./build/x86_64-unknown-linux-gnu/lld
+ python3 ../x.py dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu --include-default-paths src/tools/build-manifest --rust-profile-use=/tmp/rustc-pgo.profdata --llvm-profile-use=/tmp/llvm-pgo.profdata
    Finished dev [unoptimized] target(s) in 0.18s
---
Documenting stage2 std (x86_64-unknown-linux-gnu)
Uplifting stage1 std (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
Copying stage2 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
 Documenting core v0.0.0 (/checkout/library/core)
thread 'rustc' panicked at 'Unexpected result when selecting str::iter::EscapeDebug<'a> Obligation(predicate=Binder(ProjectionPredicate(ProjectionTy { substs: [_], item_def_id: DefId(0:7422 ~ core[c3bd]::iter::traits::collect::IntoIterator::IntoIter) }, _), []), depth=2)', compiler/rustc_trait_selection/src/traits/auto_trait.rs:771:33

error: internal compiler error: unexpected panic

error: Unrecognized option: 'markdown-css'
error: Unrecognized option: 'markdown-css'

error: could not document `core`

Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type lib --crate-name core library/core/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc --error-format=json --json=diagnostic-rendered-ansi --markdown-css rust.css --markdown-no-toc -Z unstable-options --resource-suffix 1.58.0 --index-page /checkout/src/doc/index.md -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/deps -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.58.0-nightly
  (2bf0405fa
  2021-10-20)' '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")'` (exit status: 1)


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace profiler compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "-Zskip-rustdoc-fingerprint" "--" "--markdown-css" "rust.css" "--markdown-no-toc" "-Z" "unstable-options" "--resource-suffix" "1.58.0" "--index-page" "/checkout/src/doc/index.md"


Build completed unsuccessfully in 0:25:41
