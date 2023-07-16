plain
2020-01-03T01:39:32.1959938Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-03T01:39:32.7518807Z ##[command]git config gc.auto 0
2020-01-03T01:39:32.7520920Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-03T01:39:32.7524338Z ##[command]git config --get-all http.proxy
2020-01-03T01:39:32.7527028Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67800/merge:refs/remotes/pull/67800/merge
