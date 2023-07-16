plain
Preparing bitmaps-3.1.0
[2022-07-18T08:47:32Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-07-18T08:47:32Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-07-18T08:47:32Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-07-18T08:47:32Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpbvkHoO#bitmaps@3.1.0" "--profile" "check" "--" "--skip-this-rustc"
[2022-07-18T08:47:32Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp9oVLKt#bitmaps@3.1.0" "--" "--skip-this-rustc"
[2022-07-18T08:47:32Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpxLi4pg#bitmaps@3.1.0" "--release" "--" "--skip-this-rustc"
[2022-07-18T08:47:33Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-18T08:47:33Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-07-18T08:47:33Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjxegD8#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-07-18T08:47:35Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-07-18T08:47:35Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-07-18T08:47:35Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjxegD8#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpjxegD8/incremental-state"
[2022-07-18T08:47:37Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-07-18T08:47:37Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjxegD8#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpjxegD8/incremental-state"
[2022-07-18T08:47:38Z DEBUG collector::execute] applying println to "/tmp/.tmpjxegD8"
[2022-07-18T08:47:38Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-07-18T08:47:38Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-07-18T08:47:38Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjxegD8#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpjxegD8/incremental-state"
[2022-07-18T08:47:39Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-18T08:47:39Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-07-18T08:47:39Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpIweT2g#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-07-18T08:47:41Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
---
[2022-07-18T08:47:46Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-18T08:47:46Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-07-18T08:47:46Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpVxHvRn#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-07-18T08:47:48Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-07-18T08:47:48Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpVxHvRn#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpVxHvRn/incremental-state"
[2022-07-18T08:47:52Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-07-18T08:47:52Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpVxHvRn#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpVxHvRn/incremental-state"
[2022-07-18T08:47:52Z DEBUG collector::execute] applying println to "/tmp/.tmpVxHvRn"
[2022-07-18T08:47:52Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-07-18T08:47:52Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-07-18T08:47:52Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpVxHvRn#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpVxHvRn/incremental-state"
Executing benchmark cargo-0.60.0 (2/8)
Preparing cargo-0.60.0
[2022-07-18T08:47:53Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-07-18T08:47:53Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2022-07-18T08:52:14Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-18T08:52:14Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-07-18T08:52:14Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp2gpRWm#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2022-07-18T08:53:14Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-07-18T08:53:14Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp2gpRWm#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp2gpRWm/incremental-state"
[2022-07-18T08:54:15Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-07-18T08:54:15Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp2gpRWm#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp2gpRWm/incremental-state"
[2022-07-18T08:54:25Z DEBUG collector::execute] applying println to "/tmp/.tmp2gpRWm"
[2022-07-18T08:54:25Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-07-18T08:54:25Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-07-18T08:54:25Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp2gpRWm#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp2gpRWm/incremental-state"
Executing benchmark ctfe-stress-5 (3/8)
Preparing ctfe-stress-5
[2022-07-18T08:54:53Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-07-18T08:54:53Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2022-07-18T08:55:32Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-18T08:55:32Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-07-18T08:55:32Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpTFPoS5#ctfe-stress-5@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-07-18T08:55:40Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-07-18T08:55:40Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpTFPoS5#ctfe-stress-5@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpTFPoS5/incremental-state"
[2022-07-18T08:55:51Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-07-18T08:55:51Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpTFPoS5#ctfe-stress-5@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpTFPoS5/incremental-state"
Executing benchmark diesel-1.4.8 (4/8)
Preparing diesel-1.4.8
[2022-07-18T08:55:51Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-07-18T08:55:51Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
---
[2022-07-18T08:56:10Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-18T08:56:10Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-07-18T08:56:10Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpcSs0Oh#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-07-18T08:56:27Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-07-18T08:56:27Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpcSs0Oh#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpcSs0Oh/incremental-state"
[2022-07-18T08:56:46Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-07-18T08:56:46Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpcSs0Oh#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpcSs0Oh/incremental-state"
[2022-07-18T08:56:49Z DEBUG collector::execute] applying println to "/tmp/.tmpcSs0Oh"
[2022-07-18T08:56:49Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-07-18T08:56:49Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-07-18T08:56:49Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpcSs0Oh#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpcSs0Oh/incremental-state"
[2022-07-18T08:56:53Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-18T08:56:53Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-07-18T08:56:53Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp9HtaxS#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln"
[2022-07-18T08:57:11Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-07-18T08:57:11Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-07-18T08:57:11Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp9HtaxS#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp9HtaxS/incremental-state"
[2022-07-18T08:57:33Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-07-18T08:57:33Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp9HtaxS#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp9HtaxS/incremental-state"
[2022-07-18T08:57:37Z DEBUG collector::execute] applying println to "/tmp/.tmp9HtaxS"
[2022-07-18T08:57:37Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-07-18T08:57:37Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-07-18T08:57:37Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp9HtaxS#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp9HtaxS/incremental-state"
[2022-07-18T08:57:41Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-18T08:57:41Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-07-18T08:57:41Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp0mniMM#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-07-18T08:58:00Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
[2022-07-18T08:58:33Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-18T08:58:33Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-07-18T08:58:33Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOBDOuh#externs@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-07-18T08:58:34Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-07-18T08:58:34Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOBDOuh#externs@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpOBDOuh/incremental-state"
[2022-07-18T08:58:35Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-07-18T08:58:35Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOBDOuh#externs@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpOBDOuh/incremental-state"
[2022-07-18T08:58:36Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-18T08:58:36Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-07-18T08:58:36Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp9fjdrc#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-07-18T08:58:36Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-07-18T08:58:36Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-07-18T08:58:36Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp9fjdrc#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp9fjdrc/incremental-state"
[2022-07-18T08:58:38Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-07-18T08:58:38Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp9fjdrc#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp9fjdrc/incremental-state"
Executing benchmark match-stress (6/8)
Preparing match-stress
[2022-07-18T08:58:38Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-07-18T08:58:38Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2022-07-18T08:58:46Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-18T08:58:46Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-07-18T08:58:46Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmphIKhV4#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-07-18T08:58:49Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-07-18T08:58:49Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmphIKhV4#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmphIKhV4/incremental-state"
[2022-07-18T08:58:52Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-07-18T08:58:53Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmphIKhV4#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmphIKhV4/incremental-state"
[2022-07-18T08:58:54Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-18T08:58:54Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-07-18T08:58:54Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDgk10O#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-07-18T08:58:57Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
[2022-07-18T08:59:03Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-18T08:59:03Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-07-18T08:59:03Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp6hkdaP#token-stream-stress@0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2022-07-18T08:59:03Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-07-18T08:59:03Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp6hkdaP#token-stream-stress@0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp6hkdaP/incremental-state"
[2022-07-18T08:59:03Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-07-18T08:59:03Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp6hkdaP#token-stream-stress@0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp6hkdaP/incremental-state"
[2022-07-18T08:59:03Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-18T08:59:03Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-07-18T08:59:03Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp32P3H6#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2022-07-18T08:59:03Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
---
[2022-07-18T08:59:04Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-18T08:59:04Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-07-18T08:59:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpEuocBm#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2022-07-18T08:59:04Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-07-18T08:59:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpEuocBm#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpEuocBm/incremental-state"
[2022-07-18T08:59:05Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-07-18T08:59:05Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpEuocBm#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpEuocBm/incremental-state"
Executing benchmark tuple-stress (8/8)
Preparing tuple-stress
[2022-07-18T08:59:05Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-07-18T08:59:05Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2022-07-18T08:59:24Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-18T08:59:24Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-07-18T08:59:25Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpseBwzm#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-07-18T08:59:30Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-07-18T08:59:30Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpseBwzm#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpseBwzm/incremental-state"
[2022-07-18T08:59:36Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-07-18T08:59:36Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpseBwzm#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpseBwzm/incremental-state"
[2022-07-18T08:59:38Z DEBUG collector::execute] applying new row to "/tmp/.tmpseBwzm"
[2022-07-18T08:59:38Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-07-18T08:59:38Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-07-18T08:59:38Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpseBwzm#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpseBwzm/incremental-state"
[2022-07-18T08:59:44Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-18T08:59:44Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-07-18T08:59:44Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp2HU2GD#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-07-18T08:59:50Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
   Compiling termcolor v1.1.2
[RUSTC-TIMING] unicode_xid test:false 0.087
   Compiling rustc_fs_util v0.0.0 (/checkout/compiler/rustc_fs_util)
[RUSTC-TIMING] arrayvec test:false 0.240
warning: rustc_graphviz.e22fbdc2-cgu.13: no profile data available for function _RNvXs2_NtCs7zzAukxSjOW_4core7convertNtNtNtB7_5alloc6layout11LayoutErrorINtB5_4FromBy_E4fromCsi3KMxKWQNTG_14rustc_graphviz Hash = 742261418966908927
   Compiling either v1.6.0
   Compiling either v1.6.0
warning: rustc_graphviz.e22fbdc2-cgu.10: no profile data available for function _RNvMNtNtNtCs7zzAukxSjOW_4core4iter8adapters3revINtB2_3RevINtNtNtB8_3ops5range5RangejEE3newCsi3KMxKWQNTG_14rustc_graphviz Hash = 742261418966908927
[RUSTC-TIMING] self_cell test:false 0.074
   Compiling datafrog v2.0.1
   Compiling datafrog v2.0.1
warning: rustc_fs_util.f09840a9-cgu.2: no profile data available for function _RNvXs2_NtCs7zzAukxSjOW_4core7convertNtNtNtB7_5alloc6layout11LayoutErrorINtB5_4FromBy_E4fromCs7eAO1nHmi8f_13rustc_fs_util Hash = 742261418966908927
[RUSTC-TIMING] build_script_build test:false 0.280
   Compiling memchr v2.4.1
[RUSTC-TIMING] itoa test:false 0.139
   Compiling tinyvec v0.3.4
---
   Compiling measureme v10.0.0
[RUSTC-TIMING] jobserver test:false 1.194
[RUSTC-TIMING] stacker test:false 0.292
   Compiling regex v1.5.5
warning: rustc_llvm.ef964fa5-cgu.3: no profile data available for function _RNvXs2_NtCs7zzAukxSjOW_4core7convertNtNtNtB7_5alloc6layout11LayoutErrorINtB5_4FromBy_E4fromCs6KA1KZRMfSz_10rustc_llvm Hash = 742261418966908927
[RUSTC-TIMING] generic_array test:false 0.838
   Compiling rand v0.8.5
[RUSTC-TIMING] rustc_parse_format test:false 1.033
   Compiling matchers v0.1.0
---
[RUSTC-TIMING] aho_corasick test:false 2.255
   Compiling object v0.28.4
[RUSTC-TIMING] rand test:false 1.261
   Compiling object v0.29.0
warning: rustc_serialize.ef905287-cgu.7: no profile data available for function _RINvNtCs7zzAukxSjOW_4core3ptr13drop_in_placeRjECsgIEggV8QgXX_15rustc_serialize Hash = 742261418966908927

warning: rustc_serialize.ef905287-cgu.7: no profile data available for function _RINvNtCs7zzAukxSjOW_4core9panicking13assert_failedjjECsgIEggV8QgXX_15rustc_serialize Hash = 742261418966908927

warning: rustc_serialize.ef905287-cgu.9: no profile data available for function _RNvXsV_NtCs7zzAukxSjOW_4core3fmtRjNtB5_5Debug3fmtCsgIEggV8QgXX_15rustc_serialize Hash = 1124680650125156080
[RUSTC-TIMING] rustc_serialize test:false 0.669
warning: `rustc_serialize` (lib) generated 3 warnings
[RUSTC-TIMING] tempfile test:false 0.940
[RUSTC-TIMING] sha2 test:false 1.254
---
[RUSTC-TIMING] rustc_ast_lowering test:false 1.174
error: unresolved link to `T`
    --> compiler/rustc_middle/src/ty/layout.rs:3491:24
     |
3491 | /// * uninit invalid &[T] where T has align 1 (only inside arrays)
     |                        ^ no item named `T` in scope
     |
     = note: `-D rustdoc::broken-intra-doc-links` implied by `-D warnings`
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: could not document `rustc_middle`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name rustc_middle compiler/rustc_middle/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/doc -Zunstable-options --check-cfg 'values(feature, "rustc-rayon", "rustc-rayon-core", "rustc_use_parallel_compiler")' --check-cfg 'names()' --check-cfg 'values()' --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -C metadata=4056c07e18b96fa2 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-b08786b735ebc83e.rmeta --extern chalk_ir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libchalk_ir-b7d5e9cc4212a6a4.rmeta --extern either=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libeither-c338135ee386fd69.rmeta --extern gsgdt=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libgsgdt-18ca57f6c1c877e0.rmeta --extern polonius_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libpolonius_engine-498d962f9b7b1bd1.rmeta --extern rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librand-7b6b181eb887c366.rmeta --extern rand_xoshiro=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librand_xoshiro-9b11bcaa8d4cc5a8.rmeta --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-a40e8f2b6ed34d4d.rmeta --extern rustc_arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_arena-46e5693d0a8f70ca.rmeta --extern rustc_ast=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_ast-11872d71b90facab.rmeta --extern rustc_attr=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_attr-3dc5cb232e2aeddf.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-6cc9efdf702efa25.rmeta --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-3fa4b7305376e92a.rmeta --extern rustc_feature=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_feature-4782592e27cfed10.rmeta --extern rustc_graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_graphviz-7c2234d9aef98622.rmeta --extern rustc_hir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_hir-79277f27b8ea3fac.rmeta --extern rustc_index=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_index-f77118641bc8969b.rmeta --extern rustc_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps/librustc_macros-3402b970e104daab.so --extern rustc_query_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_query_system-f141b77a99f81c51.rmeta --extern rustc_serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_serialize-699147027d40864b.rmeta --extern rustc_session=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_session-c14258e51a851045.rmeta --extern rustc_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_span-11b12bd83e996fca.rmeta --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-a9e3cbc0d109f75c.rmeta --extern rustc_type_ir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_type_ir-bc57348614b023cf.rmeta --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-9a0501c30d2a7b13.rmeta --extern tracing=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libtracing-eef4fab2e6aecb3e.rmeta --extern-html-root-url 'bitflags=https://docs.rs/bitflags/1.2.1/' --extern-html-root-url 'chalk_ir=https://docs.rs/chalk-ir/0.80.0/' --extern-html-root-url 'either=https://docs.rs/either/1.6.0/' --extern-html-root-url 'gsgdt=https://docs.rs/gsgdt/0.1.2/' --extern-html-root-url 'polonius_engine=https://docs.rs/polonius-engine/0.13.0/' --extern-html-root-url 'rand=https://docs.rs/rand/0.8.5/' --extern-html-root-url 'rand_xoshiro=https://docs.rs/rand_xoshiro/0.6.0/' --extern-html-root-url 'smallvec=https://docs.rs/smallvec/1.8.1/' --extern-html-root-url 'tracing=https://docs.rs/tracing/0.1.29/' -Zunstable-options -Csymbol-mangling-version=v0 -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(parallel_compiler)' '--check-cfg=values(no_btreemap_remove_entry)' '--check-cfg=values(crossbeam_loom)' '--check-cfg=values(span_locations)' -Clink-arg=-fuse-ld=lld -Clink-arg=-Wl,--threads=1 -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.64.0-nightly
  (af445596a
  2022-07-18)' --document-private-items '-Arustdoc::private-intra-doc-links' --enable-index-page -Zunstable-options -Znormalize-docs --show-type-layout --generate-link-to-definition --extern-html-root-url 'ena=https://docs.rs/ena/latest/'` (exit status: 1)
[RUSTC-TIMING] rustc_metadata test:false 1.558
[RUSTC-TIMING] rustc_infer test:false 3.150
[RUSTC-TIMING] rustc_query_impl test:false 6.035
Build completed unsuccessfully in 0:22:47
