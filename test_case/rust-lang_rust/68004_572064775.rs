plain
2020-01-08T13:36:41.7959026Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-08T13:36:41.8219578Z ##[command]git config gc.auto 0
2020-01-08T13:36:41.8223714Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-08T13:36:41.8225585Z ##[command]git config --get-all http.proxy
2020-01-08T13:36:41.8319420Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68004/merge:refs/remotes/pull/68004/merge
