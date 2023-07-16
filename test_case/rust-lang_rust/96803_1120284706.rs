plain
5 normal benchmarks remaining
Preparing hyper-0.14.18
[2022-05-07T19:35:21Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-07T19:35:21Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-07T19:35:21Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpr6fJ1h#hyper:0.14.18" "--features" "client,http1,http2,server,stream" "--" "--skip-this-rustc"
[2022-05-07T19:35:21Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpXOfEFP#hyper:0.14.18" "--release" "--features" "client,http1,http2,server,stream" "--" "--skip-this-rustc"
[2022-05-07T19:35:54Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-07T19:35:55Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-07T19:35:55Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpabXgP7#hyper:0.14.18" "--features" "client,http1,http2,server,stream" "--" "--wrap-rustc-with" "Eprintln"
Running hyper-0.14.18: Opt + [Full]
---
[2022-05-07T19:37:14Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpavFzNv#ripgrep:13.0.0" "--" "--wrap-rustc-with" "Eprintln"
Running ripgrep-13.0.0: Opt + [Full]
[2022-05-07T19:37:19Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-07T19:37:20Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-07T19:37:20Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpMMjSAW#ripgrep:13.0.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
Preparing serde-1.0.136
[2022-05-07T19:37:42Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-07T19:37:42Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-07T19:37:42Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpGrjFe3#serde:1.0.136" "--" "--skip-this-rustc"
[2022-05-07T19:37:42Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpGrjFe3#serde:1.0.136" "--" "--skip-this-rustc"
[2022-05-07T19:37:42Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpXdMhZL#serde:1.0.136" "--release" "--" "--skip-this-rustc"
[2022-05-07T19:37:43Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-07T19:37:43Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-07T19:37:43Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-07T19:37:43Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpnLWdvX#serde:1.0.136" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-07T19:37:46Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-07T19:37:46Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-07T19:37:46Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpGEj6GZ#serde:1.0.136" "--release" "--" "--wrap-rustc-with" "Eprintln"
1 normal benchmark remaining
---
[2022-05-07T19:37:55Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmppu13rr#syn:1.0.89" "--" "--wrap-rustc-with" "Eprintln"
Running syn-1.0.89: Opt + [Full]
[2022-05-07T19:37:59Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-07T19:37:59Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-07T19:37:59Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpYBsRQu#syn:1.0.89" "--release" "--" "--wrap-rustc-with" "Eprintln"
+ /rustroot/bin/llvm-profdata merge -o /tmp/llvm-pgo.profdata ./build/x86_64-unknown-linux-gnu/llvm/build/profiles
+ rm -r ./build/x86_64-unknown-linux-gnu/llvm ./build/x86_64-unknown-linux-gnu/lld
+ python3 ../x.py build --target=x86_64-unknown-linux-gnu --host=x86_64-unknown-linux-gnu --stage 2 library/std --rust-profile-generate=/tmp/rustc-pgo
warning: sqlite3 not available in python, skipping build directory lock
---
[2022-05-07T19:52:35Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-07T19:52:36Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-05-07T19:52:36Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpp6nUud#cargo:0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-07T19:52:59Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-05-07T19:52:59Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpp6nUud#cargo:0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpp6nUud/incremental-state"
[2022-05-07T19:53:24Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-05-07T19:53:24Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpp6nUud#cargo:0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpp6nUud/incremental-state"
[2022-05-07T19:53:29Z DEBUG collector::execute] applying println to "/tmp/.tmpp6nUud"
[2022-05-07T19:53:29Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-07T19:53:29Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-07T19:53:29Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpp6nUud#cargo:0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpp6nUud/incremental-state"
[2022-05-07T19:53:40Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-07T19:53:41Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-07T19:53:41Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp9i54Kb#cargo:0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-07T19:54:31Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
---
[2022-05-07T19:56:08Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-07T19:56:09Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-07T19:56:09Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCp1Qqb#cargo:0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-07T19:57:17Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-07T19:57:17Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCp1Qqb#cargo:0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpCp1Qqb/incremental-state"
[2022-05-07T19:58:24Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-05-07T19:58:24Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCp1Qqb#cargo:0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpCp1Qqb/incremental-state"
[2022-05-07T19:58:36Z DEBUG collector::execute] applying println to "/tmp/.tmpCp1Qqb"
[2022-05-07T19:58:36Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-07T19:58:36Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-07T19:58:36Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCp1Qqb#cargo:0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpCp1Qqb/incremental-state"
Preparing ctfe-stress-4
[2022-05-07T19:59:07Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-07T19:59:07Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-05-07T19:59:07Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
---
[2022-05-07T19:59:08Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-07T19:59:08Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-05-07T19:59:08Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwdkfFI#ctfe-stress-4:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-07T19:59:28Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-05-07T19:59:28Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwdkfFI#ctfe-stress-4:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpwdkfFI/incremental-state"
[2022-05-07T19:59:52Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-05-07T19:59:52Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwdkfFI#ctfe-stress-4:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpwdkfFI/incremental-state"
[2022-05-07T19:59:52Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-07T19:59:52Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-07T19:59:52Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFFTpji#ctfe-stress-4:0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-07T20:00:13Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-07T20:00:13Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-07T20:00:13Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFFTpji#ctfe-stress-4:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpFFTpji/incremental-state"
[2022-05-07T20:00:37Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-05-07T20:00:37Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFFTpji#ctfe-stress-4:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpFFTpji/incremental-state"
[2022-05-07T20:00:38Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-07T20:00:38Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-07T20:00:38Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp6WnF7R#ctfe-stress-4:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-07T20:00:57Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
Preparing externs
[2022-05-07T20:01:21Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-07T20:01:21Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-07T20:01:21Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-05-07T20:01:21Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQ5a8GL#externs:0.1.0" "--" "--skip-this-rustc"
[2022-05-07T20:01:21Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpiKDofQ#externs:0.1.0" "--profile" "check" "--" "--skip-this-rustc"
[2022-05-07T20:01:21Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpZPLtpx#externs:0.1.0" "--release" "--" "--skip-this-rustc"
[2022-05-07T20:01:22Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-07T20:01:22Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-05-07T20:01:22Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpP5GmZP#externs:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-07T20:01:23Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-05-07T20:01:23Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-05-07T20:01:23Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpP5GmZP#externs:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpP5GmZP/incremental-state"
[2022-05-07T20:01:23Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-05-07T20:01:23Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpP5GmZP#externs:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpP5GmZP/incremental-state"
[2022-05-07T20:01:24Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-07T20:01:24Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-07T20:01:24Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpIV8ZlW#externs:0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-07T20:01:25Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-07T20:01:25Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-07T20:01:25Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpIV8ZlW#externs:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpIV8ZlW/incremental-state"
[2022-05-07T20:01:26Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-05-07T20:01:26Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpIV8ZlW#externs:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpIV8ZlW/incremental-state"
[2022-05-07T20:01:27Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-07T20:01:27Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-07T20:01:27Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp6cZA2C#externs:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-07T20:01:27Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
[2022-05-07T20:01:38Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-07T20:01:38Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-07T20:01:38Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpvagRB7#match-stress:0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-07T20:01:41Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-07T20:01:41Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpvagRB7#match-stress:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpvagRB7/incremental-state"
[2022-05-07T20:01:45Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-05-07T20:01:45Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpvagRB7#match-stress:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpvagRB7/incremental-state"
[2022-05-07T20:01:46Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-07T20:01:46Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-07T20:01:46Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpadUrME#match-stress:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-07T20:01:49Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-07T20:01:49Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-07T20:01:49Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpadUrME#match-stress:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpadUrME/incremental-state"
[2022-05-07T20:01:53Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-05-07T20:01:53Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpadUrME#match-stress:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpadUrME/incremental-state"
Preparing token-stream-stress
[2022-05-07T20:01:54Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-05-07T20:01:54Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-07T20:01:54Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
---
[2022-05-07T20:01:55Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-07T20:01:55Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-05-07T20:01:55Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpGpHkHV#token-stream-stress:0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-07T20:01:55Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-05-07T20:01:55Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpGpHkHV#token-stream-stress:0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpGpHkHV/incremental-state"
[2022-05-07T20:01:55Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-05-07T20:01:55Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpGpHkHV#token-stream-stress:0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpGpHkHV/incremental-state"
[2022-05-07T20:01:56Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-07T20:01:56Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-07T20:01:56Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKYOn4M#token-stream-stress:0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-07T20:01:56Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-07T20:01:56Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-07T20:01:56Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKYOn4M#token-stream-stress:0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpKYOn4M/incremental-state"
[2022-05-07T20:01:56Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-05-07T20:01:56Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKYOn4M#token-stream-stress:0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpKYOn4M/incremental-state"
[2022-05-07T20:01:57Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-07T20:01:57Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-07T20:01:57Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjGQcyl#token-stream-stress:0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-07T20:01:57Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-07T20:01:57Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-07T20:01:57Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjGQcyl#token-stream-stress:0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpjGQcyl/incremental-state"
[2022-05-07T20:01:58Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-05-07T20:01:58Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjGQcyl#token-stream-stress:0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpjGQcyl/incremental-state"
Preparing tuple-stress
[2022-05-07T20:01:58Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-07T20:01:58Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-07T20:01:58Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
---
[2022-05-07T20:01:59Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-07T20:01:59Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-05-07T20:01:59Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpXZTLJE#tuple-stress:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-07T20:02:04Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-05-07T20:02:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpXZTLJE#tuple-stress:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpXZTLJE/incremental-state"
[2022-05-07T20:02:10Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-05-07T20:02:10Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpXZTLJE#tuple-stress:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpXZTLJE/incremental-state"
[2022-05-07T20:02:12Z DEBUG collector::execute] applying new row to "/tmp/.tmpXZTLJE"
[2022-05-07T20:02:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-05-07T20:02:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-05-07T20:02:12Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpXZTLJE#tuple-stress:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpXZTLJE/incremental-state"
[2022-05-07T20:02:18Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-07T20:02:18Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-07T20:02:18Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpvhUssR#tuple-stress:0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-07T20:02:24Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-07T20:02:24Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-07T20:02:24Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpvhUssR#tuple-stress:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpvhUssR/incremental-state"
[2022-05-07T20:02:30Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-05-07T20:02:30Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpvhUssR#tuple-stress:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpvhUssR/incremental-state"
[2022-05-07T20:02:32Z DEBUG collector::execute] applying new row to "/tmp/.tmpvhUssR"
[2022-05-07T20:02:32Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-05-07T20:02:32Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-05-07T20:02:32Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpvhUssR#tuple-stress:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpvhUssR/incremental-state"
[2022-05-07T20:02:38Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-07T20:02:38Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-07T20:02:38Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDaC8Ls#tuple-stress:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-07T20:02:44Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-07T20:02:44Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-07T20:02:44Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDaC8Ls#tuple-stress:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpDaC8Ls/incremental-state"
[2022-05-07T20:02:50Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-05-07T20:02:51Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDaC8Ls#tuple-stress:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpDaC8Ls/incremental-state"
[2022-05-07T20:02:53Z DEBUG collector::execute] applying new row to "/tmp/.tmpDaC8Ls"
[2022-05-07T20:02:53Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-05-07T20:02:53Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-05-07T20:02:53Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDaC8Ls#tuple-stress:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpDaC8Ls/incremental-state"
+ ./build/x86_64-unknown-linux-gnu/llvm/bin/llvm-profdata merge -o /tmp/rustc-pgo.profdata /tmp/rustc-pgo
+ rm -r ./build/x86_64-unknown-linux-gnu/llvm ./build/x86_64-unknown-linux-gnu/lld
+ python3 ../x.py dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu --include-default-paths build-manifest --rust-profile-use=/tmp/rustc-pgo.profdata --llvm-profile-use=/tmp/llvm-pgo.profdata
warning: sqlite3 not available in python, skipping build directory lock
---
[RUSTC-TIMING] scoped_tls test:false 0.102
   Compiling tinyvec v0.3.4
[RUSTC-TIMING] unicode_xid test:false 0.102
   Compiling regex-syntax v0.6.25
warning: rustc_fs_util.94537fd6-cgu.0: no profile data available for function _RNvXs2_NtCsfvvNiNxeVxr_4core7convertNtNtNtB7_5alloc6layout11LayoutErrorINtB5_4FromBy_E4fromCs6hO4mtB60wk_13rustc_fs_util Hash = 742261418966908927
[RUSTC-TIMING] unic_char_range test:false 0.194
   Compiling ansi_term v0.12.1
[RUSTC-TIMING] ppv_lite86 test:false 0.563
   Compiling snap v1.0.1
   Compiling snap v1.0.1
warning: rustc_graphviz.36d78d5c-cgu.0: no profile data available for function _RNvXs2_NtCsfvvNiNxeVxr_4core7convertNtNtNtB7_5alloc6layout11LayoutErrorINtB5_4FromBy_E4fromCs9im4dxzX1Vg_14rustc_graphviz Hash = 742261418966908927
[RUSTC-TIMING] rustc_fs_util test:false 0.123
warning: `rustc_fs_util` (lib) generated 1 warning
   Compiling crc32fast v1.2.0
   Compiling crc32fast v1.2.0
warning: rustc_graphviz.36d78d5c-cgu.6: no profile data available for function _RNvMNtNtNtCsfvvNiNxeVxr_4core4iter8adapters3revINtB2_3RevINtNtNtB8_3ops5range5RangejEE3newCs9im4dxzX1Vg_14rustc_graphviz Hash = 742261418966908927
[RUSTC-TIMING] either test:false 0.157
   Compiling adler v0.2.3
[RUSTC-TIMING] tinystr test:false 0.455
   Compiling unicode-script v0.5.3
---
[RUSTC-TIMING] matchers test:false 0.154
   Compiling block-buffer v0.10.2
[RUSTC-TIMING] rustc_parse_format test:false 1.235
   Compiling crypto-common v0.1.2
warning: rustc_llvm.c0d9ceed-cgu.1: no profile data available for function _RNvXs2_NtCsfvvNiNxeVxr_4core7convertNtNtNtB7_5alloc6layout11LayoutErrorINtB5_4FromBy_E4fromCs3hagUdXInIT_10rustc_llvm Hash = 742261418966908927
[RUSTC-TIMING] block_buffer test:false 0.133
   Compiling digest v0.10.2
[RUSTC-TIMING] crypto_common test:false 0.127
[RUSTC-TIMING] rand_chacha test:false 0.898
---
   Compiling rustc_error_messages v0.0.0 (/checkout/compiler/rustc_error_messages)
   Compiling rustc_target v0.0.0 (/checkout/compiler/rustc_target)
   Compiling rustc_feature v0.0.0 (/checkout/compiler/rustc_feature)
[RUSTC-TIMING] chalk_solve test:false 5.058
warning: rustc_error_messages.08261ec7-cgu.11: no profile data available for function _RINvMsx_NtCs6stzpUwpF8P_12tracing_core5fieldNtB6_8FieldSet9value_setATRNtB6_5FieldINtNtCsfvvNiNxeVxr_4core6option6OptionRDNtB6_5ValueEL_EEj1_ECsbbCVZPr1Wsd_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.08261ec7-cgu.11: no profile data available for function _RINvMsx_NtCs6stzpUwpF8P_12tracing_core5fieldNtB6_8FieldSet9value_setATRNtB6_5FieldINtNtCsfvvNiNxeVxr_4core6option6OptionRDNtB6_5ValueEL_EEj2_ECsbbCVZPr1Wsd_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.08261ec7-cgu.11: no profile data available for function _RINvMsx_NtCs6stzpUwpF8P_12tracing_core5fieldNtB6_8FieldSet9value_setATRNtB6_5FieldINtNtCsfvvNiNxeVxr_4core6option6OptionRDNtB6_5ValueEL_EEj5_ECsbbCVZPr1Wsd_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.08261ec7-cgu.11: no profile data available for function _RINvNtCs6stzpUwpF8P_12tracing_core5field5debugRINtNtCs13dE0rrVGCD_5alloc3vec3VecNtNtCseytYUf0AwGY_3std4path7PathBufEECsbbCVZPr1Wsd_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.08261ec7-cgu.11: no profile data available for function _RINvNtCs6stzpUwpF8P_12tracing_core5field5debugRINtNtCsfvvNiNxeVxr_4core6option6OptionNtCsbYZFK58jEp5_16unic_langid_impl18LanguageIdentifierEECsbbCVZPr1Wsd_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.08261ec7-cgu.11: no profile data available for function _RINvNtCs6stzpUwpF8P_12tracing_core5field5debugRINtNtCsfvvNiNxeVxr_4core6option6OptionNtNtCseytYUf0AwGY_3std4path7PathBufEECsbbCVZPr1Wsd_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.08261ec7-cgu.11: no profile data available for function _RINvNtCs6stzpUwpF8P_12tracing_core5field5debugRINtNtCsfvvNiNxeVxr_4core6option6OptionRNtNtCseytYUf0AwGY_3std4path4PathEECsbbCVZPr1Wsd_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.08261ec7-cgu.11: no profile data available for function _RINvNtCs6stzpUwpF8P_12tracing_core5field5debugRNtCsbYZFK58jEp5_16unic_langid_impl18LanguageIdentifierECsbbCVZPr1Wsd_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.08261ec7-cgu.11: no profile data available for function _RINvNtCs6stzpUwpF8P_12tracing_core5field5debugRNtNtCs9aZxfb2SW1O_13fluent_bundle8resource14FluentResourceECsbbCVZPr1Wsd_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.08261ec7-cgu.11: no profile data available for function _RINvNtCs6stzpUwpF8P_12tracing_core5field5debugRNtNtCseytYUf0AwGY_3std4path7PathBufECsbbCVZPr1Wsd_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.08261ec7-cgu.11: no profile data available for function _RINvNtCs6stzpUwpF8P_12tracing_core5field5debugRQNtNtCseytYUf0AwGY_3std4path7PathBufECsbbCVZPr1Wsd_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.08261ec7-cgu.11: no profile data available for function _RINvNtCs6stzpUwpF8P_12tracing_core5field5debugRRSReECsbbCVZPr1Wsd_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.08261ec7-cgu.11: no profile data available for function _RINvNtCsfvvNiNxeVxr_4core3ptr13drop_in_placeRINtNtB4_6option6OptionNtCsbYZFK58jEp5_16unic_langid_impl18LanguageIdentifierEECsbbCVZPr1Wsd_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.08261ec7-cgu.11: no profile data available for function _RINvNtCsfvvNiNxeVxr_4core3ptr13drop_in_placeRINtNtB4_6option6OptionNtNtCseytYUf0AwGY_3std4path7PathBufEECsbbCVZPr1Wsd_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.08261ec7-cgu.11: no profile data available for function _RINvNtCsfvvNiNxeVxr_4core3ptr13drop_in_placeRINtNtB4_6option6OptionRNtNtCseytYUf0AwGY_3std4path4PathEECsbbCVZPr1Wsd_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.08261ec7-cgu.11: no profile data available for function _RINvNtCsfvvNiNxeVxr_4core3ptr13drop_in_placeRINtNtCs13dE0rrVGCD_5alloc3vec3VecNtNtCseytYUf0AwGY_3std4path7PathBufEECsbbCVZPr1Wsd_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.08261ec7-cgu.11: no profile data available for function _RINvNtCsfvvNiNxeVxr_4core3ptr13drop_in_placeRNtCsbYZFK58jEp5_16unic_langid_impl18LanguageIdentifierECsbbCVZPr1Wsd_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.08261ec7-cgu.11: no profile data available for function _RINvNtCsfvvNiNxeVxr_4core3ptr13drop_in_placeRNtNtCs9aZxfb2SW1O_13fluent_bundle8resource14FluentResourceECsbbCVZPr1Wsd_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.08261ec7-cgu.11: no profile data available for function _RINvNtCsfvvNiNxeVxr_4core3ptr13drop_in_placeRNtNtCseytYUf0AwGY_3std4path7PathBufECsbbCVZPr1Wsd_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.08261ec7-cgu.11: no profile data available for function _RINvNtCsfvvNiNxeVxr_4core3ptr13drop_in_placeRQNtNtCseytYUf0AwGY_3std4path7PathBufECsbbCVZPr1Wsd_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.08261ec7-cgu.11: no profile data available for function _RINvNtCsfvvNiNxeVxr_4core3ptr13drop_in_placeRRSReECsbbCVZPr1Wsd_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.08261ec7-cgu.11: no profile data available for function _RNvXsk_NtCs6stzpUwpF8P_12tracing_core5fieldINtB5_10DebugValueRINtNtCs13dE0rrVGCD_5alloc3vec3VecNtNtCseytYUf0AwGY_3std4path7PathBufEENtB5_5Value6recordCsbbCVZPr1Wsd_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.08261ec7-cgu.11: no profile data available for function _RNvXsk_NtCs6stzpUwpF8P_12tracing_core5fieldINtB5_10DebugValueRINtNtCsfvvNiNxeVxr_4core6option6OptionNtCsbYZFK58jEp5_16unic_langid_impl18LanguageIdentifierEENtB5_5Value6recordCsbbCVZPr1Wsd_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.08261ec7-cgu.11: no profile data available for function _RNvXsk_NtCs6stzpUwpF8P_12tracing_core5fieldINtB5_10DebugValueRINtNtCsfvvNiNxeVxr_4core6option6OptionNtNtCseytYUf0AwGY_3std4path7PathBufEENtB5_5Value6recordCsbbCVZPr1Wsd_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.08261ec7-cgu.11: no profile data available for function _RNvXsk_NtCs6stzpUwpF8P_12tracing_core5fieldINtB5_10DebugValueRINtNtCsfvvNiNxeVxr_4core6option6OptionRNtNtCseytYUf0AwGY_3std4path4PathEENtB5_5Value6recordCsbbCVZPr1Wsd_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.08261ec7-cgu.11: no profile data available for function _RNvXsk_NtCs6stzpUwpF8P_12tracing_core5fieldINtB5_10DebugValueRNtCsbYZFK58jEp5_16unic_langid_impl18LanguageIdentifierENtB5_5Value6recordCsbbCVZPr1Wsd_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.08261ec7-cgu.11: no profile data available for function _RNvXsk_NtCs6stzpUwpF8P_12tracing_core5fieldINtB5_10DebugValueRNtNtCs9aZxfb2SW1O_13fluent_bundle8resource14FluentResourceENtB5_5Value6recordCsbbCVZPr1Wsd_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.08261ec7-cgu.11: no profile data available for function _RNvXsk_NtCs6stzpUwpF8P_12tracing_core5fieldINtB5_10DebugValueRNtNtCseytYUf0AwGY_3std4path7PathBufENtB5_5Value6recordCsbbCVZPr1Wsd_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.08261ec7-cgu.11: no profile data available for function _RNvXsk_NtCs6stzpUwpF8P_12tracing_core5fieldINtB5_10DebugValueRQNtNtCseytYUf0AwGY_3std4path7PathBufENtB5_5Value6recordCsbbCVZPr1Wsd_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.08261ec7-cgu.11: no profile data available for function _RNvXsk_NtCs6stzpUwpF8P_12tracing_core5fieldINtB5_10DebugValueRRSReENtB5_5Value6recordCsbbCVZPr1Wsd_20rustc_error_messages Hash = 170957022131388415
[RUSTC-TIMING] rustc_error_messages test:false 0.958
warning: `rustc_error_messages` (lib) generated 30 warnings
[RUSTC-TIMING] tikv_jemalloc_sys test:false 0.081
[RUSTC-TIMING] chalk_engine test:false 1.104
---
Dist rustc-docs-nightly-x86_64-unknown-linux-gnu
 finished in 18.720 seconds
[TIMING] dist::RustcDocs { host: x86_64-unknown-linux-gnu } -- 19.494
[TIMING] dist::Mingw { host: x86_64-unknown-linux-gnu } -- 0.000
thread 'main' panicked at 'failed to copy `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-56242c895220c3ec.so` to `/checkout/obj/build/tmp/tarball/rustc/x86_64-unknown-linux-gnu/image/lib/librustc_driver-56242c895220c3ec.so`: No such file or directory (os error 2)', src/bootstrap/lib.rs:1424:17
Build completed unsuccessfully in 0:29:13
