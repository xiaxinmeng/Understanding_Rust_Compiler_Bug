plain
2020-01-13T23:12:47.6235854Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-13T23:12:47.6251007Z ##[command]git config gc.auto 0
2020-01-13T23:12:47.6255669Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-13T23:12:47.6260435Z ##[command]git config --get-all http.proxy
2020-01-13T23:12:47.6266184Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67940/merge:refs/remotes/pull/67940/merge
---
2020-01-14T01:28:05.1297148Z 
2020-01-14T01:28:05.1297451Z If you do intend to update 'rustc-guide', please check the error messages above and
2020-01-14T01:28:05.1297503Z commit another update.
2020-01-14T01:28:05.1297549Z 
2020-01-14T01:28:05.1297990Z If you do NOT intend to update 'rustc-guide', please ensure you did not accidentally
2020-01-14T01:28:05.1298280Z change the submodule at 'src/doc/rustc-guide'. You may ask your reviewer for the
2020-01-14T01:28:05.1298362Z proper steps.
2020-01-14T01:28:05.1308447Z Build completed unsuccessfully in 0:00:01
2020-01-14T01:28:05.1427695Z == clock drift check ==
2020-01-14T01:28:05.1444591Z   local time: Tue Jan 14 01:28:05 UTC 2020
2020-01-14T01:28:05.4336913Z   network time: Tue, 14 Jan 2020 01:28:05 GMT
2020-01-14T01:28:05.4336913Z   network time: Tue, 14 Jan 2020 01:28:05 GMT
2020-01-14T01:28:05.4337403Z == end clock drift check ==
2020-01-14T01:28:06.0328504Z 
2020-01-14T01:28:06.0443841Z ##[error]Bash exited with code '1'.
2020-01-14T01:28:06.0487898Z ##[section]Starting: Checkout
2020-01-14T01:28:06.0489748Z ==============================================================================
2020-01-14T01:28:06.0489828Z Task         : Get sources
2020-01-14T01:28:06.0489883Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
