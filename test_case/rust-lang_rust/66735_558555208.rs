plain
2019-11-26T09:43:47.1613552Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-26T09:43:47.7506578Z ##[command]git config gc.auto 0
2019-11-26T09:43:47.7512043Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-26T09:43:47.7515935Z ##[command]git config --get-all http.proxy
2019-11-26T09:43:47.7521296Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66735/merge:refs/remotes/pull/66735/merge
---
2019-11-26T09:49:48.6184500Z    Compiling serde_json v1.0.40
2019-11-26T09:49:50.3584964Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-11-26T09:50:01.1653588Z     Finished release [optimized] target(s) in 1m 29s
2019-11-26T09:50:01.1756885Z tidy check
2019-11-26T09:50:02.0755650Z tidy error: /checkout/src/libcore/str/mod.rs:3817: line longer than 100 chars
2019-11-26T09:50:02.0756400Z tidy error: /checkout/src/libcore/str/mod.rs:3853: line longer than 100 chars
2019-11-26T09:50:04.0115782Z Found 486 error codes
2019-11-26T09:50:04.0115983Z Found 0 error codes with no tests
2019-11-26T09:50:04.0116049Z Done!
2019-11-26T09:50:04.0116091Z some tidy checks failed
2019-11-26T09:50:04.0116091Z some tidy checks failed
2019-11-26T09:50:04.0116122Z 
2019-11-26T09:50:04.0116148Z 
2019-11-26T09:50:04.0117104Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-26T09:50:04.0117215Z 
2019-11-26T09:50:04.0117241Z 
2019-11-26T09:50:04.0121473Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-26T09:50:04.0121548Z Build completed unsuccessfully in 0:01:33
2019-11-26T09:50:04.0121548Z Build completed unsuccessfully in 0:01:33
2019-11-26T09:50:04.0173203Z == clock drift check ==
2019-11-26T09:50:04.0183789Z   local time: Tue Nov 26 09:50:04 UTC 2019
2019-11-26T09:50:04.1675317Z   network time: Tue, 26 Nov 2019 09:50:04 GMT
2019-11-26T09:50:04.1675505Z == end clock drift check ==
2019-11-26T09:50:05.5137396Z 
2019-11-26T09:50:05.5210336Z ##[error]Bash exited with code '1'.
2019-11-26T09:50:05.5239511Z ##[section]Starting: Checkout
2019-11-26T09:50:05.5241761Z ==============================================================================
2019-11-26T09:50:05.5241818Z Task         : Get sources
2019-11-26T09:50:05.5241881Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
