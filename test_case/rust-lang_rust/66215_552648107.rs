plain
2019-11-11T22:27:53.1801086Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-11T22:27:53.2013001Z ##[command]git config gc.auto 0
2019-11-11T22:27:53.2082531Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-11T22:27:53.2148690Z ##[command]git config --get-all http.proxy
2019-11-11T22:27:53.2316610Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66215/merge:refs/remotes/pull/66215/merge
---
2019-11-11T22:35:04.7148313Z    Compiling serde_json v1.0.40
2019-11-11T22:35:06.6477362Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-11-11T22:35:18.9115068Z     Finished release [optimized] target(s) in 1m 35s
2019-11-11T22:35:18.9207097Z tidy check
2019-11-11T22:35:19.0886494Z tidy error: /checkout/src/liballoc/borrow.rs:489: line longer than 100 chars
2019-11-11T22:35:19.0937553Z tidy error: /checkout/src/liballoc/string.rs:2003: line longer than 100 chars
2019-11-11T22:35:21.9019586Z some tidy checks failed
2019-11-11T22:35:21.9019710Z Found 485 error codes
2019-11-11T22:35:21.9019794Z Found 0 error codes with no tests
2019-11-11T22:35:21.9019888Z Done!
2019-11-11T22:35:21.9019888Z Done!
2019-11-11T22:35:21.9020159Z 
2019-11-11T22:35:21.9020185Z 
2019-11-11T22:35:21.9021072Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-11T22:35:21.9021212Z 
2019-11-11T22:35:21.9021239Z 
2019-11-11T22:35:21.9030376Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-11T22:35:21.9030467Z Build completed unsuccessfully in 0:01:39
2019-11-11T22:35:21.9030467Z Build completed unsuccessfully in 0:01:39
2019-11-11T22:35:21.9082261Z == clock drift check ==
2019-11-11T22:35:21.9094733Z   local time: Mon Nov 11 22:35:21 UTC 2019
2019-11-11T22:35:22.0571668Z   network time: Mon, 11 Nov 2019 22:35:22 GMT
2019-11-11T22:35:22.0576389Z == end clock drift check ==
2019-11-11T22:35:24.4355971Z 
2019-11-11T22:35:24.4473864Z ##[error]Bash exited with code '1'.
2019-11-11T22:35:24.4501130Z ##[section]Starting: Checkout
2019-11-11T22:35:24.4502823Z ==============================================================================
2019-11-11T22:35:24.4502895Z Task         : Get sources
2019-11-11T22:35:24.4502943Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
