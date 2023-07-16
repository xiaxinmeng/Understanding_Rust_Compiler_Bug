plain
2019-10-31T02:24:30.5993660Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-31T02:24:30.6178154Z ##[command]git config gc.auto 0
2019-10-31T02:24:30.6249888Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-31T02:24:30.6320128Z ##[command]git config --get-all http.proxy
2019-10-31T02:24:30.6447553Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65939/merge:refs/remotes/pull/65939/merge
