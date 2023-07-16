plain
2019-10-01T18:14:27.8945340Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-01T18:14:27.9141374Z ##[command]git config gc.auto 0
2019-10-01T18:14:27.9196426Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-01T18:14:27.9269594Z ##[command]git config --get-all http.proxy
2019-10-01T18:14:27.9409891Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64963/merge:refs/remotes/pull/64963/merge
