plain
2019-11-30T14:07:14.0801174Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-30T14:07:14.9860057Z ##[command]git config gc.auto 0
2019-11-30T14:07:14.9862532Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-30T14:07:14.9864014Z ##[command]git config --get-all http.proxy
2019-11-30T14:07:14.9866590Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66904/merge:refs/remotes/pull/66904/merge
---
2019-11-30T14:12:41.1400072Z Done!
2019-11-30T14:12:41.1400106Z some tidy checks failed
2019-11-30T14:12:41.1400152Z 
2019-11-30T14:12:41.1400174Z 
2019-11-30T14:12:41.1400965Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-30T14:12:41.1401069Z 
2019-11-30T14:12:41.1401092Z 
2019-11-30T14:12:41.1404289Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-30T14:12:41.1404624Z Build completed unsuccessfully in 0:01:17
2019-11-30T14:12:41.1404624Z Build completed unsuccessfully in 0:01:17
2019-11-30T14:12:41.1439390Z == clock drift check ==
2019-11-30T14:12:41.1447087Z   local time: Sat Nov 30 14:12:41 UTC 2019
2019-11-30T14:12:41.4332978Z   network time: Sat, 30 Nov 2019 14:12:41 GMT
2019-11-30T14:12:41.4334536Z == end clock drift check ==
2019-11-30T14:12:42.8309409Z 
2019-11-30T14:12:42.8411755Z ##[error]Bash exited with code '1'.
2019-11-30T14:12:42.8433344Z ##[section]Starting: Checkout
2019-11-30T14:12:42.8434734Z ==============================================================================
2019-11-30T14:12:42.8434777Z Task         : Get sources
2019-11-30T14:12:42.8434813Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
