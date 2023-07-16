plain
2019-08-31T21:28:30.3085990Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-31T21:28:30.3259276Z ##[command]git config gc.auto 0
2019-08-31T21:28:30.3326275Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-31T21:28:30.3389092Z ##[command]git config --get-all http.proxy
2019-08-31T21:28:30.9282681Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63257/merge:refs/remotes/pull/63257/merge
