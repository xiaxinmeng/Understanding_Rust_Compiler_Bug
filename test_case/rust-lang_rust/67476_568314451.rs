plain
2019-12-22T23:41:03.4517466Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-22T23:41:04.2414678Z ##[command]git config gc.auto 0
2019-12-22T23:41:04.2418642Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-22T23:41:04.2420885Z ##[command]git config --get-all http.proxy
2019-12-22T23:41:04.2425236Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67476/merge:refs/remotes/pull/67476/merge
---
2019-12-22T23:46:54.2317602Z    Compiling serde_json v1.0.40
2019-12-22T23:46:56.1524885Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-12-22T23:47:06.7362935Z     Finished release [optimized] target(s) in 1m 24s
2019-12-22T23:47:06.7458352Z tidy check
2019-12-22T23:47:07.6338476Z tidy error: /checkout/src/librustc_mir/borrow_check/mod.rs:1595: TODO is deprecated; use FIXME
2019-12-22T23:47:07.6420496Z tidy error: /checkout/src/librustc_mir/borrow_check/diagnostics/region_name.rs:178: TODO is deprecated; use FIXME
2019-12-22T23:47:07.6447453Z tidy error: /checkout/src/librustc_mir/borrow_check/diagnostics/region_errors.rs:758: TODO is deprecated; use FIXME
2019-12-22T23:47:09.3877662Z some tidy checks failed
2019-12-22T23:47:09.3883507Z Found 485 error codes
2019-12-22T23:47:09.3883587Z Found 0 error codes with no tests
2019-12-22T23:47:09.3893321Z Done!
2019-12-22T23:47:09.3893321Z Done!
2019-12-22T23:47:09.3893670Z 
2019-12-22T23:47:09.3893871Z 
2019-12-22T23:47:09.3895339Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-12-22T23:47:09.3895745Z 
2019-12-22T23:47:09.3895885Z 
2019-12-22T23:47:09.3896088Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-12-22T23:47:09.3896237Z Build completed unsuccessfully in 0:01:28
2019-12-22T23:47:09.3896237Z Build completed unsuccessfully in 0:01:28
2019-12-22T23:47:09.3947378Z == clock drift check ==
2019-12-22T23:47:09.3953398Z   local time: Sun Dec 22 23:47:09 UTC 2019
2019-12-22T23:47:09.6745582Z   network time: Sun, 22 Dec 2019 23:47:09 GMT
2019-12-22T23:47:09.6753463Z == end clock drift check ==
2019-12-22T23:47:11.0908765Z 
2019-12-22T23:47:11.1016094Z ##[error]Bash exited with code '1'.
2019-12-22T23:47:11.1046279Z ##[section]Starting: Checkout
2019-12-22T23:47:11.1047799Z ==============================================================================
2019-12-22T23:47:11.1047869Z Task         : Get sources
2019-12-22T23:47:11.1047909Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
