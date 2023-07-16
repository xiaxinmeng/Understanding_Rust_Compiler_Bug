plain
2020-01-15T14:09:50.6790543Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-15T14:09:50.7142443Z ##[command]git config gc.auto 0
2020-01-15T14:09:50.7227410Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-15T14:09:50.7292201Z ##[command]git config --get-all http.proxy
2020-01-15T14:09:50.7444617Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68093/merge:refs/remotes/pull/68093/merge
