plain
2019-12-09T17:08:38.2168664Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-09T17:08:38.2182446Z ##[command]git config gc.auto 0
2019-12-09T17:08:38.2187783Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-09T17:08:38.2191739Z ##[command]git config --get-all http.proxy
2019-12-09T17:08:38.2196939Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67178/merge:refs/remotes/pull/67178/merge
---
2019-12-09T17:14:36.6984230Z    Compiling serde_json v1.0.40
2019-12-09T17:14:38.4397286Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-12-09T17:14:49.2968381Z     Finished release [optimized] target(s) in 1m 27s
2019-12-09T17:14:49.3078959Z tidy check
2019-12-09T17:14:50.3250774Z tidy error: /checkout/src/librustdoc/clean/mod.rs: ignoring file length unnecessarily
2019-12-09T17:14:52.0903370Z some tidy checks failed
2019-12-09T17:14:52.0904935Z Found 485 error codes
2019-12-09T17:14:52.0905363Z Found 0 error codes with no tests
2019-12-09T17:14:52.0905553Z Done!
2019-12-09T17:14:52.0905553Z Done!
2019-12-09T17:14:52.0905687Z 
2019-12-09T17:14:52.0905814Z 
2019-12-09T17:14:52.0906831Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-12-09T17:14:52.0907247Z 
2019-12-09T17:14:52.0907379Z 
2019-12-09T17:14:52.0915595Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-12-09T17:14:52.0915691Z Build completed unsuccessfully in 0:01:31
2019-12-09T17:14:52.0915691Z Build completed unsuccessfully in 0:01:31
2019-12-09T17:14:52.0968142Z == clock drift check ==
2019-12-09T17:14:52.0979072Z   local time: Mon Dec  9 17:14:52 UTC 2019
2019-12-09T17:14:52.3917766Z   network time: Mon, 09 Dec 2019 17:14:52 GMT
2019-12-09T17:14:52.3917925Z == end clock drift check ==
2019-12-09T17:14:53.7084612Z 
2019-12-09T17:14:53.7186971Z ##[error]Bash exited with code '1'.
2019-12-09T17:14:53.7214346Z ##[section]Starting: Checkout
2019-12-09T17:14:53.7215944Z ==============================================================================
2019-12-09T17:14:53.7215996Z Task         : Get sources
2019-12-09T17:14:53.7216056Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
