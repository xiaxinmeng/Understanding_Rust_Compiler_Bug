plain
2019-11-21T19:21:04.6356123Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-21T19:21:04.6376360Z ##[command]git config gc.auto 0
2019-11-21T19:21:04.6382915Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-21T19:21:04.6385176Z ##[command]git config --get-all http.proxy
2019-11-21T19:21:04.6393302Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66614/merge:refs/remotes/pull/66614/merge
