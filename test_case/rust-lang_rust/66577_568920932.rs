plain
2019-12-25T18:06:58.5036800Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-25T18:06:58.5258474Z ##[command]git config gc.auto 0
2019-12-25T18:06:58.5335884Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-25T18:06:58.5405204Z ##[command]git config --get-all http.proxy
2019-12-25T18:06:58.5557666Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66577/merge:refs/remotes/pull/66577/merge
