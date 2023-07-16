plain
2019-10-28T14:19:37.5979223Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-28T14:19:37.6159536Z ##[command]git config gc.auto 0
2019-10-28T14:19:37.6222193Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-28T14:19:38.4148309Z ##[command]git config --get-all http.proxy
2019-10-28T14:19:38.4156695Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65881/merge:refs/remotes/pull/65881/merge
---
2019-10-28T14:43:22.0313782Z    Compiling rustc_codegen_ssa v0.0.0 (/checkout/src/librustc_codegen_ssa)
2019-10-28T14:43:22.3304016Z error[E0425]: cannot find value `arg_count` in this scope
2019-10-28T14:43:22.3304705Z    --> src/librustc_codegen_ssa/mir/block.rs:571:45
2019-10-28T14:43:22.3305488Z     |
2019-10-28T14:43:22.3306088Z 571 |         let mut llargs = Vec::with_capacity(arg_count);
2019-10-28T14:43:22.3307182Z 
2019-10-28T14:43:24.5891052Z error: aborting due to previous error
2019-10-28T14:43:24.5892658Z 
2019-10-28T14:43:24.5898206Z For more information about this error, try `rustc --explain E0425`.
---
2019-10-28T14:43:46.9364928Z   local time: Mon Oct 28 14:43:46 UTC 2019
2019-10-28T14:43:47.2087881Z   network time: Mon, 28 Oct 2019 14:43:47 GMT
2019-10-28T14:43:47.2090846Z == end clock drift check ==
2019-10-28T14:43:48.5026252Z 
2019-10-28T14:43:48.5120716Z ##[error]Bash exited with code '1'.
2019-10-28T14:43:48.5158021Z ##[section]Starting: Checkout
2019-10-28T14:43:48.5160099Z ==============================================================================
2019-10-28T14:43:48.5160161Z Task         : Get sources
2019-10-28T14:43:48.5160199Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
