plain
2019-12-27T12:16:40.1657685Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-27T12:16:40.1847159Z ##[command]git config gc.auto 0
2019-12-27T12:16:40.1932645Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-27T12:16:40.1989239Z ##[command]git config --get-all http.proxy
2019-12-27T12:16:40.2135796Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67597/merge:refs/remotes/pull/67597/merge
