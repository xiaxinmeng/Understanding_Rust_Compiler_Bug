plain
2019-10-21T17:44:48.4470306Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-21T17:44:48.4666896Z ##[command]git config gc.auto 0
2019-10-21T17:44:48.4749753Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-21T17:44:48.4799467Z ##[command]git config --get-all http.proxy
2019-10-21T17:44:48.4949782Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65167/merge:refs/remotes/pull/65167/merge
---
2019-10-21T17:51:24.3249900Z Done!
2019-10-21T17:51:24.3249942Z some tidy checks failed
2019-10-21T17:51:24.3260482Z 
2019-10-21T17:51:24.3260563Z 
2019-10-21T17:51:24.3262459Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-21T17:51:24.3262629Z 
2019-10-21T17:51:24.3262655Z 
2019-10-21T17:51:24.3262702Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-21T17:51:24.3262813Z Build completed unsuccessfully in 0:01:33
2019-10-21T17:51:24.3262813Z Build completed unsuccessfully in 0:01:33
2019-10-21T17:51:24.3317100Z == clock drift check ==
2019-10-21T17:51:24.3336326Z   local time: Mon Oct 21 17:51:24 UTC 2019
2019-10-21T17:51:24.4120624Z   network time: Mon, 21 Oct 2019 17:51:24 GMT
2019-10-21T17:51:24.4122939Z == end clock drift check ==
2019-10-21T17:51:25.7133376Z 
2019-10-21T17:51:25.7235074Z ##[error]Bash exited with code '1'.
2019-10-21T17:51:25.7269424Z ##[section]Starting: Checkout
2019-10-21T17:51:25.7271220Z ==============================================================================
2019-10-21T17:51:25.7271293Z Task         : Get sources
2019-10-21T17:51:25.7271340Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
