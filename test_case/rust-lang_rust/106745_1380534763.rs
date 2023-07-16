plain
[2023-01-11T21:52:35Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-11T21:52:36Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-01-11T21:52:36Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpki6hxM#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-11T21:53:00Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-01-11T21:53:00Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpki6hxM#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpki6hxM/incremental-state"
[2023-01-11T21:53:30Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-01-11T21:53:30Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpki6hxM#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpki6hxM/incremental-state"
[2023-01-11T21:53:36Z DEBUG collector::execute] applying println to "/tmp/.tmpki6hxM"
[2023-01-11T21:53:36Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-11T21:53:36Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-11T21:53:36Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpki6hxM#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpki6hxM/incremental-state"
[2023-01-11T21:53:42Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-11T21:53:42Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-01-11T21:53:42Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp3P9uHK#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-11T21:54:46Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
---
[2023-01-11T21:56:31Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-11T21:56:32Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-01-11T21:56:32Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjwRQyE#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-11T21:57:49Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-01-11T21:57:49Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjwRQyE#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpjwRQyE/incremental-state"
[2023-01-11T21:59:07Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-01-11T21:59:07Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjwRQyE#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpjwRQyE/incremental-state"
[2023-01-11T21:59:22Z DEBUG collector::execute] applying println to "/tmp/.tmpjwRQyE"
[2023-01-11T21:59:22Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-11T21:59:22Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-11T21:59:22Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjwRQyE#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpjwRQyE/incremental-state"
Executing benchmark ctfe-stress-5 (3/8)
Preparing ctfe-stress-5
[2023-01-11T21:59:36Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-01-11T21:59:36Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
---
[2023-01-11T22:01:07Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-11T22:01:07Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-01-11T22:01:07Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpxGkLST#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-11T22:01:26Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-01-11T22:01:26Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpxGkLST#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpxGkLST/incremental-state"
[2023-01-11T22:01:51Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-01-11T22:01:51Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpxGkLST#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpxGkLST/incremental-state"
[2023-01-11T22:01:55Z DEBUG collector::execute] applying println to "/tmp/.tmpxGkLST"
[2023-01-11T22:01:55Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-11T22:01:55Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-11T22:01:55Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpxGkLST#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpxGkLST/incremental-state"
[2023-01-11T22:01:59Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-11T22:02:00Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-01-11T22:02:00Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpY9U2Kf#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-11T22:02:24Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
---
[2023-01-11T22:03:06Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-11T22:03:06Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-01-11T22:03:06Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOAxrWs#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-11T22:03:30Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-01-11T22:03:30Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOAxrWs#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpOAxrWs/incremental-state"
[2023-01-11T22:04:03Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-01-11T22:04:03Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOAxrWs#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpOAxrWs/incremental-state"
[2023-01-11T22:04:08Z DEBUG collector::execute] applying println to "/tmp/.tmpOAxrWs"
[2023-01-11T22:04:08Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-11T22:04:08Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-11T22:04:08Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOAxrWs#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpOAxrWs/incremental-state"
Executing benchmark externs (5/8)
Preparing externs
[2023-01-11T22:04:13Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-01-11T22:04:13Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2023-01-11T22:04:19Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-11T22:04:19Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-01-11T22:04:19Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFZzHHy#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-11T22:04:20Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-01-11T22:04:20Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFZzHHy#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpFZzHHy/incremental-state"
[2023-01-11T22:04:21Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-01-11T22:04:21Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFZzHHy#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpFZzHHy/incremental-state"
Executing benchmark match-stress (6/8)
Preparing match-stress
[2023-01-11T22:04:22Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-01-11T22:04:22Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2023-01-11T22:04:22Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-11T22:04:22Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-01-11T22:04:22Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpcyXTIm#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-11T22:04:26Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-01-11T22:04:26Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpcyXTIm#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpcyXTIm/incremental-state"
[2023-01-11T22:04:29Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-01-11T22:04:29Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpcyXTIm#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpcyXTIm/incremental-state"
[2023-01-11T22:04:31Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-11T22:04:31Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-01-11T22:04:31Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzPGKyA#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-11T22:04:34Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-01-11T22:04:34Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-01-11T22:04:34Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzPGKyA#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpzPGKyA/incremental-state"
[2023-01-11T22:04:39Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-01-11T22:04:39Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzPGKyA#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpzPGKyA/incremental-state"
[2023-01-11T22:04:40Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-11T22:04:40Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-01-11T22:04:40Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpPRVpY3#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-11T22:04:44Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-01-11T22:04:44Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-01-11T22:04:44Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpPRVpY3#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpPRVpY3/incremental-state"
[2023-01-11T22:04:48Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-01-11T22:04:48Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpPRVpY3#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpPRVpY3/incremental-state"
Executing benchmark token-stream-stress (7/8)
Preparing token-stream-stress
[2023-01-11T22:04:49Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-01-11T22:04:49Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
---
[2023-01-11T22:04:54Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-11T22:04:54Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-01-11T22:04:54Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpXiccdE#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-11T22:04:59Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-01-11T22:04:59Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpXiccdE#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpXiccdE/incremental-state"
[2023-01-11T22:05:06Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-01-11T22:05:06Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpXiccdE#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpXiccdE/incremental-state"
[2023-01-11T22:05:07Z DEBUG collector::execute] applying new row to "/tmp/.tmpXiccdE"
[2023-01-11T22:05:08Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2023-01-11T22:05:08Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2023-01-11T22:05:08Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpXiccdE#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpXiccdE/incremental-state"
[2023-01-11T22:05:14Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-11T22:05:14Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-01-11T22:05:14Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp1xumuB#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-11T22:05:20Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
---
[RUSTC-TIMING] curl_sys test:false 0.105
    Checking rustc-workspace-hack v1.0.0 (/checkout/src/tools/rustc-workspace-hack)
[RUSTC-TIMING] rustc_workspace_hack test:false 0.033
 Documenting rustfmt-nightly v1.5.1 (/checkout/src/tools/rustfmt)
error[E0004]: non-exhaustive patterns: `ExprKind::FormatArgs(_)` not covered
    |
463 |     match expr.kind {
463 |     match expr.kind {
    |           ^^^^^^^^^ pattern `ExprKind::FormatArgs(_)` not covered
note: `ExprKind` defined here
note: `ExprKind` defined here
   --> /rustc/225ab3814cb608b5f14923fe372d5713e79a2b16/compiler/rustc_ast/src/ast.rs:1364:1
   ::: /rustc/225ab3814cb608b5f14923fe372d5713e79a2b16/compiler/rustc_ast/src/ast.rs:1504:5
    = note: not covered
    = note: the matched value is of type `ExprKind`
    = note: the matched value is of type `ExprKind`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
510 ~         | ast::ExprKind::Underscore => false,
510 ~         | ast::ExprKind::Underscore => false,
511 ~         ExprKind::FormatArgs(_) => todo!(),


error[E0004]: non-exhaustive patterns: `ExprKind::FormatArgs(_)` not covered
   |
   |
68 |     let expr_rw = match expr.kind {
   |                         ^^^^^^^^^ pattern `ExprKind::FormatArgs(_)` not covered
note: `ExprKind` defined here
note: `ExprKind` defined here
  --> /rustc/225ab3814cb608b5f14923fe372d5713e79a2b16/compiler/rustc_ast/src/ast.rs:1364:1
  ::: /rustc/225ab3814cb608b5f14923fe372d5713e79a2b16/compiler/rustc_ast/src/ast.rs:1504:5
   = note: not covered
   = note: the matched value is of type `ExprKind`
   = note: the matched value is of type `ExprKind`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
403~         ast::ExprKind::Err => None,
403~         ast::ExprKind::Err => None,
404~         ExprKind::FormatArgs(_) => todo!(),

For more information about this error, try `rustc --explain E0004`.
[RUSTC-TIMING] rustfmt_nightly test:false 3.770
error: could not compile `rustfmt-nightly` due to 2 previous errors
