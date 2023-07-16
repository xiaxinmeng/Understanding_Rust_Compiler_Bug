plain
[2023-01-27T18:51:01Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-27T18:51:01Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-01-27T18:51:01Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDAXAbE#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-27T18:51:03Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-01-27T18:51:03Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDAXAbE#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpDAXAbE/incremental-state"
[2023-01-27T18:51:06Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-01-27T18:51:06Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDAXAbE#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpDAXAbE/incremental-state"
[2023-01-27T18:51:07Z DEBUG collector::execute] applying println to "/tmp/.tmpDAXAbE"
[2023-01-27T18:51:07Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-27T18:51:07Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-27T18:51:07Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDAXAbE#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpDAXAbE/incremental-state"
Executing benchmark cargo-0.60.0 (2/8)
Preparing cargo-0.60.0
[2023-01-27T18:51:08Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-01-27T18:51:08Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2023-01-27T18:52:03Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-27T18:52:04Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-01-27T18:52:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpWdBj2V#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-27T18:52:23Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-01-27T18:52:23Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpWdBj2V#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpWdBj2V/incremental-state"
[2023-01-27T18:52:47Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-01-27T18:52:47Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpWdBj2V#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpWdBj2V/incremental-state"
[2023-01-27T18:52:51Z DEBUG collector::execute] applying println to "/tmp/.tmpWdBj2V"
[2023-01-27T18:52:51Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-27T18:52:51Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-27T18:52:51Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpWdBj2V#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpWdBj2V/incremental-state"
[2023-01-27T18:52:55Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-27T18:52:56Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-01-27T18:52:56Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwAnIeN#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-27T18:53:45Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-01-27T18:53:45Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-01-27T18:53:45Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwAnIeN#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpwAnIeN/incremental-state"
[2023-01-27T18:54:46Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-01-27T18:54:46Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwAnIeN#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpwAnIeN/incremental-state"
[2023-01-27T18:54:56Z DEBUG collector::execute] applying println to "/tmp/.tmpwAnIeN"
[2023-01-27T18:54:56Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-27T18:54:56Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-27T18:54:56Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwAnIeN#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpwAnIeN/incremental-state"
[2023-01-27T18:55:07Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-27T18:55:07Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-01-27T18:55:07Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp6rhHiT#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-27T18:56:07Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
[2023-01-27T18:58:42Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-27T18:58:42Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-01-27T18:58:42Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpGwNMXD#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-27T18:58:58Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-01-27T18:58:58Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpGwNMXD#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpGwNMXD/incremental-state"
[2023-01-27T18:59:17Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-01-27T18:59:17Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpGwNMXD#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpGwNMXD/incremental-state"
[2023-01-27T18:59:20Z DEBUG collector::execute] applying println to "/tmp/.tmpGwNMXD"
[2023-01-27T18:59:20Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-27T18:59:20Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-27T18:59:20Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpGwNMXD#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpGwNMXD/incremental-state"
[2023-01-27T18:59:24Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-27T18:59:24Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-01-27T18:59:24Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp9P3hSK#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-27T18:59:42Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
---
[2023-01-27T19:00:11Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-27T19:00:11Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-01-27T19:00:12Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp9NyTwI#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-27T19:00:30Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-01-27T19:00:30Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp9NyTwI#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp9NyTwI/incremental-state"
[2023-01-27T19:00:53Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-01-27T19:00:53Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp9NyTwI#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp9NyTwI/incremental-state"
[2023-01-27T19:00:57Z DEBUG collector::execute] applying println to "/tmp/.tmp9NyTwI"
[2023-01-27T19:00:57Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-27T19:00:57Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-27T19:00:57Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp9NyTwI#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp9NyTwI/incremental-state"
Executing benchmark externs (5/8)
Preparing externs
[2023-01-27T19:01:00Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-01-27T19:01:00Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
---
[2023-01-27T19:01:03Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-27T19:01:03Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-01-27T19:01:03Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmphZKWw7#externs@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-27T19:01:03Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-01-27T19:01:03Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmphZKWw7#externs@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmphZKWw7/incremental-state"
[2023-01-27T19:01:04Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-01-27T19:01:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmphZKWw7#externs@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmphZKWw7/incremental-state"
[2023-01-27T19:01:05Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-27T19:01:05Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-01-27T19:01:05Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpndPKRU#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-27T19:01:06Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
[2023-01-27T19:01:08Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-27T19:01:08Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-01-27T19:01:08Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp5JCToa#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-27T19:01:10Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-01-27T19:01:10Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp5JCToa#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp5JCToa/incremental-state"
[2023-01-27T19:01:14Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-01-27T19:01:14Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp5JCToa#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp5JCToa/incremental-state"
[2023-01-27T19:01:15Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-27T19:01:15Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-01-27T19:01:15Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmppfDKVX#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-27T19:01:17Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-01-27T19:01:17Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-01-27T19:01:17Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmppfDKVX#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmppfDKVX/incremental-state"
[2023-01-27T19:01:21Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-01-27T19:01:21Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmppfDKVX#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmppfDKVX/incremental-state"
[2023-01-27T19:01:22Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-27T19:01:22Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-01-27T19:01:22Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQm6ql6#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-27T19:01:25Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
[2023-01-27T19:01:30Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-27T19:01:30Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-01-27T19:01:30Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpg7nwkQ#token-stream-stress@0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-27T19:01:30Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-01-27T19:01:30Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpg7nwkQ#token-stream-stress@0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpg7nwkQ/incremental-state"
[2023-01-27T19:01:31Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-01-27T19:01:31Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpg7nwkQ#token-stream-stress@0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpg7nwkQ/incremental-state"
[2023-01-27T19:01:31Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-27T19:01:31Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-01-27T19:01:31Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjsgHN8#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-27T19:01:31Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-01-27T19:01:31Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-01-27T19:01:31Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjsgHN8#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpjsgHN8/incremental-state"
[2023-01-27T19:01:31Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-01-27T19:01:31Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjsgHN8#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpjsgHN8/incremental-state"
[2023-01-27T19:01:32Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-27T19:01:32Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-01-27T19:01:32Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpP1LNBw#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-27T19:01:32Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
[2023-01-27T19:01:51Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-27T19:01:51Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-01-27T19:01:51Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpicMFCG#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-27T19:01:56Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-01-27T19:01:56Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpicMFCG#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpicMFCG/incremental-state"
[2023-01-27T19:02:02Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-01-27T19:02:02Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpicMFCG#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpicMFCG/incremental-state"
[2023-01-27T19:02:04Z DEBUG collector::execute] applying new row to "/tmp/.tmpicMFCG"
[2023-01-27T19:02:04Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2023-01-27T19:02:04Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2023-01-27T19:02:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpicMFCG#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpicMFCG/incremental-state"
[2023-01-27T19:02:10Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-27T19:02:10Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-01-27T19:02:10Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmptXrgFu#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-27T19:02:15Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-01-27T19:02:15Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-01-27T19:02:15Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmptXrgFu#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmptXrgFu/incremental-state"
[2023-01-27T19:02:21Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-01-27T19:02:21Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmptXrgFu#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmptXrgFu/incremental-state"
[2023-01-27T19:02:22Z DEBUG collector::execute] applying new row to "/tmp/.tmptXrgFu"
[2023-01-27T19:02:22Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2023-01-27T19:02:22Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2023-01-27T19:02:22Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmptXrgFu#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmptXrgFu/incremental-state"
+ cd /checkout/obj
+ RUSTC_PROFILE_MERGED_FILE=/tmp/tmp-pgo/rustc-pgo.profdata
+ /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-profdata merge -o /tmp/tmp-pgo/rustc-pgo.profdata /tmp/tmp-pgo/rustc-pgo
+ echo 'Rustc PGO statistics'
---
[RUSTC-TIMING] tempfile test:false 0.499
error[E0308]: mismatched types
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustix-0.36.5/src/backend/linux_raw/io/errno.rs:30:18
    |
25  | #[derive(Eq, PartialEq, Hash, Copy, Clone)]
...
30  | pub struct Errno(u16);
30  | pub struct Errno(u16);
    |                  ^^^ expected `u16`, found pattern `u16 is 61441..=65535`
   ::: /rustc/eb5fc32d562d9e47e2e39272f7361bce86b65f51/library/core/src/cmp.rs:236:1
    |
236 | pub macro PartialEq($item:item) {
236 | pub macro PartialEq($item:item) {
    | ------------------- in this expansion of `#[derive(PartialEq)]`
    = note:      expected type `u16`
    = note:      expected type `u16`
            found pattern type `u16 is 61441..=65535`
error[E0308]: mismatched types
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustix-0.36.5/src/backend/linux_raw/io/errno.rs:30:18
    |
    |
25  | #[derive(Eq, PartialEq, Hash, Copy, Clone)]
...
30  | pub struct Errno(u16);
30  | pub struct Errno(u16);
    |                  ^^^ expected pattern `u16 is 61441..=65535`, found `u16`
   ::: /rustc/eb5fc32d562d9e47e2e39272f7361bce86b65f51/library/core/src/cmp.rs:236:1
    |
236 | pub macro PartialEq($item:item) {
236 | pub macro PartialEq($item:item) {
    | ------------------- in this expansion of `#[derive(PartialEq)]`
    |
    = note: expected pattern type `u16 is 61441..=65535`

error[E0308]: mismatched types
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustix-0.36.5/src/backend/linux_raw/io/errno.rs:54:10
   |
   |
54 |         (self.0 as i16 as i32).wrapping_neg()
   |          |
   |          expected `i16`, found `u16`
   |          pattern type casts removing the pattern cannote also change the patterned type


error[E0308]: mismatched types
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustix-0.36.5/src/backend/linux_raw/io/errno.rs:74:23
   |
74 |         unsafe { Self(encoded) }
   |                  ---- ^^^^^^^ expected pattern `u16 is 61441..=65535`, found `u16`
   |                  arguments to this function are incorrect
   |
   |
   = note: expected pattern type `u16 is 61441..=65535`
note: tuple struct defined here
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustix-0.36.5/src/backend/linux_raw/io/errno.rs:30:12
   |
30 | pub struct Errno(u16);
30 | pub struct Errno(u16);
   |            ^^^^^

error[E0308]: mismatched types
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustix-0.36.5/src/backend/linux_raw/io/errno.rs:87:35
   |
87 |         return Err(unsafe { Errno(raw.decode_error_code()) });
   |                             ----- ^^^^^^^^^^^^^^^^^^^^^^^ expected pattern `u16 is 61441..=65535`, found `u16`
   |                             arguments to this struct are incorrect
   |
   |
   = note: expected pattern type `u16 is 61441..=65535`
note: tuple struct defined here
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustix-0.36.5/src/backend/linux_raw/io/errno.rs:30:12
   |
30 | pub struct Errno(u16);
30 | pub struct Errno(u16);
   |            ^^^^^

error[E0308]: mismatched types
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustix-0.36.5/src/backend/linux_raw/io/errno.rs:102:35
    |
102 |         return Err(unsafe { Errno(raw.decode_error_code()) });
    |                             ----- ^^^^^^^^^^^^^^^^^^^^^^^ expected pattern `u16 is 61441..=65535`, found `u16`
    |                             arguments to this struct are incorrect
    |
    |
    = note: expected pattern type `u16 is 61441..=65535`
note: tuple struct defined here
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustix-0.36.5/src/backend/linux_raw/io/errno.rs:30:12
    |
30  | pub struct Errno(u16);
30  | pub struct Errno(u16);
    |            ^^^^^

error[E0308]: mismatched types
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustix-0.36.5/src/backend/linux_raw/io/errno.rs:115:35
    |
115 |         return Err(unsafe { Errno(raw.decode_error_code()) });
    |                             ----- ^^^^^^^^^^^^^^^^^^^^^^^ expected pattern `u16 is 61441..=65535`, found `u16`
    |                             arguments to this struct are incorrect
    |
    |
    = note: expected pattern type `u16 is 61441..=65535`
note: tuple struct defined here
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustix-0.36.5/src/backend/linux_raw/io/errno.rs:30:12
    |
30  | pub struct Errno(u16);
30  | pub struct Errno(u16);
    |            ^^^^^

error[E0308]: mismatched types
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustix-0.36.5/src/backend/linux_raw/io/errno.rs:130:35
    |
130 |         return Err(unsafe { Errno(raw.decode_error_code()) });
    |                             ----- ^^^^^^^^^^^^^^^^^^^^^^^ expected pattern `u16 is 61441..=65535`, found `u16`
    |                             arguments to this struct are incorrect
    |
    |
    = note: expected pattern type `u16 is 61441..=65535`
note: tuple struct defined here
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustix-0.36.5/src/backend/linux_raw/io/errno.rs:30:12
    |
30  | pub struct Errno(u16);
30  | pub struct Errno(u16);
    |            ^^^^^

error[E0308]: mismatched types
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustix-0.36.5/src/backend/linux_raw/io/errno.rs:144:35
    |
144 |         return Err(unsafe { Errno(raw.decode_error_code()) });
    |                             ----- ^^^^^^^^^^^^^^^^^^^^^^^ expected pattern `u16 is 61441..=65535`, found `u16`
    |                             arguments to this struct are incorrect
    |
    |
    = note: expected pattern type `u16 is 61441..=65535`
note: tuple struct defined here
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustix-0.36.5/src/backend/linux_raw/io/errno.rs:30:12
    |
30  | pub struct Errno(u16);
30  | pub struct Errno(u16);
    |            ^^^^^

error[E0308]: mismatched types
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustix-0.36.5/src/backend/linux_raw/io/errno.rs:174:26
    |
174 |         return Err(Errno(raw.decode_error_code()));
    |                    ----- ^^^^^^^^^^^^^^^^^^^^^^^ expected pattern `u16 is 61441..=65535`, found `u16`
    |                    arguments to this struct are incorrect
    |
    |
    = note: expected pattern type `u16 is 61441..=65535`
note: tuple struct defined here
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustix-0.36.5/src/backend/linux_raw/io/errno.rs:30:12
    |
30  | pub struct Errno(u16);
30  | pub struct Errno(u16);
    |            ^^^^^

error[E0308]: mismatched types
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustix-0.36.5/src/backend/linux_raw/io/errno.rs:203:26
    |
203 |         return Err(Errno(raw.decode_error_code()));
    |                    ----- ^^^^^^^^^^^^^^^^^^^^^^^ expected pattern `u16 is 61441..=65535`, found `u16`
    |                    arguments to this struct are incorrect
    |
    |
    = note: expected pattern type `u16 is 61441..=65535`
note: tuple struct defined here
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustix-0.36.5/src/backend/linux_raw/io/errno.rs:30:12
    |
30  | pub struct Errno(u16);
