plain
8 benchmarks remaining
Preparing cargo
[2022-03-15T16:16:37Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2022-03-15T16:16:37Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2022-03-15T16:16:37Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpkgGmWr#cargo:0.29.0" "--release" "--lib" "--" "--skip-this-rustc"
[2022-03-15T16:16:37Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwPCh6d#cargo:0.29.0" "--lib" "--" "--skip-this-rustc"
[2022-03-15T16:18:04Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-15T16:18:05Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-03-15T16:18:05Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJ23rpL#cargo:0.29.0" "--lib" "--" "--wrap-rustc-with" "eprintln"
Running cargo: Opt + [Full]
Running cargo: Opt + [Full]
[2022-03-15T16:18:26Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-15T16:18:26Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-03-15T16:18:26Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCERXhf#cargo:0.29.0" "--release" "--lib" "--" "--wrap-rustc-with" "eprintln"
7 benchmarks remaining
Preparing clap-rs
[2022-03-15T16:19:47Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2022-03-15T16:19:47Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2022-03-15T16:19:47Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjpGBNJ#clap:2.29.0" "--" "--skip-this-rustc"
[2022-03-15T16:19:47Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpMmBgvx#clap:2.29.0" "--release" "--" "--skip-this-rustc"
[2022-03-15T16:19:48Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-15T16:19:48Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-03-15T16:19:48Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-03-15T16:19:48Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpdIxH5r#clap:2.29.0" "--" "--wrap-rustc-with" "eprintln"
[2022-03-15T16:19:56Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-15T16:19:56Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-03-15T16:19:56Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpK548Dz#clap:2.29.0" "--release" "--" "--wrap-rustc-with" "eprintln"
6 benchmarks remaining
6 benchmarks remaining
Preparing deeply-nested-async
[2022-03-15T16:20:09Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2022-03-15T16:20:09Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2022-03-15T16:20:09Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDMYeeO#deeply-nested-async:0.1.0" "--" "--skip-this-rustc"
[2022-03-15T16:20:09Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpN9ZrJ0#deeply-nested-async:0.1.0" "--release" "--" "--skip-this-rustc"
[2022-03-15T16:20:10Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-15T16:20:10Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-03-15T16:20:10Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmplR2U0S#deeply-nested-async:0.1.0" "--" "--wrap-rustc-with" "eprintln"
Running deeply-nested-async: Opt + [Full]
Running deeply-nested-async: Opt + [Full]
[2022-03-15T16:20:11Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-15T16:20:11Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-03-15T16:20:11Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpl6W1Xm#deeply-nested-async:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln"
5 benchmarks remaining
Preparing hyper-2
[2022-03-15T16:20:11Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2022-03-15T16:20:11Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2022-03-15T16:20:11Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFJbYGN#hyper:0.13.0-alpha.4" "--" "--skip-this-rustc"
[2022-03-15T16:20:11Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmplFNIA5#hyper:0.13.0-alpha.4" "--release" "--" "--skip-this-rustc"
[2022-03-15T16:20:37Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-15T16:20:37Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-03-15T16:20:37Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpvGN6qt#hyper:0.13.0-alpha.4" "--" "--wrap-rustc-with" "eprintln"
Running hyper-2: Opt + [Full]
Running hyper-2: Opt + [Full]
[2022-03-15T16:20:40Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-15T16:20:41Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-03-15T16:20:41Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpEzr7IQ#hyper:0.13.0-alpha.4" "--release" "--" "--wrap-rustc-with" "eprintln"
Preparing regex
[2022-03-15T16:20:47Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2022-03-15T16:20:47Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2022-03-15T16:20:47Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2022-03-15T16:20:47Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpYMkOzx#regex:0.1.80" "--release" "--" "--skip-this-rustc"
[2022-03-15T16:20:47Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDcUOzA#regex:0.1.80" "--" "--skip-this-rustc"
[2022-03-15T16:20:53Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-15T16:20:54Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-03-15T16:20:54Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-03-15T16:20:54Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDTYdoZ#regex:0.1.80" "--" "--wrap-rustc-with" "eprintln"
[2022-03-15T16:20:57Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-15T16:20:57Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-03-15T16:20:57Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpT2XZOW#regex:0.1.80" "--release" "--" "--wrap-rustc-with" "eprintln"
3 benchmarks remaining
3 benchmarks remaining
Preparing ripgrep
[2022-03-15T16:21:07Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2022-03-15T16:21:07Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2022-03-15T16:21:07Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpne1Wbk#ripgrep:0.8.1" "--" "--skip-this-rustc"
[2022-03-15T16:21:07Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpvXm2Wh#ripgrep:0.8.1" "--release" "--" "--skip-this-rustc"
[2022-03-15T16:21:35Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-15T16:21:35Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-03-15T16:21:35Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-03-15T16:21:35Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpIJspU6#ripgrep:0.8.1" "--" "--wrap-rustc-with" "eprintln"
[2022-03-15T16:21:40Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-15T16:21:40Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-03-15T16:21:40Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-03-15T16:21:40Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpoCm5wk#ripgrep:0.8.1" "--release" "--" "--wrap-rustc-with" "eprintln"
Preparing serde
[2022-03-15T16:21:58Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2022-03-15T16:21:58Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2022-03-15T16:21:58Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "serde/Cargo.toml" "-p" "file:///tmp/.tmpCz6ho0/serde#1.0.37" "--" "--skip-this-rustc"
[2022-03-15T16:21:58Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "serde/Cargo.toml" "-p" "file:///tmp/.tmpCz6ho0/serde#1.0.37" "--" "--skip-this-rustc"
[2022-03-15T16:21:58Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "serde/Cargo.toml" "-p" "file:///tmp/.tmp2hgnsF/serde#1.0.37" "--release" "--" "--skip-this-rustc"
[2022-03-15T16:21:59Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-15T16:21:59Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-03-15T16:21:59Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-03-15T16:21:59Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "serde/Cargo.toml" "-p" "file:///tmp/.tmpZioHMw/serde#1.0.37" "--" "--wrap-rustc-with" "eprintln"
[2022-03-15T16:22:02Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-15T16:22:02Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-03-15T16:22:02Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-03-15T16:22:02Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "serde/Cargo.toml" "-p" "file:///tmp/.tmpFXcMHE/serde#1.0.37" "--release" "--" "--wrap-rustc-with" "eprintln"
Preparing syn
[2022-03-15T16:22:06Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2022-03-15T16:22:06Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2022-03-15T16:22:06Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpcD8FOe#syn:0.11.11" "--" "--skip-this-rustc"
---
[2022-03-15T16:38:03Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-15T16:38:04Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-03-15T16:38:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpsmM9wv#cargo:0.29.0" "--lib" "--" "--wrap-rustc-with" "eprintln"
[2022-03-15T16:38:32Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-03-15T16:38:32Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpsmM9wv#cargo:0.29.0" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpsmM9wv/incremental-state"
[2022-03-15T16:39:08Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2022-03-15T16:39:08Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpsmM9wv#cargo:0.29.0" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpsmM9wv/incremental-state"
[2022-03-15T16:39:15Z DEBUG collector::execute] applying println to "/tmp/.tmpsmM9wv"
[2022-03-15T16:39:15Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-03-15T16:39:15Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-03-15T16:39:15Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpsmM9wv#cargo:0.29.0" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpsmM9wv/incremental-state"
[2022-03-15T16:39:30Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-15T16:39:31Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-03-15T16:39:31Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjpdZiw#cargo:0.29.0" "--release" "--lib" "--" "--wrap-rustc-with" "eprintln"
[2022-03-15T16:40:08Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2022-03-15T16:40:08Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2022-03-15T16:40:08Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjpdZiw#cargo:0.29.0" "--release" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpjpdZiw/incremental-state"
[2022-03-15T16:40:48Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrUnchanged), patch=None
[2022-03-15T16:40:48Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjpdZiw#cargo:0.29.0" "--release" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpjpdZiw/incremental-state"
[2022-03-15T16:40:55Z DEBUG collector::execute] applying println to "/tmp/.tmpjpdZiw"
[2022-03-15T16:40:55Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-03-15T16:40:55Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-03-15T16:40:55Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjpdZiw#cargo:0.29.0" "--release" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpjpdZiw/incremental-state"
Preparing ctfe-stress-4
[2022-03-15T16:41:15Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=None, patch=None
[2022-03-15T16:41:15Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2022-03-15T16:41:15Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
---
[2022-03-15T16:41:16Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-15T16:41:16Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2022-03-15T16:41:16Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpmLaSUN#ctfe-stress-4:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln"
[2022-03-15T16:41:41Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2022-03-15T16:41:41Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpmLaSUN#ctfe-stress-4:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpmLaSUN/incremental-state"
[2022-03-15T16:42:09Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrUnchanged), patch=None
[2022-03-15T16:42:09Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpmLaSUN#ctfe-stress-4:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpmLaSUN/incremental-state"
[2022-03-15T16:42:10Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-15T16:42:10Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-03-15T16:42:10Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpiV4ZRt#ctfe-stress-4:0.1.0" "--" "--wrap-rustc-with" "eprintln"
[2022-03-15T16:42:35Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-03-15T16:42:35Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-03-15T16:42:35Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpiV4ZRt#ctfe-stress-4:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpiV4ZRt/incremental-state"
[2022-03-15T16:43:04Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2022-03-15T16:43:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpiV4ZRt#ctfe-stress-4:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpiV4ZRt/incremental-state"
[2022-03-15T16:43:05Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-15T16:43:05Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-03-15T16:43:05Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpZGhbOT#ctfe-stress-4:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln"
[2022-03-15T16:43:28Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2022-03-15T16:43:28Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2022-03-15T16:43:28Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpZGhbOT#ctfe-stress-4:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpZGhbOT/incremental-state"
[2022-03-15T16:43:54Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrUnchanged), patch=None
[2022-03-15T16:43:54Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpZGhbOT#ctfe-stress-4:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpZGhbOT/incremental-state"
Preparing externs
[2022-03-15T16:43:55Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=None, patch=None
[2022-03-15T16:43:55Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2022-03-15T16:43:55Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
---
[2022-03-15T16:43:56Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-15T16:43:56Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2022-03-15T16:43:56Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKbCWvL#externs:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln"
[2022-03-15T16:43:57Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2022-03-15T16:43:57Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKbCWvL#externs:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpKbCWvL/incremental-state"
[2022-03-15T16:43:58Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrUnchanged), patch=None
[2022-03-15T16:43:58Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKbCWvL#externs:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpKbCWvL/incremental-state"
[2022-03-15T16:43:58Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-15T16:43:58Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-03-15T16:43:58Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpq70qww#externs:0.1.0" "--" "--wrap-rustc-with" "eprintln"
[2022-03-15T16:43:59Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
---
[2022-03-15T16:44:01Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-15T16:44:01Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-03-15T16:44:02Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpEd7aUM#externs:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln"
[2022-03-15T16:44:02Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2022-03-15T16:44:02Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpEd7aUM#externs:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpEd7aUM/incremental-state"
[2022-03-15T16:44:04Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrUnchanged), patch=None
[2022-03-15T16:44:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpEd7aUM#externs:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpEd7aUM/incremental-state"
Preparing inflate
[2022-03-15T16:44:05Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2022-03-15T16:44:05Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=None, patch=None
[2022-03-15T16:44:05Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2022-03-15T16:44:05Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2022-03-15T16:44:05Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpj2e3fr#inflate:0.1.0" "--profile" "check" "--" "--skip-this-rustc"
[2022-03-15T16:44:05Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmppCZCAm#inflate:0.1.0" "--" "--skip-this-rustc"
[2022-03-15T16:44:05Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpTG5yKI#inflate:0.1.0" "--release" "--" "--skip-this-rustc"
[2022-03-15T16:44:05Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-15T16:44:05Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2022-03-15T16:44:05Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp4SKCCQ#inflate:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln"
[2022-03-15T16:44:08Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2022-03-15T16:44:08Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2022-03-15T16:44:08Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp4SKCCQ#inflate:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmp4SKCCQ/incremental-state"
[2022-03-15T16:44:11Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrUnchanged), patch=None
[2022-03-15T16:44:11Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp4SKCCQ#inflate:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmp4SKCCQ/incremental-state"
[2022-03-15T16:44:12Z DEBUG collector::execute] applying println to "/tmp/.tmp4SKCCQ"
[2022-03-15T16:44:12Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-03-15T16:44:12Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-03-15T16:44:12Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp4SKCCQ#inflate:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmp4SKCCQ/incremental-state"
[2022-03-15T16:44:15Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-15T16:44:15Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-03-15T16:44:15Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpkYaYu0#inflate:0.1.0" "--" "--wrap-rustc-with" "eprintln"
[2022-03-15T16:44:18Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-03-15T16:44:18Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-03-15T16:44:18Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpkYaYu0#inflate:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpkYaYu0/incremental-state"
[2022-03-15T16:44:22Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2022-03-15T16:44:22Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpkYaYu0#inflate:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpkYaYu0/incremental-state"
[2022-03-15T16:44:23Z DEBUG collector::execute] applying println to "/tmp/.tmpkYaYu0"
[2022-03-15T16:44:23Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-03-15T16:44:23Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-03-15T16:44:23Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpkYaYu0#inflate:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpkYaYu0/incremental-state"
[2022-03-15T16:44:27Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-15T16:44:27Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-03-15T16:44:27Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpXPky00#inflate:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln"
[2022-03-15T16:44:33Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
---
[2022-03-15T16:44:47Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-15T16:44:47Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2022-03-15T16:44:47Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjEjykv#match-stress-enum:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln"
[2022-03-15T16:44:49Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2022-03-15T16:44:49Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjEjykv#match-stress-enum:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpjEjykv/incremental-state"
[2022-03-15T16:44:52Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrUnchanged), patch=None
[2022-03-15T16:44:52Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjEjykv#match-stress-enum:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpjEjykv/incremental-state"
[2022-03-15T16:44:52Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-15T16:44:52Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-03-15T16:44:52Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpA3xjCP#match-stress-enum:0.1.0" "--" "--wrap-rustc-with" "eprintln"
[2022-03-15T16:44:55Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-03-15T16:44:55Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-03-15T16:44:55Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpA3xjCP#match-stress-enum:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpA3xjCP/incremental-state"
[2022-03-15T16:44:57Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2022-03-15T16:44:57Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpA3xjCP#match-stress-enum:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpA3xjCP/incremental-state"
[2022-03-15T16:44:58Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-15T16:44:58Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-03-15T16:44:58Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp0vWtqr#match-stress-enum:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln"
[2022-03-15T16:45:00Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
---
Preparing token-stream-stress
[2022-03-15T16:45:04Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2022-03-15T16:45:04Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=None, patch=None
[2022-03-15T16:45:04Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2022-03-15T16:45:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpdu9A7M#token-stream-stress:0.0.0" "--profile" "check" "--bin" "token-stream-stress" "--" "--skip-this-rustc"
[2022-03-15T16:45:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpVUNiRt#token-stream-stress:0.0.0" "--bin" "token-stream-stress" "--" "--skip-this-rustc"
[2022-03-15T16:45:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpU43npe#token-stream-stress:0.0.0" "--release" "--bin" "token-stream-stress" "--" "--skip-this-rustc"
[2022-03-15T16:45:05Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-15T16:45:05Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2022-03-15T16:45:05Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2022-03-15T16:45:05Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpcGYGHr#token-stream-stress:0.0.0" "--profile" "check" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln"
[2022-03-15T16:45:05Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2022-03-15T16:45:05Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpcGYGHr#token-stream-stress:0.0.0" "--profile" "check" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpcGYGHr/incremental-state"
[2022-03-15T16:45:05Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrUnchanged), patch=None
[2022-03-15T16:45:05Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpcGYGHr#token-stream-stress:0.0.0" "--profile" "check" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpcGYGHr/incremental-state"
[2022-03-15T16:45:06Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-15T16:45:06Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-03-15T16:45:06Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpO2rY9G#token-stream-stress:0.0.0" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln"
[2022-03-15T16:45:06Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-03-15T16:45:06Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-03-15T16:45:06Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpO2rY9G#token-stream-stress:0.0.0" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpO2rY9G/incremental-state"
[2022-03-15T16:45:06Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2022-03-15T16:45:06Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpO2rY9G#token-stream-stress:0.0.0" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpO2rY9G/incremental-state"
[2022-03-15T16:45:07Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-15T16:45:07Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-03-15T16:45:07Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpaoB8lM#token-stream-stress:0.0.0" "--release" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln"
[2022-03-15T16:45:07Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2022-03-15T16:45:07Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2022-03-15T16:45:07Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpaoB8lM#token-stream-stress:0.0.0" "--release" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpaoB8lM/incremental-state"
[2022-03-15T16:45:08Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrUnchanged), patch=None
[2022-03-15T16:45:08Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpaoB8lM#token-stream-stress:0.0.0" "--release" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpaoB8lM/incremental-state"
+ ./build/x86_64-unknown-linux-gnu/llvm/bin/llvm-profdata merge -o /tmp/rustc-pgo.profdata /tmp/rustc-pgo
+ rm -r ./build/x86_64-unknown-linux-gnu/llvm ./build/x86_64-unknown-linux-gnu/lld
+ python3 ../x.py dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu --include-default-paths build-manifest --rust-profile-use=/tmp/rustc-pgo.profdata --llvm-profile-use=/tmp/llvm-pgo.profdata
warning: sqlite3 not available in python, skipping build directory lock
---
[RUSTC-TIMING] askama test:false 0.118
[RUSTC-TIMING] askama_shared test:false 5.955
[RUSTC-TIMING] rustdoc test:false 30.204
   Compiling rustdoc-tool v0.0.0 (/checkout/src/tools/rustdoc)
error: linking with `clang` failed: exit status: 1
  |
  = note: "clang" "-m64" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/rustdoc_tool_binary-fefad6b3bd361938.rustdoc_tool_binary.04a05e97-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/rustdoc_tool_binary-fefad6b3bd361938.rustdoc_tool_binary.04a05e97-cgu.1.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/rustdoc_tool_binary-fefad6b3bd361938.rustdoc_tool_binary.04a05e97-cgu.2.rcgu.o" "-Wl,--as-needed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/librustdoc-0c52010ab1bab2f9.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libtracing_tree-9dc1a959dd0d95b2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libtracing_log-43acc81fc7dd3a43.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/liblog-beefe794ad37925a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libatty-54a6251feb8da8cb.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libminifier-e43696650af0813a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libmacro_utils-9b503b5f7ebc1e93.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libserde_json-a49e92da93d29023.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libryu-cb9e3e0a42560a92.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libitoa-de64c572714d69e3.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/librayon-10151a058b4faa51.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/librayon_core-84e4e651ca9d5261.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libnum_cpus-5eae106ec17855b7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libcrossbeam_deque-01932d71980c3f2c.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libcrossbeam_epoch-e4f2ba48cd7d7347.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libmemoffset-372ad4970b9a0026.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libcrossbeam_channel-bca527c9aa261f2f.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libcrossbeam_utils-d6535897d246dc41.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libtracing_subscriber-2db37cc5f5960634.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-2ce5a388ac1a4a65.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot_core-af640f3500b9388b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/liblock_api-494f9cd6fc1af273.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libscopeguard-83157a8667a85f39.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libinstant-8a0310dda8517c5d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libthread_local-0604afdffdb690cd.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libonce_cell-6fdc45dd6f39091d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libsharded_slab-555b79017d467ab3.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libansi_term-b3bcfd17b20fc8f3.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libmatchers-4002d3eafd3506f8.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libregex_automata-c571d1da5c4a06d9.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libsmallvec-2f549419de408ef1.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libregex-e432b4fe6d46993c.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libaho_corasick-171e5a018d821371.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libregex_syntax-fda76b4bb702a007.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/librustdoc_json_types-3a198885e8f40087.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-e7752a83fb14860d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libeither-a070c4bf6884643d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libpulldown_cmark-09f220767cb2101f.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libbitflags-98398ad71f50d009.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libunicase-040c64978b6fb771.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libaskama-4410a41b796620e6.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libaskama_shared-a5addc5422be7466.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libtoml-5c157d0398bcd2cf.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-2e54a535fd884cfd.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libnom-a8237f70b0e890a8.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libmemchr-9c857b6470cb1ecc.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libquote-e84b0e199fea1c4d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libproc_macro2-a2a9f4fe22713ca5.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libunicode_xid-24545c9a51470b4c.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libaskama_escape-a36a65491317bcf8.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libserde-c2c98ff4372f4fd9.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libtempfile-3e450c2b8d217a77.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/librand-8f9b850b06e5c7af.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/librand_chacha-4f8c6fd90be4c2f4.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libppv_lite86-441f698e1b4beaff.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/librand_core-a80945fa4af51c02.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libgetrandom-7bdbe7aa7b97c176.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/liblibc-8da9668fc7bc1639.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libcfg_if-44405d85f13eb05d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libremove_dir_all-157372ebc45218f3.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libarrayvec-5a3ff19b072c6de5.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libtikv_jemalloc_sys-e0fa2a4725ab11f9.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bdynamic" "-ltest-a2772fbe70d998e8" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-lrustc_driver-74fd03a38d82f654" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libtracing-dd2a64e6ef2305d4.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libpin_project_lite-82d8dd62725b5baa.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libtracing_core-3b3ab99ca05f5079.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/liblazy_static-51bff9a82b8e1588.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libcfg_if-aa13a8868d46f835.rlib" "-Wl,--start-group" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bdynamic" "-lstd-1b64d5fe7a3c3d7f" "-Wl,--end-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-1f103368fa522bc0.rlib" "-Wl,-Bdynamic" "-lpthread" "-lLLVM-14-rust-1.61.0-nightly" "-ldl" "-lpsm_s" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/rustdoc_tool_binary-fefad6b3bd361938" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro,-znow" "-Wl,-O1" "-nodefaultlibs" "-Wl,-z,origin" "-Wl,-rpath,$ORIGIN/../lib"
  = note: /rustroot/bin/ld: cannot find -lpsm_s
          clang-13: error: linker command failed with exit code 1 (use -v to see invocation)

[RUSTC-TIMING] rustdoc_tool_binary test:false 0.673
error: could not compile `rustdoc-tool` due to previous error
Build completed unsuccessfully in 0:22:25
