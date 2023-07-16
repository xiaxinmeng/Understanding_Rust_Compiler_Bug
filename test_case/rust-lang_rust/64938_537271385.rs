plain
2019-10-01T23:17:20.9549312Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-01T23:17:20.9782648Z ##[command]git config gc.auto 0
2019-10-01T23:17:20.9852788Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-01T23:17:20.9922086Z ##[command]git config --get-all http.proxy
2019-10-01T23:17:21.0106086Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64938/merge:refs/remotes/pull/64938/merge
---
2019-10-01T23:22:04.2855994Z == clock drift check ==
2019-10-01T23:22:04.2876929Z   local time: Tue Oct  1 23:22:04 UTC 2019
2019-10-01T23:22:08.9005370Z   network time: Tue, 01 Oct 2019 23:22:08 GMT
2019-10-01T23:22:08.9006254Z == end clock drift check ==
2019-10-01T23:22:09.9234456Z ##[error]Bash exited with code '1'.
2019-10-01T23:22:09.9272734Z ##[section]Starting: Checkout
2019-10-01T23:22:09.9274667Z ==============================================================================
2019-10-01T23:22:09.9274724Z Task         : Get sources
2019-10-01T23:22:09.9274772Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
