plain
2019-12-28T04:10:46.4089749Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-28T04:10:46.9722235Z ##[command]git config gc.auto 0
2019-12-28T04:10:46.9727918Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-28T04:10:46.9730830Z ##[command]git config --get-all http.proxy
2019-12-28T04:10:46.9735074Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67671/merge:refs/remotes/pull/67671/merge
