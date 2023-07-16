plain
2019-10-03T11:44:04.6880863Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-03T11:44:04.7087589Z ##[command]git config gc.auto 0
2019-10-03T11:44:04.7159737Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-03T11:44:04.7243669Z ##[command]git config --get-all http.proxy
2019-10-03T11:44:05.6512163Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64842/merge:refs/remotes/pull/64842/merge
---
2019-10-03T11:51:21.3438870Z    Compiling serde_json v1.0.40
2019-10-03T11:51:23.1547165Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-03T11:51:34.4627502Z     Finished release [optimized] target(s) in 1m 30s
2019-10-03T11:51:34.4730528Z tidy check
2019-10-03T11:51:34.9011316Z tidy error: /checkout/src/librustc_resolve/late.rs:480: line longer than 100 chars
2019-10-03T11:51:36.5148730Z some tidy checks failed
2019-10-03T11:51:36.5155971Z 
2019-10-03T11:51:36.5155971Z 
2019-10-03T11:51:36.5157291Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-03T11:51:36.5157779Z 
2019-10-03T11:51:36.5157920Z 
2019-10-03T11:51:36.5162618Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-03T11:51:36.5163262Z Build completed unsuccessfully in 0:01:33
2019-10-03T11:51:36.5163262Z Build completed unsuccessfully in 0:01:33
2019-10-03T11:51:36.5214776Z == clock drift check ==
2019-10-03T11:51:36.5230303Z   local time: Thu Oct  3 11:51:36 UTC 2019
2019-10-03T11:51:36.6087604Z   network time: Thu, 03 Oct 2019 11:51:36 GMT
2019-10-03T11:51:36.6091863Z == end clock drift check ==
2019-10-03T11:51:37.9950459Z ##[error]Bash exited with code '1'.
2019-10-03T11:51:37.9993805Z ##[section]Starting: Checkout
2019-10-03T11:51:37.9997167Z ==============================================================================
2019-10-03T11:51:37.9997252Z Task         : Get sources
2019-10-03T11:51:37.9997294Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
