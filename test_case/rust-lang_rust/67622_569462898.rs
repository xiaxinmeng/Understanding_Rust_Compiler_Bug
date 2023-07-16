plain
2019-12-29T00:17:21.0001552Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-29T00:17:21.0184891Z ##[command]git config gc.auto 0
2019-12-29T00:17:21.0250653Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-29T00:17:21.0304286Z ##[command]git config --get-all http.proxy
2019-12-29T00:17:21.0443004Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67622/merge:refs/remotes/pull/67622/merge
