plain
2019-11-21T19:19:53.0417846Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-21T19:19:53.5764506Z ##[command]git config gc.auto 0
2019-11-21T19:19:53.5771596Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-21T19:19:53.5777283Z ##[command]git config --get-all http.proxy
2019-11-21T19:19:53.5781187Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66613/merge:refs/remotes/pull/66613/merge
