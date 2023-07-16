plain
2020-01-07T13:41:46.0823479Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-07T13:41:46.0836736Z ##[command]git config gc.auto 0
2020-01-07T13:41:46.0841561Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-07T13:41:46.0845533Z ##[command]git config --get-all http.proxy
2020-01-07T13:41:46.0850576Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67972/merge:refs/remotes/pull/67972/merge
