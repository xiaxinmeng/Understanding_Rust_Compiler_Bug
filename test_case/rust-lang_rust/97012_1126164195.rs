plain
6 normal benchmarks remaining
Preparing clap-3.1.6
[2022-05-13T14:18:05Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-13T14:18:05Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-13T14:18:05Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpeoefsh#clap:3.1.6" "--release" "--" "--skip-this-rustc"
[2022-05-13T14:18:05Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpNDmUuC#clap:3.1.6" "--" "--skip-this-rustc"
[2022-05-13T14:18:08Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-13T14:18:08Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-13T14:18:08Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp1517bN#clap:3.1.6" "--" "--wrap-rustc-with" "Eprintln"
Running clap-3.1.6: Opt + [Full]
Running clap-3.1.6: Opt + [Full]
[2022-05-13T14:18:12Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-13T14:18:12Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-13T14:18:12Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpdGgTWp#clap:3.1.6" "--release" "--" "--wrap-rustc-with" "Eprintln"
Preparing hyper-0.14.18
[2022-05-13T14:18:25Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-13T14:18:25Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-13T14:18:25Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-13T14:18:25Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpSDRcWO#hyper:0.14.18" "--release" "--features" "client,http1,http2,server,stream" "--" "--skip-this-rustc"
[2022-05-13T14:18:25Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp56LQ8O#hyper:0.14.18" "--features" "client,http1,http2,server,stream" "--" "--skip-this-rustc"
[2022-05-13T14:18:52Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-13T14:18:52Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-13T14:18:52Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-13T14:18:52Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpZjXQCs#hyper:0.14.18" "--features" "client,http1,http2,server,stream" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-13T14:18:56Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-13T14:18:56Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-13T14:18:56Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpsR7SR8#hyper:0.14.18" "--release" "--features" "client,http1,http2,server,stream" "--" "--wrap-rustc-with" "Eprintln"
4 normal benchmarks remaining
---
2 normal benchmarks remaining
Preparing serde-1.0.136
[2022-05-13T14:20:29Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-13T14:20:29Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-13T14:20:29Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpXaORpN#serde:1.0.136" "--release" "--" "--skip-this-rustc"
[2022-05-13T14:20:29Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpiO2Rvb#serde:1.0.136" "--" "--skip-this-rustc"
[2022-05-13T14:20:29Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-13T14:20:29Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-13T14:20:29Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpbVOrpZ#serde:1.0.136" "--" "--wrap-rustc-with" "Eprintln"
Running serde-1.0.136: Opt + [Full]
---
[2022-05-13T14:34:01Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-13T14:34:01Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-13T14:34:01Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpC2dWnV#cargo:0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-13T14:34:46Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-13T14:34:46Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpC2dWnV#cargo:0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpC2dWnV/incremental-state"
[2022-05-13T14:35:42Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-05-13T14:35:42Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpC2dWnV#cargo:0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpC2dWnV/incremental-state"
[2022-05-13T14:35:53Z DEBUG collector::execute] applying println to "/tmp/.tmpC2dWnV"
[2022-05-13T14:35:53Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-13T14:35:53Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-13T14:35:53Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpC2dWnV#cargo:0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpC2dWnV/incremental-state"
[2022-05-13T14:36:17Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-13T14:36:17Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-13T14:36:17Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpPOJaTu#cargo:0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-13T14:37:15Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-13T14:37:15Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-13T14:37:15Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpPOJaTu#cargo:0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpPOJaTu/incremental-state"
[2022-05-13T14:38:14Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-05-13T14:38:14Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpPOJaTu#cargo:0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpPOJaTu/incremental-state"
[2022-05-13T14:38:25Z DEBUG collector::execute] applying println to "/tmp/.tmpPOJaTu"
[2022-05-13T14:38:25Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-13T14:38:25Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-13T14:38:25Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpPOJaTu#cargo:0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpPOJaTu/incremental-state"
Preparing ctfe-stress-4
[2022-05-13T14:38:54Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-05-13T14:38:54Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-13T14:38:54Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2022-05-13T14:38:54Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-13T14:38:54Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-05-13T14:38:54Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpk0L3RU#ctfe-stress-4:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-13T14:39:14Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-05-13T14:39:14Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpk0L3RU#ctfe-stress-4:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpk0L3RU/incremental-state"
[2022-05-13T14:39:36Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-05-13T14:39:36Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpk0L3RU#ctfe-stress-4:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpk0L3RU/incremental-state"
[2022-05-13T14:39:36Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-13T14:39:36Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-13T14:39:36Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp4n2dJD#ctfe-stress-4:0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-13T14:39:55Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
---
[2022-05-13T14:41:01Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-13T14:41:01Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-05-13T14:41:01Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFbtNRe#externs:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-13T14:41:01Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-05-13T14:41:01Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFbtNRe#externs:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpFbtNRe/incremental-state"
[2022-05-13T14:41:02Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-05-13T14:41:02Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFbtNRe#externs:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpFbtNRe/incremental-state"
[2022-05-13T14:41:03Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-13T14:41:03Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-13T14:41:03Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmppuMpo8#externs:0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-13T14:41:03Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-13T14:41:03Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-13T14:41:03Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmppuMpo8#externs:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmppuMpo8/incremental-state"
[2022-05-13T14:41:04Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-05-13T14:41:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmppuMpo8#externs:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmppuMpo8/incremental-state"
[2022-05-13T14:41:05Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-13T14:41:05Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-13T14:41:05Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpuWvVKh#externs:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-13T14:41:06Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-13T14:41:06Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-13T14:41:06Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpuWvVKh#externs:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpuWvVKh/incremental-state"
[2022-05-13T14:41:07Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-05-13T14:41:07Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpuWvVKh#externs:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpuWvVKh/incremental-state"
Preparing match-stress
[2022-05-13T14:41:08Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-05-13T14:41:08Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-13T14:41:08Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-13T14:41:08Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-13T14:41:08Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp1pcCRi#match-stress:0.1.0" "--profile" "check" "--" "--skip-this-rustc"
[2022-05-13T14:41:08Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpelG1hA#match-stress:0.1.0" "--" "--skip-this-rustc"
[2022-05-13T14:41:08Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpGBpL32#match-stress:0.1.0" "--release" "--" "--skip-this-rustc"
Running match-stress: Check + [Full, IncrFull, IncrUnchanged, IncrPatched]
[2022-05-13T14:41:08Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-13T14:41:08Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-05-13T14:41:08Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpZjumCN#match-stress:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-13T14:41:11Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-05-13T14:41:11Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpZjumCN#match-stress:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpZjumCN/incremental-state"
[2022-05-13T14:41:14Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-05-13T14:41:14Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpZjumCN#match-stress:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpZjumCN/incremental-state"
[2022-05-13T14:41:15Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-13T14:41:15Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-13T14:41:15Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDLp4rM#match-stress:0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-13T14:41:18Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-13T14:41:18Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-13T14:41:18Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDLp4rM#match-stress:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpDLp4rM/incremental-state"
[2022-05-13T14:41:22Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-05-13T14:41:22Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDLp4rM#match-stress:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpDLp4rM/incremental-state"
[2022-05-13T14:41:23Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-13T14:41:23Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-13T14:41:23Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmptjeh52#match-stress:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-13T14:41:26Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
[2022-05-13T14:41:32Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-13T14:41:32Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-13T14:41:32Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmppm29KF#token-stream-stress:0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-13T14:41:32Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-13T14:41:32Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmppm29KF#token-stream-stress:0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmppm29KF/incremental-state"
[2022-05-13T14:41:32Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-05-13T14:41:32Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmppm29KF#token-stream-stress:0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmppm29KF/incremental-state"
[2022-05-13T14:41:33Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-13T14:41:33Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-13T14:41:33Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJaX5i6#token-stream-stress:0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-13T14:41:33Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-13T14:41:33Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-13T14:41:33Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJaX5i6#token-stream-stress:0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpJaX5i6/incremental-state"
[2022-05-13T14:41:33Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-05-13T14:41:33Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJaX5i6#token-stream-stress:0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpJaX5i6/incremental-state"
Preparing tuple-stress
[2022-05-13T14:41:34Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-05-13T14:41:34Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-13T14:41:34Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-13T14:41:34Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-13T14:41:34Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpTYd8Ii#tuple-stress:0.1.0" "--profile" "check" "--" "--skip-this-rustc"
[2022-05-13T14:41:34Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpUGbfoG#tuple-stress:0.1.0" "--release" "--" "--skip-this-rustc"
[2022-05-13T14:41:34Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCzt82u#tuple-stress:0.1.0" "--" "--skip-this-rustc"
[2022-05-13T14:41:34Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-13T14:41:34Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-05-13T14:41:34Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpYXTz2C#tuple-stress:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-13T14:41:39Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-05-13T14:41:39Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-05-13T14:41:39Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpYXTz2C#tuple-stress:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpYXTz2C/incremental-state"
[2022-05-13T14:41:45Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-05-13T14:41:45Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpYXTz2C#tuple-stress:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpYXTz2C/incremental-state"
[2022-05-13T14:41:46Z DEBUG collector::execute] applying new row to "/tmp/.tmpYXTz2C"
[2022-05-13T14:41:46Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-05-13T14:41:46Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-05-13T14:41:46Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpYXTz2C#tuple-stress:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpYXTz2C/incremental-state"
[2022-05-13T14:41:52Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-13T14:41:52Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-13T14:41:52Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpRg77r7#tuple-stress:0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-13T14:41:57Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
---
[RUSTC-TIMING] arrayvec test:false 0.242
   Compiling tinyvec v0.3.4
[RUSTC-TIMING] unicode_xid test:false 0.105
   Compiling crc32fast v1.2.0
warning: rustc_fs_util.d74606ae-cgu.3: no profile data available for function _RNvXs2_NtCsd47gUMYryhL_4core7convertNtNtNtB7_5alloc6layout11LayoutErrorINtB5_4FromBy_E4fromCsg6PK8ScrHR2_13rustc_fs_util Hash = 742261418966908927

warning: rustc_graphviz.f0492842-cgu.0: no profile data available for function _RNvXs2_NtCsd47gUMYryhL_4core7convertNtNtNtB7_5alloc6layout11LayoutErrorINtB5_4FromBy_E4fromCs1ptY1JgKcIU_14rustc_graphviz Hash = 742261418966908927
[RUSTC-TIMING] unic_char_range test:false 0.131
   Compiling ansi_term v0.12.1
[RUSTC-TIMING] rustc_fs_util test:false 0.112
warning: `rustc_fs_util` (lib) generated 1 warning
warning: `rustc_fs_util` (lib) generated 1 warning
   Compiling snap v1.0.1
warning: rustc_graphviz.f0492842-cgu.11: no profile data available for function _RNvMNtNtNtCsd47gUMYryhL_4core4iter8adapters3revINtB2_3RevINtNtNtB8_3ops5range5RangejEE3newCs1ptY1JgKcIU_14rustc_graphviz Hash = 742261418966908927
[RUSTC-TIMING] either test:false 0.142
   Compiling adler v0.2.3
[RUSTC-TIMING] tinystr test:false 0.398
   Compiling fixedbitset v0.2.0
---
[RUSTC-TIMING] jobserver test:false 1.237
   Compiling crypto-common v0.1.2
[RUSTC-TIMING] crypto_common test:false 0.082
   Compiling block-buffer v0.10.2
warning: rustc_llvm.9d9005cc-cgu.1: no profile data available for function _RNvXs2_NtCsd47gUMYryhL_4core7convertNtNtNtB7_5alloc6layout11LayoutErrorINtB5_4FromBy_E4fromCs2SfIIasIgH0_10rustc_llvm Hash = 742261418966908927
[RUSTC-TIMING] rustc_parse_format test:false 0.996
[RUSTC-TIMING] block_buffer test:false 0.091
   Compiling digest v0.10.2
[RUSTC-TIMING] rustc_llvm test:false 0.229
---
   Compiling rustc_error_messages v0.0.0 (/checkout/compiler/rustc_error_messages)
   Compiling chalk-engine v0.80.0
[RUSTC-TIMING] chalk_solve test:false 3.852
[RUSTC-TIMING] tikv_jemalloc_sys test:false 0.056
warning: rustc_error_messages.88c8228f-cgu.11: no profile data available for function _RINvMsx_NtCs3vctPJNqUxK_12tracing_core5fieldNtB6_8FieldSet9value_setATRNtB6_5FieldINtNtCsd47gUMYryhL_4core6option6OptionRDNtB6_5ValueEL_EEj1_ECs5qe0U6DCkvX_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.88c8228f-cgu.11: no profile data available for function _RINvMsx_NtCs3vctPJNqUxK_12tracing_core5fieldNtB6_8FieldSet9value_setATRNtB6_5FieldINtNtCsd47gUMYryhL_4core6option6OptionRDNtB6_5ValueEL_EEj2_ECs5qe0U6DCkvX_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.88c8228f-cgu.11: no profile data available for function _RINvMsx_NtCs3vctPJNqUxK_12tracing_core5fieldNtB6_8FieldSet9value_setATRNtB6_5FieldINtNtCsd47gUMYryhL_4core6option6OptionRDNtB6_5ValueEL_EEj5_ECs5qe0U6DCkvX_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.88c8228f-cgu.11: no profile data available for function _RINvNtCs3vctPJNqUxK_12tracing_core5field5debugRINtNtCs6f1CshFZMfQ_5alloc3vec3VecNtNtCs5LD6yfvl9CP_3std4path7PathBufEECs5qe0U6DCkvX_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.88c8228f-cgu.11: no profile data available for function _RINvNtCs3vctPJNqUxK_12tracing_core5field5debugRINtNtCsd47gUMYryhL_4core6option6OptionNtCsfwYmgfG11uh_16unic_langid_impl18LanguageIdentifierEECs5qe0U6DCkvX_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.88c8228f-cgu.11: no profile data available for function _RINvNtCs3vctPJNqUxK_12tracing_core5field5debugRINtNtCsd47gUMYryhL_4core6option6OptionNtNtCs5LD6yfvl9CP_3std4path7PathBufEECs5qe0U6DCkvX_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.88c8228f-cgu.11: no profile data available for function _RINvNtCs3vctPJNqUxK_12tracing_core5field5debugRINtNtCsd47gUMYryhL_4core6option6OptionRNtNtCs5LD6yfvl9CP_3std4path4PathEECs5qe0U6DCkvX_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.88c8228f-cgu.11: no profile data available for function _RINvNtCs3vctPJNqUxK_12tracing_core5field5debugRNtCsfwYmgfG11uh_16unic_langid_impl18LanguageIdentifierECs5qe0U6DCkvX_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.88c8228f-cgu.11: no profile data available for function _RINvNtCs3vctPJNqUxK_12tracing_core5field5debugRNtNtCs5LD6yfvl9CP_3std4path7PathBufECs5qe0U6DCkvX_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.88c8228f-cgu.11: no profile data available for function _RINvNtCs3vctPJNqUxK_12tracing_core5field5debugRNtNtCsk5PjkxTxh46_13fluent_bundle8resource14FluentResourceECs5qe0U6DCkvX_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.88c8228f-cgu.11: no profile data available for function _RINvNtCs3vctPJNqUxK_12tracing_core5field5debugRQNtNtCs5LD6yfvl9CP_3std4path7PathBufECs5qe0U6DCkvX_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.88c8228f-cgu.11: no profile data available for function _RINvNtCs3vctPJNqUxK_12tracing_core5field5debugRRSReECs5qe0U6DCkvX_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.88c8228f-cgu.11: no profile data available for function _RINvNtCsd47gUMYryhL_4core3ptr13drop_in_placeRINtNtB4_6option6OptionNtCsfwYmgfG11uh_16unic_langid_impl18LanguageIdentifierEECs5qe0U6DCkvX_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.88c8228f-cgu.11: no profile data available for function _RINvNtCsd47gUMYryhL_4core3ptr13drop_in_placeRINtNtB4_6option6OptionNtNtCs5LD6yfvl9CP_3std4path7PathBufEECs5qe0U6DCkvX_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.88c8228f-cgu.11: no profile data available for function _RINvNtCsd47gUMYryhL_4core3ptr13drop_in_placeRINtNtB4_6option6OptionRNtNtCs5LD6yfvl9CP_3std4path4PathEECs5qe0U6DCkvX_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.88c8228f-cgu.11: no profile data available for function _RINvNtCsd47gUMYryhL_4core3ptr13drop_in_placeRINtNtCs6f1CshFZMfQ_5alloc3vec3VecNtNtCs5LD6yfvl9CP_3std4path7PathBufEECs5qe0U6DCkvX_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.88c8228f-cgu.11: no profile data available for function _RINvNtCsd47gUMYryhL_4core3ptr13drop_in_placeRNtCsfwYmgfG11uh_16unic_langid_impl18LanguageIdentifierECs5qe0U6DCkvX_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.88c8228f-cgu.11: no profile data available for function _RINvNtCsd47gUMYryhL_4core3ptr13drop_in_placeRNtNtCs5LD6yfvl9CP_3std4path7PathBufECs5qe0U6DCkvX_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.88c8228f-cgu.11: no profile data available for function _RINvNtCsd47gUMYryhL_4core3ptr13drop_in_placeRNtNtCsk5PjkxTxh46_13fluent_bundle8resource14FluentResourceECs5qe0U6DCkvX_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.88c8228f-cgu.11: no profile data available for function _RINvNtCsd47gUMYryhL_4core3ptr13drop_in_placeRQNtNtCs5LD6yfvl9CP_3std4path7PathBufECs5qe0U6DCkvX_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.88c8228f-cgu.11: no profile data available for function _RINvNtCsd47gUMYryhL_4core3ptr13drop_in_placeRRSReECs5qe0U6DCkvX_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.88c8228f-cgu.11: no profile data available for function _RNvXsk_NtCs3vctPJNqUxK_12tracing_core5fieldINtB5_10DebugValueRINtNtCs6f1CshFZMfQ_5alloc3vec3VecNtNtCs5LD6yfvl9CP_3std4path7PathBufEENtB5_5Value6recordCs5qe0U6DCkvX_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.88c8228f-cgu.11: no profile data available for function _RNvXsk_NtCs3vctPJNqUxK_12tracing_core5fieldINtB5_10DebugValueRINtNtCsd47gUMYryhL_4core6option6OptionNtCsfwYmgfG11uh_16unic_langid_impl18LanguageIdentifierEENtB5_5Value6recordCs5qe0U6DCkvX_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.88c8228f-cgu.11: no profile data available for function _RNvXsk_NtCs3vctPJNqUxK_12tracing_core5fieldINtB5_10DebugValueRINtNtCsd47gUMYryhL_4core6option6OptionNtNtCs5LD6yfvl9CP_3std4path7PathBufEENtB5_5Value6recordCs5qe0U6DCkvX_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.88c8228f-cgu.11: no profile data available for function _RNvXsk_NtCs3vctPJNqUxK_12tracing_core5fieldINtB5_10DebugValueRINtNtCsd47gUMYryhL_4core6option6OptionRNtNtCs5LD6yfvl9CP_3std4path4PathEENtB5_5Value6recordCs5qe0U6DCkvX_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.88c8228f-cgu.11: no profile data available for function _RNvXsk_NtCs3vctPJNqUxK_12tracing_core5fieldINtB5_10DebugValueRNtCsfwYmgfG11uh_16unic_langid_impl18LanguageIdentifierENtB5_5Value6recordCs5qe0U6DCkvX_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.88c8228f-cgu.11: no profile data available for function _RNvXsk_NtCs3vctPJNqUxK_12tracing_core5fieldINtB5_10DebugValueRNtNtCs5LD6yfvl9CP_3std4path7PathBufENtB5_5Value6recordCs5qe0U6DCkvX_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.88c8228f-cgu.11: no profile data available for function _RNvXsk_NtCs3vctPJNqUxK_12tracing_core5fieldINtB5_10DebugValueRNtNtCsk5PjkxTxh46_13fluent_bundle8resource14FluentResourceENtB5_5Value6recordCs5qe0U6DCkvX_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.88c8228f-cgu.11: no profile data available for function _RNvXsk_NtCs3vctPJNqUxK_12tracing_core5fieldINtB5_10DebugValueRQNtNtCs5LD6yfvl9CP_3std4path7PathBufENtB5_5Value6recordCs5qe0U6DCkvX_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.88c8228f-cgu.11: no profile data available for function _RNvXsk_NtCs3vctPJNqUxK_12tracing_core5fieldINtB5_10DebugValueRRSReENtB5_5Value6recordCs5qe0U6DCkvX_20rustc_error_messages Hash = 170957022131388415
[RUSTC-TIMING] rustc_error_messages test:false 0.741
warning: `rustc_error_messages` (lib) generated 30 warnings
[RUSTC-TIMING] rustc_feature test:false 0.861
[RUSTC-TIMING] chalk_engine test:false 0.947
---
   |
17 | use rustc_target::spec::abi::Abi::RustIntrinsic;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `-D unused-imports` implied by `-D warnings`
[RUSTC-TIMING] build_script_build test:false 0.282
[RUSTC-TIMING] rustc_tools_util test:false 0.318
[RUSTC-TIMING] build_script_build test:false 0.238
[RUSTC-TIMING] cargo_platform test:false 0.546
---
    Finished release [optimized] target(s) in 1.74s
[TIMING] tool::ToolBuild { compiler: Compiler { stage: 1, host: x86_64-unknown-linux-gnu }, target: x86_64-unknown-linux-gnu, tool: "rust-demangler", path: "src/tools/rust-demangler", mode: ToolRustc, is_optional_tool: true, source_type: InTree, extra_features: [] } -- 1.755
[TIMING] tool::RustDemangler { compiler: Compiler { stage: 1, host: x86_64-unknown-linux-gnu }, target: x86_64-unknown-linux-gnu, extra_features: [] } -- 0.000
Dist rust-demangler-nightly-x86_64-unknown-linux-gnu
thread 'main' panicked at 'clippy expected to build - essential tool', src/bootstrap/dist.rs:1118:14
 finished in 1.766 seconds
[TIMING] dist::RustDemangler { compiler: Compiler { stage: 1, host: x86_64-unknown-linux-gnu }, target: x86_64-unknown-linux-gnu } -- 1.772
Build completed unsuccessfully in 0:30:41
