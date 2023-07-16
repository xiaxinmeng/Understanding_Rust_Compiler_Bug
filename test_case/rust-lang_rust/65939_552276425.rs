plain
2019-11-11T02:52:56.0866610Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-11T02:52:56.1048056Z ##[command]git config gc.auto 0
2019-11-11T02:52:56.1126532Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-11T02:52:56.1189263Z ##[command]git config --get-all http.proxy
2019-11-11T02:52:56.1337778Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65939/merge:refs/remotes/pull/65939/merge
