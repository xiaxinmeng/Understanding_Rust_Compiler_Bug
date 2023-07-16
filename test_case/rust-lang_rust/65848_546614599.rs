plain
2019-10-26T15:31:29.8394278Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-26T15:31:29.8589462Z ##[command]git config gc.auto 0
2019-10-26T15:31:29.8651565Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-26T15:31:29.8706291Z ##[command]git config --get-all http.proxy
2019-10-26T15:31:29.8826978Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65848/merge:refs/remotes/pull/65848/merge
---
2019-10-26T15:37:48.9541542Z    Compiling serde_json v1.0.40
2019-10-26T15:37:50.5816216Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-26T15:38:01.3701437Z     Finished release [optimized] target(s) in 1m 23s
2019-10-26T15:38:01.3772259Z tidy check
2019-10-26T15:38:01.7770035Z tidy error: /checkout/src/bootstrap/bootstrap.py:827: line longer than 100 chars
2019-10-26T15:38:03.5427586Z Found 484 error codes
2019-10-26T15:38:03.5427656Z Found 0 error codes with no tests
2019-10-26T15:38:03.5427697Z Done!
2019-10-26T15:38:03.5427756Z some tidy checks failed
2019-10-26T15:38:03.5427756Z some tidy checks failed
2019-10-26T15:38:03.5427779Z 
2019-10-26T15:38:03.5427799Z 
2019-10-26T15:38:03.5428496Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-26T15:38:03.5428596Z 
2019-10-26T15:38:03.5428617Z 
2019-10-26T15:38:03.5428669Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-26T15:38:03.5428707Z Build completed unsuccessfully in 0:01:27
2019-10-26T15:38:03.5428707Z Build completed unsuccessfully in 0:01:27
2019-10-26T15:38:03.5473041Z == clock drift check ==
2019-10-26T15:38:03.5485857Z   local time: Sat Oct 26 15:38:03 UTC 2019
2019-10-26T15:38:03.6959967Z   network time: Sat, 26 Oct 2019 15:38:03 GMT
2019-10-26T15:38:03.6968758Z == end clock drift check ==
2019-10-26T15:38:05.0970795Z 
2019-10-26T15:38:05.1043064Z ##[error]Bash exited with code '1'.
2019-10-26T15:38:05.1074805Z ##[section]Starting: Checkout
2019-10-26T15:38:05.1076799Z ==============================================================================
2019-10-26T15:38:05.1076852Z Task         : Get sources
2019-10-26T15:38:05.1076912Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
