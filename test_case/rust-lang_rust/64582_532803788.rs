plain
2019-09-18T17:54:57.8153971Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-18T17:54:57.8376707Z ##[command]git config gc.auto 0
2019-09-18T17:54:57.8449444Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-18T17:54:57.8502798Z ##[command]git config --get-all http.proxy
2019-09-18T17:54:57.8662204Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64582/merge:refs/remotes/pull/64582/merge
---
2019-09-18T18:02:35.6750803Z     Finished release [optimized] target(s) in 1m 30s
2019-09-18T18:02:35.6852832Z tidy check
2019-09-18T18:02:36.6602640Z * 578 error codes
2019-09-18T18:02:36.6603711Z * highest error code: E0733
2019-09-18T18:02:37.0703098Z Expected a gate test for the feature 'abi_swift'.
2019-09-18T18:02:37.0703493Z Hint: create a failing test file named 'feature-gate-abi_swift.rs'
2019-09-18T18:02:37.0703759Z       in the 'ui' test suite, with its failures due to
2019-09-18T18:02:37.0710676Z       missing usage of `#![feature(abi_swift)]`.
2019-09-18T18:02:37.0711496Z Hint: If you already have such a test and don't want to rename it,
2019-09-18T18:02:37.0712020Z       you can also add a // gate-test-abi_swift line to the test file.
2019-09-18T18:02:37.0712350Z tidy error: Found 1 features without a gate test.
2019-09-18T18:02:37.8292530Z some tidy checks failed
2019-09-18T18:02:37.8307402Z 
2019-09-18T18:02:37.8307402Z 
2019-09-18T18:02:37.8308858Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-09-18T18:02:37.8309677Z 
2019-09-18T18:02:37.8309813Z 
2019-09-18T18:02:37.8310429Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-09-18T18:02:37.8310636Z Build completed unsuccessfully in 0:01:33
2019-09-18T18:02:37.8310636Z Build completed unsuccessfully in 0:01:33
2019-09-18T18:02:37.8360789Z == clock drift check ==
2019-09-18T18:02:37.8379509Z   local time: Wed Sep 18 18:02:37 UTC 2019
2019-09-18T18:02:37.9420805Z   network time: Wed, 18 Sep 2019 18:02:37 GMT
2019-09-18T18:02:37.9422894Z == end clock drift check ==
2019-09-18T18:02:39.7424408Z ##[error]Bash exited with code '1'.
2019-09-18T18:02:39.7472568Z ##[section]Starting: Checkout
2019-09-18T18:02:39.7474672Z ==============================================================================
2019-09-18T18:02:39.7474732Z Task         : Get sources
2019-09-18T18:02:39.7474807Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
