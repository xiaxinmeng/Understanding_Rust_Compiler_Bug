plain
2019-08-06T18:55:23.6964053Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-06T18:55:23.7149173Z ##[command]git config gc.auto 0
2019-08-06T18:55:23.7223487Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-06T18:55:24.3503187Z ##[command]git config --get-all http.proxy
2019-08-06T18:55:24.3509907Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63334/merge:refs/remotes/pull/63334/merge
---
2019-08-06T18:55:57.9712704Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-06T18:55:57.9712763Z 
2019-08-06T18:55:57.9712984Z   git checkout -b <new-branch-name>
2019-08-06T18:55:57.9713013Z 
2019-08-06T18:55:57.9713075Z HEAD is now at fffd62603 Merge 70374c95f9f0a723f2560b8c8aee3b94d99ac36b into 188ab5c976a5696ac710b7ba78849ef5dcf0235a
2019-08-06T18:55:57.9877779Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-06T18:55:57.9880296Z ==============================================================================
2019-08-06T18:55:57.9880466Z Task         : Bash
2019-08-06T18:55:57.9880506Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-06T19:01:51.2982197Z    Compiling serde_json v1.0.40
2019-08-06T19:01:55.7707783Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-08-06T19:02:04.5991545Z     Finished release [optimized] target(s) in 1m 30s
2019-08-06T19:02:04.6056109Z tidy check
2019-08-06T19:02:05.4931359Z tidy error: /checkout/src/doc/rustc-guide/ci/build-ignore-timeouts.sh: leading newline
2019-08-06T19:02:06.6232104Z some tidy checks failed
2019-08-06T19:02:06.6237217Z 
2019-08-06T19:02:06.6237217Z 
2019-08-06T19:02:06.6239299Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-08-06T19:02:06.6239953Z 
2019-08-06T19:02:06.6240147Z 
2019-08-06T19:02:06.6244374Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-08-06T19:02:06.6244646Z Build completed unsuccessfully in 0:01:34
2019-08-06T19:02:06.6244646Z Build completed unsuccessfully in 0:01:34
2019-08-06T19:02:08.1747437Z ##[error]Bash exited with code '1'.
2019-08-06T19:02:08.1776067Z ##[section]Starting: Checkout
2019-08-06T19:02:08.1777539Z ==============================================================================
2019-08-06T19:02:08.1777598Z Task         : Get sources
2019-08-06T19:02:08.1777634Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
