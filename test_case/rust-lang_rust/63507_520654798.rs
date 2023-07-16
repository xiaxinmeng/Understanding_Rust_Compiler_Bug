plain
2019-08-13T01:16:12.1990779Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-13T01:16:12.2155721Z ##[command]git config gc.auto 0
2019-08-13T01:16:12.2220564Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-13T01:16:12.2264214Z ##[command]git config --get-all http.proxy
2019-08-13T01:16:12.2395281Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63507/merge:refs/remotes/pull/63507/merge
---
2019-08-13T01:16:47.0209373Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-13T01:16:47.0209401Z 
2019-08-13T01:16:47.0209796Z   git checkout -b <new-branch-name>
2019-08-13T01:16:47.0209952Z 
2019-08-13T01:16:47.0210209Z HEAD is now at db97a1328 Merge 3b6d46c6404cbb076ac07c568c6c4eb6d370994e into 60960a260f7b5c695fd0717311d72ce62dd4eb43
2019-08-13T01:16:47.0369013Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-13T01:16:47.0371521Z ==============================================================================
2019-08-13T01:16:47.0371568Z Task         : Bash
2019-08-13T01:16:47.0371603Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-13T01:22:13.1281226Z    Compiling serde_json v1.0.40
2019-08-13T01:22:17.1660964Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-08-13T01:22:25.3706007Z     Finished release [optimized] target(s) in 1m 24s
2019-08-13T01:22:25.3771741Z tidy check
2019-08-13T01:22:25.9522438Z tidy error: /checkout/src/test/ui/inference/cannot-infer-async-enabled-impl-trait-bindings.rs:8: trailing whitespace
2019-08-13T01:22:25.9523089Z tidy error: /checkout/src/test/ui/inference/cannot-infer-async-enabled-impl-trait-bindings.rs:15: trailing whitespace
2019-08-13T01:22:25.9523443Z tidy error: /checkout/src/test/ui/inference/cannot-infer-async.rs:6: trailing whitespace
2019-08-13T01:22:25.9523680Z tidy error: /checkout/src/test/ui/inference/cannot-infer-async.rs:13: trailing whitespace
2019-08-13T01:22:27.1280016Z some tidy checks failed
2019-08-13T01:22:27.1288880Z 
2019-08-13T01:22:27.1288880Z 
2019-08-13T01:22:27.1289770Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-08-13T01:22:27.1289943Z 
2019-08-13T01:22:27.1289971Z 
2019-08-13T01:22:27.1294108Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-08-13T01:22:27.1294223Z Build completed unsuccessfully in 0:01:26
2019-08-13T01:22:27.1294223Z Build completed unsuccessfully in 0:01:26
2019-08-13T01:22:28.4830165Z ##[error]Bash exited with code '1'.
2019-08-13T01:22:28.4860352Z ##[section]Starting: Checkout
2019-08-13T01:22:28.4861970Z ==============================================================================
2019-08-13T01:22:28.4862014Z Task         : Get sources
2019-08-13T01:22:28.4862049Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
