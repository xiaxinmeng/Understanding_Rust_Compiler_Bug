plain
2019-11-25T01:07:36.6141952Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-25T01:07:36.6360344Z ##[command]git config gc.auto 0
2019-11-25T01:07:36.6428082Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-25T01:07:36.6492954Z ##[command]git config --get-all http.proxy
2019-11-25T01:07:36.6623905Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66321/merge:refs/remotes/pull/66321/merge
