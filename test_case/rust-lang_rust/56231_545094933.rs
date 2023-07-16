plain
2019-10-22T18:29:54.0225119Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-22T18:29:54.7042857Z ##[command]git config gc.auto 0
2019-10-22T18:29:54.7047339Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-22T18:29:54.7050000Z ##[command]git config --get-all http.proxy
2019-10-22T18:29:54.7053364Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/56231/merge:refs/remotes/pull/56231/merge
