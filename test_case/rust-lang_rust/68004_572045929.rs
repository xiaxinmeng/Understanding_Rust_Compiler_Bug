plain
2020-01-08T12:56:26.3794362Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-08T12:56:26.3944156Z ##[command]git config gc.auto 0
2020-01-08T12:56:26.4025198Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-08T12:56:26.4084475Z ##[command]git config --get-all http.proxy
2020-01-08T12:56:26.4232419Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68004/merge:refs/remotes/pull/68004/merge
