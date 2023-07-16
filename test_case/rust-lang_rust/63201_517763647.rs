plain
2019-08-02T16:17:57.2214762Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-02T16:17:57.2402817Z ##[command]git config gc.auto 0
2019-08-02T16:17:57.2467631Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-02T16:17:57.2516606Z ##[command]git config --get-all http.proxy
2019-08-02T16:17:57.2659993Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63201/merge:refs/remotes/pull/63201/merge
---
2019-08-02T16:18:32.0369807Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-02T16:18:32.0369832Z 
2019-08-02T16:18:32.0370025Z   git checkout -b <new-branch-name>
2019-08-02T16:18:32.0370051Z 
2019-08-02T16:18:32.0370108Z HEAD is now at 89fd48a0b Merge 7dcf3f0b42fc713c6155a924d37cebfbeece4dbf into 1df512fcaeaf17639c5d28a3045814d6f7a7db97
2019-08-02T16:18:32.0524544Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-02T16:18:32.0527546Z ==============================================================================
2019-08-02T16:18:32.0527613Z Task         : Bash
2019-08-02T16:18:32.0527652Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-02T16:24:42.5165728Z     Finished release [optimized] target(s) in 1m 26s
2019-08-02T16:24:42.5227546Z tidy check
2019-08-02T16:24:43.4949242Z * 578 error codes
2019-08-02T16:24:43.4950272Z * highest error code: E0733
2019-08-02T16:24:43.8224997Z Expected a gate test for the feature 'cfg_doctest'.
2019-08-02T16:24:43.8225347Z Hint: create a failing test file named 'feature-gate-cfg_doctest.rs'
2019-08-02T16:24:43.8225652Z       in the 'ui' test suite, with its failures due to
2019-08-02T16:24:43.8225709Z       missing usage of `#![feature(cfg_doctest)]`.
2019-08-02T16:24:43.8225957Z Hint: If you already have such a test and don't want to rename it,
2019-08-02T16:24:43.8226233Z       you can also add a // gate-test-cfg_doctest line to the test file.
2019-08-02T16:24:43.8226288Z tidy error: Found 1 features without a gate test.
2019-08-02T16:24:44.4080413Z some tidy checks failed
2019-08-02T16:24:44.4082650Z 
2019-08-02T16:24:44.4082650Z 
2019-08-02T16:24:44.4084574Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-08-02T16:24:44.4084787Z 
2019-08-02T16:24:44.4084815Z 
2019-08-02T16:24:44.4094946Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-08-02T16:24:44.4095020Z Build completed unsuccessfully in 0:01:29
2019-08-02T16:24:44.4095020Z Build completed unsuccessfully in 0:01:29
2019-08-02T16:24:45.7042569Z ##[error]Bash exited with code '1'.
2019-08-02T16:24:45.7099444Z ##[section]Starting: Checkout
2019-08-02T16:24:45.7100895Z ==============================================================================
2019-08-02T16:24:45.7100942Z Task         : Get sources
2019-08-02T16:24:45.7100983Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
