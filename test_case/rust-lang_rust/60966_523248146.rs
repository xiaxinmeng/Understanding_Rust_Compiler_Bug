plain
2019-08-21T00:18:21.4886844Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-21T00:18:21.5068347Z ##[command]git config gc.auto 0
2019-08-21T00:18:21.5134685Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-21T00:18:21.5178948Z ##[command]git config --get-all http.proxy
2019-08-21T00:18:21.5303035Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/60966/merge:refs/remotes/pull/60966/merge
---
2019-08-21T00:18:56.0285022Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-21T00:18:56.0285283Z 
2019-08-21T00:18:56.0285745Z   git checkout -b <new-branch-name>
2019-08-21T00:18:56.0285995Z 
2019-08-21T00:18:56.0286251Z HEAD is now at b0d10127b Merge 38d5f8fc12b94281ecdb0d12097278585fa361f2 into bea0372a1a7a31b81f28cc4d9a83a2dc9a79d008
2019-08-21T00:18:56.0458138Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-21T00:18:56.0461578Z ==============================================================================
2019-08-21T00:18:56.0461647Z Task         : Bash
2019-08-21T00:18:56.0461686Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-21T00:24:47.5124391Z     Finished release [optimized] target(s) in 1m 19s
2019-08-21T00:24:47.5190230Z tidy check
2019-08-21T00:24:48.2790527Z * 578 error codes
2019-08-21T00:24:48.2790625Z * highest error code: E0733
2019-08-21T00:24:48.5738841Z tidy error: Found 1 features without a gate test.
2019-08-21T00:24:48.5739504Z Expected a gate test for the feature 'rustc_diagnostic_items'.
2019-08-21T00:24:48.5739755Z Hint: create a failing test file named 'feature-gate-rustc_diagnostic_items.rs'
2019-08-21T00:24:48.5739939Z       in the 'ui' test suite, with its failures due to
2019-08-21T00:24:48.5739982Z       missing usage of `#![feature(rustc_diagnostic_items)]`.
2019-08-21T00:24:48.5740203Z Hint: If you already have such a test and don't want to rename it,
2019-08-21T00:24:48.5740413Z       you can also add a // gate-test-rustc_diagnostic_items line to the test file.
2019-08-21T00:24:49.1315819Z some tidy checks failed
2019-08-21T00:24:49.1316958Z 
2019-08-21T00:24:49.1316958Z 
2019-08-21T00:24:49.1317693Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-08-21T00:24:49.1317804Z 
2019-08-21T00:24:49.1317826Z 
2019-08-21T00:24:49.1322253Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-08-21T00:24:49.1322323Z Build completed unsuccessfully in 0:01:22
2019-08-21T00:24:49.1322323Z Build completed unsuccessfully in 0:01:22
2019-08-21T00:24:49.1367109Z == clock drift check ==
2019-08-21T00:24:49.1382596Z   local time: Wed Aug 21 00:24:49 UTC 2019
2019-08-21T00:24:49.2724061Z   network time: Wed, 21 Aug 2019 00:24:49 GMT
2019-08-21T00:24:49.2727462Z == end clock drift check ==
2019-08-21T00:24:50.8079491Z ##[error]Bash exited with code '1'.
2019-08-21T00:24:50.8108558Z ##[section]Starting: Checkout
2019-08-21T00:24:50.8109937Z ==============================================================================
2019-08-21T00:24:50.8110000Z Task         : Get sources
2019-08-21T00:24:50.8110037Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
