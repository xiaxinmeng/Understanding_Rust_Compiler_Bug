plain
2019-12-26T08:33:20.2513309Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-26T08:33:20.2715961Z ##[command]git config gc.auto 0
2019-12-26T08:33:21.1923120Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-26T08:33:21.1925713Z ##[command]git config --get-all http.proxy
2019-12-26T08:33:21.1928409Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67622/merge:refs/remotes/pull/67622/merge
