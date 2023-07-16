plain
2020-01-10T06:03:19.4780981Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-10T06:03:19.4792474Z ##[command]git config gc.auto 0
2020-01-10T06:03:19.4795074Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-10T06:03:19.4797234Z ##[command]git config --get-all http.proxy
2020-01-10T06:03:19.4800483Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68082/merge:refs/remotes/pull/68082/merge
