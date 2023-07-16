plain
2019-07-30T15:24:43.5949524Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-30T15:24:43.6123742Z ##[command]git config gc.auto 0
2019-07-30T15:24:44.1811160Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-30T15:24:44.1815423Z ##[command]git config --get-all http.proxy
2019-07-30T15:24:44.1820651Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63127/merge:refs/remotes/pull/63127/merge
---
2019-07-30T15:25:17.6902697Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-30T15:25:17.6902861Z 
2019-07-30T15:25:17.6915942Z   git checkout -b <new-branch-name>
2019-07-30T15:25:17.6916372Z 
2019-07-30T15:25:17.6917079Z HEAD is now at f413af835 Merge bef8be3dc73b904c4a8ef55cd9716c4029e44334 into f690098e6d65ad7b33dc7fdefccc387806782027
2019-07-30T15:25:17.7067551Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-30T15:25:17.7070969Z ==============================================================================
2019-07-30T15:25:17.7071017Z Task         : Bash
2019-07-30T15:25:17.7071054Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-30T15:31:02.2308940Z    Compiling serde_json v1.0.40
2019-07-30T15:31:06.2671249Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-07-30T15:31:14.4843843Z     Finished release [optimized] target(s) in 1m 24s
2019-07-30T15:31:14.4913219Z tidy check
2019-07-30T15:31:14.7973648Z tidy error: /checkout/src/librustc/hir/lowering.rs:353: trailing whitespace
2019-07-30T15:31:14.7980708Z tidy error: /checkout/src/librustc/hir/lowering.rs:3279: line longer than 100 chars
2019-07-30T15:31:14.8151325Z tidy error: /checkout/src/librustc/traits/error_reporting.rs:1047: line longer than 100 chars
2019-07-30T15:31:16.2976465Z some tidy checks failed
2019-07-30T15:31:16.2981885Z 
2019-07-30T15:31:16.2981885Z 
2019-07-30T15:31:16.3007243Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-07-30T15:31:16.3007903Z 
2019-07-30T15:31:16.3007931Z 
2019-07-30T15:31:16.3027764Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-07-30T15:31:16.3027852Z Build completed unsuccessfully in 0:01:27
2019-07-30T15:31:16.3027852Z Build completed unsuccessfully in 0:01:27
2019-07-30T15:31:17.5872316Z ##[error]Bash exited with code '1'.
2019-07-30T15:31:17.5903447Z ##[section]Starting: Checkout
2019-07-30T15:31:17.5905046Z ==============================================================================
2019-07-30T15:31:17.5905091Z Task         : Get sources
2019-07-30T15:31:17.5905144Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
