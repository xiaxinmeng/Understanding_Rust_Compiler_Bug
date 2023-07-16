plain
2019-11-20T22:45:47.7700590Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-20T22:45:48.3641871Z ##[command]git config gc.auto 0
2019-11-20T22:45:48.3644989Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-20T22:45:48.3647613Z ##[command]git config --get-all http.proxy
2019-11-20T22:45:48.3653700Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66590/merge:refs/remotes/pull/66590/merge
