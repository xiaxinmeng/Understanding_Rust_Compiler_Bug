plain
2019-08-24T13:55:28.0979814Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-24T13:55:28.1521116Z ##[command]git config gc.auto 0
2019-08-24T13:55:28.1598788Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-24T13:55:28.1646638Z ##[command]git config --get-all http.proxy
2019-08-24T13:55:28.1770920Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62975/merge:refs/remotes/pull/62975/merge
---
2019-08-24T13:56:01.4491180Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-24T13:56:01.4491229Z 
2019-08-24T13:56:01.4491440Z   git checkout -b <new-branch-name>
2019-08-24T13:56:01.4491469Z 
2019-08-24T13:56:01.4491517Z HEAD is now at 4c2750cf9 Merge 1954174ec1ba7acabfdf8359f55ff5d77094005a into 478464570e60523adc6d303577d1782229ca1f93
2019-08-24T13:56:01.4663022Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-24T13:56:01.4665484Z ==============================================================================
2019-08-24T13:56:01.4666701Z Task         : Bash
2019-08-24T13:56:01.4666755Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-24T14:03:43.5348761Z     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
2019-08-24T14:03:54.5957343Z error[E0308]: mismatched types
2019-08-24T14:03:54.5957749Z   --> src/librustc/hir/lowering/item.rs:48:34
2019-08-24T14:03:54.5958142Z    |
2019-08-24T14:03:54.5958470Z 48 |         self.lctx.modules.insert(n, hir::ModuleItems {
2019-08-24T14:03:54.5958975Z    |                                  ^ expected struct `hir::HirId`, found struct `syntax::ast::NodeId`
2019-08-24T14:03:54.5959430Z    = note: expected type `hir::HirId`
2019-08-24T14:03:54.5968598Z               found type `syntax::ast::NodeId`
2019-08-24T14:03:54.5968637Z 
2019-08-24T14:03:54.6817109Z error[E0308]: mismatched types
2019-08-24T14:03:54.6817109Z error[E0308]: mismatched types
2019-08-24T14:03:54.6817428Z   --> src/librustc/hir/lowering/item.rs:55:36
2019-08-24T14:03:54.6817830Z    |
2019-08-24T14:03:54.6818263Z 55 |         self.lctx.current_module = n;
2019-08-24T14:03:54.6818599Z    |                                    ^ expected struct `hir::HirId`, found struct `syntax::ast::NodeId`
2019-08-24T14:03:54.6819579Z    = note: expected type `hir::HirId`
2019-08-24T14:03:54.6820047Z               found type `syntax::ast::NodeId`
2019-08-24T14:03:54.6820096Z 
2019-08-24T14:04:08.2234468Z error: aborting due to 2 previous errors
---
2019-08-24T14:04:08.4112081Z == clock drift check ==
2019-08-24T14:04:08.4124400Z   local time: Sat Aug 24 14:04:08 UTC 2019
2019-08-24T14:04:08.6166054Z   network time: Sat, 24 Aug 2019 14:04:08 GMT
2019-08-24T14:04:08.6166940Z == end clock drift check ==
2019-08-24T14:04:09.6322781Z ##[error]Bash exited with code '1'.
2019-08-24T14:04:09.6353667Z ##[section]Starting: Checkout
2019-08-24T14:04:09.6355410Z ==============================================================================
2019-08-24T14:04:09.6355467Z Task         : Get sources
2019-08-24T14:04:09.6355519Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
