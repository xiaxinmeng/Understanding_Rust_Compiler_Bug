plain
2019-09-22T03:55:31.0472080Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-22T03:55:31.0678969Z ##[command]git config gc.auto 0
2019-09-22T03:55:31.0772243Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-22T03:55:31.0841242Z ##[command]git config --get-all http.proxy
2019-09-22T03:55:31.1002713Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64648/merge:refs/remotes/pull/64648/merge
---
2019-09-22T04:02:44.3715308Z     Finished release [optimized] target(s) in 1m 32s
2019-09-22T04:02:44.3803616Z tidy check
2019-09-22T04:02:45.4765550Z * 578 error codes
2019-09-22T04:02:45.4765698Z * highest error code: E0733
2019-09-22T04:02:45.5199688Z tidy error: /checkout/src/libsyntax/feature_gate/active.rs:177: feature omit_gdb_pretty_printer_section is not sorted by since
2019-09-22T04:02:45.8584014Z Expected a gate test for the feature 'interp_user_fn'.
2019-09-22T04:02:45.8584493Z Hint: create a failing test file named 'feature-gate-interp_user_fn.rs'
2019-09-22T04:02:45.8584780Z       in the 'ui' test suite, with its failures due to
2019-09-22T04:02:45.8584847Z       missing usage of `#![feature(interp_user_fn)]`.
2019-09-22T04:02:45.8585599Z Hint: If you already have such a test and don't want to rename it,
2019-09-22T04:02:45.8585983Z       you can also add a // gate-test-interp_user_fn line to the test file.
2019-09-22T04:02:45.8586201Z tidy error: Found 1 features without a gate test.
2019-09-22T04:02:46.5613678Z some tidy checks failed
2019-09-22T04:02:46.5614542Z 
2019-09-22T04:02:46.5614542Z 
2019-09-22T04:02:46.5615865Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-09-22T04:02:46.5616609Z 
2019-09-22T04:02:46.5616863Z 
2019-09-22T04:02:46.5624699Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-09-22T04:02:46.5625088Z Build completed unsuccessfully in 0:01:36
2019-09-22T04:02:46.5625088Z Build completed unsuccessfully in 0:01:36
2019-09-22T04:02:46.5684022Z == clock drift check ==
2019-09-22T04:02:46.5697633Z   local time: Sun Sep 22 04:02:46 UTC 2019
2019-09-22T04:02:46.8505919Z   network time: Sun, 22 Sep 2019 04:02:46 GMT
2019-09-22T04:02:46.8506062Z == end clock drift check ==
2019-09-22T04:02:48.1345636Z ##[error]Bash exited with code '1'.
2019-09-22T04:02:48.1383229Z ##[section]Starting: Checkout
2019-09-22T04:02:48.1385123Z ==============================================================================
2019-09-22T04:02:48.1385202Z Task         : Get sources
2019-09-22T04:02:48.1385255Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
