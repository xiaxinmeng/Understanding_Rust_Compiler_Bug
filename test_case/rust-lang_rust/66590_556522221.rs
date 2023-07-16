plain
2019-11-20T23:01:31.6317146Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-20T23:01:31.6540035Z ##[command]git config gc.auto 0
2019-11-20T23:01:32.6180144Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-20T23:01:32.6187784Z ##[command]git config --get-all http.proxy
2019-11-20T23:01:32.6193695Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66590/merge:refs/remotes/pull/66590/merge
