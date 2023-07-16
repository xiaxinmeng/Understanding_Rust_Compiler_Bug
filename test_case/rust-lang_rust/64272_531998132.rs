plain
2019-09-16T21:13:18.5712299Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-16T21:13:19.3611971Z ##[command]git config gc.auto 0
2019-09-16T21:13:19.3616994Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-16T21:13:19.3620865Z ##[command]git config --get-all http.proxy
2019-09-16T21:13:19.3624821Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64272/merge:refs/remotes/pull/64272/merge
---
2019-09-16T23:07:07.4402372Z  Caused By: "https://rust-lang.github.io/chalk/doc/chalk_engine/context/index.html" returned 404 Not Found
2019-09-16T23:07:07.4402814Z  Caused By: "https://rust-lang.github.io/chalk/doc/chalk_parse/index.html" returned 404 Not Found
2019-09-16T23:07:07.4403265Z  Caused By: "https://rust-lang.github.io/chalk/doc/chalk/index.html" returned 404 Not Found
2019-09-16T23:07:07.4403696Z  Caused By: "https://rust-lang.github.io/chalk/doc/chalki/index.html" returned 404 Not Found
2019-09-16T23:07:07.4404732Z  Caused By: There was an error while fetching "http://fitzgeraldnick.com/2018/12/13/rust-raps.html", http://fitzgeraldnick.com/2018/12/13/rust-raps.html: error trying to connect: Connection refused (os error 111)
2019-09-16T23:07:07.4405204Z 
2019-09-16T23:07:07.4405898Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rustbook" "linkcheck" "/checkout/src/doc/rustc-guide"
2019-09-16T23:07:07.4406171Z expected success, got: exit code: 101
2019-09-16T23:07:07.4406304Z 
---
2019-09-16T23:26:25.1853039Z    |
2019-09-16T23:26:25.1853517Z 76 |     rustc_driver::report_ices_to_stderr_if_any(|| {
2019-09-16T23:26:25.1854019Z    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in `rustc_driver`
2019-09-16T23:26:25.1859410Z 
2019-09-16T23:26:25.1933834Z warning: use of deprecated item 'rustc_plugin': import this through `rustc_driver::plugin` instead to make TLS work correctly. See ***/issues/62717
2019-09-16T23:26:25.1935362Z   |
2019-09-16T23:26:25.1935634Z 7 | extern crate rustc_plugin;
2019-09-16T23:26:25.1935913Z   | ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-16T23:26:25.1936127Z   |
---
2019-09-16T23:36:38.8692660Z Verifying status of edition-guide...
2019-09-16T23:36:38.8705350Z Verifying status of rls...
2019-09-16T23:36:38.8719262Z This PR updated 'src/tools/rls', verifying if status is 'test-pass'...
2019-09-16T23:36:38.8733394Z 
2019-09-16T23:36:38.8734497Z ⚠️ We detected that this PR updated 'rls', but its tests failed.
2019-09-16T23:36:38.8734600Z 
2019-09-16T23:36:38.8735114Z If you do intend to update 'rls', please check the error messages above and
2019-09-16T23:36:38.8735189Z commit another update.
2019-09-16T23:36:38.8735218Z 
2019-09-16T23:36:38.8735507Z If you do NOT intend to update 'rls', please ensure you did not accidentally
2019-09-16T23:36:38.8735742Z change the submodule at 'src/tools/rls'. You may ask your reviewer for the
2019-09-16T23:36:38.8735790Z proper steps.
2019-09-16T23:36:38.8753531Z   local time: Mon Sep 16 23:36:38 UTC 2019
2019-09-16T23:36:39.0361137Z   network time: Mon, 16 Sep 2019 23:36:39 GMT
2019-09-16T23:36:39.0364466Z == end clock drift check ==
2019-09-16T23:36:39.0364466Z == end clock drift check ==
2019-09-16T23:36:40.3282519Z ##[error]Bash exited with code '3'.
2019-09-16T23:36:40.3327555Z ##[section]Starting: Checkout
2019-09-16T23:36:40.3329809Z ==============================================================================
2019-09-16T23:36:40.3329888Z Task         : Get sources
2019-09-16T23:36:40.3329941Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
