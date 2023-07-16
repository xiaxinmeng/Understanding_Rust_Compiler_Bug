plain
2019-11-20T23:16:09.2923003Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-20T23:16:09.2940000Z ##[command]git config gc.auto 0
2019-11-20T23:16:09.2942450Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-20T23:16:09.2946586Z ##[command]git config --get-all http.proxy
2019-11-20T23:16:09.2950512Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66590/merge:refs/remotes/pull/66590/merge
