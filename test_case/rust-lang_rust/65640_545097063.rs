plain
2019-10-22T18:35:35.6170328Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-22T18:35:35.6367524Z ##[command]git config gc.auto 0
2019-10-22T18:35:35.6442986Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-22T18:35:35.6502740Z ##[command]git config --get-all http.proxy
2019-10-22T18:35:35.6639500Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65640/merge:refs/remotes/pull/65640/merge
