plain
2019-12-17T16:44:32.1414598Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-17T16:44:32.1666158Z ##[command]git config gc.auto 0
2019-12-17T16:44:32.1751283Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-17T16:44:32.1828384Z ##[command]git config --get-all http.proxy
2019-12-17T16:44:32.1983662Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67241/merge:refs/remotes/pull/67241/merge
---
2019-12-17T16:50:33.7444649Z    Compiling serde_json v1.0.40
2019-12-17T16:50:35.4847562Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-12-17T16:50:46.0510057Z     Finished release [optimized] target(s) in 1m 24s
2019-12-17T16:50:46.0611745Z tidy check
2019-12-17T16:50:47.0063246Z tidy error: /checkout/src/librustc_mir/borrow_check/mod.rs:1570: TODO is deprecated; use FIXME
2019-12-17T16:50:47.0063644Z tidy error: /checkout/src/librustc_mir/borrow_check/mod.rs:1572: line longer than 100 chars
2019-12-17T16:50:48.8210295Z some tidy checks failed
2019-12-17T16:50:48.8213689Z Found 485 error codes
2019-12-17T16:50:48.8216728Z Found 0 error codes with no tests
2019-12-17T16:50:48.8216982Z Done!
2019-12-17T16:50:48.8216982Z Done!
2019-12-17T16:50:48.8217228Z 
2019-12-17T16:50:48.8217368Z 
2019-12-17T16:50:48.8221446Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-12-17T16:50:48.8222189Z 
2019-12-17T16:50:48.8222315Z 
2019-12-17T16:50:48.8230217Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-12-17T16:50:48.8230568Z Build completed unsuccessfully in 0:01:28
2019-12-17T16:50:48.8230568Z Build completed unsuccessfully in 0:01:28
2019-12-17T16:50:48.8293735Z == clock drift check ==
2019-12-17T16:50:48.8306504Z   local time: Tue Dec 17 16:50:48 UTC 2019
2019-12-17T16:50:48.8698635Z   network time: Tue, 17 Dec 2019 16:50:48 GMT
2019-12-17T16:50:48.8702704Z == end clock drift check ==
2019-12-17T16:50:50.0921804Z 
2019-12-17T16:50:50.1037066Z ##[error]Bash exited with code '1'.
2019-12-17T16:50:50.1092657Z ##[section]Starting: Checkout
2019-12-17T16:50:50.1094732Z ==============================================================================
2019-12-17T16:50:50.1094796Z Task         : Get sources
2019-12-17T16:50:50.1094872Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
