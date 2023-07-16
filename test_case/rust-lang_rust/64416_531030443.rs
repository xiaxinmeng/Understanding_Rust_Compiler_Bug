plain
2019-09-12T22:05:30.4414447Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-12T22:05:30.4605984Z ##[command]git config gc.auto 0
2019-09-12T22:05:30.4685794Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-12T22:05:30.4762415Z ##[command]git config --get-all http.proxy
2019-09-12T22:05:30.4902419Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64416/merge:refs/remotes/pull/64416/merge
---
2019-09-12T22:15:05.6118839Z     Checking rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2019-09-12T22:15:06.4068533Z error[E0432]: unresolved import `crate::borrow_check::nll::region_infer::error_reporting::outlives_suggestion`
2019-09-12T22:15:06.4068969Z   --> src/librustc_mir/borrow_check/nll/region_infer/mod.rs:14:36
2019-09-12T22:15:06.4069259Z    |
2019-09-12T22:15:06.4069577Z 14 |     region_infer::error_reporting::outlives_suggestion::OutlivesSuggestionBuilder,
2019-09-12T22:15:06.4070298Z    |                                    ^^^^^^^^^^^^^^^^^^^ could not find `outlives_suggestion` in `error_reporting`
2019-09-12T22:15:06.7763970Z error[E0425]: cannot find value `errors_buffer` in this scope
2019-09-12T22:15:06.7764570Z     --> src/librustc_mir/borrow_check/nll/region_infer/mod.rs:1497:19
2019-09-12T22:15:06.7765450Z      |
2019-09-12T22:15:06.7765450Z      |
2019-09-12T22:15:06.7766179Z 1497 |         db.buffer(errors_buffer);
2019-09-12T22:15:06.7766701Z 
2019-09-12T22:15:09.4752006Z     Checking rustc_lint v0.0.0 (/checkout/src/librustc_lint)
2019-09-12T22:15:11.1921556Z     Checking rustc_passes v0.0.0 (/checkout/src/librustc_passes)
2019-09-12T22:15:11.9414770Z     Checking rustc_codegen_utils v0.0.0 (/checkout/src/librustc_codegen_utils)
---
2019-09-12T22:15:16.2336918Z == clock drift check ==
2019-09-12T22:15:16.2358406Z   local time: Thu Sep 12 22:15:16 UTC 2019
2019-09-12T22:15:16.5142957Z   network time: Thu, 12 Sep 2019 22:15:16 GMT
2019-09-12T22:15:16.5152159Z == end clock drift check ==
2019-09-12T22:15:17.6804657Z ##[error]Bash exited with code '1'.
2019-09-12T22:15:17.6838725Z ##[section]Starting: Checkout
2019-09-12T22:15:17.6840645Z ==============================================================================
2019-09-12T22:15:17.6840699Z Task         : Get sources
2019-09-12T22:15:17.6840748Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
