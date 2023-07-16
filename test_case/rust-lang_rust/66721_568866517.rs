plain
2019-12-25T08:21:03.3493590Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-25T08:21:03.3717879Z ##[command]git config gc.auto 0
2019-12-25T08:21:03.3808891Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-25T08:21:03.3873511Z ##[command]git config --get-all http.proxy
2019-12-25T08:21:03.4038220Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66721/merge:refs/remotes/pull/66721/merge
