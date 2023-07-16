plain
2019-12-03T01:58:32.6785420Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-03T01:58:32.6984305Z ##[command]git config gc.auto 0
2019-12-03T01:58:32.7058669Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-03T01:58:32.7131091Z ##[command]git config --get-all http.proxy
2019-12-03T01:58:32.7298195Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66884/merge:refs/remotes/pull/66884/merge
---
2019-12-03T02:04:22.4700442Z    Compiling serde_json v1.0.40
2019-12-03T02:04:24.1942671Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-12-03T02:04:35.2438727Z     Finished release [optimized] target(s) in 1m 27s
2019-12-03T02:04:35.2551918Z tidy check
2019-12-03T02:04:36.1975030Z tidy error: /checkout/src/libcore/num/mod.rs:846: TODO is deprecated; use FIXME
2019-12-03T02:04:37.9826655Z some tidy checks failed
2019-12-03T02:04:37.9826761Z Found 486 error codes
2019-12-03T02:04:37.9826931Z Found 0 error codes with no tests
2019-12-03T02:04:37.9827213Z Done!
2019-12-03T02:04:37.9827213Z Done!
2019-12-03T02:04:37.9827258Z 
2019-12-03T02:04:37.9827283Z 
2019-12-03T02:04:37.9828636Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-12-03T02:04:37.9828774Z 
2019-12-03T02:04:37.9828798Z 
2019-12-03T02:04:37.9828865Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-12-03T02:04:37.9828916Z Build completed unsuccessfully in 0:01:31
2019-12-03T02:04:37.9828916Z Build completed unsuccessfully in 0:01:31
2019-12-03T02:04:37.9877494Z == clock drift check ==
2019-12-03T02:04:37.9885898Z   local time: Tue Dec  3 02:04:37 UTC 2019
2019-12-03T02:04:38.5238475Z   network time: Tue, 03 Dec 2019 02:04:38 GMT
2019-12-03T02:04:38.5238594Z == end clock drift check ==
2019-12-03T02:04:39.8816431Z 
2019-12-03T02:04:39.8917030Z ##[error]Bash exited with code '1'.
2019-12-03T02:04:39.8945261Z ##[section]Starting: Checkout
2019-12-03T02:04:39.8946810Z ==============================================================================
2019-12-03T02:04:39.8946888Z Task         : Get sources
2019-12-03T02:04:39.8946931Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
