plain
2019-11-23T20:52:06.4600144Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-23T20:52:07.2108499Z ##[command]git config gc.auto 0
2019-11-23T20:52:07.2113036Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-23T20:52:07.2114763Z ##[command]git config --get-all http.proxy
2019-11-23T20:52:07.2118784Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66677/merge:refs/remotes/pull/66677/merge
