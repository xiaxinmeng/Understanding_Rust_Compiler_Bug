plain
2020-01-09T21:48:34.0841955Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-09T21:48:34.1126760Z ##[command]git config gc.auto 0
2020-01-09T21:48:34.1197343Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-09T21:48:34.1254908Z ##[command]git config --get-all http.proxy
2020-01-09T21:48:34.1403954Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68071/merge:refs/remotes/pull/68071/merge
