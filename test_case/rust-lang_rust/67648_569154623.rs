plain
2019-12-26T22:30:39.3131960Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-26T22:30:39.3340195Z ##[command]git config gc.auto 0
2019-12-26T22:30:39.3411663Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-26T22:30:39.3476434Z ##[command]git config --get-all http.proxy
2019-12-26T22:30:40.2899574Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67648/merge:refs/remotes/pull/67648/merge
