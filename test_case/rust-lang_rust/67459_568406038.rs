plain
2019-12-23T08:27:43.7166642Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-23T08:27:43.7184217Z ##[command]git config gc.auto 0
2019-12-23T08:27:43.7189166Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-23T08:27:43.7193837Z ##[command]git config --get-all http.proxy
2019-12-23T08:27:43.7196241Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67459/merge:refs/remotes/pull/67459/merge
