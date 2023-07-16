plain
2019-10-22T20:23:01.2949724Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-22T20:23:01.3174449Z ##[command]git config gc.auto 0
2019-10-22T20:23:01.3241444Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-22T20:23:01.3298086Z ##[command]git config --get-all http.proxy
2019-10-22T20:23:01.3481292Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65705/merge:refs/remotes/pull/65705/merge
