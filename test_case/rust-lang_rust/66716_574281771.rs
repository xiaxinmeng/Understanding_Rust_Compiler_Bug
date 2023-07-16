plain
2020-01-14T16:59:00.3877795Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-14T16:59:00.3889384Z ##[command]git config gc.auto 0
2020-01-14T16:59:00.3892030Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-14T16:59:00.3895173Z ##[command]git config --get-all http.proxy
2020-01-14T16:59:00.3898060Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66716/merge:refs/remotes/pull/66716/merge
