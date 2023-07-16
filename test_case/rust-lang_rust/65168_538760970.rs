plain
2019-10-06T15:53:42.6358971Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-06T15:53:42.6559281Z ##[command]git config gc.auto 0
2019-10-06T15:53:42.6657048Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-06T15:53:42.6740853Z ##[command]git config --get-all http.proxy
2019-10-06T15:53:42.6945186Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65168/merge:refs/remotes/pull/65168/merge
---
2019-10-06T16:00:29.4289881Z == clock drift check ==
2019-10-06T16:00:29.4307218Z   local time: Sun Oct  6 16:00:29 UTC 2019
2019-10-06T16:00:29.5837155Z   network time: Sun, 06 Oct 2019 16:00:29 GMT
2019-10-06T16:00:29.5840437Z == end clock drift check ==
2019-10-06T16:00:37.6262314Z ##[error]Bash exited with code '1'.
2019-10-06T16:00:37.6305104Z ##[section]Starting: Checkout
2019-10-06T16:00:37.6306766Z ==============================================================================
2019-10-06T16:00:37.6307323Z Task         : Get sources
2019-10-06T16:00:37.6307370Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
