plain
2019-09-01T11:13:48.8524332Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-01T11:13:48.8720632Z ##[command]git config gc.auto 0
2019-09-01T11:13:48.8798078Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-01T11:13:48.8853842Z ##[command]git config --get-all http.proxy
2019-09-01T11:13:48.8993615Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64060/merge:refs/remotes/pull/64060/merge
