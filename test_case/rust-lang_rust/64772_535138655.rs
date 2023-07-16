plain
2019-09-25T17:28:48.1067982Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-25T17:28:48.1343699Z ##[command]git config gc.auto 0
2019-09-25T17:28:48.1415086Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-25T17:28:48.1485611Z ##[command]git config --get-all http.proxy
2019-09-25T17:28:48.1708696Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64772/merge:refs/remotes/pull/64772/merge
---
2019-09-25T17:38:31.5806194Z     Checking rustc_resolve v0.0.0 (/checkout/src/librustc_resolve)
2019-09-25T17:38:34.4443024Z     Checking rustc_plugin_impl v0.0.0 (/checkout/src/librustc_plugin)
2019-09-25T17:38:34.7425663Z     Checking rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
2019-09-25T17:38:35.3664650Z     Checking rustc_codegen_ssa v0.0.0 (/checkout/src/librustc_codegen_ssa)
2019-09-25T17:38:35.6703481Z error[E0433]: failed to resolve: use of undeclared type or module `mpsc`
2019-09-25T17:38:35.6703991Z    --> src/librustc_codegen_ssa/back/write.rs:381:51
2019-09-25T17:38:35.6704281Z     |
2019-09-25T17:38:35.6704629Z 381 |     let (coordinator_send, coordinator_receive) = mpsc::channel();
2019-09-25T17:38:35.6705003Z     |                                                   ^^^^ use of undeclared type or module `mpsc`
2019-09-25T17:38:35.7594192Z error: unused import: `std::sync::mpsc`
2019-09-25T17:38:35.7594573Z   --> src/librustc_codegen_ssa/base.rs:49:5
2019-09-25T17:38:35.7594988Z    |
2019-09-25T17:38:35.7595423Z 49 | use std::sync::mpsc;
---
2019-09-25T17:38:41.6995437Z == clock drift check ==
2019-09-25T17:38:41.7014841Z   local time: Wed Sep 25 17:38:41 UTC 2019
2019-09-25T17:38:41.7763805Z   network time: Wed, 25 Sep 2019 17:38:41 GMT
2019-09-25T17:38:41.7770204Z == end clock drift check ==
2019-09-25T17:38:42.9920311Z ##[error]Bash exited with code '1'.
2019-09-25T17:38:42.9959321Z ##[section]Starting: Checkout
2019-09-25T17:38:42.9961547Z ==============================================================================
2019-09-25T17:38:42.9961607Z Task         : Get sources
2019-09-25T17:38:42.9961679Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
