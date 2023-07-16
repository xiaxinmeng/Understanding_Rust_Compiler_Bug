plain
Executing benchmark regex-1.5.5 (4/7)
Preparing regex-1.5.5
[2022-11-11T14:10:49Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-11-11T14:10:49Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-11-11T14:10:49Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpBMRPDY#regex@1.5.5" "--" "--skip-this-rustc"
[2022-11-11T14:10:49Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpbSAyvE#regex@1.5.5" "--release" "--" "--skip-this-rustc"
[2022-11-11T14:11:00Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-11-11T14:11:00Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-11-11T14:11:00Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpgSo4HA#regex@1.5.5" "--" "--wrap-rustc-with" "Eprintln"
Running regex-1.5.5: Opt + [Full]
---
[2022-11-11T14:26:46Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-11-11T14:26:46Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-11-11T14:26:46Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpIuRgBA#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-11-11T14:26:49Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-11-11T14:26:49Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpIuRgBA#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpIuRgBA/incremental-state"
[2022-11-11T14:26:53Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-11-11T14:26:53Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpIuRgBA#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpIuRgBA/incremental-state"
[2022-11-11T14:26:54Z DEBUG collector::execute] applying println to "/tmp/.tmpIuRgBA"
[2022-11-11T14:26:54Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-11-11T14:26:54Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-11-11T14:26:54Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpIuRgBA#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpIuRgBA/incremental-state"
[2022-11-11T14:26:55Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-11-11T14:26:55Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-11-11T14:26:55Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpPbHRYS#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-11-11T14:26:58Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-11-11T14:26:58Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-11-11T14:26:58Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpPbHRYS#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpPbHRYS/incremental-state"
[2022-11-11T14:27:03Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-11-11T14:27:03Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpPbHRYS#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpPbHRYS/incremental-state"
[2022-11-11T14:27:04Z DEBUG collector::execute] applying println to "/tmp/.tmpPbHRYS"
[2022-11-11T14:27:04Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-11-11T14:27:04Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-11-11T14:27:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpPbHRYS#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpPbHRYS/incremental-state"
[2022-11-11T14:27:05Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-11-11T14:27:05Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-11-11T14:27:05Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpXoF8Wh#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-11-11T14:27:08Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-11-11T14:27:08Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-11-11T14:27:08Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpXoF8Wh#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpXoF8Wh/incremental-state"
[2022-11-11T14:27:13Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-11-11T14:27:13Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpXoF8Wh#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpXoF8Wh/incremental-state"
[2022-11-11T14:27:14Z DEBUG collector::execute] applying println to "/tmp/.tmpXoF8Wh"
[2022-11-11T14:27:14Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-11-11T14:27:14Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-11-11T14:27:14Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpXoF8Wh#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpXoF8Wh/incremental-state"
Executing benchmark cargo-0.60.0 (2/8)
Preparing cargo-0.60.0
[2022-11-11T14:27:16Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-11-11T14:27:16Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2022-11-11T14:36:27Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-11-11T14:36:27Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-11-11T14:36:27Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpYC1nFD#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-11-11T14:36:36Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-11-11T14:36:36Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpYC1nFD#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpYC1nFD/incremental-state"
[2022-11-11T14:36:49Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-11-11T14:36:49Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpYC1nFD#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpYC1nFD/incremental-state"
[2022-11-11T14:36:49Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-11-11T14:36:49Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-11-11T14:36:49Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDpnxzz#ctfe-stress-5@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-11-11T14:36:59Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-11-11T14:36:59Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-11-11T14:36:59Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDpnxzz#ctfe-stress-5@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpDpnxzz/incremental-state"
[2022-11-11T14:37:11Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-11-11T14:37:11Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDpnxzz#ctfe-stress-5@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpDpnxzz/incremental-state"
[2022-11-11T14:37:11Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-11-11T14:37:11Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-11-11T14:37:11Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpd7VTQA#ctfe-stress-5@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-11-11T14:37:20Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
[2022-11-11T14:41:05Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-11-11T14:41:05Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-11-11T14:41:05Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpACWAnE#externs@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-11-11T14:41:06Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-11-11T14:41:06Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpACWAnE#externs@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpACWAnE/incremental-state"
[2022-11-11T14:41:07Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-11-11T14:41:07Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpACWAnE#externs@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpACWAnE/incremental-state"
[2022-11-11T14:41:08Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-11-11T14:41:08Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-11-11T14:41:08Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpeRy6zI#externs@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-11-11T14:41:09Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
---
[2022-11-11T14:41:32Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-11-11T14:41:32Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-11-11T14:41:32Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpmBapTu#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-11-11T14:41:35Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-11-11T14:41:35Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpmBapTu#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpmBapTu/incremental-state"
[2022-11-11T14:41:40Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-11-11T14:41:40Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpmBapTu#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpmBapTu/incremental-state"
Executing benchmark token-stream-stress (7/8)
Preparing token-stream-stress
[2022-11-11T14:41:41Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-11-11T14:41:41Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2022-11-11T14:42:06Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-11-11T14:42:06Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-11-11T14:42:06Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpaZIHVY#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-11-11T14:42:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-11-11T14:42:12Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpaZIHVY#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpaZIHVY/incremental-state"
[2022-11-11T14:42:19Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-11-11T14:42:19Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpaZIHVY#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpaZIHVY/incremental-state"
[2022-11-11T14:42:21Z DEBUG collector::execute] applying new row to "/tmp/.tmpaZIHVY"
[2022-11-11T14:42:21Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-11-11T14:42:21Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-11-11T14:42:21Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpaZIHVY#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpaZIHVY/incremental-state"
[2022-11-11T14:42:28Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-11-11T14:42:28Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-11-11T14:42:28Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp5OvJmR#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-11-11T14:42:33Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-11-11T14:42:33Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-11-11T14:42:33Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp5OvJmR#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp5OvJmR/incremental-state"
[2022-11-11T14:42:40Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-11-11T14:42:40Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp5OvJmR#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp5OvJmR/incremental-state"
[2022-11-11T14:42:42Z DEBUG collector::execute] applying new row to "/tmp/.tmp5OvJmR"
[2022-11-11T14:42:42Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-11-11T14:42:42Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-11-11T14:42:42Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp5OvJmR#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp5OvJmR/incremental-state"
+ cd /checkout/obj
+ RUSTC_PROFILE_MERGED_FILE=/tmp/tmp-pgo/rustc-pgo.profdata
+ /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-profdata merge -o /tmp/tmp-pgo/rustc-pgo.profdata /tmp/tmp-pgo/rustc-pgo
+ echo 'Rustc PGO statistics'
---
[RUSTC-TIMING] rustc_mir_transform test:false 92.596
[RUSTC-TIMING] rustc_borrowck test:false 101.117
[RUSTC-TIMING] rustc_query_impl test:false 129.235
   Compiling rustc_driver v0.0.0 (/checkout/compiler/rustc_driver)
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-45f38b6c99863dbb.so(+0x112aa63)[0x7f1e0fdcea63]
/lib64/libpthread.so.0(+0xf630)[0x7f1e0e68b630]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libLLVM-15-rust-1.67.0-nightly.so(+0x7b7cbdd)[0x7f1e08f6dbdd]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libLLVM-15-rust-1.67.0-nightly.so(+0x7b7c1f7)[0x7f1e08f6d1f7]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libLLVM-15-rust-1.67.0-nightly.so(_ZN4llvm26LoopVectorizationCostModel22getInterleaveGroupCostEPNS_11InstructionENS_12ElementCountE+0x8b9)[0x7f1e08bc9bff]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libLLVM-15-rust-1.67.0-nightly.so(_ZN4llvm26LoopVectorizationCostModel28setCostBasedWideningDecisionENS_12ElementCountE+0x4d3)[0x7f1e08baf913]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libLLVM-15-rust-1.67.0-nightly.so(+0x77b3f4b)[0x7f1e08ba4f4b]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libLLVM-15-rust-1.67.0-nightly.so(_ZN4llvm24LoopVectorizationPlanner4planENS_12ElementCountEj+0x23a)[0x7f1e08ba47fa]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libLLVM-15-rust-1.67.0-nightly.so(_ZN4llvm17LoopVectorizePass11processLoopEPNS_4LoopE+0x123b)[0x7f1e08b8224f]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libLLVM-15-rust-1.67.0-nightly.so(_ZN4llvm17LoopVectorizePass7runImplERNS_8FunctionERNS_15ScalarEvolutionERNS_8LoopInfoERNS_19TargetTransformInfoERNS_13DominatorTreeERNS_18BlockFrequencyInfoEPNS_17TargetLibraryInfoERNS_12DemandedBitsERNS_9AAResultsERNS_15AssumptionCacheERSt8functionIFRKNS_14LoopAccessInfoERNS_4LoopEEERNS_25OptimizationRemarkEmitterEPNS_18ProfileSummaryInfoE+0x20f)[0x7f1e08b806d3]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libLLVM-15-rust-1.67.0-nightly.so(_ZN4llvm17LoopVectorizePass3runERNS_8FunctionERNS_15AnalysisManagerIS1_JEEE+0x3b3)[0x7f1e08b7fff3]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libLLVM-15-rust-1.67.0-nightly.so(+0x778ec2f)[0x7f1e08b7fc2f]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libLLVM-15-rust-1.67.0-nightly.so(_ZN4llvm11PassManagerINS_8FunctionENS_15AnalysisManagerIS1_JEEEJEE3runERS1_RS3_+0x726)[0x7f1e06abbce6]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libLLVM-15-rust-1.67.0-nightly.so(_ZN4llvm27ModuleToFunctionPassAdaptor3runERNS_6ModuleERNS_15AnalysisManagerIS1_JEEE+0xfc2)[0x7f1e08509042]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libLLVM-15-rust-1.67.0-nightly.so(+0x7117071)[0x7f1e08508071]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libLLVM-15-rust-1.67.0-nightly.so(_ZN4llvm11PassManagerINS_6ModuleENS_15AnalysisManagerIS1_JEEEJEE3runERS1_RS3_+0x1b1)[0x7f1e08983dcf]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-45f38b6c99863dbb.so(+0x13d7181)[0x7f1e1007b181]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-45f38b6c99863dbb.so(+0x12b8851)[0x7f1e0ff5c851]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-45f38b6c99863dbb.so(+0x132e390)[0x7f1e0ffd2390]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-45f38b6c99863dbb.so(+0x132eee0)[0x7f1e0ffd2ee0]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-45f38b6c99863dbb.so(+0x139b767)[0x7f1e1003f767]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-45f38b6c99863dbb.so(+0x12c55ab)[0x7f1e0ff695ab]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-45f38b6c99863dbb.so(+0x1373174)[0x7f1e10017174]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-45f38b6c99863dbb.so(+0x13566be)[0x7f1e0fffa6be]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libstd-3c0a69a209c841a9.so(rust_metadata_std_56a6bcf53d9f0388+0x10aeb3)[0x7f1e12b26eb3]
/lib64/libpthread.so.0(+0x7ea5)[0x7f1e0e683ea5]
/lib64/libc.so.6(clone+0x6d)[0x7f1e0e3acb0d]
[RUSTC-TIMING] rustc_driver test:false 24.066
rustc exited with signal: 11 (SIGSEGV) (core dumped)

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_driver --edition=2021 compiler/rustc_driver/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 --cfg 'feature="llvm"' --cfg 'feature="max_level_info"' -Zunstable-options --check-cfg 'values(feature, "llvm", "max_level_info", "rustc_use_parallel_compiler")' --check-cfg 'names()' --check-cfg 'values()' -C metadata=9706661b8d7761f0 -C extra-filename=-9706661b8d7761f0 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -C linker=clang -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblibc-c0605a3ee1291a5d.rlib --extern rustc_ast=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_ast-9ba34215bae63019.rlib --extern rustc_ast_pretty=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_ast_pretty-9ba4c56ed3b45613.rlib --extern rustc_codegen_ssa=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_codegen_ssa-0833103ad459e674.rlib --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-4f7c34df3ef97c1e.rlib --extern rustc_error_codes=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_error_codes-626a768a67da3436.rlib --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-7787ec3810a8133f.rlib --extern rustc_feature=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_feature-d967398d952ee82f.rlib --extern rustc_hir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_hir-867dc1ff75e8ec98.rlib --extern rustc_hir_analysis=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_hir_analysis-47d5e5323ea35b77.rlib --extern rustc_hir_pretty=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_hir_pretty-6fb824d11e1a8c85.rlib --extern rustc_interface=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_interface-37dda5b764449c22.rlib --extern rustc_lint=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_lint-6b1cdd9061462d81.rlib --extern rustc_log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_log-1f52d1a88946ed41.rlib --extern rustc_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/librustc_macros-2ea2b6c504738db8.so --extern rustc_metadata=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_metadata-3beba06324695c6d.rlib --extern rustc_middle=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_middle-3985021be9913455.rlib --extern rustc_parse=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_parse-4b8cbb55a34b1722.rlib --extern rustc_plugin_impl=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_plugin_impl-d4ece4072c97f0f6.rlib --extern rustc_save_analysis=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_save_analysis-e0459597a5be1047.rlib --extern rustc_session=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_session-453bc735c2434f4e.rlib --extern rustc_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_span-a6589842a9d999ba.rlib --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-65cab79681289f79.rlib --extern serde_json=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libserde_json-31058dac518a3b09.rlib --extern tracing=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libtracing-f6a073c317930f5c.rlib -Csymbol-mangling-version=v0 -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(parallel_compiler)' '--check-cfg=values(no_btreemap_remove_entry)' '--check-cfg=values(crossbeam_loom)' '--check-cfg=values(span_locations)' '--check-cfg=values(rustix_use_libc)' -Zmacro-backtrace -Clink-args=-Wl,-z,origin '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Clink-args=-fuse-ld=lld -Csplit-debuginfo=off -Zunstable-options '-Wrustc::internal' -Cprefer-dynamic -Clink-args=-Wl,--icf=all -Zdylib-lto -Clto=thin -Cembed-bitcode=yes -Z binary-dep-depinfo -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/psm-d9ed4981f6e79019/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/rustc_llvm-98ca7780e475a128/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lib -L native=/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/7.5.0/../../../../lib64` (exit status: 254)
