plain
2019-10-22T18:43:01.5589883Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-22T18:43:01.5780780Z ##[command]git config gc.auto 0
2019-10-22T18:43:01.5848013Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-22T18:43:01.5906218Z ##[command]git config --get-all http.proxy
2019-10-22T18:43:01.6047752Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65705/merge:refs/remotes/pull/65705/merge
