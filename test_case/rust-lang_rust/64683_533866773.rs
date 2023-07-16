plain
2019-09-22T09:56:26.9589278Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-22T09:56:26.9780309Z ##[command]git config gc.auto 0
2019-09-22T09:56:26.9861098Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-22T09:56:26.9912585Z ##[command]git config --get-all http.proxy
2019-09-22T09:56:27.0046185Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64683/merge:refs/remotes/pull/64683/merge
