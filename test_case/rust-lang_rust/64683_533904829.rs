plain
2019-09-22T18:14:23.3018388Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-22T18:14:23.3189806Z ##[command]git config gc.auto 0
2019-09-22T18:14:23.3262275Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-22T18:14:23.3322917Z ##[command]git config --get-all http.proxy
2019-09-22T18:14:23.3455184Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64683/merge:refs/remotes/pull/64683/merge
