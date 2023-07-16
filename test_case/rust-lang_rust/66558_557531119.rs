plain
2019-11-22T13:21:47.3804367Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-22T13:21:47.3978178Z ##[command]git config gc.auto 0
2019-11-22T13:21:47.4050700Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-22T13:21:47.4103018Z ##[command]git config --get-all http.proxy
2019-11-22T13:21:47.4241491Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66558/merge:refs/remotes/pull/66558/merge
