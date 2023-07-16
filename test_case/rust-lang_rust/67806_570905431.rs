plain
2020-01-05T11:44:04.1035855Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-05T11:44:04.8930664Z ##[command]git config gc.auto 0
2020-01-05T11:44:04.8936619Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-05T11:44:04.8940638Z ##[command]git config --get-all http.proxy
2020-01-05T11:44:04.8943769Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67806/merge:refs/remotes/pull/67806/merge
