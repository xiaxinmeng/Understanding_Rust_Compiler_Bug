plain
2019-11-20T22:53:13.2651882Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-20T22:53:13.2825759Z ##[command]git config gc.auto 0
2019-11-20T22:53:13.2890771Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-20T22:53:13.2948634Z ##[command]git config --get-all http.proxy
2019-11-20T22:53:13.3087535Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66592/merge:refs/remotes/pull/66592/merge
