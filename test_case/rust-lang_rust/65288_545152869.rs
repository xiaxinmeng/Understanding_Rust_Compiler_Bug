plain
2019-10-22T20:25:14.3832767Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-22T20:25:14.4036731Z ##[command]git config gc.auto 0
2019-10-22T20:25:14.4130980Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-22T20:25:14.4193074Z ##[command]git config --get-all http.proxy
2019-10-22T20:25:14.4337959Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65288/merge:refs/remotes/pull/65288/merge
