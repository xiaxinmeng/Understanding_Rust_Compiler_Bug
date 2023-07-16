plain
2020-01-14T15:01:23.6866987Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-14T15:01:23.6877772Z ##[command]git config gc.auto 0
2020-01-14T15:01:23.6915524Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-14T15:01:23.6993860Z ##[command]git config --get-all http.proxy
2020-01-14T15:01:23.7191359Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68212/merge:refs/remotes/pull/68212/merge
