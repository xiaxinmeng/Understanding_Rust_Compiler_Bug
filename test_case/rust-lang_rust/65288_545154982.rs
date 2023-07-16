plain
2019-10-22T21:03:12.4584021Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-22T21:03:12.4777101Z ##[command]git config gc.auto 0
2019-10-22T21:03:12.4831596Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-22T21:03:12.4897268Z ##[command]git config --get-all http.proxy
2019-10-22T21:03:12.5017785Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65288/merge:refs/remotes/pull/65288/merge
