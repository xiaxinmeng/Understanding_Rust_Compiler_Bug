plain
2020-01-08T15:30:01.8901442Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-08T15:30:01.8912237Z ##[command]git config gc.auto 0
2020-01-08T15:30:01.8915732Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-08T15:30:01.8917650Z ##[command]git config --get-all http.proxy
2020-01-08T15:30:01.8920212Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68011/merge:refs/remotes/pull/68011/merge
