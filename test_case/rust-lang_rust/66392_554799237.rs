plain
2019-11-17T22:59:12.4664699Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-17T22:59:12.4865586Z ##[command]git config gc.auto 0
2019-11-17T22:59:12.4928807Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-17T22:59:12.4975809Z ##[command]git config --get-all http.proxy
2019-11-17T22:59:12.5095558Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66392/merge:refs/remotes/pull/66392/merge
---
2019-11-17T23:04:22.5540207Z    Compiling serde_json v1.0.40
2019-11-17T23:04:23.8783325Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-11-17T23:04:32.7498809Z     Finished release [optimized] target(s) in 1m 10s
2019-11-17T23:04:32.7578938Z tidy check
2019-11-17T23:04:33.5056540Z tidy error: /checkout/src/librustc_typeck/astconv.rs:1412: line longer than 100 chars
2019-11-17T23:04:34.8123712Z Found 441 error codes
2019-11-17T23:04:34.8129988Z Found 0 error codes with no tests
2019-11-17T23:04:34.8130223Z Done!
2019-11-17T23:04:34.8133760Z 
2019-11-17T23:04:34.8133760Z 
2019-11-17T23:04:34.8133843Z 
2019-11-17T23:04:34.8134858Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-17T23:04:34.8135142Z 
2019-11-17T23:04:34.8135163Z 
2019-11-17T23:04:34.8135201Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-17T23:04:34.8135500Z Build completed unsuccessfully in 0:01:13
2019-11-17T23:04:34.8135500Z Build completed unsuccessfully in 0:01:13
2019-11-17T23:04:34.8173305Z some tidy checks failed
2019-11-17T23:04:34.8176049Z == clock drift check ==
2019-11-17T23:04:34.8185407Z   local time: Sun Nov 17 23:04:34 UTC 2019
2019-11-17T23:04:35.0816752Z   network time: Sun, 17 Nov 2019 23:04:35 GMT
2019-11-17T23:04:35.0818954Z == end clock drift check ==
2019-11-17T23:04:36.4457197Z 
2019-11-17T23:04:36.4537970Z ##[error]Bash exited with code '1'.
2019-11-17T23:04:36.4558971Z ##[section]Starting: Checkout
2019-11-17T23:04:36.4560356Z ==============================================================================
2019-11-17T23:04:36.4560398Z Task         : Get sources
2019-11-17T23:04:36.4560436Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
