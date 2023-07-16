plain
2019-09-01T02:50:26.2561895Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-01T02:50:26.8027751Z ##[command]git config gc.auto 0
2019-09-01T02:50:26.8032716Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-01T02:50:26.8037694Z ##[command]git config --get-all http.proxy
2019-09-01T02:50:26.8040304Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64051/merge:refs/remotes/pull/64051/merge
