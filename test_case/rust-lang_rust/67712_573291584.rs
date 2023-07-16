plain
2020-01-11T06:56:30.7921823Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-11T06:56:30.7934193Z ##[command]git config gc.auto 0
2020-01-11T06:56:30.7936954Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-11T06:56:30.7939275Z ##[command]git config --get-all http.proxy
2020-01-11T06:56:30.7942291Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67712/merge:refs/remotes/pull/67712/merge
---
2020-01-11T07:33:27.8609261Z   local time: Sat Jan 11 07:33:27 UTC 2020
2020-01-11T07:33:28.4021757Z   network time: Sat, 11 Jan 2020 07:33:28 GMT
2020-01-11T07:33:28.4025633Z == end clock drift check ==
2020-01-11T07:33:29.9495171Z 
2020-01-11T07:33:29.9620917Z ##[error]Bash exited with code '1'.
2020-01-11T07:33:29.9660542Z ##[section]Starting: Checkout
2020-01-11T07:33:29.9662466Z ==============================================================================
2020-01-11T07:33:29.9662526Z Task         : Get sources
2020-01-11T07:33:29.9662578Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
