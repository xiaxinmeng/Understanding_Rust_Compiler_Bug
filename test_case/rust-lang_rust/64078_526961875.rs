plain
2019-09-01T21:32:38.5570687Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-01T21:32:39.2193758Z ##[command]git config gc.auto 0
2019-09-01T21:32:39.2198293Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-01T21:32:39.2202023Z ##[command]git config --get-all http.proxy
2019-09-01T21:32:39.2208206Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64078/merge:refs/remotes/pull/64078/merge
