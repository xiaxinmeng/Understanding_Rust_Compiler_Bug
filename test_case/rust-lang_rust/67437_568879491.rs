plain
2019-12-25T09:50:15.1629300Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-25T09:50:15.1842117Z ##[command]git config gc.auto 0
2019-12-25T09:50:15.1917857Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-25T09:50:15.1989881Z ##[command]git config --get-all http.proxy
2019-12-25T09:50:15.2131571Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67437/merge:refs/remotes/pull/67437/merge
