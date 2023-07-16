plain
2019-09-10T01:22:54.8389032Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-10T01:22:54.8602114Z ##[command]git config gc.auto 0
2019-09-10T01:22:54.8687292Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-10T01:22:54.8758160Z ##[command]git config --get-all http.proxy
2019-09-10T01:22:54.8915987Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64328/merge:refs/remotes/pull/64328/merge
---
2019-09-10T01:32:38.5303952Z     Checking rustc_codegen_ssa v0.0.0 (/checkout/src/librustc_codegen_ssa)
2019-09-10T01:32:41.4686364Z     Checking rustc_save_analysis v0.0.0 (/checkout/src/librustc_save_analysis)
2019-09-10T01:32:42.5712868Z     Checking rustc_plugin v0.0.0 (/checkout/src/librustc_plugin/deprecated)
2019-09-10T01:32:43.3126420Z     Checking rustc_interface v0.0.0 (/checkout/src/librustc_interface)
2019-09-10T01:32:43.5725174Z error[E0425]: cannot find function `current_dll_path` in this scope
2019-09-10T01:32:43.5725512Z    --> src/librustc_interface/util.rs:319:16
2019-09-10T01:32:43.5725753Z     |
2019-09-10T01:32:43.5726019Z 319 |     let path = current_dll_path().and_then(|s| s.canonicalize().ok());
2019-09-10T01:32:43.5726361Z 
2019-09-10T01:32:43.6013399Z error[E0106]: missing lifetime specifier
2019-09-10T01:32:43.6013780Z    --> src/librustc_interface/util.rs:295:31
2019-09-10T01:32:43.6014074Z     |
2019-09-10T01:32:43.6014074Z     |
2019-09-10T01:32:43.6014351Z 295 | pub fn rustc_path() -> Option<&Path> {
2019-09-10T01:32:43.6014698Z     |                               ^ help: consider giving it a 'static lifetime: `&'static`
2019-09-10T01:32:43.6015258Z     = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
2019-09-10T01:32:43.6015302Z 
2019-09-10T01:32:43.7469925Z error: aborting due to 2 previous errors
2019-09-10T01:32:43.7470041Z 
---
2019-09-10T01:32:43.7771112Z == clock drift check ==
2019-09-10T01:32:43.7787515Z   local time: Tue Sep 10 01:32:43 UTC 2019
2019-09-10T01:32:43.9293971Z   network time: Tue, 10 Sep 2019 01:32:43 GMT
2019-09-10T01:32:43.9294131Z == end clock drift check ==
2019-09-10T01:32:45.2411455Z ##[error]Bash exited with code '1'.
2019-09-10T01:32:45.2450896Z ##[section]Starting: Checkout
2019-09-10T01:32:45.2452689Z ==============================================================================
2019-09-10T01:32:45.2452745Z Task         : Get sources
2019-09-10T01:32:45.2452791Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
