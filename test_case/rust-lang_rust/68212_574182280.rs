plain
2020-01-14T13:27:00.1317442Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-14T13:27:00.1346766Z ##[command]git config gc.auto 0
2020-01-14T13:27:00.1349250Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-14T13:27:00.1351267Z ##[command]git config --get-all http.proxy
2020-01-14T13:27:00.1353661Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68212/merge:refs/remotes/pull/68212/merge
