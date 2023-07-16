plain
2019-08-26T17:03:08.7892500Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-26T17:03:08.8073873Z ##[command]git config gc.auto 0
2019-08-26T17:03:08.8138538Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-26T17:03:08.8194401Z ##[command]git config --get-all http.proxy
2019-08-26T17:03:08.8348416Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63860/merge:refs/remotes/pull/63860/merge
---
2019-08-26T17:03:44.3594508Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-26T17:03:44.3594536Z 
2019-08-26T17:03:44.3594732Z   git checkout -b <new-branch-name>
2019-08-26T17:03:44.3594759Z 
2019-08-26T17:03:44.3594990Z HEAD is now at 9a1cb903f Merge c970931f5b00ee12766df8dc4747a0a1f3856cb4 into 555d7a2fd6165b614cfc01136d8e3f5c465a1582
2019-08-26T17:03:44.3752224Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-26T17:03:44.3755347Z ==============================================================================
2019-08-26T17:03:44.3755431Z Task         : Bash
2019-08-26T17:03:44.3755477Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-26T17:12:31.4655013Z     Checking rustc_lint v0.0.0 (/checkout/src/librustc_lint)
2019-08-26T17:12:33.0116823Z     Checking rustc_passes v0.0.0 (/checkout/src/librustc_passes)
2019-08-26T17:12:33.6883093Z     Checking rustc_ast_borrowck v0.0.0 (/checkout/src/librustc_ast_borrowck)
2019-08-26T17:12:34.8390239Z     Checking rustc_codegen_utils v0.0.0 (/checkout/src/librustc_codegen_utils)
2019-08-26T17:12:35.1017539Z error[E0277]: `&rustc_data_structures::bit_set::BitSet<rustc::mir::Local>` is not an iterator
2019-08-26T17:12:35.1018108Z     --> src/librustc_mir/transform/qualify_consts.rs:1066:22
2019-08-26T17:12:35.1019103Z 1066 |         for local in borrowed_locals.get() {
2019-08-26T17:12:35.1019103Z 1066 |         for local in borrowed_locals.get() {
2019-08-26T17:12:35.1019513Z      |                      ^^^^^^^^^^^^^^^^^^^^^ `&rustc_data_structures::bit_set::BitSet<rustc::mir::Local>` is not an iterator
2019-08-26T17:12:35.1019746Z      |
2019-08-26T17:12:35.1020306Z      = help: the trait `std::iter::Iterator` is not implemented for `&rustc_data_structures::bit_set::BitSet<rustc::mir::Local>`
2019-08-26T17:12:35.1020653Z 
2019-08-26T17:12:35.4195729Z     Checking rustc_resolve v0.0.0 (/checkout/src/librustc_resolve)
2019-08-26T17:12:37.8757774Z     Checking rustc_plugin_impl v0.0.0 (/checkout/src/librustc_plugin)
2019-08-26T17:12:38.1346465Z     Checking rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
---
2019-08-26T17:12:38.6835615Z == clock drift check ==
2019-08-26T17:12:38.6852495Z   local time: Mon Aug 26 17:12:38 UTC 2019
2019-08-26T17:12:38.8354423Z   network time: Mon, 26 Aug 2019 17:12:38 GMT
2019-08-26T17:12:38.8358370Z == end clock drift check ==
2019-08-26T17:12:40.1152216Z ##[error]Bash exited with code '1'.
2019-08-26T17:12:40.1223113Z ##[section]Starting: Checkout
2019-08-26T17:12:40.1224696Z ==============================================================================
2019-08-26T17:12:40.1224750Z Task         : Get sources
2019-08-26T17:12:40.1224813Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
