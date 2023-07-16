plain
2019-10-19T12:09:38.0280871Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-19T12:09:38.0480073Z ##[command]git config gc.auto 0
2019-10-19T12:09:38.0577529Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-19T12:09:38.0656448Z ##[command]git config --get-all http.proxy
2019-10-19T12:09:38.0815032Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65168/merge:refs/remotes/pull/65168/merge
---
2019-10-19T12:15:47.1959327Z   local time: Sat Oct 19 12:15:47 UTC 2019
2019-10-19T12:15:47.3596689Z   network time: Sat, 19 Oct 2019 12:15:47 GMT
2019-10-19T12:15:47.3601001Z == end clock drift check ==
2019-10-19T12:15:50.0931355Z 
2019-10-19T12:15:50.1084358Z ##[error]Bash exited with code '1'.
2019-10-19T12:15:50.1141705Z ##[section]Starting: Checkout
2019-10-19T12:15:50.1145045Z ==============================================================================
2019-10-19T12:15:50.1145109Z Task         : Get sources
2019-10-19T12:15:50.1145155Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
