plain
2019-08-02T15:40:20.2461838Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-02T15:40:20.2653034Z ##[command]git config gc.auto 0
2019-08-02T15:40:20.2737363Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-02T15:40:20.2833360Z ##[command]git config --get-all http.proxy
2019-08-02T15:40:20.2973903Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63201/merge:refs/remotes/pull/63201/merge
---
2019-08-02T15:40:57.8074284Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-02T15:40:57.8074316Z 
2019-08-02T15:40:57.8074673Z   git checkout -b <new-branch-name>
2019-08-02T15:40:57.8074701Z 
2019-08-02T15:40:57.8074759Z HEAD is now at 8ff8fc543 Merge ca0d1d0ef78ec63eccbc8cf2d84b98270f4964ac into 1df512fcaeaf17639c5d28a3045814d6f7a7db97
2019-08-02T15:40:57.8234165Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-02T15:40:57.8236725Z ==============================================================================
2019-08-02T15:40:57.8236773Z Task         : Bash
2019-08-02T15:40:57.8236829Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-02T15:47:14.3647711Z     Finished release [optimized] target(s) in 1m 30s
2019-08-02T15:47:14.3685119Z tidy check
2019-08-02T15:47:15.3188993Z * 578 error codes
2019-08-02T15:47:15.3189731Z * highest error code: E0733
2019-08-02T15:47:15.6966399Z Expected a gate test for the feature 'cfg_doctest'.
2019-08-02T15:47:15.6967656Z Hint: create a failing test file named 'feature-gate-cfg_doctest.rs'
2019-08-02T15:47:15.6968198Z       in the 'ui' test suite, with its failures due to
2019-08-02T15:47:15.6968482Z       missing usage of `#![feature(cfg_doctest)]`.
2019-08-02T15:47:15.6968972Z Hint: If you already have such a test and don't want to rename it,
2019-08-02T15:47:15.6969484Z       you can also add a // gate-test-cfg_doctest line to the test file.
2019-08-02T15:47:15.6969930Z tidy error: Found 1 features without a gate test.
2019-08-02T15:47:16.3309204Z some tidy checks failed
2019-08-02T15:47:16.3309389Z 
2019-08-02T15:47:16.3309389Z 
2019-08-02T15:47:16.3310452Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-08-02T15:47:16.3310834Z 
2019-08-02T15:47:16.3310862Z 
2019-08-02T15:47:16.3317588Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-08-02T15:47:16.3317709Z Build completed unsuccessfully in 0:01:33
2019-08-02T15:47:16.3317709Z Build completed unsuccessfully in 0:01:33
2019-08-02T15:47:17.6627090Z ##[error]Bash exited with code '1'.
2019-08-02T15:47:17.6658095Z ##[section]Starting: Checkout
2019-08-02T15:47:17.6659743Z ==============================================================================
2019-08-02T15:47:17.6659799Z Task         : Get sources
2019-08-02T15:47:17.6659863Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
