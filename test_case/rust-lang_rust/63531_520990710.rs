plain
2019-08-13T19:52:12.1000544Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-13T19:52:12.1193740Z ##[command]git config gc.auto 0
2019-08-13T19:52:12.1269911Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-13T19:52:12.1335282Z ##[command]git config --get-all http.proxy
2019-08-13T19:52:12.1489312Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63531/merge:refs/remotes/pull/63531/merge
---
2019-08-13T19:52:48.5197821Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-13T19:52:48.5197874Z 
2019-08-13T19:52:48.5198116Z   git checkout -b <new-branch-name>
2019-08-13T19:52:48.5198147Z 
2019-08-13T19:52:48.5198211Z HEAD is now at 25db08dd5 Merge 3773bcf6ee32ff432c67583172342a14ba8e475b into 60960a260f7b5c695fd0717311d72ce62dd4eb43
2019-08-13T19:52:48.5366490Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-13T19:52:48.5369262Z ==============================================================================
2019-08-13T19:52:48.5369351Z Task         : Bash
2019-08-13T19:52:48.5369396Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-13T19:58:51.6829528Z    Compiling serde_json v1.0.40
2019-08-13T19:58:56.0753775Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-08-13T19:59:04.9003319Z     Finished release [optimized] target(s) in 1m 31s
2019-08-13T19:59:04.9081120Z tidy check
2019-08-13T19:59:05.6884325Z tidy error: /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs:15: line longer than 100 chars
2019-08-13T19:59:05.6889811Z tidy error: /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs:16: line longer than 100 chars
2019-08-13T19:59:06.9049290Z some tidy checks failed
2019-08-13T19:59:06.9056334Z 
2019-08-13T19:59:06.9056334Z 
2019-08-13T19:59:06.9058102Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-08-13T19:59:06.9058650Z 
2019-08-13T19:59:06.9058792Z 
2019-08-13T19:59:06.9062971Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-08-13T19:59:06.9063253Z Build completed unsuccessfully in 0:01:34
2019-08-13T19:59:06.9063253Z Build completed unsuccessfully in 0:01:34
2019-08-13T19:59:08.1965877Z ##[error]Bash exited with code '1'.
2019-08-13T19:59:08.2009757Z ##[section]Starting: Checkout
2019-08-13T19:59:08.2011465Z ==============================================================================
2019-08-13T19:59:08.2011540Z Task         : Get sources
2019-08-13T19:59:08.2011586Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
