plain
[2022-12-01T23:19:39Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-12-01T23:19:39Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-12-01T23:19:39Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpuM0wLf#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-12-01T23:19:41Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-12-01T23:19:41Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpuM0wLf#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpuM0wLf/incremental-state"
[2022-12-01T23:19:44Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-12-01T23:19:44Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpuM0wLf#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpuM0wLf/incremental-state"
[2022-12-01T23:19:44Z DEBUG collector::execute] applying println to "/tmp/.tmpuM0wLf"
[2022-12-01T23:19:44Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-12-01T23:19:44Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-12-01T23:19:44Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpuM0wLf#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpuM0wLf/incremental-state"
[2022-12-01T23:19:45Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-12-01T23:19:45Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-12-01T23:19:45Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpAjbeXq#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-12-01T23:19:47Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-12-01T23:19:47Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-12-01T23:19:47Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpAjbeXq#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpAjbeXq/incremental-state"
[2022-12-01T23:19:50Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-12-01T23:19:50Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpAjbeXq#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpAjbeXq/incremental-state"
[2022-12-01T23:19:51Z DEBUG collector::execute] applying println to "/tmp/.tmpAjbeXq"
[2022-12-01T23:19:51Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-12-01T23:19:51Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-12-01T23:19:51Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpAjbeXq#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpAjbeXq/incremental-state"
Executing benchmark cargo-0.60.0 (2/8)
Preparing cargo-0.60.0
[2022-12-01T23:19:52Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-12-01T23:19:52Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
---
[2022-12-01T23:21:35Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-12-01T23:21:36Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-12-01T23:21:36Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpsucBG7#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2022-12-01T23:22:21Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-12-01T23:22:21Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpsucBG7#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpsucBG7/incremental-state"
[2022-12-01T23:23:17Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-12-01T23:23:17Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpsucBG7#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpsucBG7/incremental-state"
[2022-12-01T23:23:26Z DEBUG collector::execute] applying println to "/tmp/.tmpsucBG7"
[2022-12-01T23:23:26Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-12-01T23:23:26Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-12-01T23:23:26Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpsucBG7#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpsucBG7/incremental-state"
[2022-12-01T23:23:42Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-12-01T23:23:42Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-12-01T23:23:42Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpNiJxQq#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2022-12-01T23:24:37Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-12-01T23:24:37Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-12-01T23:24:38Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpNiJxQq#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpNiJxQq/incremental-state"
[2022-12-01T23:25:33Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-12-01T23:25:33Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpNiJxQq#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpNiJxQq/incremental-state"
[2022-12-01T23:25:42Z DEBUG collector::execute] applying println to "/tmp/.tmpNiJxQq"
[2022-12-01T23:25:42Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-12-01T23:25:42Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-12-01T23:25:42Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpNiJxQq#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpNiJxQq/incremental-state"
Executing benchmark ctfe-stress-5 (3/8)
Preparing ctfe-stress-5
[2022-12-01T23:26:03Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-12-01T23:26:03Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2022-12-01T23:26:03Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-12-01T23:26:03Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-12-01T23:26:03Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpWuTPsx#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-12-01T23:26:11Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-12-01T23:26:11Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpWuTPsx#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpWuTPsx/incremental-state"
[2022-12-01T23:26:21Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-12-01T23:26:21Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpWuTPsx#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpWuTPsx/incremental-state"
[2022-12-01T23:26:21Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-12-01T23:26:21Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-12-01T23:26:21Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp7bNSqY#ctfe-stress-5@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-12-01T23:26:29Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-12-01T23:26:29Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-12-01T23:26:29Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp7bNSqY#ctfe-stress-5@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp7bNSqY/incremental-state"
[2022-12-01T23:26:39Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-12-01T23:26:39Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp7bNSqY#ctfe-stress-5@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp7bNSqY/incremental-state"
[2022-12-01T23:26:40Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-12-01T23:26:40Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-12-01T23:26:40Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpEqBL9m#ctfe-stress-5@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-12-01T23:26:48Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
[2022-12-01T23:28:37Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-12-01T23:28:38Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-12-01T23:28:38Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpaDESGC#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-12-01T23:28:55Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-12-01T23:28:55Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpaDESGC#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpaDESGC/incremental-state"
[2022-12-01T23:29:16Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-12-01T23:29:16Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpaDESGC#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpaDESGC/incremental-state"
[2022-12-01T23:29:19Z DEBUG collector::execute] applying println to "/tmp/.tmpaDESGC"
[2022-12-01T23:29:19Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-12-01T23:29:19Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-12-01T23:29:19Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpaDESGC#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpaDESGC/incremental-state"
Executing benchmark externs (5/8)
Preparing externs
[2022-12-01T23:29:22Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-12-01T23:29:22Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2022-12-01T23:29:23Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-12-01T23:29:23Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-12-01T23:29:23Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCMfH7V#externs@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-12-01T23:29:23Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-12-01T23:29:23Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCMfH7V#externs@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpCMfH7V/incremental-state"
[2022-12-01T23:29:24Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-12-01T23:29:24Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCMfH7V#externs@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpCMfH7V/incremental-state"
[2022-12-01T23:29:25Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-12-01T23:29:25Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-12-01T23:29:25Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpaF7K0r#externs@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-12-01T23:29:25Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
---
[2022-12-01T23:29:27Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-12-01T23:29:27Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-12-01T23:29:27Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpE3appN#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-12-01T23:29:27Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-12-01T23:29:27Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpE3appN#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpE3appN/incremental-state"
[2022-12-01T23:29:28Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-12-01T23:29:28Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpE3appN#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpE3appN/incremental-state"
Executing benchmark match-stress (6/8)
Preparing match-stress
[2022-12-01T23:29:29Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-12-01T23:29:29Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
---
[2022-12-01T23:29:35Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-12-01T23:29:35Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-12-01T23:29:35Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmprDouEv#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-12-01T23:29:38Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-12-01T23:29:38Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmprDouEv#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmprDouEv/incremental-state"
[2022-12-01T23:29:41Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-12-01T23:29:41Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmprDouEv#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmprDouEv/incremental-state"
[2022-12-01T23:29:42Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-12-01T23:29:42Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-12-01T23:29:42Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp8B5YER#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-12-01T23:29:44Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
[2022-12-01T23:29:49Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-12-01T23:29:49Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-12-01T23:29:49Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmprJknsz#token-stream-stress@0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2022-12-01T23:29:49Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-12-01T23:29:49Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmprJknsz#token-stream-stress@0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmprJknsz/incremental-state"
[2022-12-01T23:29:50Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-12-01T23:29:50Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmprJknsz#token-stream-stress@0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmprJknsz/incremental-state"
[2022-12-01T23:29:50Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-12-01T23:29:50Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-12-01T23:29:50Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpA6pW0l#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2022-12-01T23:29:50Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
---
[2022-12-01T23:29:52Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-12-01T23:29:52Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-12-01T23:29:52Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmprDQeOf#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-12-01T23:29:57Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-12-01T23:29:57Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmprDQeOf#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmprDQeOf/incremental-state"
[2022-12-01T23:30:02Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-12-01T23:30:02Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmprDQeOf#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmprDQeOf/incremental-state"
[2022-12-01T23:30:04Z DEBUG collector::execute] applying new row to "/tmp/.tmprDQeOf"
[2022-12-01T23:30:04Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-12-01T23:30:04Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-12-01T23:30:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmprDQeOf#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmprDQeOf/incremental-state"
[2022-12-01T23:30:09Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-12-01T23:30:09Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-12-01T23:30:09Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpG75iz1#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-12-01T23:30:14Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
---
[2022-12-01T23:30:27Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-12-01T23:30:27Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-12-01T23:30:27Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpYPKDYc#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-12-01T23:30:32Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-12-01T23:30:32Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpYPKDYc#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpYPKDYc/incremental-state"
[2022-12-01T23:30:38Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-12-01T23:30:38Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpYPKDYc#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpYPKDYc/incremental-state"
[2022-12-01T23:30:40Z DEBUG collector::execute] applying new row to "/tmp/.tmpYPKDYc"
[2022-12-01T23:30:40Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-12-01T23:30:40Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-12-01T23:30:40Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpYPKDYc#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpYPKDYc/incremental-state"
+ cd /checkout/obj
+ RUSTC_PROFILE_MERGED_FILE=/tmp/tmp-pgo/rustc-pgo.profdata
+ /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-profdata merge -o /tmp/tmp-pgo/rustc-pgo.profdata /tmp/tmp-pgo/rustc-pgo
+ echo 'Rustc PGO statistics'
---
[RUSTC-TIMING] git2_curl test:false 0.484
[RUSTC-TIMING] git2 test:false 4.136
[RUSTC-TIMING] toml_edit test:false 32.381
[RUSTC-TIMING] cargo test:false 41.276
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', compiler/rustc_monomorphize/src/collector.rs:934:13
   0:     0x7f056f79b290 - std::backtrace_rs::backtrace::libunwind::trace::h72d1596dd528bb82
                               at /rustc/b1d014fcd89dcaa80e7936dd197f722589415b31/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7f056f79b290 - std::backtrace_rs::backtrace::trace_unsynchronized::hee13e3b68af6f52f
                               at /rustc/b1d014fcd89dcaa80e7936dd197f722589415b31/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
                               at /rustc/b1d014fcd89dcaa80e7936dd197f722589415b31/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f056f79b290 - std::sys_common::backtrace::_print_fmt::h12232e77732816bc
                               at /rustc/b1d014fcd89dcaa80e7936dd197f722589415b31/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7f056f79b290 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h296e2080d428f42d
                               at /rustc/b1d014fcd89dcaa80e7936dd197f722589415b31/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7f056f7fd73e - core::fmt::write::hcb131ddd0c15f718
                               at /rustc/b1d014fcd89dcaa80e7936dd197f722589415b31/library/core/src/fmt/mod.rs:1208:17
   5:     0x7f056f78b5f5 - std::io::Write::write_fmt::h92d7e4cbd76dded8
                               at /rustc/b1d014fcd89dcaa80e7936dd197f722589415b31/library/std/src/io/mod.rs:1682:15
   6:     0x7f056f79b055 - std::sys_common::backtrace::_print::hfd155602eae71eae
                               at /rustc/b1d014fcd89dcaa80e7936dd197f722589415b31/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x7f056f79b055 - std::sys_common::backtrace::print::h6ba96d5b1ac612bf
                               at /rustc/b1d014fcd89dcaa80e7936dd197f722589415b31/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x7f056f79ddaf - std::panicking::default_hook::{{closure}}::hbcb3e10feaf14525
                               at /rustc/b1d014fcd89dcaa80e7936dd197f722589415b31/library/std/src/panicking.rs:267:22
   9:     0x7f056f79daea - std::panicking::default_hook::he45b9a8155940a82
                               at /rustc/b1d014fcd89dcaa80e7936dd197f722589415b31/library/std/src/panicking.rs:286:9
  10:     0x7f056c9984a6 - rustc_driver[db4a5a26c7bb9a9c]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7f056f79e5d9 - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::h87b8f61fb9274598
                               at /rustc/b1d014fcd89dcaa80e7936dd197f722589415b31/library/alloc/src/boxed.rs:2024:9
  12:     0x7f056f79e5d9 - std::panicking::rust_panic_with_hook::h46a63a5f115f0d1a
                               at /rustc/b1d014fcd89dcaa80e7936dd197f722589415b31/library/std/src/panicking.rs:692:13
  13:     0x7f056f79e311 - std::panicking::begin_panic_handler::{{closure}}::hf78e69605879d797
                               at /rustc/b1d014fcd89dcaa80e7936dd197f722589415b31/library/std/src/panicking.rs:577:13
  14:     0x7f056f79b73c - std::sys_common::backtrace::__rust_end_short_backtrace::h97caef2d8425e271
                               at /rustc/b1d014fcd89dcaa80e7936dd197f722589415b31/library/std/src/sys_common/backtrace.rs:137:18
                               at /rustc/b1d014fcd89dcaa80e7936dd197f722589415b31/library/std/src/panicking.rs:575:5
  16:     0x7f056f7fa123 - core::panicking::panic_fmt::h852ef51d3640d2fc
                               at /rustc/b1d014fcd89dcaa80e7936dd197f722589415b31/library/core/src/panicking.rs:65:14
  17:     0x7f056f7fa1fd - core::panicking::panic::h7031aae91bc06edb
  17:     0x7f056f7fa1fd - core::panicking::panic::h7031aae91bc06edb
                               at /rustc/b1d014fcd89dcaa80e7936dd197f722589415b31/library/core/src/panicking.rs:114:5
  18:     0x7f056d12d093 - <rustc_monomorphize[48dfcf2f23be60b3]::collector::MirNeighborCollector as rustc_middle[ca359ce8580ebda0]::mir::visit::Visitor>::visit_terminator
  19:     0x7f056d133123 - rustc_monomorphize[48dfcf2f23be60b3]::collector::collect_neighbours
  20:     0x7f056d13110c - rustc_monomorphize[48dfcf2f23be60b3]::collector::collect_items_rec
  21:     0x7f056d131747 - rustc_monomorphize[48dfcf2f23be60b3]::collector::collect_items_rec
  22:     0x7f056d131747 - rustc_monomorphize[48dfcf2f23be60b3]::collector::collect_items_rec
  23:     0x7f056d131747 - rustc_monomorphize[48dfcf2f23be60b3]::collector::collect_items_rec
  24:     0x7f056d131747 - rustc_monomorphize[48dfcf2f23be60b3]::collector::collect_items_rec
  25:     0x7f056d131747 - rustc_monomorphize[48dfcf2f23be60b3]::collector::collect_items_rec
  26:     0x7f056d134050 - std[a9f97bebfc7cd8c0]::panicking::try::<(), core[ae65f184c67bc919]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[49608448ea8a0fb8]::sync::par_for_each_in<alloc[c4b6aa748b2a1692]::vec::Vec<rustc_middle[ca359ce8580ebda0]::mir::mono::MonoItem>, rustc_monomorphize[48dfcf2f23be60b3]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}>>
  27:     0x7f056d15957f - <rustc_session[73e4673686d5debc]::session::Session>::time::<(), rustc_monomorphize[48dfcf2f23be60b3]::collector::collect_crate_mono_items::{closure#1}>
  28:     0x7f056d12ecfd - rustc_monomorphize[48dfcf2f23be60b3]::collector::collect_crate_mono_items
  29:     0x7f056d13ea6a - rustc_monomorphize[48dfcf2f23be60b3]::partitioning::collect_and_partition_mono_items
  30:     0x7f056df86792 - rustc_query_system[fccd579ae59229bc]::query::plumbing::try_execute_query::<rustc_query_impl[67f99e74d0bdbef1]::plumbing::QueryCtxt, rustc_query_system[fccd579ae59229bc]::query::caches::DefaultCache<(), (&std[a9f97bebfc7cd8c0]::collections::hash::set::HashSet<rustc_span[1c29d7493b5e59c8]::def_id::DefId, core[ae65f184c67bc919]::hash::BuildHasherDefault<rustc_hash[dbc584c8a978da26]::FxHasher>>, &[rustc_middle[ca359ce8580ebda0]::mir::mono::CodegenUnit])>>
  31:     0x7f056e042327 - rustc_query_system[fccd579ae59229bc]::query::plumbing::get_query::<rustc_query_impl[67f99e74d0bdbef1]::queries::collect_and_partition_mono_items, rustc_query_impl[67f99e74d0bdbef1]::plumbing::QueryCtxt>
  32:     0x7f056de2b48b - <rustc_query_impl[67f99e74d0bdbef1]::Queries as rustc_middle[ca359ce8580ebda0]::ty::query::QueryEngine>::collect_and_partition_mono_items
  33:     0x7f056cb63367 - rustc_codegen_ssa[69bbb58b96e84bc8]::base::codegen_crate::<rustc_codegen_llvm[4b6fe2f02c92684f]::LlvmCodegenBackend>
  34:     0x7f056cb170f8 - <rustc_codegen_llvm[4b6fe2f02c92684f]::LlvmCodegenBackend as rustc_codegen_ssa[69bbb58b96e84bc8]::traits::backend::CodegenBackend>::codegen_crate
  35:     0x7f056ca32eaf - <rustc_session[73e4673686d5debc]::session::Session>::time::<alloc[c4b6aa748b2a1692]::boxed::Box<dyn core[ae65f184c67bc919]::any::Any>, rustc_interface[a506aaeab4659e18]::passes::start_codegen::{closure#0}>
  36:     0x7f056ca643e7 - rustc_interface[a506aaeab4659e18]::passes::start_codegen
  37:     0x7f056ca6abcb - <rustc_interface[a506aaeab4659e18]::queries::Queries>::ongoing_codegen
  38:     0x7f056c979e15 - <rustc_interface[a506aaeab4659e18]::interface::Compiler>::enter::<rustc_driver[db4a5a26c7bb9a9c]::run_compiler::{closure#1}::{closure#2}, core[ae65f184c67bc919]::result::Result<core[ae65f184c67bc919]::option::Option<rustc_interface[a506aaeab4659e18]::queries::Linker>, rustc_errors[103a132b600ea385]::ErrorGuaranteed>>
  39:     0x7f056c90ead2 - rustc_span[1c29d7493b5e59c8]::with_source_map::<core[ae65f184c67bc919]::result::Result<(), rustc_errors[103a132b600ea385]::ErrorGuaranteed>, rustc_interface[a506aaeab4659e18]::interface::run_compiler<core[ae65f184c67bc919]::result::Result<(), rustc_errors[103a132b600ea385]::ErrorGuaranteed>, rustc_driver[db4a5a26c7bb9a9c]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  40:     0x7f056c96d148 - <scoped_tls[27cd29d9afbae785]::ScopedKey<rustc_span[1c29d7493b5e59c8]::SessionGlobals>>::set::<rustc_interface[a506aaeab4659e18]::interface::run_compiler<core[ae65f184c67bc919]::result::Result<(), rustc_errors[103a132b600ea385]::ErrorGuaranteed>, rustc_driver[db4a5a26c7bb9a9c]::run_compiler::{closure#1}>::{closure#0}, core[ae65f184c67bc919]::result::Result<(), rustc_errors[103a132b600ea385]::ErrorGuaranteed>>
  41:     0x7f056c934e90 - std[a9f97bebfc7cd8c0]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[a506aaeab4659e18]::util::run_in_thread_pool_with_globals<rustc_interface[a506aaeab4659e18]::interface::run_compiler<core[ae65f184c67bc919]::result::Result<(), rustc_errors[103a132b600ea385]::ErrorGuaranteed>, rustc_driver[db4a5a26c7bb9a9c]::run_compiler::{closure#1}>::{closure#0}, core[ae65f184c67bc919]::result::Result<(), rustc_errors[103a132b600ea385]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[ae65f184c67bc919]::result::Result<(), rustc_errors[103a132b600ea385]::ErrorGuaranteed>>
  42:     0x7f056c914e54 - <<std[a9f97bebfc7cd8c0]::thread::Builder>::spawn_unchecked_<rustc_interface[a506aaeab4659e18]::util::run_in_thread_pool_with_globals<rustc_interface[a506aaeab4659e18]::interface::run_compiler<core[ae65f184c67bc919]::result::Result<(), rustc_errors[103a132b600ea385]::ErrorGuaranteed>, rustc_driver[db4a5a26c7bb9a9c]::run_compiler::{closure#1}>::{closure#0}, core[ae65f184c67bc919]::result::Result<(), rustc_errors[103a132b600ea385]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[ae65f184c67bc919]::result::Result<(), rustc_errors[103a132b600ea385]::ErrorGuaranteed>>::{closure#1} as core[ae65f184c67bc919]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  43:     0x7f056f7a8403 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h7a85031506c31306
                               at /rustc/b1d014fcd89dcaa80e7936dd197f722589415b31/library/alloc/src/boxed.rs:1990:9
  44:     0x7f056f7a8403 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h2c3592766d4d2f9f
                               at /rustc/b1d014fcd89dcaa80e7936dd197f722589415b31/library/alloc/src/boxed.rs:1990:9
  45:     0x7f056f7a8403 - std::sys::unix::thread::Thread::new::thread_start::h1f529fb95785e7e8
                               at /rustc/b1d014fcd89dcaa80e7936dd197f722589415b31/library/std/src/sys/unix/thread.rs:108:17
  46:     0x7f056b1efea5 - start_thread
  48:                0x0 - <unknown>

error: internal compiler error: unexpected panic


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (b1d014fcd 2022-12-01) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type bin -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -Z unstable-options -C linker=clang -C symbol-mangling-version=v0 -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C link-args=-fuse-ld=lld -C split-debuginfo=off -Z binary-dep-depinfo -Z tls-model=initial-exec
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [collect_and_partition_mono_items] collect_and_partition_mono_items
[RUSTC-TIMING] cargo test:false 0.669
error: could not compile `cargo`
Build completed unsuccessfully in 0:29:21
