plain
2019-08-30T23:18:02.5098467Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-30T23:18:02.5310827Z ##[command]git config gc.auto 0
2019-08-30T23:18:02.5385295Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-30T23:18:02.5457304Z ##[command]git config --get-all http.proxy
2019-08-30T23:18:02.5593775Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64028/merge:refs/remotes/pull/64028/merge
---
2019-08-30T23:25:08.1691413Z     Finished release [optimized] target(s) in 1m 31s
2019-08-30T23:25:08.1768137Z tidy check
2019-08-30T23:25:09.0952164Z * 578 error codes
2019-08-30T23:25:09.0953468Z * highest error code: E0733
2019-08-30T23:25:09.1583409Z tidy error: /checkout/src/liballoc/raw_vec.rs:125: malformed stability attribute: missing `feature` key
2019-08-30T23:25:10.0936258Z some tidy checks failed
2019-08-30T23:25:10.0942950Z 
2019-08-30T23:25:10.0942950Z 
2019-08-30T23:25:10.0945031Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-08-30T23:25:10.0945156Z 
2019-08-30T23:25:10.0945216Z 
2019-08-30T23:25:10.0951839Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-08-30T23:25:10.0951924Z Build completed unsuccessfully in 0:01:34
2019-08-30T23:25:10.0951924Z Build completed unsuccessfully in 0:01:34
2019-08-30T23:25:10.0996507Z == clock drift check ==
2019-08-30T23:25:10.1021139Z   local time: Fri Aug 30 23:25:10 UTC 2019
2019-08-30T23:25:10.1833173Z   network time: Fri, 30 Aug 2019 23:25:10 GMT
2019-08-30T23:25:10.1837878Z == end clock drift check ==
2019-08-30T23:25:11.5488226Z ##[error]Bash exited with code '1'.
2019-08-30T23:25:11.5518983Z ##[section]Starting: Checkout
2019-08-30T23:25:11.5520672Z ==============================================================================
2019-08-30T23:25:11.5520742Z Task         : Get sources
2019-08-30T23:25:11.5520785Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
