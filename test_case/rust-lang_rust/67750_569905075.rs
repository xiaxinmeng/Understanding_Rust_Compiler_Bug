plain
2019-12-31T10:20:00.2310398Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-31T10:20:00.2326377Z ##[command]git config gc.auto 0
2019-12-31T10:20:00.2328637Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-31T10:20:00.2331373Z ##[command]git config --get-all http.proxy
2019-12-31T10:20:00.2334135Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67750/merge:refs/remotes/pull/67750/merge
---
2019-12-31T10:25:16.6202978Z   local time: Tue Dec 31 10:25:16 UTC 2019
2019-12-31T10:25:17.3124678Z   network time: Tue, 31 Dec 2019 10:25:17 GMT
2019-12-31T10:25:17.3125251Z == end clock drift check ==
2019-12-31T10:25:32.2680465Z 
2019-12-31T10:25:32.2782602Z ##[error]Bash exited with code '1'.
2019-12-31T10:25:32.2813466Z ##[section]Starting: Checkout
2019-12-31T10:25:32.2814987Z ==============================================================================
2019-12-31T10:25:32.2815056Z Task         : Get sources
2019-12-31T10:25:32.2815100Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
