plain
2019-10-22T19:53:44.6812391Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-22T19:53:44.7000920Z ##[command]git config gc.auto 0
2019-10-22T19:53:44.7081513Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-22T19:53:44.7140390Z ##[command]git config --get-all http.proxy
2019-10-22T19:53:45.5040678Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65685/merge:refs/remotes/pull/65685/merge
