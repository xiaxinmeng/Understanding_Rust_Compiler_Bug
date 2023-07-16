plain
2020-01-10T01:38:33.8023589Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-10T01:38:33.8038521Z ##[command]git config gc.auto 0
2020-01-10T01:38:33.8047677Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-10T01:38:33.8052450Z ##[command]git config --get-all http.proxy
2020-01-10T01:38:33.8056422Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68077/merge:refs/remotes/pull/68077/merge
