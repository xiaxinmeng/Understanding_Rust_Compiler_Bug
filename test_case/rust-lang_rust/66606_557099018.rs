plain
2019-11-21T13:47:00.8602089Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-21T13:47:01.4910891Z ##[command]git config gc.auto 0
2019-11-21T13:47:01.4913438Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-21T13:47:01.4915253Z ##[command]git config --get-all http.proxy
2019-11-21T13:47:01.4918055Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66606/merge:refs/remotes/pull/66606/merge
---
2019-11-21T13:53:26.3802655Z     Finished release [optimized] target(s) in 1m 31s
2019-11-21T13:53:26.3912964Z tidy check
2019-11-21T13:53:27.7774355Z * 588 error codes
2019-11-21T13:53:27.7774732Z * highest error code: E0744
2019-11-21T13:53:27.8211661Z tidy error: /checkout/src/libsyntax/feature_gate/active.rs:533: no tracking issue for feature const_fn_mut_refs
2019-11-21T13:53:29.1344892Z Found 441 error codes
2019-11-21T13:53:29.1345016Z Found 0 error codes with no tests
2019-11-21T13:53:29.1345061Z Done!
2019-11-21T13:53:29.1345152Z some tidy checks failed
2019-11-21T13:53:29.1345152Z some tidy checks failed
2019-11-21T13:53:29.1345183Z 
2019-11-21T13:53:29.1345208Z 
2019-11-21T13:53:29.1346140Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-21T13:53:29.1346269Z 
2019-11-21T13:53:29.1346294Z 
2019-11-21T13:53:29.1351017Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-21T13:53:29.1351124Z Build completed unsuccessfully in 0:01:35
2019-11-21T13:53:29.1351124Z Build completed unsuccessfully in 0:01:35
2019-11-21T13:53:29.1402702Z == clock drift check ==
2019-11-21T13:53:29.1420604Z   local time: Thu Nov 21 13:53:29 UTC 2019
2019-11-21T13:53:29.4306804Z   network time: Thu, 21 Nov 2019 13:53:29 GMT
2019-11-21T13:53:29.4308776Z == end clock drift check ==
2019-11-21T13:53:30.7061527Z 
2019-11-21T13:53:30.7179681Z ##[error]Bash exited with code '1'.
2019-11-21T13:53:30.7220222Z ##[section]Starting: Checkout
2019-11-21T13:53:30.7222597Z ==============================================================================
2019-11-21T13:53:30.7222675Z Task         : Get sources
2019-11-21T13:53:30.7222728Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
