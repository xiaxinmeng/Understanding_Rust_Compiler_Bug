plain
2019-09-22T10:24:18.3109467Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-22T10:24:18.3295560Z ##[command]git config gc.auto 0
2019-09-22T10:24:18.3368343Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-22T10:24:18.3417740Z ##[command]git config --get-all http.proxy
2019-09-22T10:24:18.3546999Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64683/merge:refs/remotes/pull/64683/merge
