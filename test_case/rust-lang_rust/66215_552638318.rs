plain
2019-11-11T21:59:37.1761815Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-11T21:59:37.1982863Z ##[command]git config gc.auto 0
2019-11-11T21:59:37.2050379Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-11T21:59:37.2106418Z ##[command]git config --get-all http.proxy
2019-11-11T21:59:37.2240883Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66215/merge:refs/remotes/pull/66215/merge
---
2019-11-11T22:06:09.6536008Z    Compiling serde_json v1.0.40
2019-11-11T22:06:11.4154014Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-11-11T22:06:22.7990306Z     Finished release [optimized] target(s) in 1m 28s
2019-11-11T22:06:22.8069175Z tidy check
2019-11-11T22:06:22.9466892Z tidy error: /checkout/src/liballoc/borrow.rs:489: line longer than 100 chars
2019-11-11T22:06:22.9511740Z tidy error: /checkout/src/liballoc/string.rs:2003: line longer than 100 chars
2019-11-11T22:06:25.5001445Z Found 485 error codes
2019-11-11T22:06:25.5001525Z Found 0 error codes with no tests
2019-11-11T22:06:25.5001600Z Done!
2019-11-11T22:06:25.5001661Z some tidy checks failed
2019-11-11T22:06:25.5001661Z some tidy checks failed
2019-11-11T22:06:25.5001693Z 
2019-11-11T22:06:25.5001720Z 
2019-11-11T22:06:25.5003020Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-11T22:06:25.5003160Z 
2019-11-11T22:06:25.5003185Z 
2019-11-11T22:06:25.5010973Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-11T22:06:25.5011068Z Build completed unsuccessfully in 0:01:32
2019-11-11T22:06:25.5011068Z Build completed unsuccessfully in 0:01:32
2019-11-11T22:06:25.5058163Z == clock drift check ==
2019-11-11T22:06:25.5067571Z   local time: Mon Nov 11 22:06:25 UTC 2019
2019-11-11T22:06:25.5920144Z   network time: Mon, 11 Nov 2019 22:06:25 GMT
2019-11-11T22:06:25.5921417Z == end clock drift check ==
2019-11-11T22:06:26.9082116Z 
2019-11-11T22:06:26.9182055Z ##[error]Bash exited with code '1'.
2019-11-11T22:06:26.9210499Z ##[section]Starting: Checkout
2019-11-11T22:06:26.9212136Z ==============================================================================
2019-11-11T22:06:26.9212190Z Task         : Get sources
2019-11-11T22:06:26.9212254Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
