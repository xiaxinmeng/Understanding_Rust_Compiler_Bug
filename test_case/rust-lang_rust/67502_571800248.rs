plain
2020-01-07T22:00:16.5661419Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-07T22:00:16.5741504Z ##[command]git config gc.auto 0
2020-01-07T22:00:16.5837269Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-07T22:00:16.5894602Z ##[command]git config --get-all http.proxy
2020-01-07T22:00:16.6041708Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67502/merge:refs/remotes/pull/67502/merge
