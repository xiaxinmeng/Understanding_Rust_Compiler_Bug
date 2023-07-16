plain
[2023-01-27T21:09:43Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-27T21:09:43Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-01-27T21:09:43Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpnrGNW2#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-27T21:09:45Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-01-27T21:09:45Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpnrGNW2#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpnrGNW2/incremental-state"
[2023-01-27T21:09:47Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-01-27T21:09:47Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpnrGNW2#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpnrGNW2/incremental-state"
[2023-01-27T21:09:48Z DEBUG collector::execute] applying println to "/tmp/.tmpnrGNW2"
[2023-01-27T21:09:48Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-27T21:09:48Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-27T21:09:48Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpnrGNW2#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpnrGNW2/incremental-state"
[2023-01-27T21:09:49Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-27T21:09:49Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-01-27T21:09:49Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpsMmnl6#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-27T21:09:50Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
---
[2023-01-27T21:09:55Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-27T21:09:55Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-01-27T21:09:55Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmppDwtns#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-27T21:09:57Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-01-27T21:09:57Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmppDwtns#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmppDwtns/incremental-state"
[2023-01-27T21:09:59Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-01-27T21:09:59Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmppDwtns#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmppDwtns/incremental-state"
[2023-01-27T21:10:00Z DEBUG collector::execute] applying println to "/tmp/.tmppDwtns"
[2023-01-27T21:10:00Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-27T21:10:00Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-27T21:10:00Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmppDwtns#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmppDwtns/incremental-state"
Executing benchmark cargo-0.60.0 (2/8)
Preparing cargo-0.60.0
[2023-01-27T21:10:01Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-01-27T21:10:01Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
---
[2023-01-27T21:10:53Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-27T21:10:53Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-01-27T21:10:53Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpVnB8nL#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-27T21:11:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-01-27T21:11:12Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpVnB8nL#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpVnB8nL/incremental-state"
[2023-01-27T21:11:34Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-01-27T21:11:34Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpVnB8nL#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpVnB8nL/incremental-state"
[2023-01-27T21:11:38Z DEBUG collector::execute] applying println to "/tmp/.tmpVnB8nL"
[2023-01-27T21:11:38Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-27T21:11:38Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-27T21:11:38Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpVnB8nL#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpVnB8nL/incremental-state"
[2023-01-27T21:11:42Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-27T21:11:43Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-01-27T21:11:43Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpd2BMKg#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-27T21:12:28Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
---
[2023-01-27T21:15:57Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-27T21:15:57Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-01-27T21:15:58Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpfUYXnP#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-27T21:16:06Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-01-27T21:16:06Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpfUYXnP#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpfUYXnP/incremental-state"
[2023-01-27T21:16:17Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-01-27T21:16:17Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpfUYXnP#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpfUYXnP/incremental-state"
[2023-01-27T21:16:17Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-27T21:16:17Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-01-27T21:16:17Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpsaH1Ar#ctfe-stress-5@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-27T21:16:25Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-01-27T21:16:25Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-01-27T21:16:25Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpsaH1Ar#ctfe-stress-5@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpsaH1Ar/incremental-state"
[2023-01-27T21:16:36Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-01-27T21:16:36Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpsaH1Ar#ctfe-stress-5@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpsaH1Ar/incremental-state"
[2023-01-27T21:16:37Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-27T21:16:37Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-01-27T21:16:37Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpYY60Zn#ctfe-stress-5@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-27T21:16:45Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
Preparing diesel-1.4.8
[2023-01-27T21:16:55Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-01-27T21:16:55Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-01-27T21:16:55Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-01-27T21:16:55Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpm9sG8A#diesel@1.4.8" "--profile" "check" "--" "--skip-this-rustc"
[2023-01-27T21:16:55Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpqWICDE#diesel@1.4.8" "--" "--skip-this-rustc"
[2023-01-27T21:16:55Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpnFU5sX#diesel@1.4.8" "--release" "--" "--skip-this-rustc"
[2023-01-27T21:17:13Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-27T21:17:13Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-01-27T21:17:13Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp3Qv7gH#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-27T21:17:27Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
---
[2023-01-27T21:17:51Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-27T21:17:51Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-01-27T21:17:51Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpYaRyhU#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-27T21:18:08Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-01-27T21:18:08Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpYaRyhU#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpYaRyhU/incremental-state"
[2023-01-27T21:18:29Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-01-27T21:18:29Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpYaRyhU#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpYaRyhU/incremental-state"
[2023-01-27T21:18:32Z DEBUG collector::execute] applying println to "/tmp/.tmpYaRyhU"
[2023-01-27T21:18:32Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-27T21:18:32Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-27T21:18:32Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpYaRyhU#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpYaRyhU/incremental-state"
[2023-01-27T21:18:36Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-27T21:18:36Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-01-27T21:18:36Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpMqn1ZA#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-27T21:18:54Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
[2023-01-27T21:19:22Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-27T21:19:22Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-01-27T21:19:22Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpuwMVtK#externs@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-27T21:19:23Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-01-27T21:19:23Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpuwMVtK#externs@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpuwMVtK/incremental-state"
[2023-01-27T21:19:23Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-01-27T21:19:23Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpuwMVtK#externs@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpuwMVtK/incremental-state"
[2023-01-27T21:19:24Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-27T21:19:24Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-01-27T21:19:24Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKlLWA0#externs@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-27T21:19:24Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-01-27T21:19:24Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-01-27T21:19:24Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKlLWA0#externs@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpKlLWA0/incremental-state"
[2023-01-27T21:19:25Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-01-27T21:19:25Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKlLWA0#externs@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpKlLWA0/incremental-state"
[2023-01-27T21:19:26Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-27T21:19:26Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-01-27T21:19:26Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp7dpEKb#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-27T21:19:27Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
[2023-01-27T21:19:48Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-27T21:19:48Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-01-27T21:19:48Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjbvPUk#token-stream-stress@0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-27T21:19:48Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-01-27T21:19:48Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjbvPUk#token-stream-stress@0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpjbvPUk/incremental-state"
[2023-01-27T21:19:49Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-01-27T21:19:49Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjbvPUk#token-stream-stress@0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpjbvPUk/incremental-state"
[2023-01-27T21:19:49Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-27T21:19:49Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-01-27T21:19:49Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpaWGejp#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-27T21:19:49Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-01-27T21:19:49Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-01-27T21:19:49Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpaWGejp#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpaWGejp/incremental-state"
[2023-01-27T21:19:49Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-01-27T21:19:49Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpaWGejp#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpaWGejp/incremental-state"
[2023-01-27T21:19:49Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-27T21:19:50Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-01-27T21:19:50Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpdyHYH7#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-27T21:19:50Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-01-27T21:19:50Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-01-27T21:19:50Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpdyHYH7#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpdyHYH7/incremental-state"
[2023-01-27T21:19:50Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-01-27T21:19:50Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpdyHYH7#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpdyHYH7/incremental-state"
Executing benchmark tuple-stress (8/8)
Preparing tuple-stress
[2023-01-27T21:19:50Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-01-27T21:19:50Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
---
[2023-01-27T21:20:07Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-27T21:20:07Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-01-27T21:20:07Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpkHxTfl#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-27T21:20:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-01-27T21:20:12Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpkHxTfl#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpkHxTfl/incremental-state"
[2023-01-27T21:20:18Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-01-27T21:20:18Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpkHxTfl#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpkHxTfl/incremental-state"
[2023-01-27T21:20:19Z DEBUG collector::execute] applying new row to "/tmp/.tmpkHxTfl"
[2023-01-27T21:20:19Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2023-01-27T21:20:19Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2023-01-27T21:20:19Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpkHxTfl#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpkHxTfl/incremental-state"
[2023-01-27T21:20:25Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-27T21:20:25Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-01-27T21:20:25Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmprNHlqP#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-27T21:20:30Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-01-27T21:20:30Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-01-27T21:20:30Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmprNHlqP#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmprNHlqP/incremental-state"
[2023-01-27T21:20:35Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-01-27T21:20:35Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmprNHlqP#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmprNHlqP/incremental-state"
[2023-01-27T21:20:37Z DEBUG collector::execute] applying new row to "/tmp/.tmprNHlqP"
[2023-01-27T21:20:37Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2023-01-27T21:20:37Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2023-01-27T21:20:37Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmprNHlqP#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmprNHlqP/incremental-state"
+ cd /checkout/obj
+ RUSTC_PROFILE_MERGED_FILE=/tmp/tmp-pgo/rustc-pgo.profdata
+ /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-profdata merge -o /tmp/tmp-pgo/rustc-pgo.profdata /tmp/tmp-pgo/rustc-pgo
Rustc PGO statistics
---
Documenting {build-manifest, bootstrap} stage2 library (x86_64-unknown-linux-gnu) in HTML format
 Documenting core v0.0.0 (/checkout/library/core)
error: Compilation failed, aborting rustdoc

error[E0425]: cannot find function `vld2_s8` in this scope
    --> library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:8772:15
     |
8772 |     transmute(vld2_s8(transmute(a)))
     |               ^^^^^^^ help: a function with a similar name exists: `vld2_p8`
...
8849 | pub unsafe fn vld2_p8(a: *const p8) -> poly8x8x2_t {
     | -------------------------------------------------- similarly named function `vld2_p8` defined here

error[E0425]: cannot find function `vld2_s16` in this scope
    --> library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:8785:15
     |
8785 |     transmute(vld2_s16(transmute(a)))
     |               ^^^^^^^^ help: a function with a similar name exists: `vld2_p16`
...
8862 | pub unsafe fn vld2_p16(a: *const p16) -> poly16x4x2_t {
     | ----------------------------------------------------- similarly named function `vld2_p16` defined here

error[E0425]: cannot find function `vld2_s32` in this scope
    --> library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:8798:15
     |
8798 |     transmute(vld2_s32(transmute(a)))


error[E0425]: cannot find function `vld2q_s8` in this scope
    --> library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:8811:15
     |
8811 |     transmute(vld2q_s8(transmute(a)))
     |               ^^^^^^^^ help: a function with a similar name exists: `vld2q_p8`
...
8875 | pub unsafe fn vld2q_p8(a: *const p8) -> poly8x16x2_t {
     | ---------------------------------------------------- similarly named function `vld2q_p8` defined here

error[E0425]: cannot find function `vld2q_s16` in this scope
    --> library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:8824:15
     |
8824 |     transmute(vld2q_s16(transmute(a)))
     |               ^^^^^^^^^ help: a function with a similar name exists: `vld2q_p16`
...
8888 | pub unsafe fn vld2q_p16(a: *const p16) -> poly16x8x2_t {
     | ------------------------------------------------------ similarly named function `vld2q_p16` defined here

error[E0425]: cannot find function `vld2q_s32` in this scope
    --> library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:8837:15
     |
8837 |     transmute(vld2q_s32(transmute(a)))


error[E0425]: cannot find function `vld2_s8` in this scope
    --> library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:8850:15
     |
8850 |     transmute(vld2_s8(transmute(a)))


error[E0425]: cannot find function `vld2_s16` in this scope
    --> library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:8863:15
     |
8863 |     transmute(vld2_s16(transmute(a)))


error[E0425]: cannot find function `vld2q_s8` in this scope
    --> library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:8876:15
     |
8876 |     transmute(vld2q_s8(transmute(a)))


error[E0425]: cannot find function `vld2q_s16` in this scope
    --> library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:8889:15
     |
8889 |     transmute(vld2q_s16(transmute(a)))


error[E0425]: cannot find function `vld2_s64` in this scope
    --> library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:8902:15
     |
8902 |     transmute(vld2_s64(transmute(a)))
     |               ^^^^^^^^ help: a function with a similar name exists: `vld2_p64`
...
8914 | pub unsafe fn vld2_p64(a: *const p64) -> poly64x1x2_t {
     | ----------------------------------------------------- similarly named function `vld2_p64` defined here

error[E0425]: cannot find function `vld2_s64` in this scope
    --> library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:8915:15
     |
8915 |     transmute(vld2_s64(transmute(a)))


error[E0425]: cannot find function `vld2_dup_s8` in this scope
    --> library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:9225:15
     |
9225 |     transmute(vld2_dup_s8(transmute(a)))
     |               ^^^^^^^^^^^ help: a function with a similar name exists: `vld1_dup_s8`
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/mod.rs:1364:1
     |
1364 | pub unsafe fn vld1_dup_s8(ptr: *const i8) -> int8x8_t {
     | ----------------------------------------------------- similarly named function `vld1_dup_s8` defined here

error[E0425]: cannot find function `vld2_dup_s16` in this scope
    --> library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:9238:15
     |
9238 |     transmute(vld2_dup_s16(transmute(a)))
     |               ^^^^^^^^^^^^ help: a function with a similar name exists: `vld1_dup_s16`
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/mod.rs:1394:1
     |
1394 | pub unsafe fn vld1_dup_s16(ptr: *const i16) -> int16x4_t {
     | -------------------------------------------------------- similarly named function `vld1_dup_s16` defined here

error[E0425]: cannot find function `vld2_dup_s32` in this scope
    --> library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:9251:15
     |
9251 |     transmute(vld2_dup_s32(transmute(a)))
     |               ^^^^^^^^^^^^ help: a function with a similar name exists: `vld1_dup_s32`
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/mod.rs:1424:1
     |
1424 | pub unsafe fn vld1_dup_s32(ptr: *const i32) -> int32x2_t {
     | -------------------------------------------------------- similarly named function `vld1_dup_s32` defined here

error[E0425]: cannot find function `vld2q_dup_s8` in this scope
    --> library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:9264:15
     |
9264 |     transmute(vld2q_dup_s8(transmute(a)))
     |               ^^^^^^^^^^^^ help: a function with a similar name exists: `vld1q_dup_s8`
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/mod.rs:1379:1
     |
1379 | pub unsafe fn vld1q_dup_s8(ptr: *const i8) -> int8x16_t {
     | ------------------------------------------------------- similarly named function `vld1q_dup_s8` defined here

error[E0425]: cannot find function `vld2q_dup_s16` in this scope
    --> library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:9277:15
     |
9277 |     transmute(vld2q_dup_s16(transmute(a)))
     |               ^^^^^^^^^^^^^ help: a function with a similar name exists: `vld1q_dup_s16`
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/mod.rs:1409:1
     |
1409 | pub unsafe fn vld1q_dup_s16(ptr: *const i16) -> int16x8_t {
     | --------------------------------------------------------- similarly named function `vld1q_dup_s16` defined here

error[E0425]: cannot find function `vld2q_dup_s32` in this scope
    --> library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:9290:15
     |
9290 |     transmute(vld2q_dup_s32(transmute(a)))
     |               ^^^^^^^^^^^^^ help: a function with a similar name exists: `vld1q_dup_s32`
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/mod.rs:1439:1
     |
1439 | pub unsafe fn vld1q_dup_s32(ptr: *const i32) -> int32x4_t {
     | --------------------------------------------------------- similarly named function `vld1q_dup_s32` defined here

error[E0425]: cannot find function `vld2_dup_s8` in this scope
    --> library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:9303:15
     |
9303 |     transmute(vld2_dup_s8(transmute(a)))
     |               ^^^^^^^^^^^ help: a function with a similar name exists: `vld1_dup_s8`
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/mod.rs:1364:1
     |
1364 | pub unsafe fn vld1_dup_s8(ptr: *const i8) -> int8x8_t {
     | ----------------------------------------------------- similarly named function `vld1_dup_s8` defined here

error[E0425]: cannot find function `vld2_dup_s16` in this scope
    --> library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:9316:15
     |
9316 |     transmute(vld2_dup_s16(transmute(a)))
     |               ^^^^^^^^^^^^ help: a function with a similar name exists: `vld1_dup_s16`
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/mod.rs:1394:1
     |
1394 | pub unsafe fn vld1_dup_s16(ptr: *const i16) -> int16x4_t {
     | -------------------------------------------------------- similarly named function `vld1_dup_s16` defined here

error[E0425]: cannot find function `vld2q_dup_s8` in this scope
    --> library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:9329:15
     |
9329 |     transmute(vld2q_dup_s8(transmute(a)))
     |               ^^^^^^^^^^^^ help: a function with a similar name exists: `vld1q_dup_s8`
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/mod.rs:1379:1
     |
1379 | pub unsafe fn vld1q_dup_s8(ptr: *const i8) -> int8x16_t {
     | ------------------------------------------------------- similarly named function `vld1q_dup_s8` defined here

error[E0425]: cannot find function `vld2q_dup_s16` in this scope
    --> library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:9342:15
     |
9342 |     transmute(vld2q_dup_s16(transmute(a)))
     |               ^^^^^^^^^^^^^ help: a function with a similar name exists: `vld1q_dup_s16`
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/mod.rs:1409:1
     |
1409 | pub unsafe fn vld1q_dup_s16(ptr: *const i16) -> int16x8_t {
     | --------------------------------------------------------- similarly named function `vld1q_dup_s16` defined here

error[E0425]: cannot find function `vld2_dup_s64` in this scope
    --> library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:9355:15
     |
9355 |     transmute(vld2_dup_s64(transmute(a)))
     |               ^^^^^^^^^^^^ help: a function with a similar name exists: `vld1_dup_s64`
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/mod.rs:1454:1
     |
1454 | pub unsafe fn vld1_dup_s64(ptr: *const i64) -> int64x1_t {
     | -------------------------------------------------------- similarly named function `vld1_dup_s64` defined here

error[E0425]: cannot find function `vld2_dup_s64` in this scope
    --> library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:9368:15
     |
9368 |     transmute(vld2_dup_s64(transmute(a)))
     |               ^^^^^^^^^^^^ help: a function with a similar name exists: `vld1_dup_s64`
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/mod.rs:1454:1
     |
1454 | pub unsafe fn vld1_dup_s64(ptr: *const i64) -> int64x1_t {
     | -------------------------------------------------------- similarly named function `vld1_dup_s64` defined here

error[E0425]: cannot find function `vld2_lane_s8` in this scope
    --> library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:9634:15
     |
9634 |     transmute(vld2_lane_s8::<LANE>(transmute(a), transmute(b)))
     |               ^^^^^^^^^^^^ help: a function with a similar name exists: `vld1_lane_s8`
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/mod.rs:977:1
     |
977  | pub unsafe fn vld1_lane_s8<const LANE: i32>(ptr: *const i8, src: int8x8_t) -> int8x8_t {
     | -------------------------------------------------------------------------------------- similarly named function `vld1_lane_s8` defined here

error[E0425]: cannot find function `vld2_lane_s16` in this scope
    --> library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:9649:15
     |
9649 |     transmute(vld2_lane_s16::<LANE>(transmute(a), transmute(b)))
     |               ^^^^^^^^^^^^^ help: a function with a similar name exists: `vld1_lane_s16`
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/mod.rs:1009:1
     |
1009 | pub unsafe fn vld1_lane_s16<const LANE: i32>(ptr: *const i16, src: int16x4_t) -> int16x4_t {
     | ------------------------------------------------------------------------------------------ similarly named function `vld1_lane_s16` defined here

error[E0425]: cannot find function `vld2_lane_s32` in this scope
    --> library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:9664:15
     |
9664 |     transmute(vld2_lane_s32::<LANE>(transmute(a), transmute(b)))
     |               ^^^^^^^^^^^^^ help: a function with a similar name exists: `vld1_lane_s32`
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/mod.rs:1041:1
     |
1041 | pub unsafe fn vld1_lane_s32<const LANE: i32>(ptr: *const i32, src: int32x2_t) -> int32x2_t {
     | ------------------------------------------------------------------------------------------ similarly named function `vld1_lane_s32` defined here

error[E0425]: cannot find function `vld2q_lane_s16` in this scope
    --> library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:9679:15
     |
9679 |     transmute(vld2q_lane_s16::<LANE>(transmute(a), transmute(b)))
     |               ^^^^^^^^^^^^^^ help: a function with a similar name exists: `vld1q_lane_s16`
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/mod.rs:1025:1
     |
1025 | pub unsafe fn vld1q_lane_s16<const LANE: i32>(ptr: *const i16, src: int16x8_t) -> int16x8_t {
     | ------------------------------------------------------------------------------------------- similarly named function `vld1q_lane_s16` defined here

error[E0425]: cannot find function `vld2q_lane_s32` in this scope
    --> library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:9694:15
     |
9694 |     transmute(vld2q_lane_s32::<LANE>(transmute(a), transmute(b)))
     |               ^^^^^^^^^^^^^^ help: a function with a similar name exists: `vld1q_lane_s32`
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/mod.rs:1057:1
     |
1057 | pub unsafe fn vld1q_lane_s32<const LANE: i32>(ptr: *const i32, src: int32x4_t) -> int32x4_t {
     | ------------------------------------------------------------------------------------------- similarly named function `vld1q_lane_s32` defined here

error[E0425]: cannot find function `vld2_lane_s8` in this scope
    --> library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:9709:15
     |
9709 |     transmute(vld2_lane_s8::<LANE>(transmute(a), transmute(b)))
     |               ^^^^^^^^^^^^ help: a function with a similar name exists: `vld1_lane_s8`
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/mod.rs:977:1
     |
977  | pub unsafe fn vld1_lane_s8<const LANE: i32>(ptr: *const i8, src: int8x8_t) -> int8x8_t {
     | -------------------------------------------------------------------------------------- similarly named function `vld1_lane_s8` defined here

error[E0425]: cannot find function `vld2_lane_s16` in this scope
    --> library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:9724:15
     |
9724 |     transmute(vld2_lane_s16::<LANE>(transmute(a), transmute(b)))
     |               ^^^^^^^^^^^^^ help: a function with a similar name exists: `vld1_lane_s16`
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/mod.rs:1009:1
     |
1009 | pub unsafe fn vld1_lane_s16<const LANE: i32>(ptr: *const i16, src: int16x4_t) -> int16x4_t {
     | ------------------------------------------------------------------------------------------ similarly named function `vld1_lane_s16` defined here

error[E0425]: cannot find function `vld2q_lane_s16` in this scope
    --> library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:9739:15
     |
9739 |     transmute(vld2q_lane_s16::<LANE>(transmute(a), transmute(b)))
     |               ^^^^^^^^^^^^^^ help: a function with a similar name exists: `vld1q_lane_s16`
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/mod.rs:1025:1
     |
1025 | pub unsafe fn vld1q_lane_s16<const LANE: i32>(ptr: *const i16, src: int16x8_t) -> int16x8_t {
     | ------------------------------------------------------------------------------------------- similarly named function `vld1q_lane_s16` defined here

error[E0425]: cannot find function `vld3_s8` in this scope
     --> library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:10057:15
      |
10057 |     transmute(vld3_s8(transmute(a)))
      |               ^^^^^^^ help: a function with a similar name exists: `vld3_p8`
...
10134 | pub unsafe fn vld3_p8(a: *const p8) -> poly8x8x3_t {
      | -------------------------------------------------- similarly named function `vld3_p8` defined here

error[E0425]: cannot find function `vld3_s16` in this scope
     --> library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:10070:15
      |
10070 |     transmute(vld3_s16(transmute(a)))
      |               ^^^^^^^^ help: a function with a similar name exists: `vld3_p16`
...
10147 | pub unsafe fn vld3_p16(a: *const p16) -> poly16x4x3_t {
      | ----------------------------------------------------- similarly named function `vld3_p16` defined here

error[E0425]: cannot find function `vld3_s32` in this scope
     --> library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:10083:15
      |
10083 |     transmute(vld3_s32(transmute(a)))


error[E0425]: cannot find function `vld3q_s8` in this scope
     --> library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:10096:15
      |
10096 |     transmute(vld3q_s8(transmute(a)))
      |               ^^^^^^^^ help: a function with a similar name exists: `vld3q_p8`
...
10160 | pub unsafe fn vld3q_p8(a: *const p8) -> poly8x16x3_t {
      | ---------------------------------------------------- similarly named function `vld3q_p8` defined here

error[E0425]: cannot find function `vld3q_s16` in this scope
     --> library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:10109:15
      |
10109 |     transmute(vld3q_s16(transmute(a)))
      |               ^^^^^^^^^ help: a function with a similar name exists: `vld3q_p16`
...
10173 | pub unsafe fn vld3q_p16(a: *const p16) -> poly16x8x3_t {
      | ------------------------------------------------------ similarly named function `vld3q_p16` defined here

error[E0425]: cannot find function `vld3q_s32` in this scope
     --> library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:10122:15
      |
10122 |     transmute(vld3q_s32(transmute(a)))


error[E0425]: cannot find function `vld3_s8` in this scope
     --> library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:10135:15
      |
10135 |     transmute(vld3_s8(transmute(a)))


error[E0425]: cannot find function `vld3_s16` in this scope
     --> library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:10148:15
      |
10148 |     transmute(vld3_s16(transmute(a)))


error[E0425]: cannot find function `vld3q_s8` in this scope
     --> library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:10161:15
      |
10161 |     transmute(vld3q_s8(transmute(a)))


error[E0425]: cannot find function `vld3q_s16` in this scope
     --> library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:10174:15
      |
10174 |     transmute(vld3q_s16(transmute(a)))


error[E0425]: cannot find function `vld3_s64` in this scope
     --> library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:10187:15
      |
10187 |     transmute(vld3_s64(transmute(a)))
      |               ^^^^^^^^ help: a function with a similar name exists: `vld3_p64`
...
10199 | pub unsafe fn vld3_p64(a: *const p64) -> poly64x1x3_t {
      | ----------------------------------------------------- similarly named function `vld3_p64` defined here

error[E0425]: cannot find function `vld3_s64` in this scope
     --> library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:10200:15
      |
10200 |     transmute(vld3_s64(transmute(a)))


error[E0425]: cannot find function `vld3_dup_s8` in this scope
     --> library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:10510:15
      |
10510 |     transmute(vld3_dup_s8(transmute(a)))
      |               ^^^^^^^^^^^ help: a function with a similar name exists: `vld1_dup_s8`
      |
     ::: library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/mod.rs:1364:1
      |
1364  | pub unsafe fn vld1_dup_s8(ptr: *const i8) -> int8x8_t {
      | ----------------------------------------------------- similarly named function `vld1_dup_s8` defined here

error[E0425]: cannot find function `vld3_dup_s16` in this scope
     --> library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:10523:15
      |
10523 |     transmute(vld3_dup_s16(transmute(a)))
      |               ^^^^^^^^^^^^ help: a function with a similar name exists: `vld1_dup_s16`
      |
     ::: library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/mod.rs:1394:1
      |
1394  | pub unsafe fn vld1_dup_s16(ptr: *const i16) -> int16x4_t {
      | -------------------------------------------------------- similarly named function `vld1_dup_s16` defined here

error[E0425]: cannot find function `vld3_dup_s32` in this scope
     --> library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:10536:15
      |
10536 |     transmute(vld3_dup_s32(transmute(a)))
      |               ^^^^^^^^^^^^ help: a function with a similar name exists: `vld1_dup_s32`
      |
     ::: library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/mod.rs:1424:1
      |
1424  | pub unsafe fn vld1_dup_s32(ptr: *const i32) -> int32x2_t {
      | -------------------------------------------------------- similarly named function `vld1_dup_s32` defined here

error[E0425]: cannot find function `vld3q_dup_s8` in this scope
     --> library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:10549:15
      |
10549 |     transmute(vld3q_dup_s8(transmute(a)))
      |               ^^^^^^^^^^^^ help: a function with a similar name exists: `vld1q_dup_s8`
      |
     ::: library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/mod.rs:1379:1
      |
1379  | pub unsafe fn vld1q_dup_s8(ptr: *const i8) -> int8x16_t {
      | ------------------------------------------------------- similarly named function `vld1q_dup_s8` defined here

error[E0425]: cannot find function `vld3q_dup_s16` in this scope
     --> library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:10562:15
      |
10562 |     transmute(vld3q_dup_s16(transmute(a)))
      |               ^^^^^^^^^^^^^ help: a function with a similar name exists: `vld1q_dup_s16`
      |
     ::: library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/mod.rs:1409:1
      |
1409  | pub unsafe fn vld1q_dup_s16(ptr: *const i16) -> int16x8_t {
      | --------------------------------------------------------- similarly named function `vld1q_dup_s16` defined here

error[E0425]: cannot find function `vld3q_dup_s32` in this scope
     --> library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:10575:15
      |
10575 |     transmute(vld3q_dup_s32(transmute(a)))
      |               ^^^^^^^^^^^^^ help: a function with a similar name exists: `vld1q_dup_s32`
      |
     ::: library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/mod.rs:1439:1
      |
1439  | pub unsafe fn vld1q_dup_s32(ptr: *const i32) -> int32x4_t {
      | --------------------------------------------------------- similarly named function `vld1q_dup_s32` defined here

error[E0425]: cannot find function `vld3_dup_s8` in this scope
     --> library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:10588:15
      |
10588 |     transmute(vld3_dup_s8(transmute(a)))
      |               ^^^^^^^^^^^ help: a function with a similar name exists: `vld1_dup_s8`
      |
     ::: library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/mod.rs:1364:1
      |
1364  | pub unsafe fn vld1_dup_s8(ptr: *const i8) -> int8x8_t {
      | ----------------------------------------------------- similarly named function `vld1_dup_s8` defined here

error[E0425]: cannot find function `vld3_dup_s16` in this scope
     --> library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:10601:15
      |
10601 |     transmute(vld3_dup_s16(transmute(a)))
      |               ^^^^^^^^^^^^ help: a function with a similar name exists: `vld1_dup_s16`
      |
     ::: library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/mod.rs:1394:1
      |
1394  | pub unsafe fn vld1_dup_s16(ptr: *const i16) -> int16x4_t {
      | -------------------------------------------------------- similarly named function `vld1_dup_s16` defined here

error[E0425]: cannot find function `vld3q_dup_s8` in this scope
     --> library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:10614:15
      |
10614 |     transmute(vld3q_dup_s8(transmute(a)))
      |               ^^^^^^^^^^^^ help: a function with a similar name exists: `vld1q_dup_s8`
      |
     ::: library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/mod.rs:1379:1
      |
1379  | pub unsafe fn vld1q_dup_s8(ptr: *const i8) -> int8x16_t {
      | ------------------------------------------------------- similarly named function `vld1q_dup_s8` defined here

error[E0425]: cannot find function `vld3q_dup_s16` in this scope
     --> library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:10627:15
      |
