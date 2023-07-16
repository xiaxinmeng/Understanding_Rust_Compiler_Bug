plain
2019-08-02T17:22:29.5942539Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-02T17:22:29.6154107Z ##[command]git config gc.auto 0
2019-08-02T17:22:29.6235094Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-02T17:22:29.6300155Z ##[command]git config --get-all http.proxy
2019-08-02T17:22:29.6439700Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62547/merge:refs/remotes/pull/62547/merge
---
2019-08-02T17:23:05.1087559Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-02T17:23:05.1087602Z 
2019-08-02T17:23:05.1087805Z   git checkout -b <new-branch-name>
2019-08-02T17:23:05.1087833Z 
2019-08-02T17:23:05.1087897Z HEAD is now at 6989a4267 Merge 3e1d1fb75a4e0830cb3690bf21dc5f1004e9d4f6 into 1df512fcaeaf17639c5d28a3045814d6f7a7db97
2019-08-02T17:23:05.1243660Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-02T17:23:05.1246662Z ==============================================================================
2019-08-02T17:23:05.1246744Z Task         : Bash
2019-08-02T17:23:05.1246792Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-02T17:31:24.2547450Z     Checking rustc_ast_borrowck v0.0.0 (/checkout/src/librustc_ast_borrowck)
2019-08-02T17:31:25.4899217Z     Checking rustc_passes v0.0.0 (/checkout/src/librustc_passes)
2019-08-02T17:31:26.2452945Z     Checking rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2019-08-02T17:31:26.6965462Z     Checking rustc_traits v0.0.0 (/checkout/src/librustc_traits)
2019-08-02T17:31:27.5024206Z error[E0425]: cannot find value `arg_place` in this scope
2019-08-02T17:31:27.5029187Z    --> src/librustc_mir/transform/rustc_peek.rs:213:25
2019-08-02T17:31:27.5029505Z     |
2019-08-02T17:31:27.5029763Z 213 |                     } = arg_place {
2019-08-02T17:31:27.5030133Z 
2019-08-02T17:31:27.9825647Z     Checking rustc_codegen_utils v0.0.0 (/checkout/src/librustc_codegen_utils)
2019-08-02T17:31:28.6088244Z     Checking rustc_plugin v0.0.0 (/checkout/src/librustc_plugin)
2019-08-02T17:31:28.8947320Z     Checking rustc_resolve v0.0.0 (/checkout/src/librustc_resolve)
---
2019-08-02T17:31:36.4200154Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
2019-08-02T17:31:36.4200853Z expected success, got: exit code: 101
2019-08-02T17:31:36.4211201Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-08-02T17:31:36.4211523Z Build completed unsuccessfully in 0:05:40
2019-08-02T17:31:37.6590017Z ##[error]Bash exited with code '1'.
2019-08-02T17:31:37.6675519Z ##[section]Starting: Checkout
2019-08-02T17:31:37.6677174Z ==============================================================================
2019-08-02T17:31:37.6677304Z Task         : Get sources
2019-08-02T17:31:37.6677347Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
