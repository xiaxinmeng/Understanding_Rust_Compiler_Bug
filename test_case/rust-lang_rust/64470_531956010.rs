plain
2019-09-16T20:36:49.9393182Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-16T20:36:49.9616316Z ##[command]git config gc.auto 0
2019-09-16T20:36:49.9697732Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-16T20:36:50.0130339Z ##[command]git config --get-all http.proxy
2019-09-16T20:36:50.0308492Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64470/merge:refs/remotes/pull/64470/merge
---
2019-09-16T20:46:42.8225416Z     Checking rustc_ast_borrowck v0.0.0 (/checkout/src/librustc_ast_borrowck)
2019-09-16T20:46:43.3431569Z error: unused variable: `location`
2019-09-16T20:46:43.3431975Z    --> src/librustc_mir/dataflow/impls/indirect_mutation.rs:138:44
2019-09-16T20:46:43.3432278Z     |
2019-09-16T20:46:43.3432594Z 138 |         if let mir::TerminatorKind::Drop { location, .. }
2019-09-16T20:46:43.3432915Z     |                                            ^^^^^^^^
2019-09-16T20:46:43.3434095Z 139 |              | mir::TerminatorKind::DropAndReplace { location, .. } = &terminator.kind {
2019-09-16T20:46:43.3434906Z     |
2019-09-16T20:46:43.3435186Z     = note: `-D unused-variables` implied by `-D warnings`
2019-09-16T20:46:43.3435445Z help: try ignoring the field
2019-09-16T20:46:43.3435658Z     |
2019-09-16T20:46:43.3435658Z     |
2019-09-16T20:46:43.3436398Z 138 |         if let mir::TerminatorKind::Drop { location: _location: _, .. }
2019-09-16T20:46:43.3447296Z 
2019-09-16T20:46:44.2317365Z     Checking rustc_codegen_utils v0.0.0 (/checkout/src/librustc_codegen_utils)
2019-09-16T20:46:44.8638659Z     Checking rustc_plugin_impl v0.0.0 (/checkout/src/librustc_plugin)
2019-09-16T20:46:45.1620365Z     Checking rustc_resolve v0.0.0 (/checkout/src/librustc_resolve)
---
2019-09-16T20:46:53.4755268Z == clock drift check ==
2019-09-16T20:46:53.4783092Z   local time: Mon Sep 16 20:46:53 UTC 2019
2019-09-16T20:46:54.0123614Z   network time: Mon, 16 Sep 2019 20:46:54 GMT
2019-09-16T20:46:54.0130788Z == end clock drift check ==
2019-09-16T20:46:55.2051905Z ##[error]Bash exited with code '1'.
2019-09-16T20:46:55.2091382Z ##[section]Starting: Checkout
2019-09-16T20:46:55.2093142Z ==============================================================================
2019-09-16T20:46:55.2093216Z Task         : Get sources
2019-09-16T20:46:55.2093264Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
