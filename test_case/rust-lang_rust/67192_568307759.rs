plain
2019-12-22T20:32:11.3010034Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-22T20:32:11.3215074Z ##[command]git config gc.auto 0
2019-12-22T20:32:11.3272146Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-22T20:32:11.3327302Z ##[command]git config --get-all http.proxy
2019-12-22T20:32:11.3442729Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67192/merge:refs/remotes/pull/67192/merge
