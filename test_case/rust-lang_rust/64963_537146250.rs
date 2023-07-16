plain
2019-10-01T17:35:36.8922214Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-01T17:35:36.9112505Z ##[command]git config gc.auto 0
2019-10-01T17:35:37.8577865Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-01T17:35:37.8581477Z ##[command]git config --get-all http.proxy
2019-10-01T17:35:37.8586132Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64963/merge:refs/remotes/pull/64963/merge
