plain
2019-10-09T07:30:03.0162921Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-09T07:30:03.0362109Z ##[command]git config gc.auto 0
2019-10-09T07:30:03.0447037Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-09T07:30:03.0507067Z ##[command]git config --get-all http.proxy
2019-10-09T07:30:03.0653061Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64873/merge:refs/remotes/pull/64873/merge
---
2019-10-09T07:37:13.9115880Z tidy check
2019-10-09T07:37:14.7871207Z * 584 error codes
2019-10-09T07:37:14.7871339Z * highest error code: E0739
2019-10-09T07:37:15.1662688Z * 266 features
2019-10-09T07:37:15.4266120Z tidy error: The Unstable Book has a 'library feature' section 'report-time' which doesn't correspond to an unstable library feature
2019-10-09T07:37:16.0906742Z Found 482 error codes
2019-10-09T07:37:16.0906872Z Found 0 error codes with no tests
2019-10-09T07:37:16.0906922Z Done!
2019-10-09T07:37:16.0906968Z some tidy checks failed
2019-10-09T07:37:16.0906968Z some tidy checks failed
2019-10-09T07:37:16.0907021Z 
2019-10-09T07:37:16.0907051Z 
2019-10-09T07:37:16.0908036Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-09T07:37:16.0908497Z 
2019-10-09T07:37:16.0908544Z 
2019-10-09T07:37:16.0911949Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-09T07:37:16.0912054Z Build completed unsuccessfully in 0:01:32
2019-10-09T07:37:16.0912054Z Build completed unsuccessfully in 0:01:32
2019-10-09T07:37:16.0966556Z == clock drift check ==
2019-10-09T07:37:16.0985653Z   local time: Wed Oct  9 07:37:16 UTC 2019
2019-10-09T07:37:16.2585543Z   network time: Wed, 09 Oct 2019 07:37:16 GMT
2019-10-09T07:37:16.2585896Z == end clock drift check ==
2019-10-09T07:37:17.6502378Z ##[error]Bash exited with code '1'.
2019-10-09T07:37:17.6544575Z ##[section]Starting: Checkout
2019-10-09T07:37:17.6546504Z ==============================================================================
2019-10-09T07:37:17.6546564Z Task         : Get sources
2019-10-09T07:37:17.6546647Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
