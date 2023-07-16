plain
2019-08-22T09:30:48.5798182Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-22T09:30:48.6018608Z ##[command]git config gc.auto 0
2019-08-22T09:30:48.6095962Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-22T09:30:48.6145030Z ##[command]git config --get-all http.proxy
2019-08-22T09:30:48.6296509Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63803/merge:refs/remotes/pull/63803/merge
2019-08-22T09:30:50.6930153Z remote:                                                                                         
---
2019-08-22T09:31:24.0770250Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-22T09:31:24.0770518Z 
2019-08-22T09:31:24.0771003Z   git checkout -b <new-branch-name>
2019-08-22T09:31:24.0771256Z 
2019-08-22T09:31:24.0771495Z HEAD is now at 9096e1a21 Merge 2e000c03bb60d32e49cb3ece7a36d7194c74ee90 into 42dcd4b7c5fb7b61bc2f4c0842f66e5ad40057e4
2019-08-22T09:31:24.0931973Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-22T09:31:24.0935108Z ==============================================================================
2019-08-22T09:31:24.0935183Z Task         : Bash
2019-08-22T09:31:24.0935226Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-22T09:37:55.2917135Z     Finished release [optimized] target(s) in 1m 29s
2019-08-22T09:37:55.2992930Z tidy check
2019-08-22T09:37:56.2276996Z * 578 error codes
2019-08-22T09:37:56.2290719Z * highest error code: E0733
2019-08-22T09:37:56.5981484Z tidy error: Found 1 features without a gate test.
2019-08-22T09:37:56.5982197Z Expected a gate test for the feature 'cfg_doctest'.
2019-08-22T09:37:56.5982472Z Hint: create a failing test file named 'feature-gate-cfg_doctest.rs'
2019-08-22T09:37:56.5982715Z       in the 'ui' test suite, with its failures due to
2019-08-22T09:37:56.5982767Z       missing usage of `#![feature(cfg_doctest)]`.
2019-08-22T09:37:56.5982994Z Hint: If you already have such a test and don't want to rename it,
2019-08-22T09:37:56.5983222Z       you can also add a // gate-test-cfg_doctest line to the test file.
2019-08-22T09:37:57.2811687Z some tidy checks failed
2019-08-22T09:37:57.2812652Z 
2019-08-22T09:37:57.2812652Z 
2019-08-22T09:37:57.2813762Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-08-22T09:37:57.2814237Z 
2019-08-22T09:37:57.2814275Z 
2019-08-22T09:37:57.2821073Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-08-22T09:37:57.2821346Z Build completed unsuccessfully in 0:01:33
2019-08-22T09:37:57.2821346Z Build completed unsuccessfully in 0:01:33
2019-08-22T09:37:57.2873477Z == clock drift check ==
2019-08-22T09:37:57.2883640Z   local time: Thu Aug 22 09:37:57 UTC 2019
2019-08-22T09:37:57.3729628Z   network time: Thu, 22 Aug 2019 09:37:57 GMT
2019-08-22T09:37:57.3729920Z == end clock drift check ==
2019-08-22T09:37:58.7510558Z ##[error]Bash exited with code '1'.
2019-08-22T09:37:58.7558292Z ##[section]Starting: Checkout
2019-08-22T09:37:58.7560117Z ==============================================================================
2019-08-22T09:37:58.7560187Z Task         : Get sources
2019-08-22T09:37:58.7560232Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
