plain
2019-12-10T15:14:13.6061120Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-10T15:14:13.6074525Z ##[command]git config gc.auto 0
2019-12-10T15:14:13.6076936Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-10T15:14:13.6080106Z ##[command]git config --get-all http.proxy
2019-12-10T15:14:13.6083643Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67198/merge:refs/remotes/pull/67198/merge
---
2019-12-10T17:02:13.5599660Z Verifying status of edition-guide...
2019-12-10T17:02:13.5610394Z Verifying status of rls...
2019-12-10T17:02:13.5622194Z This PR updated 'src/tools/rls', verifying if status is 'test-pass'...
2019-12-10T17:02:13.5630863Z 
2019-12-10T17:02:13.5632185Z ⚠️ We detected that this PR updated 'rls', but its tests failed.
2019-12-10T17:02:13.5632252Z 
2019-12-10T17:02:13.5632496Z If you do intend to update 'rls', please check the error messages above and
2019-12-10T17:02:13.5632539Z commit another update.
2019-12-10T17:02:13.5632561Z 
2019-12-10T17:02:13.5632755Z If you do NOT intend to update 'rls', please ensure you did not accidentally
2019-12-10T17:02:13.5632966Z change the submodule at 'src/tools/rls'. You may ask your reviewer for the
2019-12-10T17:02:13.5633004Z proper steps.
2019-12-10T17:02:13.5648122Z   local time: Tue Dec 10 17:02:13 UTC 2019
2019-12-10T17:02:13.8358946Z   network time: Tue, 10 Dec 2019 17:02:13 GMT
2019-12-10T17:02:13.8362543Z == end clock drift check ==
2019-12-10T17:02:14.3552518Z 
2019-12-10T17:02:14.3552518Z 
2019-12-10T17:02:14.3632950Z ##[error]Bash exited with code '3'.
2019-12-10T17:02:14.3660964Z ##[section]Starting: Checkout
2019-12-10T17:02:14.3662424Z ==============================================================================
2019-12-10T17:02:14.3662466Z Task         : Get sources
2019-12-10T17:02:14.3662519Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
