plain
2019-11-21T17:40:55.2337465Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-21T17:40:55.9427906Z ##[command]git config gc.auto 0
2019-11-21T17:40:55.9432226Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-21T17:40:55.9438861Z ##[command]git config --get-all http.proxy
2019-11-21T17:40:55.9454950Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66606/merge:refs/remotes/pull/66606/merge
---
2019-11-21T17:47:08.8251768Z     Finished release [optimized] target(s) in 1m 28s
2019-11-21T17:47:08.8373972Z tidy check
2019-11-21T17:47:10.2006786Z * 588 error codes
2019-11-21T17:47:10.2006905Z * highest error code: E0744
2019-11-21T17:47:10.5919965Z Expected a gate test for the feature 'const_mut_refs'.
2019-11-21T17:47:10.5920862Z Hint: create a failing test file named 'feature-gate-const_mut_refs.rs'
2019-11-21T17:47:10.5921620Z       in the 'ui' test suite, with its failures due to
2019-11-21T17:47:10.5921823Z       missing usage of `#![feature(const_mut_refs)]`.
2019-11-21T17:47:10.5924015Z Hint: If you already have such a test and don't want to rename it,
2019-11-21T17:47:10.5925521Z       you can also add a // gate-test-const_mut_refs line to the test file.
2019-11-21T17:47:10.5926332Z tidy error: Found 1 features without a gate test.
2019-11-21T17:47:11.5407181Z Found 441 error codes
2019-11-21T17:47:11.5408344Z Found 0 error codes with no tests
2019-11-21T17:47:11.5408940Z Done!
2019-11-21T17:47:11.5409110Z some tidy checks failed
2019-11-21T17:47:11.5409110Z some tidy checks failed
2019-11-21T17:47:11.5409168Z 
2019-11-21T17:47:11.5409192Z 
2019-11-21T17:47:11.5410089Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-21T17:47:11.5410212Z 
2019-11-21T17:47:11.5410236Z 
2019-11-21T17:47:11.5421984Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-21T17:47:11.5422084Z Build completed unsuccessfully in 0:01:32
2019-11-21T17:47:11.5422084Z Build completed unsuccessfully in 0:01:32
2019-11-21T17:47:11.5477074Z == clock drift check ==
2019-11-21T17:47:11.5486241Z   local time: Thu Nov 21 17:47:11 UTC 2019
2019-11-21T17:47:12.5450796Z   network time: Thu, 21 Nov 2019 17:47:11 GMT
2019-11-21T17:47:12.5451905Z == end clock drift check ==
2019-11-21T17:47:13.1668789Z 
2019-11-21T17:47:13.1787916Z ##[error]Bash exited with code '1'.
2019-11-21T17:47:13.1823540Z ##[section]Starting: Checkout
2019-11-21T17:47:13.1825969Z ==============================================================================
2019-11-21T17:47:13.1826053Z Task         : Get sources
2019-11-21T17:47:13.1826107Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
