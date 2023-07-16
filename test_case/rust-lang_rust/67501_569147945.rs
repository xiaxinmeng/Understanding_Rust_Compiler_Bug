plain
2019-12-26T22:55:28.0677629Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-26T22:55:28.0877646Z ##[command]git config gc.auto 0
2019-12-26T22:55:28.0958304Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-26T22:55:28.1003323Z ##[command]git config --get-all http.proxy
2019-12-26T22:55:28.1157041Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67501/merge:refs/remotes/pull/67501/merge
