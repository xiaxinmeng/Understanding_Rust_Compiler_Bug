plain
2019-12-31T08:46:37.4294910Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-31T08:46:37.4503077Z ##[command]git config gc.auto 0
2019-12-31T08:46:37.4562202Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-31T08:46:37.4615674Z ##[command]git config --get-all http.proxy
2019-12-31T08:46:37.4774022Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67749/merge:refs/remotes/pull/67749/merge
