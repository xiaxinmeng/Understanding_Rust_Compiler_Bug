plain
2019-12-31T16:41:26.3849704Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-31T16:41:26.4074649Z ##[command]git config gc.auto 0
2019-12-31T16:41:26.4139845Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-31T16:41:26.4197114Z ##[command]git config --get-all http.proxy
2019-12-31T16:41:26.4353126Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66470/merge:refs/remotes/pull/66470/merge
