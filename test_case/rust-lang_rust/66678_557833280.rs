plain
2019-11-23T21:11:34.3005452Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-23T21:11:34.3200190Z ##[command]git config gc.auto 0
2019-11-23T21:11:34.3316130Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-23T21:11:34.3347612Z ##[command]git config --get-all http.proxy
2019-11-23T21:11:34.3500476Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66678/merge:refs/remotes/pull/66678/merge
