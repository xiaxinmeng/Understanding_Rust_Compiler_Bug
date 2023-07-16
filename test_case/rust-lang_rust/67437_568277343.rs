plain
2019-12-22T16:00:42.8036286Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-22T16:00:42.8238519Z ##[command]git config gc.auto 0
2019-12-22T16:00:42.8318530Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-22T16:00:42.8383145Z ##[command]git config --get-all http.proxy
2019-12-22T16:00:42.8531192Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67437/merge:refs/remotes/pull/67437/merge
---
2019-12-22T16:06:26.8583886Z    Compiling serde_json v1.0.40
2019-12-22T16:06:28.4531984Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-12-22T16:06:39.2427165Z     Finished release [optimized] target(s) in 1m 25s
2019-12-22T16:06:39.2527880Z tidy check
2019-12-22T16:06:39.9636159Z tidy error: /checkout/src/bootstrap/native.rs:78: line longer than 100 chars
2019-12-22T16:06:41.9550051Z some tidy checks failed
2019-12-22T16:06:41.9550876Z Found 485 error codes
2019-12-22T16:06:41.9551173Z Found 0 error codes with no tests
2019-12-22T16:06:41.9551479Z Done!
2019-12-22T16:06:41.9551479Z Done!
2019-12-22T16:06:41.9551799Z 
2019-12-22T16:06:41.9552132Z 
2019-12-22T16:06:41.9553367Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-12-22T16:06:41.9554048Z 
2019-12-22T16:06:41.9554273Z 
2019-12-22T16:06:41.9557654Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-12-22T16:06:41.9558014Z Build completed unsuccessfully in 0:01:28
2019-12-22T16:06:41.9558014Z Build completed unsuccessfully in 0:01:28
2019-12-22T16:06:41.9604235Z == clock drift check ==
2019-12-22T16:06:41.9615242Z   local time: Sun Dec 22 16:06:41 UTC 2019
2019-12-22T16:06:42.0472508Z   network time: Sun, 22 Dec 2019 16:06:42 GMT
2019-12-22T16:06:42.0475715Z == end clock drift check ==
2019-12-22T16:06:43.4507818Z 
2019-12-22T16:06:43.4617350Z ##[error]Bash exited with code '1'.
2019-12-22T16:06:43.4645910Z ##[section]Starting: Checkout
2019-12-22T16:06:43.4651890Z ==============================================================================
2019-12-22T16:06:43.4651969Z Task         : Get sources
2019-12-22T16:06:43.4652016Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
