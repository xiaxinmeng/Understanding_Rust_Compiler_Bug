plain
2020-01-09T14:10:16.3361336Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-09T14:10:16.3449680Z ##[command]git config gc.auto 0
2020-01-09T14:10:16.3526257Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-09T14:10:16.3578131Z ##[command]git config --get-all http.proxy
2020-01-09T14:10:16.3729383Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67516/merge:refs/remotes/pull/67516/merge
