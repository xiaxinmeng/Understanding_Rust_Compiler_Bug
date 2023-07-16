plain
2019-12-29T02:03:02.4895743Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-29T02:03:02.4919745Z ##[command]git config gc.auto 0
2019-12-29T02:03:02.4922632Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-29T02:03:02.4927077Z ##[command]git config --get-all http.proxy
2019-12-29T02:03:02.4930161Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67476/merge:refs/remotes/pull/67476/merge
---
2019-12-29T02:09:03.4655917Z    Compiling serde_json v1.0.40
2019-12-29T02:09:04.9887093Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-12-29T02:09:14.9761465Z     Finished release [optimized] target(s) in 1m 19s
2019-12-29T02:09:14.9854090Z tidy check
2019-12-29T02:09:15.8375689Z tidy error: /checkout/src/librustc_mir/borrow_check/diagnostics/explain_borrow.rs:231: TODO is deprecated; use FIXME
2019-12-29T02:09:15.8385176Z tidy error: /checkout/src/librustc_mir/borrow_check/diagnostics/region_errors.rs:154: TODO is deprecated; use FIXME
2019-12-29T02:09:17.4896376Z Found 486 error codes
2019-12-29T02:09:17.4897074Z Found 0 error codes with no tests
2019-12-29T02:09:17.4897175Z Done!
2019-12-29T02:09:17.4897242Z some tidy checks failed
2019-12-29T02:09:17.4897242Z some tidy checks failed
2019-12-29T02:09:17.4897300Z 
2019-12-29T02:09:17.4897494Z 
2019-12-29T02:09:17.4898480Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-12-29T02:09:17.4898676Z 
2019-12-29T02:09:17.4898714Z 
2019-12-29T02:09:17.4915585Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-12-29T02:09:17.4915674Z Build completed unsuccessfully in 0:01:29
2019-12-29T02:09:17.4915674Z Build completed unsuccessfully in 0:01:29
2019-12-29T02:09:17.4975498Z == clock drift check ==
2019-12-29T02:09:17.5006224Z   local time: Sun Dec 29 02:09:17 UTC 2019
2019-12-29T02:09:18.0212535Z   network time: Sun, 29 Dec 2019 02:09:18 GMT
2019-12-29T02:09:18.0212642Z == end clock drift check ==
2019-12-29T02:09:19.6162274Z 
2019-12-29T02:09:19.6265354Z ##[error]Bash exited with code '1'.
2019-12-29T02:09:19.6319119Z ##[section]Starting: Checkout
2019-12-29T02:09:19.6320694Z ==============================================================================
2019-12-29T02:09:19.6320763Z Task         : Get sources
2019-12-29T02:09:19.6320807Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
