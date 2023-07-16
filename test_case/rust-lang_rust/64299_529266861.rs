plain
2019-09-09T00:49:42.8978572Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-09T00:49:42.9165247Z ##[command]git config gc.auto 0
2019-09-09T00:49:42.9232034Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-09T00:49:42.9274343Z ##[command]git config --get-all http.proxy
2019-09-09T00:49:42.9405130Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64299/merge:refs/remotes/pull/64299/merge
---
2019-09-09T00:56:30.7727351Z    Compiling serde_json v1.0.40
2019-09-09T00:56:32.3814842Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-09-09T00:56:42.3686758Z     Finished release [optimized] target(s) in 1m 23s
2019-09-09T00:56:42.3756111Z tidy check
2019-09-09T00:56:42.4953009Z tidy error: /checkout/src/librustc_target/abi/mod.rs:552: line longer than 100 chars
2019-09-09T00:56:42.4954228Z tidy error: /checkout/src/librustc_target/abi/mod.rs:681: line longer than 100 chars
2019-09-09T00:56:42.9383305Z tidy error: /checkout/src/librustc/mir/interpret/allocation.rs:507: line longer than 100 chars
2019-09-09T00:56:42.9592234Z tidy error: /checkout/src/librustc/ty/layout.rs:245: trailing whitespace
2019-09-09T00:56:42.9593568Z tidy error: /checkout/src/librustc/ty/layout.rs:250: trailing whitespace
2019-09-09T00:56:42.9593639Z tidy error: /checkout/src/librustc/ty/layout.rs:309: trailing whitespace
2019-09-09T00:56:42.9593712Z tidy error: /checkout/src/librustc/ty/layout.rs:310: trailing whitespace
2019-09-09T00:56:43.0288353Z tidy error: /checkout/src/librustc_mir/interpret/place.rs:11: line longer than 100 chars
2019-09-09T00:56:43.0292472Z tidy error: /checkout/src/librustc_mir/interpret/intrinsics.rs:153: line longer than 100 chars
2019-09-09T00:56:43.0296627Z tidy error: /checkout/src/librustc_mir/interpret/memory.rs:363: trailing whitespace
2019-09-09T00:56:43.0298798Z tidy error: /checkout/src/librustc_mir/interpret/memory.rs:770: line longer than 100 chars
2019-09-09T00:56:43.0310979Z tidy error: /checkout/src/librustc_mir/interpret/operand.rs:383: line longer than 100 chars
2019-09-09T00:56:43.0311045Z tidy error: /checkout/src/librustc_mir/interpret/operand.rs:634: line longer than 100 chars
2019-09-09T00:56:43.3167692Z tidy error: /checkout/src/librustc_codegen_ssa/mir/place.rs:402: line longer than 100 chars
2019-09-09T00:56:43.3227222Z tidy error: /checkout/src/librustc_codegen_llvm/type_of.rs:5: line longer than 100 chars
2019-09-09T00:56:43.3237474Z tidy error: /checkout/src/librustc_codegen_llvm/debuginfo/metadata.rs:1921: trailing whitespace
2019-09-09T00:56:43.3255916Z tidy error: /checkout/src/librustc_codegen_llvm/abi.rs:272: line longer than 100 chars
2019-09-09T00:56:43.3597452Z tidy error: /checkout/src/librustc_passes/layout_test.rs:68: line longer than 100 chars
2019-09-09T00:56:43.3597701Z tidy error: /checkout/src/librustc_passes/layout_test.rs:74: line longer than 100 chars
2019-09-09T00:56:44.3799592Z some tidy checks failed
2019-09-09T00:56:44.3799730Z 
2019-09-09T00:56:44.3799730Z 
2019-09-09T00:56:44.3800638Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-09-09T00:56:44.3800727Z 
2019-09-09T00:56:44.3800763Z 
2019-09-09T00:56:44.3809203Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-09-09T00:56:44.3810115Z Build completed unsuccessfully in 0:01:26
2019-09-09T00:56:44.3810115Z Build completed unsuccessfully in 0:01:26
2019-09-09T00:56:44.3860047Z == clock drift check ==
2019-09-09T00:56:44.3872110Z   local time: Mon Sep  9 00:56:44 UTC 2019
2019-09-09T00:56:44.5342833Z   network time: Mon, 09 Sep 2019 00:56:44 GMT
2019-09-09T00:56:44.5348037Z == end clock drift check ==
2019-09-09T00:56:46.0303481Z ##[error]Bash exited with code '1'.
2019-09-09T00:56:46.0330991Z ##[section]Starting: Checkout
2019-09-09T00:56:46.0333025Z ==============================================================================
2019-09-09T00:56:46.0333082Z Task         : Get sources
2019-09-09T00:56:46.0333149Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
