plain
2019-12-21T02:15:44.9119622Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-21T02:15:44.9215621Z ##[command]git config gc.auto 0
2019-12-21T02:15:44.9305873Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-21T02:15:44.9359910Z ##[command]git config --get-all http.proxy
2019-12-21T02:15:44.9514213Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67458/merge:refs/remotes/pull/67458/merge
---
2019-12-21T02:21:38.7568900Z    Compiling serde_json v1.0.40
2019-12-21T02:21:40.3557294Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-12-21T02:21:50.8567525Z     Finished release [optimized] target(s) in 1m 23s
2019-12-21T02:21:50.8661813Z tidy check
2019-12-21T02:21:51.3613210Z tidy error: /checkout/src/test/ui/non-ice-error-on-worker-io-fail.rs:10: line longer than 100 chars
2019-12-21T02:21:53.5238054Z some tidy checks failed
2019-12-21T02:21:53.5241230Z Found 485 error codes
2019-12-21T02:21:53.5241541Z Found 0 error codes with no tests
2019-12-21T02:21:53.5241602Z Done!
2019-12-21T02:21:53.5241602Z Done!
2019-12-21T02:21:53.5241657Z 
2019-12-21T02:21:53.5241788Z 
2019-12-21T02:21:53.5242699Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-12-21T02:21:53.5242831Z 
2019-12-21T02:21:53.5242856Z 
2019-12-21T02:21:53.5242905Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-12-21T02:21:53.5242972Z Build completed unsuccessfully in 0:01:27
2019-12-21T02:21:53.5242972Z Build completed unsuccessfully in 0:01:27
2019-12-21T02:21:53.5287376Z == clock drift check ==
2019-12-21T02:21:53.5297480Z   local time: Sat Dec 21 02:21:53 UTC 2019
2019-12-21T02:21:53.6047137Z   network time: Sat, 21 Dec 2019 02:21:53 GMT
2019-12-21T02:21:53.6047240Z == end clock drift check ==
2019-12-21T02:21:54.8993562Z 
2019-12-21T02:21:54.9094409Z ##[error]Bash exited with code '1'.
2019-12-21T02:21:54.9122276Z ##[section]Starting: Checkout
2019-12-21T02:21:54.9123979Z ==============================================================================
2019-12-21T02:21:54.9124034Z Task         : Get sources
2019-12-21T02:21:54.9124085Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
