plain
2019-10-22T18:58:31.2175573Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-22T18:58:31.2383798Z ##[command]git config gc.auto 0
2019-10-22T18:58:31.2467510Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-22T18:58:31.2525257Z ##[command]git config --get-all http.proxy
2019-10-22T18:58:31.2647637Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65640/merge:refs/remotes/pull/65640/merge
