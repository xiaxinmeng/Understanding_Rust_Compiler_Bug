plain
2019-11-06T02:10:45.2470522Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-06T02:10:45.2634266Z ##[command]git config gc.auto 0
2019-11-06T02:10:45.2710713Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-06T02:10:45.2772120Z ##[command]git config --get-all http.proxy
2019-11-06T02:10:45.2909229Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66050/merge:refs/remotes/pull/66050/merge
---
2019-11-06T02:20:35.0030407Z   local time: Wed Nov  6 02:20:35 UTC 2019
2019-11-06T02:20:35.0889911Z   network time: Wed, 06 Nov 2019 02:20:35 GMT
2019-11-06T02:20:35.0890696Z == end clock drift check ==
2019-11-06T02:20:36.5407499Z 
2019-11-06T02:20:36.5524171Z ##[error]Bash exited with code '1'.
2019-11-06T02:20:36.5550292Z ##[section]Starting: Checkout
2019-11-06T02:20:36.5552492Z ==============================================================================
2019-11-06T02:20:36.5552549Z Task         : Get sources
2019-11-06T02:20:36.5552616Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
