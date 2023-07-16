plain
2020-01-14T22:22:19.6976335Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-14T22:22:19.7234912Z ##[command]git config gc.auto 0
2020-01-14T22:22:19.7329061Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-14T22:22:19.7401192Z ##[command]git config --get-all http.proxy
2020-01-14T22:22:19.7559058Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67398/merge:refs/remotes/pull/67398/merge
