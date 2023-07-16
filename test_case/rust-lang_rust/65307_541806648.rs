plain
2019-10-14T15:20:01.4693733Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-14T15:20:01.4805184Z ##[command]git config gc.auto 0
2019-10-14T15:20:01.4877095Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-14T15:20:01.4939132Z ##[command]git config --get-all http.proxy
2019-10-14T15:20:01.5086379Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65307/merge:refs/remotes/pull/65307/merge
