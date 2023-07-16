plain
[RUSTC-TIMING] remove_dir_all test:false 0.037
   Compiling arrayvec v0.7.0
[RUSTC-TIMING] once_cell test:false 0.242
[RUSTC-TIMING] build_script_build test:false 0.259
   Compiling rustc-hash v1.1.0 (https://github.com/lqd/rustc-hash/?branch=experiment-a#5847e17a)
[RUSTC-TIMING] build_script_build test:false 0.293
   Compiling rustc_graphviz v0.0.0 (/checkout/compiler/rustc_graphviz)
   Compiling cpufeatures v0.2.1
[RUSTC-TIMING] rustc_hash test:false 0.048
---
[RUSTC-TIMING] remove_dir_all test:false 0.106
[RUSTC-TIMING] once_cell test:false 0.413
   Compiling rustc_graphviz v0.0.0 (/checkout/compiler/rustc_graphviz)
[RUSTC-TIMING] build_script_build test:false 0.426
   Compiling rustc-hash v1.1.0 (https://github.com/lqd/rustc-hash/?branch=experiment-a#5847e17a)
   Compiling unicode-width v0.1.8
[RUSTC-TIMING] arrayvec test:false 0.247
   Compiling scoped-tls v1.0.0
[RUSTC-TIMING] build_script_build test:false 0.520
---
Executing benchmark clap-3.1.6 (2/7)
Preparing clap-3.1.6
[2022-05-26T08:24:22Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-26T08:24:22Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-26T08:24:22Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpHkAqDJ#clap:3.1.6" "--release" "--" "--skip-this-rustc"
[2022-05-26T08:24:22Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpyRxBRR#clap:3.1.6" "--" "--skip-this-rustc"
[2022-05-26T08:24:24Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T08:24:25Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-26T08:24:25Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpYFNqoe#clap:3.1.6" "--" "--wrap-rustc-with" "Eprintln"
Running clap-3.1.6: Opt + [Full]
---
[2022-05-26T08:25:31Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpGid2cj#regex:1.5.5" "--" "--wrap-rustc-with" "Eprintln"
Running regex-1.5.5: Opt + [Full]
[2022-05-26T08:25:34Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T08:25:34Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-26T08:25:34Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCeJjTE#regex:1.5.5" "--release" "--" "--wrap-rustc-with" "Eprintln"
Executing benchmark ripgrep-13.0.0 (5/7)
Preparing ripgrep-13.0.0
[2022-05-26T08:25:47Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-26T08:25:47Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
---
[2022-05-26T08:26:14Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCD3Fpa#ripgrep:13.0.0" "--" "--wrap-rustc-with" "Eprintln"
Running ripgrep-13.0.0: Opt + [Full]
[2022-05-26T08:26:18Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T08:26:19Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-26T08:26:19Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpmfNplV#ripgrep:13.0.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
Executing benchmark serde-1.0.136 (6/7)
Preparing serde-1.0.136
[2022-05-26T08:26:37Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-26T08:26:37Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
---
[RUSTC-TIMING] build_script_build test:false 0.304
   Compiling rustc_graphviz v0.0.0 (/checkout/compiler/rustc_graphviz)
[RUSTC-TIMING] cpufeatures test:false 0.037
[RUSTC-TIMING] build_script_build test:false 0.318
   Compiling rustc-hash v1.1.0 (https://github.com/lqd/rustc-hash/?branch=experiment-a#5847e17a)
   Compiling unicode-width v0.1.8
[RUSTC-TIMING] rustc_hash test:false 0.060
   Compiling unic-char-range v0.9.0
[RUSTC-TIMING] tinystr test:false 0.235
---
Preparing bitmaps-3.1.0
[2022-05-26T08:38:18Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-05-26T08:38:18Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-26T08:38:18Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-26T08:38:18Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQnCAk1#bitmaps:3.1.0" "--profile" "check" "--" "--skip-this-rustc"
[2022-05-26T08:38:18Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpTQJMdc#bitmaps:3.1.0" "--" "--skip-this-rustc"
[2022-05-26T08:38:18Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp8h3SM3#bitmaps:3.1.0" "--release" "--" "--skip-this-rustc"
[2022-05-26T08:38:18Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T08:38:18Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-05-26T08:38:18Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-05-26T08:38:18Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpudKnik#bitmaps:3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T08:38:20Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-05-26T08:38:20Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpudKnik#bitmaps:3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpudKnik/incremental-state"
[2022-05-26T08:38:23Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-05-26T08:38:23Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpudKnik#bitmaps:3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpudKnik/incremental-state"
[2022-05-26T08:38:24Z DEBUG collector::execute] applying println to "/tmp/.tmpudKnik"
[2022-05-26T08:38:24Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-26T08:38:24Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-26T08:38:24Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpudKnik#bitmaps:3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpudKnik/incremental-state"
[2022-05-26T08:38:25Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T08:38:25Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-26T08:38:25Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp0UqPHw#bitmaps:3.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T08:38:27Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-26T08:38:27Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-26T08:38:27Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp0UqPHw#bitmaps:3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp0UqPHw/incremental-state"
[2022-05-26T08:38:30Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-05-26T08:38:30Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp0UqPHw#bitmaps:3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp0UqPHw/incremental-state"
[2022-05-26T08:38:31Z DEBUG collector::execute] applying println to "/tmp/.tmp0UqPHw"
[2022-05-26T08:38:31Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-26T08:38:31Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-26T08:38:31Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp0UqPHw#bitmaps:3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp0UqPHw/incremental-state"
[2022-05-26T08:38:32Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T08:38:32Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-26T08:38:32Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp6Dvace#bitmaps:3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T08:38:34Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-26T08:38:34Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-26T08:38:34Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp6Dvace#bitmaps:3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp6Dvace/incremental-state"
[2022-05-26T08:38:37Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-05-26T08:38:37Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp6Dvace#bitmaps:3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp6Dvace/incremental-state"
[2022-05-26T08:38:38Z DEBUG collector::execute] applying println to "/tmp/.tmp6Dvace"
[2022-05-26T08:38:38Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-26T08:38:38Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-26T08:38:38Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp6Dvace#bitmaps:3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp6Dvace/incremental-state"
Executing benchmark cargo-0.60.0 (2/8)
Preparing cargo-0.60.0
[2022-05-26T08:38:39Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-05-26T08:38:39Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
---
[2022-05-26T08:42:41Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T08:42:42Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-26T08:42:42Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpkndbTi#cargo:0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T08:43:39Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-26T08:43:39Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpkndbTi#cargo:0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpkndbTi/incremental-state"
[2022-05-26T08:44:37Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-05-26T08:44:37Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpkndbTi#cargo:0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpkndbTi/incremental-state"
[2022-05-26T08:44:48Z DEBUG collector::execute] applying println to "/tmp/.tmpkndbTi"
[2022-05-26T08:44:48Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-26T08:44:48Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-26T08:44:48Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpkndbTi#cargo:0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpkndbTi/incremental-state"
Executing benchmark ctfe-stress-5 (3/8)
Preparing ctfe-stress-5
[2022-05-26T08:45:16Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-05-26T08:45:16Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
---
[2022-05-26T08:45:16Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T08:45:16Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-05-26T08:45:16Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpdeRyJV#ctfe-stress-5:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T08:45:24Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-05-26T08:45:24Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpdeRyJV#ctfe-stress-5:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpdeRyJV/incremental-state"
[2022-05-26T08:45:33Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-05-26T08:45:33Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpdeRyJV#ctfe-stress-5:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpdeRyJV/incremental-state"
[2022-05-26T08:45:33Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T08:45:33Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-26T08:45:33Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmptiswCF#ctfe-stress-5:0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T08:45:41Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-26T08:45:41Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-26T08:45:41Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmptiswCF#ctfe-stress-5:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmptiswCF/incremental-state"
[2022-05-26T08:45:50Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-05-26T08:45:50Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmptiswCF#ctfe-stress-5:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmptiswCF/incremental-state"
[2022-05-26T08:45:51Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T08:45:51Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-26T08:45:51Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpcWR2Vy#ctfe-stress-5:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T08:45:58Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-26T08:45:58Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-26T08:45:58Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpcWR2Vy#ctfe-stress-5:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpcWR2Vy/incremental-state"
[2022-05-26T08:46:07Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-05-26T08:46:07Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpcWR2Vy#ctfe-stress-5:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpcWR2Vy/incremental-state"
Executing benchmark diesel-1.4.8 (4/8)
Preparing diesel-1.4.8
[2022-05-26T08:46:07Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-26T08:46:07Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-26T08:46:07Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-26T08:46:07Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-05-26T08:46:07Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpEVjvVe#diesel:1.4.8" "--" "--skip-this-rustc"
[2022-05-26T08:46:07Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpHtbUGV#diesel:1.4.8" "--release" "--" "--skip-this-rustc"
[2022-05-26T08:46:07Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp0DMJwt#diesel:1.4.8" "--profile" "check" "--" "--skip-this-rustc"
[2022-05-26T08:46:26Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T08:46:26Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-05-26T08:46:26Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp34D3Qd#diesel:1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T08:46:41Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
---
[2022-05-26T08:47:06Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T08:47:06Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-26T08:47:06Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmps2hTdb#diesel:1.4.8" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T08:47:23Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-26T08:47:23Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmps2hTdb#diesel:1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmps2hTdb/incremental-state"
[2022-05-26T08:47:44Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-05-26T08:47:44Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmps2hTdb#diesel:1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmps2hTdb/incremental-state"
[2022-05-26T08:47:47Z DEBUG collector::execute] applying println to "/tmp/.tmps2hTdb"
[2022-05-26T08:47:47Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-26T08:47:47Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-26T08:47:47Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmps2hTdb#diesel:1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmps2hTdb/incremental-state"
[2022-05-26T08:47:51Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T08:47:51Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-26T08:47:51Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpRZkaF0#diesel:1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T08:48:09Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-26T08:48:09Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-26T08:48:09Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpRZkaF0#diesel:1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpRZkaF0/incremental-state"
[2022-05-26T08:48:30Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-05-26T08:48:30Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpRZkaF0#diesel:1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpRZkaF0/incremental-state"
[2022-05-26T08:48:33Z DEBUG collector::execute] applying println to "/tmp/.tmpRZkaF0"
[2022-05-26T08:48:33Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-26T08:48:33Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-26T08:48:33Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpRZkaF0#diesel:1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpRZkaF0/incremental-state"
Executing benchmark externs (5/8)
Preparing externs
[2022-05-26T08:48:38Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-05-26T08:48:38Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
---
[2022-05-26T08:48:38Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T08:48:38Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-05-26T08:48:38Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpHfDBm1#externs:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T08:48:39Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-05-26T08:48:39Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpHfDBm1#externs:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpHfDBm1/incremental-state"
[2022-05-26T08:48:40Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-05-26T08:48:40Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpHfDBm1#externs:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpHfDBm1/incremental-state"
[2022-05-26T08:48:40Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T08:48:40Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-26T08:48:40Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp5u64WG#externs:0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T08:48:41Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
---
[2022-05-26T08:48:43Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T08:48:43Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-26T08:48:43Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpbSaeAm#externs:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T08:48:43Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-26T08:48:43Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpbSaeAm#externs:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpbSaeAm/incremental-state"
[2022-05-26T08:48:44Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-05-26T08:48:44Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpbSaeAm#externs:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpbSaeAm/incremental-state"
Executing benchmark match-stress (6/8)
Preparing match-stress
[2022-05-26T08:48:45Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-05-26T08:48:45Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
---
[2022-05-26T08:48:45Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T08:48:45Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-05-26T08:48:45Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQfYKLJ#match-stress:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T08:48:48Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-05-26T08:48:48Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQfYKLJ#match-stress:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpQfYKLJ/incremental-state"
[2022-05-26T08:48:52Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-05-26T08:48:52Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQfYKLJ#match-stress:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpQfYKLJ/incremental-state"
[2022-05-26T08:48:53Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T08:48:53Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-26T08:48:53Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpPatyJw#match-stress:0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T08:48:56Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-26T08:48:56Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-26T08:48:56Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpPatyJw#match-stress:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpPatyJw/incremental-state"
[2022-05-26T08:49:00Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-05-26T08:49:00Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpPatyJw#match-stress:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpPatyJw/incremental-state"
[2022-05-26T08:49:01Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T08:49:01Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-26T08:49:01Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-26T08:49:01Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpewENck#match-stress:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T08:49:04Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-26T08:49:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpewENck#match-stress:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpewENck/incremental-state"
[2022-05-26T08:49:08Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-05-26T08:49:08Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpewENck#match-stress:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpewENck/incremental-state"
Executing benchmark token-stream-stress (7/8)
Preparing token-stream-stress
[2022-05-26T08:49:09Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-05-26T08:49:09Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2022-05-26T08:49:10Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T08:49:10Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-05-26T08:49:10Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpX2aivZ#token-stream-stress:0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T08:49:10Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-05-26T08:49:10Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpX2aivZ#token-stream-stress:0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpX2aivZ/incremental-state"
[2022-05-26T08:49:10Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-05-26T08:49:10Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpX2aivZ#token-stream-stress:0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpX2aivZ/incremental-state"
[2022-05-26T08:49:10Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T08:49:10Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-26T08:49:10Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpNqbLtq#token-stream-stress:0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T08:49:11Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-26T08:49:11Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-26T08:49:11Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpNqbLtq#token-stream-stress:0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpNqbLtq/incremental-state"
[2022-05-26T08:49:11Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-05-26T08:49:11Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpNqbLtq#token-stream-stress:0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpNqbLtq/incremental-state"
[2022-05-26T08:49:11Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T08:49:11Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-26T08:49:11Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFuDBHT#token-stream-stress:0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T08:49:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-26T08:49:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-26T08:49:12Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFuDBHT#token-stream-stress:0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpFuDBHT/incremental-state"
[2022-05-26T08:49:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-05-26T08:49:12Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFuDBHT#token-stream-stress:0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpFuDBHT/incremental-state"
Executing benchmark tuple-stress (8/8)
Preparing tuple-stress
[2022-05-26T08:49:12Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-05-26T08:49:12Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
---
[2022-05-26T08:49:31Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T08:49:31Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-26T08:49:31Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmplnwskP#tuple-stress:0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T08:49:37Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-26T08:49:37Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmplnwskP#tuple-stress:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmplnwskP/incremental-state"
[2022-05-26T08:49:43Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-05-26T08:49:43Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmplnwskP#tuple-stress:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmplnwskP/incremental-state"
[2022-05-26T08:49:45Z DEBUG collector::execute] applying new row to "/tmp/.tmplnwskP"
[2022-05-26T08:49:45Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-05-26T08:49:45Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-05-26T08:49:45Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmplnwskP#tuple-stress:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmplnwskP/incremental-state"
[2022-05-26T08:49:50Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T08:49:50Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-26T08:49:50Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpszuDwm#tuple-stress:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T08:49:56Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-26T08:49:56Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-26T08:49:56Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpszuDwm#tuple-stress:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpszuDwm/incremental-state"
[2022-05-26T08:50:02Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-05-26T08:50:02Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpszuDwm#tuple-stress:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpszuDwm/incremental-state"
[2022-05-26T08:50:03Z DEBUG collector::execute] applying new row to "/tmp/.tmpszuDwm"
[2022-05-26T08:50:04Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-05-26T08:50:04Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-05-26T08:50:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpszuDwm#tuple-stress:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpszuDwm/incremental-state"
+ cd /checkout/obj
+ ./build/x86_64-unknown-linux-gnu/llvm/bin/llvm-profdata merge -o /tmp/rustc-pgo.profdata /tmp/rustc-pgo
+ rm -r ./build/x86_64-unknown-linux-gnu/llvm ./build/x86_64-unknown-linux-gnu/lld
+ python3 ../x.py dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu --include-default-paths build-manifest --rust-profile-use=/tmp/rustc-pgo.profdata --llvm-profile-use=/tmp/llvm-pgo.profdata
---
[RUSTC-TIMING] build_script_build test:false 0.281
[RUSTC-TIMING] build_script_build test:false 0.324
   Compiling rustc_graphviz v0.0.0 (/checkout/compiler/rustc_graphviz)
[RUSTC-TIMING] cpufeatures test:false 0.043
   Compiling rustc-hash v1.1.0 (https://github.com/lqd/rustc-hash/?branch=experiment-a#5847e17a)
   Compiling unicode-width v0.1.8
[RUSTC-TIMING] tinystr test:false 0.213
   Compiling unic-common v0.9.0
[RUSTC-TIMING] rustc_hash test:false 0.059
---
[RUSTC-TIMING] self_cell test:false 0.083
   Compiling regex-syntax v0.6.25
[RUSTC-TIMING] unicode_xid test:false 0.096
   Compiling ansi_term v0.12.1
warning: rustc_fs_util.70df431e-cgu.5: no profile data available for function _RNvXs2_NtCs2Q4oIXA50W8_4core7convertNtNtNtB7_5alloc6layout11LayoutErrorINtB5_4FromBy_E4fromCs2EGgguo5ZPS_13rustc_fs_util Hash = 742261418966908927
[RUSTC-TIMING] unic_char_range test:false 0.141
   Compiling snap v1.0.1
   Compiling snap v1.0.1
warning: rustc_graphviz.2d790472-cgu.4: no profile data available for function _RNvXs2_NtCs2Q4oIXA50W8_4core7convertNtNtNtB7_5alloc6layout11LayoutErrorINtB5_4FromBy_E4fromCsF4rMDsErZu_14rustc_graphviz Hash = 742261418966908927

warning: rustc_graphviz.2d790472-cgu.10: no profile data available for function _RNvMNtNtNtCs2Q4oIXA50W8_4core4iter8adapters3revINtB2_3RevINtNtNtB8_3ops5range5RangejEE3newCsF4rMDsErZu_14rustc_graphviz Hash = 742261418966908927
[RUSTC-TIMING] rustc_fs_util test:false 0.123
warning: `rustc_fs_util` (lib) generated 1 warning
   Compiling crc32fast v1.2.0
[RUSTC-TIMING] either test:false 0.124
---
[RUSTC-TIMING] hashbrown test:false 0.593
[RUSTC-TIMING] block_buffer test:false 0.098
[RUSTC-TIMING] crypto_common test:false 0.075
   Compiling digest v0.10.2
warning: rustc_llvm.21f88ee4-cgu.0: no profile data available for function _RNvXs2_NtCs2Q4oIXA50W8_4core7convertNtNtNtB7_5alloc6layout11LayoutErrorINtB5_4FromBy_E4fromCsbaqq2y0afrI_10rustc_llvm Hash = 742261418966908927
[RUSTC-TIMING] rustc_llvm test:false 0.220
warning: `rustc_llvm` (lib) generated 1 warning
   Compiling sha-1 v0.10.0
[RUSTC-TIMING] rand_chacha test:false 0.776
---
   Compiling rustc_error_messages v0.0.0 (/checkout/compiler/rustc_error_messages)
   Compiling rustc_target v0.0.0 (/checkout/compiler/rustc_target)
   Compiling rustc_feature v0.0.0 (/checkout/compiler/rustc_feature)
   Compiling chalk-engine v0.80.0
warning: rustc_error_messages.6850f159-cgu.11: no profile data available for function _RINvMsx_NtCsdUcjgkzqO7u_12tracing_core5fieldNtB6_8FieldSet9value_setATRNtB6_5FieldINtNtCs2Q4oIXA50W8_4core6option6OptionRDNtB6_5ValueEL_EEj1_ECs6rIDi4KOLEr_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.6850f159-cgu.11: no profile data available for function _RINvMsx_NtCsdUcjgkzqO7u_12tracing_core5fieldNtB6_8FieldSet9value_setATRNtB6_5FieldINtNtCs2Q4oIXA50W8_4core6option6OptionRDNtB6_5ValueEL_EEj2_ECs6rIDi4KOLEr_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.6850f159-cgu.11: no profile data available for function _RINvMsx_NtCsdUcjgkzqO7u_12tracing_core5fieldNtB6_8FieldSet9value_setATRNtB6_5FieldINtNtCs2Q4oIXA50W8_4core6option6OptionRDNtB6_5ValueEL_EEj5_ECs6rIDi4KOLEr_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.6850f159-cgu.11: no profile data available for function _RINvNtCs2Q4oIXA50W8_4core3ptr13drop_in_placeRINtNtB4_6option6OptionNtCsgpZyefxJpFM_16unic_langid_impl18LanguageIdentifierEECs6rIDi4KOLEr_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.6850f159-cgu.11: no profile data available for function _RINvNtCs2Q4oIXA50W8_4core3ptr13drop_in_placeRINtNtB4_6option6OptionNtNtCs1fAehbXJ0fo_3std4path7PathBufEECs6rIDi4KOLEr_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.6850f159-cgu.11: no profile data available for function _RINvNtCs2Q4oIXA50W8_4core3ptr13drop_in_placeRINtNtB4_6option6OptionRNtNtCs1fAehbXJ0fo_3std4path4PathEECs6rIDi4KOLEr_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.6850f159-cgu.11: no profile data available for function _RINvNtCs2Q4oIXA50W8_4core3ptr13drop_in_placeRINtNtCsa2Kf4XtuQ3K_5alloc3vec3VecNtNtCs1fAehbXJ0fo_3std4path7PathBufEECs6rIDi4KOLEr_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.6850f159-cgu.11: no profile data available for function _RINvNtCs2Q4oIXA50W8_4core3ptr13drop_in_placeRNtCsgpZyefxJpFM_16unic_langid_impl18LanguageIdentifierECs6rIDi4KOLEr_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.6850f159-cgu.11: no profile data available for function _RINvNtCs2Q4oIXA50W8_4core3ptr13drop_in_placeRNtNtCs1fAehbXJ0fo_3std4path7PathBufECs6rIDi4KOLEr_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.6850f159-cgu.11: no profile data available for function _RINvNtCs2Q4oIXA50W8_4core3ptr13drop_in_placeRNtNtCscZRqoOJ8URW_13fluent_bundle8resource14FluentResourceECs6rIDi4KOLEr_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.6850f159-cgu.11: no profile data available for function _RINvNtCs2Q4oIXA50W8_4core3ptr13drop_in_placeRQNtNtCs1fAehbXJ0fo_3std4path7PathBufECs6rIDi4KOLEr_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.6850f159-cgu.11: no profile data available for function _RINvNtCs2Q4oIXA50W8_4core3ptr13drop_in_placeRRSReECs6rIDi4KOLEr_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.6850f159-cgu.11: no profile data available for function _RINvNtCsdUcjgkzqO7u_12tracing_core5field5debugRINtNtCs2Q4oIXA50W8_4core6option6OptionNtCsgpZyefxJpFM_16unic_langid_impl18LanguageIdentifierEECs6rIDi4KOLEr_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.6850f159-cgu.11: no profile data available for function _RINvNtCsdUcjgkzqO7u_12tracing_core5field5debugRINtNtCs2Q4oIXA50W8_4core6option6OptionNtNtCs1fAehbXJ0fo_3std4path7PathBufEECs6rIDi4KOLEr_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.6850f159-cgu.11: no profile data available for function _RINvNtCsdUcjgkzqO7u_12tracing_core5field5debugRINtNtCs2Q4oIXA50W8_4core6option6OptionRNtNtCs1fAehbXJ0fo_3std4path4PathEECs6rIDi4KOLEr_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.6850f159-cgu.11: no profile data available for function _RINvNtCsdUcjgkzqO7u_12tracing_core5field5debugRINtNtCsa2Kf4XtuQ3K_5alloc3vec3VecNtNtCs1fAehbXJ0fo_3std4path7PathBufEECs6rIDi4KOLEr_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.6850f159-cgu.11: no profile data available for function _RINvNtCsdUcjgkzqO7u_12tracing_core5field5debugRNtCsgpZyefxJpFM_16unic_langid_impl18LanguageIdentifierECs6rIDi4KOLEr_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.6850f159-cgu.11: no profile data available for function _RINvNtCsdUcjgkzqO7u_12tracing_core5field5debugRNtNtCs1fAehbXJ0fo_3std4path7PathBufECs6rIDi4KOLEr_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.6850f159-cgu.11: no profile data available for function _RINvNtCsdUcjgkzqO7u_12tracing_core5field5debugRNtNtCscZRqoOJ8URW_13fluent_bundle8resource14FluentResourceECs6rIDi4KOLEr_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.6850f159-cgu.11: no profile data available for function _RINvNtCsdUcjgkzqO7u_12tracing_core5field5debugRQNtNtCs1fAehbXJ0fo_3std4path7PathBufECs6rIDi4KOLEr_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.6850f159-cgu.11: no profile data available for function _RINvNtCsdUcjgkzqO7u_12tracing_core5field5debugRRSReECs6rIDi4KOLEr_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.6850f159-cgu.11: no profile data available for function _RNvXsk_NtCsdUcjgkzqO7u_12tracing_core5fieldINtB5_10DebugValueRINtNtCs2Q4oIXA50W8_4core6option6OptionNtCsgpZyefxJpFM_16unic_langid_impl18LanguageIdentifierEENtB5_5Value6recordCs6rIDi4KOLEr_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.6850f159-cgu.11: no profile data available for function _RNvXsk_NtCsdUcjgkzqO7u_12tracing_core5fieldINtB5_10DebugValueRINtNtCs2Q4oIXA50W8_4core6option6OptionNtNtCs1fAehbXJ0fo_3std4path7PathBufEENtB5_5Value6recordCs6rIDi4KOLEr_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.6850f159-cgu.11: no profile data available for function _RNvXsk_NtCsdUcjgkzqO7u_12tracing_core5fieldINtB5_10DebugValueRINtNtCs2Q4oIXA50W8_4core6option6OptionRNtNtCs1fAehbXJ0fo_3std4path4PathEENtB5_5Value6recordCs6rIDi4KOLEr_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.6850f159-cgu.11: no profile data available for function _RNvXsk_NtCsdUcjgkzqO7u_12tracing_core5fieldINtB5_10DebugValueRINtNtCsa2Kf4XtuQ3K_5alloc3vec3VecNtNtCs1fAehbXJ0fo_3std4path7PathBufEENtB5_5Value6recordCs6rIDi4KOLEr_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.6850f159-cgu.11: no profile data available for function _RNvXsk_NtCsdUcjgkzqO7u_12tracing_core5fieldINtB5_10DebugValueRNtCsgpZyefxJpFM_16unic_langid_impl18LanguageIdentifierENtB5_5Value6recordCs6rIDi4KOLEr_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.6850f159-cgu.11: no profile data available for function _RNvXsk_NtCsdUcjgkzqO7u_12tracing_core5fieldINtB5_10DebugValueRNtNtCs1fAehbXJ0fo_3std4path7PathBufENtB5_5Value6recordCs6rIDi4KOLEr_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.6850f159-cgu.11: no profile data available for function _RNvXsk_NtCsdUcjgkzqO7u_12tracing_core5fieldINtB5_10DebugValueRNtNtCscZRqoOJ8URW_13fluent_bundle8resource14FluentResourceENtB5_5Value6recordCs6rIDi4KOLEr_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.6850f159-cgu.11: no profile data available for function _RNvXsk_NtCsdUcjgkzqO7u_12tracing_core5fieldINtB5_10DebugValueRQNtNtCs1fAehbXJ0fo_3std4path7PathBufENtB5_5Value6recordCs6rIDi4KOLEr_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.6850f159-cgu.11: no profile data available for function _RNvXsk_NtCsdUcjgkzqO7u_12tracing_core5fieldINtB5_10DebugValueRRSReENtB5_5Value6recordCs6rIDi4KOLEr_20rustc_error_messages Hash = 170957022131388415
[RUSTC-TIMING] chalk_solve test:false 3.637
[RUSTC-TIMING] rustc_error_messages test:false 0.768
warning: `rustc_error_messages` (lib) generated 30 warnings
[RUSTC-TIMING] rustc_feature test:false 0.875
---
[RUSTC-TIMING] smallvec test:false 0.138
    Checking rustc_graphviz v0.0.0 (/checkout/compiler/rustc_graphviz)
[RUSTC-TIMING] remove_dir_all test:false 0.029
[RUSTC-TIMING] stable_deref_trait test:false 0.034
    Checking rustc-hash v1.1.0 (https://github.com/lqd/rustc-hash/?branch=experiment-a#5847e17a)
[RUSTC-TIMING] cpufeatures test:false 0.030
    Checking unicode-width v0.1.8
[RUSTC-TIMING] rustc_hash test:false 0.045
    Checking scoped-tls v1.0.0
