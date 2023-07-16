plain
2019-08-13T23:49:48.9863276Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-13T23:49:49.0042814Z ##[command]git config gc.auto 0
2019-08-13T23:49:49.0090370Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-13T23:49:49.0137032Z ##[command]git config --get-all http.proxy
2019-08-13T23:49:49.0275049Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63269/merge:refs/remotes/pull/63269/merge
---
2019-08-13T23:50:23.8100287Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-13T23:50:23.8100320Z 
2019-08-13T23:50:23.8100551Z   git checkout -b <new-branch-name>
2019-08-13T23:50:23.8100605Z 
2019-08-13T23:50:23.8100661Z HEAD is now at 48edf7fda Merge 6ff269308d88711ac3efbd4b0e73700605159ca9 into 60960a260f7b5c695fd0717311d72ce62dd4eb43
2019-08-13T23:50:23.8277050Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-13T23:50:23.8280226Z ==============================================================================
2019-08-13T23:50:23.8280290Z Task         : Bash
2019-08-13T23:50:23.8280355Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-13T23:56:02.1266583Z    Compiling serde_json v1.0.40
2019-08-13T23:56:06.2489768Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-08-13T23:56:14.4673315Z     Finished release [optimized] target(s) in 1m 25s
2019-08-13T23:56:14.4747165Z tidy check
2019-08-13T23:56:14.5868535Z tidy error: /checkout/src/librustc_metadata/cstore_impl.rs:442: line longer than 100 chars
2019-08-13T23:56:16.3325619Z some tidy checks failed
2019-08-13T23:56:16.3326563Z 
2019-08-13T23:56:16.3326563Z 
2019-08-13T23:56:16.3327763Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-08-13T23:56:16.3328461Z 
2019-08-13T23:56:16.3328673Z 
2019-08-13T23:56:16.3332873Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-08-13T23:56:16.3333223Z Build completed unsuccessfully in 0:01:28
2019-08-13T23:56:16.3333223Z Build completed unsuccessfully in 0:01:28
2019-08-13T23:56:17.7762566Z ##[error]Bash exited with code '1'.
2019-08-13T23:56:17.7794105Z ##[section]Starting: Checkout
2019-08-13T23:56:17.7795846Z ==============================================================================
2019-08-13T23:56:17.7795920Z Task         : Get sources
2019-08-13T23:56:17.7795962Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
