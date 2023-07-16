plain
2019-11-23T18:35:30.5404015Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-23T18:35:30.5606338Z ##[command]git config gc.auto 0
2019-11-23T18:35:30.5684458Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-23T18:35:30.5731404Z ##[command]git config --get-all http.proxy
2019-11-23T18:35:30.5865650Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66383/merge:refs/remotes/pull/66383/merge
---
2019-11-23T19:01:49.6123284Z   local time: Sat Nov 23 19:01:49 UTC 2019
2019-11-23T19:01:49.8930167Z   network time: Sat, 23 Nov 2019 19:01:49 GMT
2019-11-23T19:01:49.8930273Z == end clock drift check ==
2019-11-23T19:01:52.6931337Z 
2019-11-23T19:01:52.7029441Z ##[error]Bash exited with code '1'.
2019-11-23T19:01:52.7060386Z ##[section]Starting: Checkout
2019-11-23T19:01:52.7061931Z ==============================================================================
2019-11-23T19:01:52.7061980Z Task         : Get sources
2019-11-23T19:01:52.7062037Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
