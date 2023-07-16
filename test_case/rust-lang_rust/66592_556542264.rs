plain
2019-11-20T23:21:05.4338227Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-20T23:21:05.4519712Z ##[command]git config gc.auto 0
2019-11-20T23:21:05.4590656Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-20T23:21:05.4624185Z ##[command]git config --get-all http.proxy
2019-11-20T23:21:05.4821057Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66592/merge:refs/remotes/pull/66592/merge
