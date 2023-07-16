plain
2019-11-11T17:11:13.0212284Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-11T17:11:13.0417256Z ##[command]git config gc.auto 0
2019-11-11T17:11:13.0496068Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-11T17:11:13.0560363Z ##[command]git config --get-all http.proxy
2019-11-11T17:11:13.0705152Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66299/merge:refs/remotes/pull/66299/merge
---
2019-11-11T17:17:58.2175658Z     Finished release [optimized] target(s) in 1m 23s
2019-11-11T17:17:58.2256507Z tidy check
2019-11-11T17:17:59.5934682Z * 588 error codes
2019-11-11T17:17:59.5934962Z * highest error code: E0743
2019-11-11T17:17:59.6288340Z tidy error: /checkout/src/libsyntax/feature_gate/active.rs:213: no tracking issue for feature test_2018_feature
2019-11-11T17:18:00.7868868Z Found 485 error codes
2019-11-11T17:18:00.7868952Z Found 0 error codes with no tests
2019-11-11T17:18:00.7869018Z Done!
2019-11-11T17:18:00.7869221Z some tidy checks failed
2019-11-11T17:18:00.7869221Z some tidy checks failed
2019-11-11T17:18:00.7869249Z 
2019-11-11T17:18:00.7869270Z 
2019-11-11T17:18:00.7870096Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-11T17:18:00.7870185Z 
2019-11-11T17:18:00.7870207Z 
2019-11-11T17:18:00.7880382Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-11T17:18:00.7880491Z Build completed unsuccessfully in 0:01:27
2019-11-11T17:18:00.7880491Z Build completed unsuccessfully in 0:01:27
2019-11-11T17:18:00.7936384Z == clock drift check ==
2019-11-11T17:18:00.7946476Z   local time: Mon Nov 11 17:18:00 UTC 2019
2019-11-11T17:18:00.8349496Z   network time: Mon, 11 Nov 2019 17:18:00 GMT
2019-11-11T17:18:00.8354009Z == end clock drift check ==
2019-11-11T17:18:02.3527728Z 
2019-11-11T17:18:02.3623705Z ##[error]Bash exited with code '1'.
2019-11-11T17:18:02.3655491Z ##[section]Starting: Checkout
2019-11-11T17:18:02.3656970Z ==============================================================================
2019-11-11T17:18:02.3657014Z Task         : Get sources
2019-11-11T17:18:02.3657069Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
