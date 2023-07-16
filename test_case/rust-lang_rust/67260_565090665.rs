plain
2019-12-12T16:35:49.4448577Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-12T16:35:49.4635588Z ##[command]git config gc.auto 0
2019-12-12T16:35:49.4704658Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-12T16:35:49.4764391Z ##[command]git config --get-all http.proxy
2019-12-12T16:35:49.4917029Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67260/merge:refs/remotes/pull/67260/merge
---
2019-12-12T16:41:02.2882385Z    Compiling serde_json v1.0.40
2019-12-12T16:41:03.8089940Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-12-12T16:41:13.5348451Z     Finished release [optimized] target(s) in 1m 15s
2019-12-12T16:41:13.5450997Z tidy check
2019-12-12T16:41:14.1398020Z tidy error: /checkout/src/test/ui/consts/const_limit/const_limit_is_feature_gated.rs: missing trailing newline
2019-12-12T16:41:14.2706204Z tidy error: /checkout/src/librustc/middle/const_limit.rs: leading newline
2019-12-12T16:41:15.0916805Z tidy error: Found 1 features without a gate test.
2019-12-12T16:41:15.0918780Z Expected a gate test for the feature 'const_limit'.
2019-12-12T16:41:15.0919339Z Hint: create a failing test file named 'feature-gate-const_limit.rs'
2019-12-12T16:41:15.0920088Z       in the 'ui' test suite, with its failures due to
2019-12-12T16:41:15.0920203Z       missing usage of `#![feature(const_limit)]`.
2019-12-12T16:41:15.0920444Z Hint: If you already have such a test and don't want to rename it,
2019-12-12T16:41:15.0920695Z       you can also add a // gate-test-const_limit line to the test file.
2019-12-12T16:41:16.6984285Z some tidy checks failed
2019-12-12T16:41:16.6988282Z Found 485 error codes
2019-12-12T16:41:16.6988332Z Found 0 error codes with no tests
2019-12-12T16:41:16.6988370Z Done!
2019-12-12T16:41:16.6988370Z Done!
2019-12-12T16:41:16.6988395Z 
2019-12-12T16:41:16.6988418Z 
2019-12-12T16:41:16.6989628Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-12-12T16:41:16.6989752Z 
2019-12-12T16:41:16.6989794Z 
2019-12-12T16:41:16.6989998Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-12-12T16:41:16.6990039Z Build completed unsuccessfully in 0:01:18
2019-12-12T16:41:16.6990039Z Build completed unsuccessfully in 0:01:18
2019-12-12T16:41:16.6990091Z == clock drift check ==
2019-12-12T16:41:16.6990127Z   local time: Thu Dec 12 16:41:15 UTC 2019
2019-12-12T16:41:16.6990165Z   network time: Thu, 12 Dec 2019 16:41:16 GMT
2019-12-12T16:41:16.6990217Z == end clock drift check ==
2019-12-12T16:41:17.4729565Z 
2019-12-12T16:41:17.4830950Z ##[error]Bash exited with code '1'.
2019-12-12T16:41:17.4855810Z ##[section]Starting: Checkout
2019-12-12T16:41:17.4857233Z ==============================================================================
2019-12-12T16:41:17.4857294Z Task         : Get sources
2019-12-12T16:41:17.4857348Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
