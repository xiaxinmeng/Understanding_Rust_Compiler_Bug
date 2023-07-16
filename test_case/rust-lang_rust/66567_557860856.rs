plain
2019-11-24T05:45:38.9626410Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-24T05:45:39.7167472Z ##[command]git config gc.auto 0
2019-11-24T05:45:39.7174626Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-24T05:45:39.7179623Z ##[command]git config --get-all http.proxy
2019-11-24T05:45:39.7186215Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66567/merge:refs/remotes/pull/66567/merge
---
2019-11-24T05:50:46.6278256Z Done!
2019-11-24T05:50:46.6278486Z some tidy checks failed
2019-11-24T05:50:46.6282678Z 
2019-11-24T05:50:46.6282847Z 
2019-11-24T05:50:46.6284442Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-24T05:50:46.6284800Z 
2019-11-24T05:50:46.6284940Z 
2019-11-24T05:50:46.6293354Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-24T05:50:46.6293574Z Build completed unsuccessfully in 0:01:13
2019-11-24T05:50:46.6293574Z Build completed unsuccessfully in 0:01:13
2019-11-24T05:50:46.6343069Z == clock drift check ==
2019-11-24T05:50:46.6351346Z   local time: Sun Nov 24 05:50:46 UTC 2019
2019-11-24T05:50:46.9118994Z   network time: Sun, 24 Nov 2019 05:50:46 GMT
2019-11-24T05:50:46.9119681Z == end clock drift check ==
2019-11-24T05:50:48.3279044Z 
2019-11-24T05:50:48.3397719Z ##[error]Bash exited with code '1'.
2019-11-24T05:50:48.3428278Z ##[section]Starting: Checkout
2019-11-24T05:50:48.3430095Z ==============================================================================
2019-11-24T05:50:48.3430145Z Task         : Get sources
2019-11-24T05:50:48.3430205Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
