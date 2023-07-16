plain
   Compiling scopeguard v1.1.0
[RUSTC-TIMING] cfg_if test:false 0.045
   Compiling typenum v1.12.0
[RUSTC-TIMING] cfg_if test:false 0.047
   Compiling rustc-hash v1.1.0 (https://github.com/lqd/rustc-hash/?branch=experiment-a#5847e17a)
   Compiling pin-project-lite v0.2.8
[RUSTC-TIMING] scopeguard test:false 0.066
   Compiling bitflags v1.2.1
[RUSTC-TIMING] unicode_xid test:false 0.105
---
   Compiling lazy_static v1.4.0
   Compiling parking_lot_core v0.8.5
   Compiling scopeguard v1.1.0
[RUSTC-TIMING] cfg_if test:false 0.095
   Compiling rustc-hash v1.1.0 (https://github.com/lqd/rustc-hash/?branch=experiment-a#5847e17a)
   Compiling pin-project-lite v0.2.8
[RUSTC-TIMING] scopeguard test:false 0.138
   Compiling typenum v1.12.0
[RUSTC-TIMING] lazy_static test:false 0.170
---
Executing benchmark serde-1.0.136 (6/7)
Preparing serde-1.0.136
[2022-05-26T11:20:17Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-26T11:20:17Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-26T11:20:17Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp6JdN67#serde:1.0.136" "--" "--skip-this-rustc"
[2022-05-26T11:20:17Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpXPFVVs#serde:1.0.136" "--release" "--" "--skip-this-rustc"
[2022-05-26T11:20:18Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T11:20:18Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-26T11:20:18Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-26T11:20:18Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpLvXFVl#serde:1.0.136" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T11:20:23Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T11:20:23Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-26T11:20:23Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-26T11:20:23Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpagcTOE#serde:1.0.136" "--release" "--" "--wrap-rustc-with" "Eprintln"
Executing benchmark syn-1.0.89 (7/7)
Preparing syn-1.0.89
[2022-05-26T11:20:27Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-26T11:20:27Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
   Compiling typenum v1.12.0
[RUSTC-TIMING] cfg_if test:false 0.051
   Compiling pin-project-lite v0.2.8
[RUSTC-TIMING] lazy_static test:false 0.075
   Compiling rustc-hash v1.1.0 (https://github.com/lqd/rustc-hash/?branch=experiment-a#5847e17a)
   Compiling bitflags v1.2.1
[RUSTC-TIMING] unicode_xid test:false 0.124
   Compiling ppv-lite86 v0.2.8
[RUSTC-TIMING] pin_project_lite test:false 0.072
---
[2022-05-26T11:35:48Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T11:35:48Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-05-26T11:35:48Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpw3BFdG#bitmaps:3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T11:35:50Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-05-26T11:35:50Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpw3BFdG#bitmaps:3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpw3BFdG/incremental-state"
[2022-05-26T11:35:54Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-05-26T11:35:54Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpw3BFdG#bitmaps:3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpw3BFdG/incremental-state"
[2022-05-26T11:35:55Z DEBUG collector::execute] applying println to "/tmp/.tmpw3BFdG"
[2022-05-26T11:35:55Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-26T11:35:55Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-26T11:35:55Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpw3BFdG#bitmaps:3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpw3BFdG/incremental-state"
[2022-05-26T11:35:56Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T11:35:56Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-26T11:35:56Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp7GXsb0#bitmaps:3.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T11:35:59Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-26T11:35:59Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-26T11:35:59Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp7GXsb0#bitmaps:3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp7GXsb0/incremental-state"
[2022-05-26T11:36:04Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-05-26T11:36:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp7GXsb0#bitmaps:3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp7GXsb0/incremental-state"
[2022-05-26T11:36:05Z DEBUG collector::execute] applying println to "/tmp/.tmp7GXsb0"
[2022-05-26T11:36:05Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-26T11:36:05Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-26T11:36:05Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp7GXsb0#bitmaps:3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp7GXsb0/incremental-state"
[2022-05-26T11:36:06Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T11:36:06Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-26T11:36:06Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpgQm5mk#bitmaps:3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T11:36:09Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-26T11:36:09Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-26T11:36:09Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpgQm5mk#bitmaps:3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpgQm5mk/incremental-state"
[2022-05-26T11:36:13Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-05-26T11:36:13Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpgQm5mk#bitmaps:3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpgQm5mk/incremental-state"
[2022-05-26T11:36:14Z DEBUG collector::execute] applying println to "/tmp/.tmpgQm5mk"
[2022-05-26T11:36:14Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-26T11:36:14Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-26T11:36:14Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpgQm5mk#bitmaps:3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpgQm5mk/incremental-state"
Executing benchmark cargo-0.60.0 (2/8)
Preparing cargo-0.60.0
[2022-05-26T11:36:16Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-05-26T11:36:16Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
---
[2022-05-26T11:42:03Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T11:42:04Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-26T11:42:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpNAKQ7m#cargo:0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T11:43:26Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-26T11:43:26Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpNAKQ7m#cargo:0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpNAKQ7m/incremental-state"
[2022-05-26T11:44:54Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-05-26T11:44:54Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpNAKQ7m#cargo:0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpNAKQ7m/incremental-state"
[2022-05-26T11:45:08Z DEBUG collector::execute] applying println to "/tmp/.tmpNAKQ7m"
[2022-05-26T11:45:08Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-26T11:45:08Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-26T11:45:08Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpNAKQ7m#cargo:0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpNAKQ7m/incremental-state"
Executing benchmark ctfe-stress-5 (3/8)
Preparing ctfe-stress-5
[2022-05-26T11:45:45Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-05-26T11:45:45Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2022-05-26T11:46:08Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T11:46:08Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-26T11:46:08Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpmzQ9NI#ctfe-stress-5:0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T11:46:17Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-26T11:46:17Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpmzQ9NI#ctfe-stress-5:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpmzQ9NI/incremental-state"
[2022-05-26T11:46:29Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-05-26T11:46:29Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpmzQ9NI#ctfe-stress-5:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpmzQ9NI/incremental-state"
[2022-05-26T11:46:30Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T11:46:30Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-26T11:46:30Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpSTWmdm#ctfe-stress-5:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T11:46:39Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-26T11:46:39Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-26T11:46:39Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpSTWmdm#ctfe-stress-5:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpSTWmdm/incremental-state"
[2022-05-26T11:46:50Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-05-26T11:46:50Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpSTWmdm#ctfe-stress-5:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpSTWmdm/incremental-state"
Executing benchmark diesel-1.4.8 (4/8)
Preparing diesel-1.4.8
[2022-05-26T11:46:51Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-05-26T11:46:51Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-26T11:46:51Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-26T11:46:51Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-26T11:46:51Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp02C1Qb#diesel:1.4.8" "--profile" "check" "--" "--skip-this-rustc"
[2022-05-26T11:46:51Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpXToq6C#diesel:1.4.8" "--release" "--" "--skip-this-rustc"
[2022-05-26T11:46:51Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpmDxRQR#diesel:1.4.8" "--" "--skip-this-rustc"
[2022-05-26T11:47:17Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T11:47:17Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-05-26T11:47:17Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-05-26T11:47:17Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmptiNiRU#diesel:1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T11:47:38Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-05-26T11:47:38Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmptiNiRU#diesel:1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmptiNiRU/incremental-state"
[2022-05-26T11:48:03Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-05-26T11:48:03Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmptiNiRU#diesel:1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmptiNiRU/incremental-state"
[2022-05-26T11:48:07Z DEBUG collector::execute] applying println to "/tmp/.tmptiNiRU"
[2022-05-26T11:48:07Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-26T11:48:07Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-26T11:48:07Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmptiNiRU#diesel:1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmptiNiRU/incremental-state"
[2022-05-26T11:48:11Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T11:48:11Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-26T11:48:11Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-26T11:48:11Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpxUiFQO#diesel:1.4.8" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T11:48:35Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-26T11:48:35Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpxUiFQO#diesel:1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpxUiFQO/incremental-state"
[2022-05-26T11:49:04Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-05-26T11:49:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpxUiFQO#diesel:1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpxUiFQO/incremental-state"
[2022-05-26T11:49:09Z DEBUG collector::execute] applying println to "/tmp/.tmpxUiFQO"
[2022-05-26T11:49:09Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-26T11:49:09Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-26T11:49:09Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpxUiFQO#diesel:1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpxUiFQO/incremental-state"
[2022-05-26T11:49:14Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T11:49:14Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-26T11:49:14Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-26T11:49:14Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpaX4FtM#diesel:1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T11:49:38Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-26T11:49:38Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpaX4FtM#diesel:1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpaX4FtM/incremental-state"
[2022-05-26T11:50:07Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-05-26T11:50:07Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpaX4FtM#diesel:1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpaX4FtM/incremental-state"
[2022-05-26T11:50:11Z DEBUG collector::execute] applying println to "/tmp/.tmpaX4FtM"
[2022-05-26T11:50:11Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-26T11:50:11Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-05-26T11:50:11Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpaX4FtM#diesel:1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpaX4FtM/incremental-state"
Executing benchmark externs (5/8)
Preparing externs
[2022-05-26T11:50:16Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-05-26T11:50:16Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2022-05-26T11:50:19Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T11:50:19Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-26T11:50:19Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjSNk3x#externs:0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T11:50:20Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-26T11:50:20Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjSNk3x#externs:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpjSNk3x/incremental-state"
[2022-05-26T11:50:22Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-05-26T11:50:22Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjSNk3x#externs:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpjSNk3x/incremental-state"
[2022-05-26T11:50:23Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T11:50:23Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-26T11:50:23Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpkrhLPr#externs:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T11:50:24Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-26T11:50:24Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-26T11:50:24Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpkrhLPr#externs:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpkrhLPr/incremental-state"
[2022-05-26T11:50:25Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-05-26T11:50:25Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpkrhLPr#externs:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpkrhLPr/incremental-state"
Executing benchmark match-stress (6/8)
Preparing match-stress
[2022-05-26T11:50:26Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-26T11:50:26Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
---
[2022-05-26T11:50:27Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T11:50:27Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-05-26T11:50:27Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDnbBp3#match-stress:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T11:50:30Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-05-26T11:50:30Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDnbBp3#match-stress:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpDnbBp3/incremental-state"
[2022-05-26T11:50:34Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-05-26T11:50:34Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDnbBp3#match-stress:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpDnbBp3/incremental-state"
[2022-05-26T11:50:36Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T11:50:36Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-26T11:50:36Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpbPun8P#match-stress:0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T11:50:39Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-26T11:50:39Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-26T11:50:39Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpbPun8P#match-stress:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpbPun8P/incremental-state"
[2022-05-26T11:50:44Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-05-26T11:50:44Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpbPun8P#match-stress:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpbPun8P/incremental-state"
[2022-05-26T11:50:45Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T11:50:45Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-26T11:50:45Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp3eokeh#match-stress:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T11:50:49Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-26T11:50:49Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-26T11:50:49Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp3eokeh#match-stress:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp3eokeh/incremental-state"
[2022-05-26T11:50:54Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-05-26T11:50:54Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp3eokeh#match-stress:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp3eokeh/incremental-state"
Executing benchmark token-stream-stress (7/8)
Preparing token-stream-stress
[2022-05-26T11:50:55Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-05-26T11:50:55Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
---
[2022-05-26T11:50:56Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T11:50:56Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-05-26T11:50:56Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpn1cOUS#token-stream-stress:0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T11:50:56Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-05-26T11:50:56Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpn1cOUS#token-stream-stress:0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpn1cOUS/incremental-state"
[2022-05-26T11:50:57Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-05-26T11:50:57Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpn1cOUS#token-stream-stress:0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpn1cOUS/incremental-state"
[2022-05-26T11:50:57Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T11:50:57Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-26T11:50:57Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKgpj8d#token-stream-stress:0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T11:50:57Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-26T11:50:57Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-26T11:50:57Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKgpj8d#token-stream-stress:0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpKgpj8d/incremental-state"
[2022-05-26T11:50:58Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-05-26T11:50:58Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKgpj8d#token-stream-stress:0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpKgpj8d/incremental-state"
[2022-05-26T11:50:58Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T11:50:58Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-26T11:50:58Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp2VeRPg#token-stream-stress:0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T11:50:58Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
[2022-05-26T11:51:14Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp8Zh8Pf#tuple-stress:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp8Zh8Pf/incremental-state"
Running tuple-stress: Debug + [Full, IncrFull, IncrUnchanged, IncrPatched]
[2022-05-26T11:51:21Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T11:51:21Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-26T11:51:21Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpfyViWz#tuple-stress:0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T11:51:27Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-05-26T11:51:27Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpfyViWz#tuple-stress:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpfyViWz/incremental-state"
[2022-05-26T11:51:34Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-05-26T11:51:34Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpfyViWz#tuple-stress:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpfyViWz/incremental-state"
[2022-05-26T11:51:36Z DEBUG collector::execute] applying new row to "/tmp/.tmpfyViWz"
[2022-05-26T11:51:36Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-05-26T11:51:36Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-05-26T11:51:36Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpfyViWz#tuple-stress:0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpfyViWz/incremental-state"
[2022-05-26T11:51:43Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-26T11:51:43Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-26T11:51:43Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp0piSLz#tuple-stress:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-26T11:51:49Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-26T11:51:49Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-05-26T11:51:49Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp0piSLz#tuple-stress:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp0piSLz/incremental-state"
[2022-05-26T11:51:56Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-05-26T11:51:56Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp0piSLz#tuple-stress:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp0piSLz/incremental-state"
[2022-05-26T11:51:58Z DEBUG collector::execute] applying new row to "/tmp/.tmp0piSLz"
[2022-05-26T11:51:58Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-05-26T11:51:58Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-05-26T11:51:58Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp0piSLz#tuple-stress:0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp0piSLz/incremental-state"
+ cd /checkout/obj
+ ./build/x86_64-unknown-linux-gnu/llvm/bin/llvm-profdata merge -o /tmp/rustc-pgo.profdata /tmp/rustc-pgo
+ rm -r ./build/x86_64-unknown-linux-gnu/llvm ./build/x86_64-unknown-linux-gnu/lld
+ python3 ../x.py dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu --include-default-paths build-manifest --rust-profile-use=/tmp/rustc-pgo.profdata --llvm-profile-use=/tmp/llvm-pgo.profdata
---
   Compiling lazy_static v1.4.0
   Compiling parking_lot_core v0.8.5
   Compiling scopeguard v1.1.0
[RUSTC-TIMING] cfg_if test:false 0.046
   Compiling rustc-hash v1.1.0 (https://github.com/lqd/rustc-hash/?branch=experiment-a#5847e17a)
   Compiling typenum v1.12.0
[RUSTC-TIMING] lazy_static test:false 0.066
   Compiling pin-project-lite v0.2.8
[RUSTC-TIMING] scopeguard test:false 0.080
---
[RUSTC-TIMING] scoped_tls test:false 0.129
   Compiling regex-syntax v0.6.25
[RUSTC-TIMING] unicode_xid test:false 0.112
   Compiling tinyvec v0.3.4
warning: rustc_fs_util.780c1269-cgu.5: no profile data available for function _RNvXs2_NtCsfY2IPxZYHg5_4core7convertNtNtNtB7_5alloc6layout11LayoutErrorINtB5_4FromBy_E4fromCsfJyl3q0ZEnt_13rustc_fs_util Hash = 742261418966908927

warning: rustc_graphviz.c719fdaa-cgu.8: no profile data available for function _RNvXs2_NtCsfY2IPxZYHg5_4core7convertNtNtNtB7_5alloc6layout11LayoutErrorINtB5_4FromBy_E4fromCsdJuibxgfnle_14rustc_graphviz Hash = 742261418966908927
[RUSTC-TIMING] unic_char_range test:false 0.185
   Compiling crc32fast v1.2.0
[RUSTC-TIMING] rustc_fs_util test:false 0.135
warning: `rustc_fs_util` (lib) generated 1 warning
warning: `rustc_fs_util` (lib) generated 1 warning
   Compiling ansi_term v0.12.1
warning: rustc_graphviz.c719fdaa-cgu.12: no profile data available for function _RNvMNtNtNtCsfY2IPxZYHg5_4core4iter8adapters3revINtB2_3RevINtNtNtB8_3ops5range5RangejEE3newCsdJuibxgfnle_14rustc_graphviz Hash = 742261418966908927
[RUSTC-TIMING] ppv_lite86 test:false 0.632
   Compiling snap v1.0.1
[RUSTC-TIMING] either test:false 0.179
   Compiling adler v0.2.3
---
   Compiling rand_chacha v0.3.0
[RUSTC-TIMING] rand_core test:false 0.424
   Compiling rand_xoshiro v0.6.0
[RUSTC-TIMING] tracing_log test:false 1.942
warning: rustc_llvm.12176132-cgu.5: no profile data available for function _RNvXs2_NtCsfY2IPxZYHg5_4core7convertNtNtNtB7_5alloc6layout11LayoutErrorINtB5_4FromBy_E4fromCshjPMP0ZmDjq_10rustc_llvm Hash = 742261418966908927
[RUSTC-TIMING] stacker test:false 0.361
   Compiling hashbrown v0.11.2
[RUSTC-TIMING] rustc_llvm test:false 0.302
warning: `rustc_llvm` (lib) generated 1 warning
---
   Compiling rustc_ast v0.0.0 (/checkout/compiler/rustc_ast)
   Compiling rustc_feature v0.0.0 (/checkout/compiler/rustc_feature)
   Compiling rustc_error_messages v0.0.0 (/checkout/compiler/rustc_error_messages)
   Compiling rustc_target v0.0.0 (/checkout/compiler/rustc_target)
warning: rustc_error_messages.f3e9d844-cgu.11: no profile data available for function _RINvMsx_NtCsju6FPRoMItK_12tracing_core5fieldNtB6_8FieldSet9value_setATRNtB6_5FieldINtNtCsfY2IPxZYHg5_4core6option6OptionRDNtB6_5ValueEL_EEj1_ECs4P0LsrO2Eim_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.f3e9d844-cgu.11: no profile data available for function _RINvMsx_NtCsju6FPRoMItK_12tracing_core5fieldNtB6_8FieldSet9value_setATRNtB6_5FieldINtNtCsfY2IPxZYHg5_4core6option6OptionRDNtB6_5ValueEL_EEj2_ECs4P0LsrO2Eim_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.f3e9d844-cgu.11: no profile data available for function _RINvMsx_NtCsju6FPRoMItK_12tracing_core5fieldNtB6_8FieldSet9value_setATRNtB6_5FieldINtNtCsfY2IPxZYHg5_4core6option6OptionRDNtB6_5ValueEL_EEj5_ECs4P0LsrO2Eim_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.f3e9d844-cgu.11: no profile data available for function _RINvNtCsfY2IPxZYHg5_4core3ptr13drop_in_placeRINtNtB4_6option6OptionNtCs2QCeFxd6RwA_16unic_langid_impl18LanguageIdentifierEECs4P0LsrO2Eim_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.f3e9d844-cgu.11: no profile data available for function _RINvNtCsfY2IPxZYHg5_4core3ptr13drop_in_placeRINtNtB4_6option6OptionNtNtCs3XYnHxdgWuh_3std4path7PathBufEECs4P0LsrO2Eim_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.f3e9d844-cgu.11: no profile data available for function _RINvNtCsfY2IPxZYHg5_4core3ptr13drop_in_placeRINtNtB4_6option6OptionRNtNtCs3XYnHxdgWuh_3std4path4PathEECs4P0LsrO2Eim_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.f3e9d844-cgu.11: no profile data available for function _RINvNtCsfY2IPxZYHg5_4core3ptr13drop_in_placeRINtNtCsdeGPVCYKVzt_5alloc3vec3VecNtNtCs3XYnHxdgWuh_3std4path7PathBufEECs4P0LsrO2Eim_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.f3e9d844-cgu.11: no profile data available for function _RINvNtCsfY2IPxZYHg5_4core3ptr13drop_in_placeRNtCs2QCeFxd6RwA_16unic_langid_impl18LanguageIdentifierECs4P0LsrO2Eim_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.f3e9d844-cgu.11: no profile data available for function _RINvNtCsfY2IPxZYHg5_4core3ptr13drop_in_placeRNtNtCs3XYnHxdgWuh_3std4path7PathBufECs4P0LsrO2Eim_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.f3e9d844-cgu.11: no profile data available for function _RINvNtCsfY2IPxZYHg5_4core3ptr13drop_in_placeRNtNtCsP46s2CopLW_13fluent_bundle8resource14FluentResourceECs4P0LsrO2Eim_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.f3e9d844-cgu.11: no profile data available for function _RINvNtCsfY2IPxZYHg5_4core3ptr13drop_in_placeRQNtNtCs3XYnHxdgWuh_3std4path7PathBufECs4P0LsrO2Eim_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.f3e9d844-cgu.11: no profile data available for function _RINvNtCsfY2IPxZYHg5_4core3ptr13drop_in_placeRRSReECs4P0LsrO2Eim_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.f3e9d844-cgu.11: no profile data available for function _RINvNtCsju6FPRoMItK_12tracing_core5field5debugRINtNtCsdeGPVCYKVzt_5alloc3vec3VecNtNtCs3XYnHxdgWuh_3std4path7PathBufEECs4P0LsrO2Eim_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.f3e9d844-cgu.11: no profile data available for function _RINvNtCsju6FPRoMItK_12tracing_core5field5debugRINtNtCsfY2IPxZYHg5_4core6option6OptionNtCs2QCeFxd6RwA_16unic_langid_impl18LanguageIdentifierEECs4P0LsrO2Eim_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.f3e9d844-cgu.11: no profile data available for function _RINvNtCsju6FPRoMItK_12tracing_core5field5debugRINtNtCsfY2IPxZYHg5_4core6option6OptionNtNtCs3XYnHxdgWuh_3std4path7PathBufEECs4P0LsrO2Eim_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.f3e9d844-cgu.11: no profile data available for function _RINvNtCsju6FPRoMItK_12tracing_core5field5debugRINtNtCsfY2IPxZYHg5_4core6option6OptionRNtNtCs3XYnHxdgWuh_3std4path4PathEECs4P0LsrO2Eim_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.f3e9d844-cgu.11: no profile data available for function _RINvNtCsju6FPRoMItK_12tracing_core5field5debugRNtCs2QCeFxd6RwA_16unic_langid_impl18LanguageIdentifierECs4P0LsrO2Eim_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.f3e9d844-cgu.11: no profile data available for function _RINvNtCsju6FPRoMItK_12tracing_core5field5debugRNtNtCs3XYnHxdgWuh_3std4path7PathBufECs4P0LsrO2Eim_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.f3e9d844-cgu.11: no profile data available for function _RINvNtCsju6FPRoMItK_12tracing_core5field5debugRNtNtCsP46s2CopLW_13fluent_bundle8resource14FluentResourceECs4P0LsrO2Eim_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.f3e9d844-cgu.11: no profile data available for function _RINvNtCsju6FPRoMItK_12tracing_core5field5debugRQNtNtCs3XYnHxdgWuh_3std4path7PathBufECs4P0LsrO2Eim_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.f3e9d844-cgu.11: no profile data available for function _RINvNtCsju6FPRoMItK_12tracing_core5field5debugRRSReECs4P0LsrO2Eim_20rustc_error_messages Hash = 742261418966908927

warning: rustc_error_messages.f3e9d844-cgu.11: no profile data available for function _RNvXsk_NtCsju6FPRoMItK_12tracing_core5fieldINtB5_10DebugValueRINtNtCsdeGPVCYKVzt_5alloc3vec3VecNtNtCs3XYnHxdgWuh_3std4path7PathBufEENtB5_5Value6recordCs4P0LsrO2Eim_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.f3e9d844-cgu.11: no profile data available for function _RNvXsk_NtCsju6FPRoMItK_12tracing_core5fieldINtB5_10DebugValueRINtNtCsfY2IPxZYHg5_4core6option6OptionNtCs2QCeFxd6RwA_16unic_langid_impl18LanguageIdentifierEENtB5_5Value6recordCs4P0LsrO2Eim_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.f3e9d844-cgu.11: no profile data available for function _RNvXsk_NtCsju6FPRoMItK_12tracing_core5fieldINtB5_10DebugValueRINtNtCsfY2IPxZYHg5_4core6option6OptionNtNtCs3XYnHxdgWuh_3std4path7PathBufEENtB5_5Value6recordCs4P0LsrO2Eim_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.f3e9d844-cgu.11: no profile data available for function _RNvXsk_NtCsju6FPRoMItK_12tracing_core5fieldINtB5_10DebugValueRINtNtCsfY2IPxZYHg5_4core6option6OptionRNtNtCs3XYnHxdgWuh_3std4path4PathEENtB5_5Value6recordCs4P0LsrO2Eim_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.f3e9d844-cgu.11: no profile data available for function _RNvXsk_NtCsju6FPRoMItK_12tracing_core5fieldINtB5_10DebugValueRNtCs2QCeFxd6RwA_16unic_langid_impl18LanguageIdentifierENtB5_5Value6recordCs4P0LsrO2Eim_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.f3e9d844-cgu.11: no profile data available for function _RNvXsk_NtCsju6FPRoMItK_12tracing_core5fieldINtB5_10DebugValueRNtNtCs3XYnHxdgWuh_3std4path7PathBufENtB5_5Value6recordCs4P0LsrO2Eim_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.f3e9d844-cgu.11: no profile data available for function _RNvXsk_NtCsju6FPRoMItK_12tracing_core5fieldINtB5_10DebugValueRNtNtCsP46s2CopLW_13fluent_bundle8resource14FluentResourceENtB5_5Value6recordCs4P0LsrO2Eim_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.f3e9d844-cgu.11: no profile data available for function _RNvXsk_NtCsju6FPRoMItK_12tracing_core5fieldINtB5_10DebugValueRQNtNtCs3XYnHxdgWuh_3std4path7PathBufENtB5_5Value6recordCs4P0LsrO2Eim_20rustc_error_messages Hash = 170957022131388415

warning: rustc_error_messages.f3e9d844-cgu.11: no profile data available for function _RNvXsk_NtCsju6FPRoMItK_12tracing_core5fieldINtB5_10DebugValueRRSReENtB5_5Value6recordCs4P0LsrO2Eim_20rustc_error_messages Hash = 170957022131388415
[RUSTC-TIMING] rustc_error_messages test:false 1.000
warning: `rustc_error_messages` (lib) generated 30 warnings
[RUSTC-TIMING] chalk_engine test:false 1.313
[RUSTC-TIMING] rustc_feature test:false 1.231
---
    Checking pin-project-lite v0.2.8
[RUSTC-TIMING] cfg_if test:false 0.044
   Compiling bitflags v1.2.1
[RUSTC-TIMING] lazy_static test:false 0.061
    Checking rustc-hash v1.1.0 (https://github.com/lqd/rustc-hash/?branch=experiment-a#5847e17a)
   Compiling typenum v1.12.0
[RUSTC-TIMING] unicode_xid test:false 0.096
    Checking ppv-lite86 v0.2.8
[RUSTC-TIMING] pin_project_lite test:false 0.052
