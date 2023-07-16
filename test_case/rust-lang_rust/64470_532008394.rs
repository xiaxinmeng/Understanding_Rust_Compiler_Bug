plain
2019-09-17T00:23:31.0456700Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-17T00:23:31.0655056Z ##[command]git config gc.auto 0
2019-09-17T00:23:31.0740488Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-17T00:23:31.0796593Z ##[command]git config --get-all http.proxy
2019-09-17T00:23:31.0945867Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64470/merge:refs/remotes/pull/64470/merge
