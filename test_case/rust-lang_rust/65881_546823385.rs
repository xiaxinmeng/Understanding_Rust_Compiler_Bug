plain
2019-10-28T06:58:57.0639450Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-28T06:58:57.5835059Z ##[command]git config gc.auto 0
2019-10-28T06:58:57.5840920Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-28T06:58:57.5845012Z ##[command]git config --get-all http.proxy
2019-10-28T06:58:57.5849383Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65881/merge:refs/remotes/pull/65881/merge
---
2019-10-28T07:24:40.1148347Z    Compiling rustc_codegen_ssa v0.0.0 (/checkout/src/librustc_codegen_ssa)
2019-10-28T07:24:40.4486299Z error[E0425]: cannot find value `arg_count` in this scope
2019-10-28T07:24:40.4486774Z    --> src/librustc_codegen_ssa/mir/block.rs:571:45
2019-10-28T07:24:40.4487015Z     |
2019-10-28T07:24:40.4487325Z 571 |         let mut llargs = Vec::with_capacity(arg_count);
2019-10-28T07:24:40.4494157Z 
2019-10-28T07:24:42.8999526Z error: aborting due to previous error
2019-10-28T07:24:42.9005313Z 
2019-10-28T07:24:42.9017256Z For more information about this error, try `rustc --explain E0425`.
---
2019-10-28T07:24:52.1903945Z   local time: Mon Oct 28 07:24:52 UTC 2019
2019-10-28T07:24:52.3410138Z   network time: Mon, 28 Oct 2019 07:24:52 GMT
2019-10-28T07:24:52.3412658Z == end clock drift check ==
2019-10-28T07:24:53.2236628Z 
2019-10-28T07:24:53.2365491Z ##[error]Bash exited with code '1'.
2019-10-28T07:24:53.2406338Z ##[section]Starting: Checkout
2019-10-28T07:24:53.2408155Z ==============================================================================
2019-10-28T07:24:53.2408233Z Task         : Get sources
2019-10-28T07:24:53.2408281Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
