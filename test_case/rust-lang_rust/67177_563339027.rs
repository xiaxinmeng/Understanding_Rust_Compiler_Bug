plain
2019-12-09T16:56:00.3916826Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-09T16:56:00.3931401Z ##[command]git config gc.auto 0
2019-12-09T16:56:00.3936569Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-09T16:56:00.3940324Z ##[command]git config --get-all http.proxy
2019-12-09T16:56:00.3946115Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67177/merge:refs/remotes/pull/67177/merge
---
2019-12-09T17:01:47.3960484Z    Compiling serde_json v1.0.40
2019-12-09T17:01:49.0755779Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-12-09T17:01:59.5935292Z     Finished release [optimized] target(s) in 1m 24s
2019-12-09T17:01:59.6039858Z tidy check
2019-12-09T17:02:00.7671648Z tidy error: /checkout/src/librustc_errors/emitter.rs:462: line longer than 100 chars
2019-12-09T17:02:02.3753914Z Found 485 error codes
2019-12-09T17:02:02.3754092Z Found 0 error codes with no tests
2019-12-09T17:02:02.3754141Z Done!
2019-12-09T17:02:02.3813002Z some tidy checks failed
2019-12-09T17:02:02.3813002Z some tidy checks failed
2019-12-09T17:02:02.3814070Z 
2019-12-09T17:02:02.3814261Z 
2019-12-09T17:02:02.3815930Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-12-09T17:02:02.3816329Z 
2019-12-09T17:02:02.3816471Z 
2019-12-09T17:02:02.3816689Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-12-09T17:02:02.3816846Z Build completed unsuccessfully in 0:01:28
2019-12-09T17:02:02.3816846Z Build completed unsuccessfully in 0:01:28
2019-12-09T17:02:02.3821242Z == clock drift check ==
2019-12-09T17:02:02.3821449Z   local time: Mon Dec  9 17:02:02 UTC 2019
2019-12-09T17:02:02.6712912Z   network time: Mon, 09 Dec 2019 17:02:02 GMT
2019-12-09T17:02:02.6715475Z == end clock drift check ==
2019-12-09T17:02:04.1828538Z 
2019-12-09T17:02:04.1948083Z ##[error]Bash exited with code '1'.
2019-12-09T17:02:04.1993454Z ##[section]Starting: Checkout
2019-12-09T17:02:04.1995847Z ==============================================================================
2019-12-09T17:02:04.1995902Z Task         : Get sources
2019-12-09T17:02:04.1995986Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
