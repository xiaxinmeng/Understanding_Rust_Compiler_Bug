plain
2019-10-22T18:20:30.9636625Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-22T18:20:30.9854292Z ##[command]git config gc.auto 0
2019-10-22T18:20:30.9925141Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-22T18:20:30.9978119Z ##[command]git config --get-all http.proxy
2019-10-22T18:20:31.6159844Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65703/merge:refs/remotes/pull/65703/merge
