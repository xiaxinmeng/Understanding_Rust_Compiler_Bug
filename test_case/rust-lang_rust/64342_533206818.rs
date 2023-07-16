plain
2019-09-19T16:03:38.1662843Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-19T16:03:38.1859754Z ##[command]git config gc.auto 0
2019-09-19T16:03:38.1938779Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-19T16:03:38.2000995Z ##[command]git config --get-all http.proxy
2019-09-19T16:03:38.2147447Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64342/merge:refs/remotes/pull/64342/merge
---
2019-09-19T16:10:23.2966591Z    Compiling serde_json v1.0.40
2019-09-19T16:10:25.0892429Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-09-19T16:10:35.9262792Z     Finished release [optimized] target(s) in 1m 30s
2019-09-19T16:10:35.9354907Z tidy check
2019-09-19T16:10:36.0669548Z tidy error: /checkout/src/librustc_lint/unused.rs:152: line longer than 100 chars
2019-09-19T16:10:37.9812516Z some tidy checks failed
2019-09-19T16:10:37.9817378Z 
2019-09-19T16:10:37.9817378Z 
2019-09-19T16:10:37.9819236Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-09-19T16:10:37.9820851Z 
2019-09-19T16:10:37.9820998Z 
2019-09-19T16:10:37.9838888Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-09-19T16:10:37.9839379Z Build completed unsuccessfully in 0:01:33
2019-09-19T16:10:37.9839379Z Build completed unsuccessfully in 0:01:33
2019-09-19T16:10:37.9906889Z == clock drift check ==
2019-09-19T16:10:37.9945129Z   local time: Thu Sep 19 16:10:37 UTC 2019
2019-09-19T16:10:38.1498416Z   network time: Thu, 19 Sep 2019 16:10:38 GMT
2019-09-19T16:10:38.1506462Z == end clock drift check ==
2019-09-19T16:10:39.4794905Z ##[error]Bash exited with code '1'.
2019-09-19T16:10:39.4833904Z ##[section]Starting: Checkout
2019-09-19T16:10:39.4836224Z ==============================================================================
2019-09-19T16:10:39.4836281Z Task         : Get sources
2019-09-19T16:10:39.4836393Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
