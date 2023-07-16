plain
2019-11-30T13:10:10.4499105Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-30T13:10:10.4697804Z ##[command]git config gc.auto 0
2019-11-30T13:10:10.4769953Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-30T13:10:10.4828302Z ##[command]git config --get-all http.proxy
2019-11-30T13:10:10.4977543Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66878/merge:refs/remotes/pull/66878/merge
---
2019-11-30T13:20:38.6088443Z   local time: Sat Nov 30 13:20:38 UTC 2019
2019-11-30T13:20:38.8853811Z   network time: Sat, 30 Nov 2019 13:20:38 GMT
2019-11-30T13:20:38.8854821Z == end clock drift check ==
2019-11-30T13:20:39.9259140Z 
2019-11-30T13:20:39.9378610Z ##[error]Bash exited with code '1'.
2019-11-30T13:20:39.9406795Z ##[section]Starting: Checkout
2019-11-30T13:20:39.9408856Z ==============================================================================
2019-11-30T13:20:39.9408933Z Task         : Get sources
2019-11-30T13:20:39.9408982Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
