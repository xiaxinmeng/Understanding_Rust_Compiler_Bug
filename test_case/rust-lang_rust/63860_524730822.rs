plain
2019-08-26T06:06:21.7197050Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-26T06:06:21.7414607Z ##[command]git config gc.auto 0
2019-08-26T06:06:21.7509332Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-26T06:06:21.7569327Z ##[command]git config --get-all http.proxy
2019-08-26T06:06:21.7725312Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63860/merge:refs/remotes/pull/63860/merge
---
2019-08-26T06:06:57.3664025Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-26T06:06:57.3664063Z 
2019-08-26T06:06:57.3664306Z   git checkout -b <new-branch-name>
2019-08-26T06:06:57.3664359Z 
2019-08-26T06:06:57.3664413Z HEAD is now at b3f6c2df8 Merge de55d91717a604eae4f65044d5e49a9b5b333ec6 into 4c58535d09d1261d21569df0036b974811544256
2019-08-26T06:06:57.3849301Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-26T06:06:57.3852102Z ==============================================================================
2019-08-26T06:06:57.3852176Z Task         : Bash
2019-08-26T06:06:57.3852220Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-26T06:15:53.6458212Z     Checking rustc_passes v0.0.0 (/checkout/src/librustc_passes)
2019-08-26T06:15:54.3996511Z     Checking rustc_ast_borrowck v0.0.0 (/checkout/src/librustc_ast_borrowck)
2019-08-26T06:15:55.6616385Z     Checking rustc_codegen_utils v0.0.0 (/checkout/src/librustc_codegen_utils)
2019-08-26T06:15:56.2878255Z     Checking rustc_resolve v0.0.0 (/checkout/src/librustc_resolve)
2019-08-26T06:15:56.4916316Z error[E0277]: `&rustc_data_structures::bit_set::BitSet<rustc::mir::Local>` is not an iterator
2019-08-26T06:15:56.4916881Z     --> src/librustc_mir/transform/qualify_consts.rs:1066:22
2019-08-26T06:15:56.4917603Z 1066 |         for local in borrowed_locals.get() {
2019-08-26T06:15:56.4917603Z 1066 |         for local in borrowed_locals.get() {
2019-08-26T06:15:56.4918068Z      |                      ^^^^^^^^^^^^^^^^^^^^^ `&rustc_data_structures::bit_set::BitSet<rustc::mir::Local>` is not an iterator
2019-08-26T06:15:56.4918332Z      |
2019-08-26T06:15:56.4918726Z      = help: the trait `std::iter::Iterator` is not implemented for `&rustc_data_structures::bit_set::BitSet<rustc::mir::Local>`
2019-08-26T06:15:56.4919162Z 
2019-08-26T06:15:58.8955289Z     Checking rustc_plugin_impl v0.0.0 (/checkout/src/librustc_plugin)
2019-08-26T06:15:59.1889750Z     Checking rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
2019-08-26T06:16:00.6592199Z     Checking rustc_codegen_ssa v0.0.0 (/checkout/src/librustc_codegen_ssa)
---
2019-08-26T06:16:04.1081623Z == clock drift check ==
2019-08-26T06:16:04.1103394Z   local time: Mon Aug 26 06:16:04 UTC 2019
2019-08-26T06:16:04.2604271Z   network time: Mon, 26 Aug 2019 06:16:04 GMT
2019-08-26T06:16:04.2609181Z == end clock drift check ==
2019-08-26T06:16:05.5714609Z ##[error]Bash exited with code '1'.
2019-08-26T06:16:05.5745669Z ##[section]Starting: Checkout
2019-08-26T06:16:05.5747560Z ==============================================================================
2019-08-26T06:16:05.5747624Z Task         : Get sources
2019-08-26T06:16:05.5747665Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
