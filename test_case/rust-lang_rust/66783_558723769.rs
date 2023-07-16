plain
2019-11-26T16:58:09.8793223Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-26T16:58:09.8806368Z ##[command]git config gc.auto 0
2019-11-26T16:58:09.8809357Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-26T16:58:09.8812267Z ##[command]git config --get-all http.proxy
2019-11-26T16:58:09.8815052Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66783/merge:refs/remotes/pull/66783/merge
