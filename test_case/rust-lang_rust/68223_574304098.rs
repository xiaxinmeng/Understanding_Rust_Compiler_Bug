plain
2020-01-14T18:09:01.8103983Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-14T18:09:01.8191464Z ##[command]git config gc.auto 0
2020-01-14T18:09:01.8267174Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-14T18:09:01.8335209Z ##[command]git config --get-all http.proxy
2020-01-14T18:09:01.8469877Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68223/merge:refs/remotes/pull/68223/merge
