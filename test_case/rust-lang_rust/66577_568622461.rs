plain
2019-12-24T01:20:02.3601948Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-24T01:20:02.3775248Z ##[command]git config gc.auto 0
2019-12-24T01:20:02.3859149Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-24T01:20:02.3917138Z ##[command]git config --get-all http.proxy
2019-12-24T01:20:02.4069016Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66577/merge:refs/remotes/pull/66577/merge
