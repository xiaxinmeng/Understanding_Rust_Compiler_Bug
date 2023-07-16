plain
2019-08-30T09:25:14.7427393Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-30T09:25:15.5603440Z ##[command]git config gc.auto 0
2019-08-30T09:25:15.5606423Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-30T09:25:15.5608974Z ##[command]git config --get-all http.proxy
2019-08-30T09:25:15.5611869Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64018/merge:refs/remotes/pull/64018/merge
