plain
2019-12-31T06:52:12.8216044Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-31T06:52:12.8397746Z ##[command]git config gc.auto 0
2019-12-31T06:52:12.8469638Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-31T06:52:12.8524814Z ##[command]git config --get-all http.proxy
2019-12-31T06:52:12.8658826Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67727/merge:refs/remotes/pull/67727/merge
