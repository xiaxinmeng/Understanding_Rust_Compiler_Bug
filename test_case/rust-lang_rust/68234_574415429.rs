plain
2020-01-14T22:56:47.5748686Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-14T22:56:47.5844440Z ##[command]git config gc.auto 0
2020-01-14T22:56:47.5915144Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-14T22:56:47.5969615Z ##[command]git config --get-all http.proxy
2020-01-14T22:56:47.6097637Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68234/merge:refs/remotes/pull/68234/merge
---
2020-01-14T23:01:21.7860515Z   local time: Tue Jan 14 23:01:21 UTC 2020
2020-01-14T23:01:22.0794347Z   network time: Tue, 14 Jan 2020 23:01:22 GMT
2020-01-14T23:01:22.0795050Z == end clock drift check ==
2020-01-14T23:01:23.1927466Z 
2020-01-14T23:01:23.2020318Z ##[error]Bash exited with code '1'.
2020-01-14T23:01:23.2045319Z ##[section]Starting: Checkout
2020-01-14T23:01:23.2047604Z ==============================================================================
2020-01-14T23:01:23.2047660Z Task         : Get sources
2020-01-14T23:01:23.2047709Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
