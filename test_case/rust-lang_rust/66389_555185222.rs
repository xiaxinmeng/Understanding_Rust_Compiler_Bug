plain
2019-11-18T19:45:16.6643569Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-18T19:45:16.6856215Z ##[command]git config gc.auto 0
2019-11-18T19:45:16.6899600Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-18T19:45:16.6946885Z ##[command]git config --get-all http.proxy
2019-11-18T19:45:16.7073856Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66389/merge:refs/remotes/pull/66389/merge
---
2019-11-18T19:50:55.8633775Z tidy check
2019-11-18T19:50:57.0213457Z * 588 error codes
2019-11-18T19:50:57.0214905Z * highest error code: E0744
2019-11-18T19:50:57.3547551Z * 270 features
2019-11-18T19:50:57.9642500Z Stray file with UI testing output: "/checkout/src/test/ui/unboxed-closures/issue-30904.stderr"
2019-11-18T19:50:58.1362760Z some tidy checks failed
2019-11-18T19:50:58.1363733Z Found 441 error codes
2019-11-18T19:50:58.1364009Z Found 0 error codes with no tests
2019-11-18T19:50:58.1364157Z Done!
2019-11-18T19:50:58.1364157Z Done!
2019-11-18T19:50:58.1364278Z 
2019-11-18T19:50:58.1364397Z 
2019-11-18T19:50:58.1365416Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-18T19:50:58.1365803Z 
2019-11-18T19:50:58.1365925Z 
2019-11-18T19:50:58.1369140Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-18T19:50:58.1369955Z Build completed unsuccessfully in 0:01:21
2019-11-18T19:50:58.1369955Z Build completed unsuccessfully in 0:01:21
2019-11-18T19:50:58.1419939Z == clock drift check ==
2019-11-18T19:50:58.1427288Z   local time: Mon Nov 18 19:50:58 UTC 2019
2019-11-18T19:50:58.4206653Z   network time: Mon, 18 Nov 2019 19:50:58 GMT
2019-11-18T19:50:58.4207342Z == end clock drift check ==
2019-11-18T19:50:59.8470065Z 
2019-11-18T19:50:59.8563495Z ##[error]Bash exited with code '1'.
2019-11-18T19:50:59.8592107Z ##[section]Starting: Checkout
2019-11-18T19:50:59.8593756Z ==============================================================================
2019-11-18T19:50:59.8593824Z Task         : Get sources
2019-11-18T19:50:59.8593864Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
