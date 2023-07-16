plain
2019-10-01T07:30:06.6648612Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-01T07:30:06.6892238Z ##[command]git config gc.auto 0
2019-10-01T07:30:06.6976788Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-01T07:30:06.7041077Z ##[command]git config --get-all http.proxy
2019-10-01T07:30:06.7199317Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64739/merge:refs/remotes/pull/64739/merge
---
2019-10-01T07:39:07.5371849Z == clock drift check ==
2019-10-01T07:39:07.5392479Z   local time: Tue Oct  1 07:39:07 UTC 2019
2019-10-01T07:39:07.7239403Z   network time: Tue, 01 Oct 2019 07:39:07 GMT
2019-10-01T07:39:07.7246295Z == end clock drift check ==
2019-10-01T07:39:09.0160322Z ##[error]Bash exited with code '1'.
2019-10-01T07:39:09.0240700Z ##[section]Starting: Checkout
2019-10-01T07:39:09.0242841Z ==============================================================================
2019-10-01T07:39:09.0242926Z Task         : Get sources
2019-10-01T07:39:09.0242979Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
