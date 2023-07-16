plain
2019-10-28T00:32:18.3461312Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-28T00:32:18.3677633Z ##[command]git config gc.auto 0
2019-10-28T00:32:18.3729214Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-28T00:32:18.3774118Z ##[command]git config --get-all http.proxy
2019-10-28T00:32:18.3911061Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65881/merge:refs/remotes/pull/65881/merge
---
2019-10-28T00:56:29.8569601Z    Compiling rustc_codegen_ssa v0.0.0 (/checkout/src/librustc_codegen_ssa)
2019-10-28T00:56:30.1990154Z error[E0425]: cannot find value `arg_count` in this scope
2019-10-28T00:56:30.1992137Z    --> src/librustc_codegen_ssa/mir/block.rs:571:45
2019-10-28T00:56:30.1992393Z     |
2019-10-28T00:56:30.1992668Z 571 |         let mut llargs = Vec::with_capacity(arg_count);
2019-10-28T00:56:30.1996382Z 
2019-10-28T00:56:32.4400514Z error: aborting due to previous error
2019-10-28T00:56:32.4405136Z 
2019-10-28T00:56:32.4434180Z For more information about this error, try `rustc --explain E0425`.
---
2019-10-28T00:56:43.1136232Z   local time: Mon Oct 28 00:56:43 UTC 2019
2019-10-28T00:56:43.2325625Z   network time: Mon, 28 Oct 2019 00:56:43 GMT
2019-10-28T00:56:43.2328032Z == end clock drift check ==
2019-10-28T00:56:44.0834481Z 
2019-10-28T00:56:44.0920383Z ##[error]Bash exited with code '1'.
2019-10-28T00:56:44.0986916Z ##[section]Starting: Checkout
2019-10-28T00:56:44.0989727Z ==============================================================================
2019-10-28T00:56:44.0989788Z Task         : Get sources
2019-10-28T00:56:44.0989836Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
