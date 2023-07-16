plain
2019-09-15T11:57:51.5936542Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-15T11:57:51.6150284Z ##[command]git config gc.auto 0
2019-09-15T11:57:51.6225275Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-15T11:57:51.6283449Z ##[command]git config --get-all http.proxy
2019-09-15T11:57:51.6417118Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64485/merge:refs/remotes/pull/64485/merge
---
2019-09-15T14:02:00.1524899Z    |
2019-09-15T14:02:00.1525349Z 76 |     rustc_driver::report_ices_to_stderr_if_any(|| {
2019-09-15T14:02:00.1525853Z    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in `rustc_driver`
2019-09-15T14:02:00.1529941Z 
2019-09-15T14:02:00.1594011Z warning: use of deprecated item 'rustc_plugin': import this through `rustc_driver::plugin` instead to make TLS work correctly. See ***/issues/62717
2019-09-15T14:02:00.1595002Z   |
2019-09-15T14:02:00.1595421Z 7 | extern crate rustc_plugin;
2019-09-15T14:02:00.1595814Z   | ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-15T14:02:00.1596818Z   |
---
2019-09-15T14:11:22.4375426Z    Compiling rustc-workspace-hack v1.0.0 (/checkout/src/tools/rustc-workspace-hack)
2019-09-15T14:12:57.6478538Z error[E0425]: cannot find function `report_ices_to_stderr_if_any` in crate `rustc_driver`
2019-09-15T14:12:57.6480512Z    --> src/tools/miri/src/bin/miri.rs:214:32
2019-09-15T14:12:57.6481269Z     |
2019-09-15T14:12:57.6481940Z 214 |     let result = rustc_driver::report_ices_to_stderr_if_any(move || {
2019-09-15T14:12:57.6483009Z 
2019-09-15T14:12:57.8741872Z error: aborting due to previous error
2019-09-15T14:12:57.8742989Z 
2019-09-15T14:12:57.8747702Z For more information about this error, try `rustc --explain E0425`.
---
2019-09-15T14:13:04.2531054Z Verifying status of clippy-driver...
2019-09-15T14:13:04.2544471Z Verifying status of miri...
2019-09-15T14:13:04.2557682Z This PR updated 'src/tools/miri', verifying if status is 'test-pass'...
2019-09-15T14:13:04.2568403Z 
2019-09-15T14:13:04.2568860Z ⚠️ We detected that this PR updated 'miri', but its tests failed.
2019-09-15T14:13:04.2568893Z 
2019-09-15T14:13:04.2569131Z If you do intend to update 'miri', please check the error messages above and
2019-09-15T14:13:04.2569173Z commit another update.
2019-09-15T14:13:04.2569195Z 
2019-09-15T14:13:04.2569407Z If you do NOT intend to update 'miri', please ensure you did not accidentally
2019-09-15T14:13:04.2569607Z change the submodule at 'src/tools/miri'. You may ask your reviewer for the
2019-09-15T14:13:04.2569645Z proper steps.
2019-09-15T14:13:04.2590969Z   local time: Sun Sep 15 14:13:04 UTC 2019
2019-09-15T14:13:04.5446545Z   network time: Sun, 15 Sep 2019 14:13:04 GMT
2019-09-15T14:13:04.5452846Z == end clock drift check ==
2019-09-15T14:13:04.5452846Z == end clock drift check ==
2019-09-15T14:13:05.4694640Z ##[error]Bash exited with code '3'.
2019-09-15T14:13:05.4727296Z ##[section]Starting: Checkout
2019-09-15T14:13:05.4728857Z ==============================================================================
2019-09-15T14:13:05.4728917Z Task         : Get sources
2019-09-15T14:13:05.4728953Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
