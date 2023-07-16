plain
2020-01-09T11:35:52.2515066Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-09T11:35:52.2614722Z ##[command]git config gc.auto 0
2020-01-09T11:35:52.2701430Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-09T11:35:52.2756253Z ##[command]git config --get-all http.proxy
2020-01-09T11:35:52.2914983Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68051/merge:refs/remotes/pull/68051/merge
