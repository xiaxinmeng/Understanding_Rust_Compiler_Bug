plain
2019-10-09T13:21:09.8771456Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-09T13:21:10.4015763Z ##[command]git config gc.auto 0
2019-10-09T13:21:10.4021429Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-09T13:21:10.4023352Z ##[command]git config --get-all http.proxy
2019-10-09T13:21:10.4032132Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65182/merge:refs/remotes/pull/65182/merge
---
2019-10-09T13:28:05.4032573Z    Compiling serde_json v1.0.40
2019-10-09T13:28:07.3115087Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-09T13:28:19.1535583Z     Finished release [optimized] target(s) in 1m 31s
2019-10-09T13:28:19.1615031Z tidy check
2019-10-09T13:28:19.4196590Z tidy error: /checkout/src/test/ui/rfc-2091-track-caller/taking-fn-pointer.rs: missing trailing newline
2019-10-09T13:28:20.5210218Z tidy error: /checkout/src/librustc_mir/shim.rs:48: TODO is deprecated; use FIXME
2019-10-09T13:28:21.3370500Z some tidy checks failed
2019-10-09T13:28:21.3370686Z Found 482 error codes
2019-10-09T13:28:21.3370735Z Found 0 error codes with no tests
2019-10-09T13:28:21.3370780Z Done!
2019-10-09T13:28:21.3370780Z Done!
2019-10-09T13:28:21.3370811Z 
2019-10-09T13:28:21.3370857Z 
2019-10-09T13:28:21.3371793Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-09T13:28:21.3371932Z 
2019-10-09T13:28:21.3371960Z 
2019-10-09T13:28:21.3378931Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-09T13:28:21.3379262Z Build completed unsuccessfully in 0:01:34
2019-10-09T13:28:21.3379262Z Build completed unsuccessfully in 0:01:34
2019-10-09T13:28:21.3433539Z == clock drift check ==
2019-10-09T13:28:21.3450161Z   local time: Wed Oct  9 13:28:21 UTC 2019
2019-10-09T13:28:21.6350594Z   network time: Wed, 09 Oct 2019 13:28:21 GMT
2019-10-09T13:28:21.6350838Z == end clock drift check ==
2019-10-09T13:28:22.9998885Z ##[error]Bash exited with code '1'.
2019-10-09T13:28:23.0042836Z ##[section]Starting: Checkout
2019-10-09T13:28:23.0044644Z ==============================================================================
2019-10-09T13:28:23.0044721Z Task         : Get sources
2019-10-09T13:28:23.0044769Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
