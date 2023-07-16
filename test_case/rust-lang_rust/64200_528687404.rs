plain
2019-09-06T00:57:26.8269972Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-06T00:57:26.8448965Z ##[command]git config gc.auto 0
2019-09-06T00:57:26.8496798Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-06T00:57:26.8550825Z ##[command]git config --get-all http.proxy
2019-09-06T00:57:26.8677935Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64200/merge:refs/remotes/pull/64200/merge
---
2019-09-06T02:57:41.3317069Z Verifying status of rustfmt...
2019-09-06T02:57:41.3329862Z Verifying status of clippy-driver...
2019-09-06T02:57:41.3344150Z This PR updated 'src/tools/clippy', verifying if status is 'test-pass'...
2019-09-06T02:57:41.3354560Z 
2019-09-06T02:57:41.3355184Z ⚠️ We detected that this PR updated 'clippy-driver', but its tests failed.
2019-09-06T02:57:41.3355349Z 
2019-09-06T02:57:41.3355868Z If you do intend to update 'clippy-driver', please check the error messages above and
2019-09-06T02:57:41.3355916Z commit another update.
2019-09-06T02:57:41.3355938Z 
2019-09-06T02:57:41.3356571Z If you do NOT intend to update 'clippy-driver', please ensure you did not accidentally
2019-09-06T02:57:41.3356887Z change the submodule at 'src/tools/clippy'. You may ask your reviewer for the
2019-09-06T02:57:41.3356939Z proper steps.
2019-09-06T02:57:41.3384243Z   local time: Fri Sep  6 02:57:41 UTC 2019
2019-09-06T02:57:41.4241501Z   network time: Fri, 06 Sep 2019 02:57:41 GMT
2019-09-06T02:57:41.4247069Z == end clock drift check ==
2019-09-06T02:57:41.4247069Z == end clock drift check ==
2019-09-06T02:57:42.9844363Z ##[error]Bash exited with code '3'.
2019-09-06T02:57:42.9886234Z ##[section]Starting: Checkout
2019-09-06T02:57:42.9889019Z ==============================================================================
2019-09-06T02:57:42.9889082Z Task         : Get sources
2019-09-06T02:57:42.9889133Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
