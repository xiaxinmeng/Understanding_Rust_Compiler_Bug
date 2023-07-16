plain
[2022-05-16T13:23:45Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp5TqQ1e#clap:3.1.6" "--release" "--" "--skip-this-rustc"
Running clap-3.1.6: Debug + [Full]
[2022-05-16T13:23:48Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-16T13:23:48Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-16T13:23:48Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpGQBdUh#clap:3.1.6" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-16T13:23:53Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-16T13:23:53Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-16T13:23:53Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpSngDlr#clap:3.1.6" "--release" "--" "--wrap-rustc-with" "Eprintln"
5 normal benchmarks remaining
5 normal benchmarks remaining
Preparing hyper-0.14.18
[2022-05-16T13:24:06Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-16T13:24:06Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-16T13:24:06Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpSQOUoN#hyper:0.14.18" "--release" "--features" "client,http1,http2,server,stream" "--" "--skip-this-rustc"
[2022-05-16T13:24:06Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpNV2OOJ#hyper:0.14.18" "--features" "client,http1,http2,server,stream" "--" "--skip-this-rustc"
[2022-05-16T13:24:36Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-16T13:24:37Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-16T13:24:37Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpleOTWW#hyper:0.14.18" "--features" "client,http1,http2,server,stream" "--" "--wrap-rustc-with" "Eprintln"
Running hyper-0.14.18: Opt + [Full]
Running hyper-0.14.18: Opt + [Full]
[2022-05-16T13:24:41Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-16T13:24:41Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-16T13:24:41Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpxx2J3V#hyper:0.14.18" "--release" "--features" "client,http1,http2,server,stream" "--" "--wrap-rustc-with" "Eprintln"
4 normal benchmarks remaining
Preparing regex-1.5.5
[2022-05-16T13:24:52Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-16T13:24:52Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-16T13:24:52Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpONEQbn#regex:1.5.5" "--release" "--" "--skip-this-rustc"
[2022-05-16T13:24:52Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp2I8aCg#regex:1.5.5" "--" "--skip-this-rustc"
[2022-05-16T13:25:03Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-16T13:25:03Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-16T13:25:03Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpsFwuXd#regex:1.5.5" "--" "--wrap-rustc-with" "Eprintln"
Running regex-1.5.5: Opt + [Full]
---
2 normal benchmarks remaining
Preparing serde-1.0.136
[2022-05-16T13:26:18Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-16T13:26:18Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-16T13:26:18Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpIejcSZ#serde:1.0.136" "--" "--skip-this-rustc"
[2022-05-16T13:26:18Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmps0SEub#serde:1.0.136" "--release" "--" "--skip-this-rustc"
[2022-05-16T13:26:19Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-16T13:26:19Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-16T13:26:19Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmprrpgFh#serde:1.0.136" "--" "--wrap-rustc-with" "Eprintln"
Running serde-1.0.136: Opt + [Full]
---
[2022-05-16T13:39:45Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-16T13:39:46Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-05-16T13:39:46Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp4aQWot#cargo:0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-16T13:40:07Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-05-16T13:40:07Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp4aQWot#cargo:0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp4aQWot/incremental-state"
[2022-05-16T13:40:34Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-05-16T13:40:34Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp4aQWot#cargo:0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp4aQWot/incremental-state"
[2022-05-16T13:40:39Z DEBUG collector::execute] applying println to "/tmp/.tmp4aQWot"
[2022-05-16T13:40:39Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-16T13:40:39Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-16T13:40:39Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp4aQWot#cargo:0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp4aQWot/incremental-state"
[2022-05-16T13:40:50Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-16T13:40:51Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-16T13:40:51Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp00XyO9#cargo:0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-16T13:41:42Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
---
[2022-05-16T13:43:20Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-16T13:43:21Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-16T13:43:21Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpWdBsTP#cargo:0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-16T13:44:26Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-16T13:44:26Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpWdBsTP#cargo:0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpWdBsTP/incremental-state"
[2022-05-16T13:45:31Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-05-16T13:45:31Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpWdBsTP#cargo:0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpWdBsTP/incremental-state"
[2022-05-16T13:45:42Z DEBUG collector::execute] applying println to "/tmp/.tmpWdBsTP"
[2022-05-16T13:45:42Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-16T13:45:42Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-16T13:45:42Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpWdBsTP#cargo:0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpWdBsTP/incremental-state"
Preparing ctfe-stress-4
[2022-05-16T13:46:13Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-16T13:46:13Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-05-16T13:46:13Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2022-05-16T13:48:34Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpUb7PBH#externs:0.1.0" "--release" "--" "--skip-this-rustc"
Running externs: Check + [Full, IncrFull, IncrUnchanged, IncrPatched]
[2022-05-16T13:48:34Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-16T13:48:34Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-05-16T13:48:34Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpnRrHDL#externs:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-16T13:48:35Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-05-16T13:48:35Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpnRrHDL#externs:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpnRrHDL/incremental-state"
[2022-05-16T13:48:36Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-05-16T13:48:36Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpnRrHDL#externs:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpnRrHDL/incremental-state"
[2022-05-16T13:48:36Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-16T13:48:36Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-16T13:48:36Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpy7N8Yz#externs:0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-16T13:48:37Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
---
[2022-05-16T13:48:39Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-16T13:48:39Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-16T13:48:39Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpbfKDhN#externs:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-16T13:48:40Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-16T13:48:40Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpbfKDhN#externs:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpbfKDhN/incremental-state"
[2022-05-16T13:48:41Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-05-16T13:48:41Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpbfKDhN#externs:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpbfKDhN/incremental-state"
Preparing match-stress
[2022-05-16T13:48:42Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-16T13:48:42Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-16T13:48:42Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
---
[2022-05-16T13:48:42Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-16T13:48:42Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-05-16T13:48:42Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpXWlfdv#match-stress:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-16T13:48:45Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-05-16T13:48:45Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpXWlfdv#match-stress:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpXWlfdv/incremental-state"
[2022-05-16T13:48:49Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-05-16T13:48:49Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpXWlfdv#match-stress:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpXWlfdv/incremental-state"
[2022-05-16T13:48:50Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-16T13:48:50Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-16T13:48:50Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpfdZaSc#match-stress:0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-16T13:48:53Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-16T13:48:53Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-16T13:48:53Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpfdZaSc#match-stress:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpfdZaSc/incremental-state"
[2022-05-16T13:48:57Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-05-16T13:48:57Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpfdZaSc#match-stress:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpfdZaSc/incremental-state"
[2022-05-16T13:48:58Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-16T13:48:58Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-16T13:48:58Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpkCgh7i#match-stress:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-16T13:49:01Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
[2022-05-16T13:49:08Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-16T13:49:08Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-05-16T13:49:08Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp4nHMWD#token-stream-stress:0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-16T13:49:08Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-05-16T13:49:08Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp4nHMWD#token-stream-stress:0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp4nHMWD/incremental-state"
[2022-05-16T13:49:08Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-05-16T13:49:08Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp4nHMWD#token-stream-stress:0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp4nHMWD/incremental-state"
[2022-05-16T13:49:08Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-16T13:49:08Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-16T13:49:08Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpYnorIG#token-stream-stress:0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-16T13:49:09Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-16T13:49:09Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-16T13:49:09Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpYnorIG#token-stream-stress:0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpYnorIG/incremental-state"
[2022-05-16T13:49:09Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-05-16T13:49:09Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpYnorIG#token-stream-stress:0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpYnorIG/incremental-state"
[2022-05-16T13:49:10Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-16T13:49:10Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-16T13:49:10Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmprTncF0#token-stream-stress:0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-16T13:49:10Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-16T13:49:10Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-16T13:49:10Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmprTncF0#token-stream-stress:0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmprTncF0/incremental-state"
[2022-05-16T13:49:11Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-05-16T13:49:11Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmprTncF0#token-stream-stress:0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmprTncF0/incremental-state"
Preparing tuple-stress
[2022-05-16T13:49:12Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-05-16T13:49:12Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-16T13:49:12Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
---
[2022-05-16T13:49:12Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-16T13:49:12Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-05-16T13:49:12Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpvqznhJ#tuple-stress:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-16T13:49:18Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-05-16T13:49:18Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpvqznhJ#tuple-stress:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpvqznhJ/incremental-state"
[2022-05-16T13:49:24Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-05-16T13:49:24Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpvqznhJ#tuple-stress:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpvqznhJ/incremental-state"
[2022-05-16T13:49:26Z DEBUG collector::execute] applying new row to "/tmp/.tmpvqznhJ"
[2022-05-16T13:49:26Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-05-16T13:49:26Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-05-16T13:49:26Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpvqznhJ#tuple-stress:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpvqznhJ/incremental-state"
[2022-05-16T13:49:32Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-16T13:49:32Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-16T13:49:32Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJjWmHU#tuple-stress:0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-16T13:49:38Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-16T13:49:38Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-16T13:49:38Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJjWmHU#tuple-stress:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpJjWmHU/incremental-state"
[2022-05-16T13:49:45Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-05-16T13:49:45Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJjWmHU#tuple-stress:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpJjWmHU/incremental-state"
[2022-05-16T13:49:47Z DEBUG collector::execute] applying new row to "/tmp/.tmpJjWmHU"
[2022-05-16T13:49:47Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-05-16T13:49:47Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-05-16T13:49:47Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJjWmHU#tuple-stress:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpJjWmHU/incremental-state"
[2022-05-16T13:49:54Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-16T13:49:54Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-16T13:49:54Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpAVDIjV#tuple-stress:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-16T13:50:00Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-16T13:50:00Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-16T13:50:00Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpAVDIjV#tuple-stress:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpAVDIjV/incremental-state"
[2022-05-16T13:50:06Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-05-16T13:50:06Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpAVDIjV#tuple-stress:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpAVDIjV/incremental-state"
[2022-05-16T13:50:09Z DEBUG collector::execute] applying new row to "/tmp/.tmpAVDIjV"
[2022-05-16T13:50:09Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-05-16T13:50:09Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-05-16T13:50:09Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpAVDIjV#tuple-stress:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpAVDIjV/incremental-state"
+ ./build/x86_64-unknown-linux-gnu/llvm/bin/llvm-profdata merge -o /tmp/rustc-pgo.profdata /tmp/rustc-pgo
+ ./build/x86_64-unknown-linux-gnu/llvm/bin/llvm-profdata merge -o /tmp/libstd-pgo.profdata /tmp/libstd-pgo
+ ./build/x86_64-unknown-linux-gnu/llvm/bin/llvm-profdata merge -o /tmp/libstd-pgo.profdata /tmp/libstd-pgo
error: /tmp/libstd-pgo: No such file or directory
