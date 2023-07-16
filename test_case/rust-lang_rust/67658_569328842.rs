plain
2019-12-27T18:52:47.8624771Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-27T18:52:47.8636518Z ##[command]git config gc.auto 0
2019-12-27T18:52:47.8639285Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-27T18:52:47.8641105Z ##[command]git config --get-all http.proxy
2019-12-27T18:52:47.8643723Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67658/merge:refs/remotes/pull/67658/merge
