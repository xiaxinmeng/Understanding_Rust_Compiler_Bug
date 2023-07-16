plain
2019-12-17T19:56:55.2315532Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-17T19:56:56.1562171Z ##[command]git config gc.auto 0
2019-12-17T19:56:56.1566571Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-17T19:56:56.1569269Z ##[command]git config --get-all http.proxy
2019-12-17T19:56:56.1575181Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66254/merge:refs/remotes/pull/66254/merge
---
2019-12-17T20:02:12.3163052Z   local time: Tue Dec 17 20:02:12 UTC 2019
2019-12-17T20:02:12.5950495Z   network time: Tue, 17 Dec 2019 20:02:12 GMT
2019-12-17T20:02:12.5954233Z == end clock drift check ==
2019-12-17T20:02:27.5891711Z 
2019-12-17T20:02:27.6003893Z ##[error]Bash exited with code '1'.
2019-12-17T20:02:27.6038584Z ##[section]Starting: Checkout
2019-12-17T20:02:27.6040603Z ==============================================================================
2019-12-17T20:02:27.6040660Z Task         : Get sources
2019-12-17T20:02:27.6040710Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
