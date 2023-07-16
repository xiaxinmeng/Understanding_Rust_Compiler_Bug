plain
2019-10-09T17:22:49.6616175Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-09T17:22:49.7909367Z ##[command]git config gc.auto 0
2019-10-09T17:22:49.7990475Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-09T17:22:49.8042178Z ##[command]git config --get-all http.proxy
2019-10-09T17:22:49.8173368Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63810/merge:refs/remotes/pull/63810/merge
---
2019-10-09T17:30:14.0663478Z tidy check
2019-10-09T17:30:14.8191502Z * 584 error codes
2019-10-09T17:30:14.8192796Z * highest error code: E0739
2019-10-09T17:30:15.1580346Z * 266 features
2019-10-09T17:30:15.8233773Z Stray file with UI testing output: "/checkout/src/test/ui/consts/offset_of_ub.stderr"
2019-10-09T17:30:16.0298200Z some tidy checks failed
2019-10-09T17:30:16.0298372Z Found 482 error codes
2019-10-09T17:30:16.0298412Z Found 0 error codes with no tests
2019-10-09T17:30:16.0301245Z Done!
2019-10-09T17:30:16.0301245Z Done!
2019-10-09T17:30:16.0308631Z 
2019-10-09T17:30:16.0308708Z 
2019-10-09T17:30:16.0309732Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-09T17:30:16.0309828Z 
2019-10-09T17:30:16.0309851Z 
2019-10-09T17:30:16.0335527Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-09T17:30:16.0335585Z Build completed unsuccessfully in 0:01:32
2019-10-09T17:30:16.0335585Z Build completed unsuccessfully in 0:01:32
2019-10-09T17:30:16.0355811Z == clock drift check ==
2019-10-09T17:30:16.0369729Z   local time: Wed Oct  9 17:30:16 UTC 2019
2019-10-09T17:30:16.1218284Z   network time: Wed, 09 Oct 2019 17:30:16 GMT
2019-10-09T17:30:16.1222800Z == end clock drift check ==
2019-10-09T17:30:17.1317586Z ##[error]Bash exited with code '1'.
2019-10-09T17:30:17.1354215Z ##[section]Starting: Checkout
2019-10-09T17:30:17.1356035Z ==============================================================================
2019-10-09T17:30:17.1356094Z Task         : Get sources
2019-10-09T17:30:17.1356143Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
