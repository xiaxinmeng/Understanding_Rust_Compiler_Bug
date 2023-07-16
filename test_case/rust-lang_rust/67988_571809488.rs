plain
2020-01-07T22:30:31.2451187Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-07T22:30:31.2551204Z ##[command]git config gc.auto 0
2020-01-07T22:30:31.2616492Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-07T22:30:31.2669825Z ##[command]git config --get-all http.proxy
2020-01-07T22:30:31.2806839Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67988/merge:refs/remotes/pull/67988/merge
