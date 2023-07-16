plain
2019-10-27T23:58:30.3863084Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-27T23:58:30.4044754Z ##[command]git config gc.auto 0
2019-10-27T23:58:30.4129276Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-27T23:58:30.4182887Z ##[command]git config --get-all http.proxy
2019-10-27T23:58:30.4334099Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65539/merge:refs/remotes/pull/65539/merge
---
2019-10-28T00:04:47.8549690Z    Compiling serde_json v1.0.40
2019-10-28T00:04:49.7220684Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-28T00:05:01.8196592Z     Finished release [optimized] target(s) in 1m 33s
2019-10-28T00:05:01.8282553Z tidy check
2019-10-28T00:05:02.2604233Z tidy error: /checkout/src/librustc_resolve/resolve_imports.rs:981: line longer than 100 chars
2019-10-28T00:05:04.2496682Z some tidy checks failed
2019-10-28T00:05:04.2500114Z Found 484 error codes
2019-10-28T00:05:04.2500624Z Found 0 error codes with no tests
2019-10-28T00:05:04.2500857Z Done!
2019-10-28T00:05:04.2500857Z Done!
2019-10-28T00:05:04.2501165Z 
2019-10-28T00:05:04.2501376Z 
2019-10-28T00:05:04.2502518Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-28T00:05:04.2503110Z 
2019-10-28T00:05:04.2503296Z 
2019-10-28T00:05:04.2508046Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-28T00:05:04.2508381Z Build completed unsuccessfully in 0:01:37
2019-10-28T00:05:04.2508381Z Build completed unsuccessfully in 0:01:37
2019-10-28T00:05:04.2559535Z == clock drift check ==
2019-10-28T00:05:04.2569354Z   local time: Mon Oct 28 00:05:04 UTC 2019
2019-10-28T00:05:04.3989670Z   network time: Mon, 28 Oct 2019 00:05:04 GMT
2019-10-28T00:05:04.3991324Z == end clock drift check ==
2019-10-28T00:05:06.4361308Z 
2019-10-28T00:05:06.4426518Z ##[error]Bash exited with code '1'.
2019-10-28T00:05:06.4462014Z ##[section]Starting: Checkout
2019-10-28T00:05:06.4463638Z ==============================================================================
2019-10-28T00:05:06.4463686Z Task         : Get sources
2019-10-28T00:05:06.4463744Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
