plain
2019-09-15T14:32:44.6234569Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-15T14:32:44.6414235Z ##[command]git config gc.auto 0
2019-09-15T14:32:44.6478050Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-15T14:32:44.6522317Z ##[command]git config --get-all http.proxy
2019-09-15T14:32:44.6651490Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64488/merge:refs/remotes/pull/64488/merge
---
2019-09-15T16:35:52.9216457Z    |
2019-09-15T16:35:52.9217249Z 76 |     rustc_driver::report_ices_to_stderr_if_any(|| {
2019-09-15T16:35:52.9217716Z    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in `rustc_driver`
2019-09-15T16:35:52.9217878Z 
2019-09-15T16:35:52.9269181Z warning: use of deprecated item 'rustc_plugin': import this through `rustc_driver::plugin` instead to make TLS work correctly. See ***/issues/62717
2019-09-15T16:35:52.9270602Z   |
2019-09-15T16:35:52.9271035Z 7 | extern crate rustc_plugin;
2019-09-15T16:35:52.9271571Z   | ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-15T16:35:52.9272318Z   |
---
2019-09-15T16:45:13.7046463Z    Compiling rustc-workspace-hack v1.0.0 (/checkout/src/tools/rustc-workspace-hack)
2019-09-15T16:46:51.4417230Z error[E0425]: cannot find function `report_ices_to_stderr_if_any` in crate `rustc_driver`
2019-09-15T16:46:51.4419276Z    --> src/tools/miri/src/bin/miri.rs:214:32
2019-09-15T16:46:51.4419862Z     |
2019-09-15T16:46:51.4420444Z 214 |     let result = rustc_driver::report_ices_to_stderr_if_any(move || {
2019-09-15T16:46:51.4421694Z 
2019-09-15T16:46:51.6824289Z error: aborting due to previous error
2019-09-15T16:46:51.6825914Z 
2019-09-15T16:46:51.6831101Z For more information about this error, try `rustc --explain E0425`.
---
2019-09-15T16:46:57.9646827Z Verifying status of clippy-driver...
2019-09-15T16:46:57.9660249Z Verifying status of miri...
2019-09-15T16:46:57.9672140Z This PR updated 'src/tools/miri', verifying if status is 'test-pass'...
2019-09-15T16:46:57.9682277Z 
2019-09-15T16:46:57.9682848Z ⚠️ We detected that this PR updated 'miri', but its tests failed.
2019-09-15T16:46:57.9682904Z 
2019-09-15T16:46:57.9683683Z If you do intend to update 'miri', please check the error messages above and
2019-09-15T16:46:57.9683960Z commit another update.
2019-09-15T16:46:57.9684291Z 
2019-09-15T16:46:57.9684567Z If you do NOT intend to update 'miri', please ensure you did not accidentally
2019-09-15T16:46:57.9684995Z change the submodule at 'src/tools/miri'. You may ask your reviewer for the
2019-09-15T16:46:57.9685176Z proper steps.
2019-09-15T16:46:57.9705411Z   local time: Sun Sep 15 16:46:57 UTC 2019
2019-09-15T16:46:58.1460041Z   network time: Sun, 15 Sep 2019 16:46:58 GMT
2019-09-15T16:46:58.1464328Z == end clock drift check ==
2019-09-15T16:46:58.1464328Z == end clock drift check ==
2019-09-15T16:46:59.0081179Z ##[error]Bash exited with code '3'.
2019-09-15T16:46:59.0117253Z ##[section]Starting: Checkout
2019-09-15T16:46:59.0119587Z ==============================================================================
2019-09-15T16:46:59.0119647Z Task         : Get sources
2019-09-15T16:46:59.0119685Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
