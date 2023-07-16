plain
2019-09-22T10:44:46.6574294Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-22T10:44:46.6776859Z ##[command]git config gc.auto 0
2019-09-22T10:44:46.6848050Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-22T10:44:46.6905598Z ##[command]git config --get-all http.proxy
2019-09-22T10:44:46.7050793Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64683/merge:refs/remotes/pull/64683/merge
