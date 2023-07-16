plain
2019-12-23T14:51:04.7615589Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-23T14:51:05.6838741Z ##[command]git config gc.auto 0
2019-12-23T14:51:05.6841187Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-23T14:51:05.6843311Z ##[command]git config --get-all http.proxy
2019-12-23T14:51:05.6847318Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66498/merge:refs/remotes/pull/66498/merge
