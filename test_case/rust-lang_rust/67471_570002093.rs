plain
2019-12-31T22:34:04.8387293Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-31T22:34:05.6919091Z ##[command]git config gc.auto 0
2019-12-31T22:34:05.6924634Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-31T22:34:05.6926816Z ##[command]git config --get-all http.proxy
2019-12-31T22:34:05.6933149Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67471/merge:refs/remotes/pull/67471/merge
