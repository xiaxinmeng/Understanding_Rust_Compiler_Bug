plain
2019-08-03T23:00:38.6354331Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-03T23:00:38.6541520Z ##[command]git config gc.auto 0
2019-08-03T23:00:38.6625925Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-03T23:00:38.6684564Z ##[command]git config --get-all http.proxy
2019-08-03T23:00:38.6834136Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63152/merge:refs/remotes/pull/63152/merge
---
2019-08-03T23:01:14.0031549Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-03T23:01:14.0031584Z 
2019-08-03T23:01:14.0031854Z   git checkout -b <new-branch-name>
2019-08-03T23:01:14.0031887Z 
2019-08-03T23:01:14.0031953Z HEAD is now at a3b6496aa Merge ffa38739d324644cd6efccb28dacc08e282f90df into 452087b4bf18cece2d52d7cd8c9147195e5404bf
2019-08-03T23:01:14.0186441Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-03T23:01:14.0189321Z ==============================================================================
2019-08-03T23:01:14.0189510Z Task         : Bash
2019-08-03T23:01:14.0189555Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-03T23:07:23.8195485Z    Compiling serde_json v1.0.40
2019-08-03T23:07:28.2613506Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-08-03T23:07:37.1968561Z     Finished release [optimized] target(s) in 1m 31s
2019-08-03T23:07:37.2044234Z tidy check
2019-08-03T23:07:37.8610115Z tidy error: /checkout/src/librustc_codegen_ssa/mir/rvalue.rs:173: line longer than 100 chars
2019-08-03T23:07:39.2097192Z some tidy checks failed
2019-08-03T23:07:39.2097314Z 
2019-08-03T23:07:39.2097314Z 
2019-08-03T23:07:39.2100635Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-08-03T23:07:39.2100799Z 
2019-08-03T23:07:39.2105324Z 
2019-08-03T23:07:39.2155734Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-08-03T23:07:39.2155817Z Build completed unsuccessfully in 0:01:35
2019-08-03T23:07:39.2155817Z Build completed unsuccessfully in 0:01:35
2019-08-03T23:07:40.4935985Z ##[error]Bash exited with code '1'.
2019-08-03T23:07:40.4967594Z ##[section]Starting: Checkout
2019-08-03T23:07:40.4970167Z ==============================================================================
2019-08-03T23:07:40.4970226Z Task         : Get sources
2019-08-03T23:07:40.4970273Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
