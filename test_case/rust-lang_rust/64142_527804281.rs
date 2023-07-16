plain
2019-09-04T08:38:35.7079512Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-04T08:38:35.7269511Z ##[command]git config gc.auto 0
2019-09-04T08:38:35.7351988Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-04T08:38:35.7407488Z ##[command]git config --get-all http.proxy
2019-09-04T08:38:35.7554243Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64142/merge:refs/remotes/pull/64142/merge
