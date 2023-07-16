plain
2019-08-27T15:54:41.1679959Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-27T15:54:41.1966025Z ##[command]git config gc.auto 0
2019-08-27T15:54:41.2041510Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-27T15:54:41.2084112Z ##[command]git config --get-all http.proxy
2019-08-27T15:54:41.2215033Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63948/merge:refs/remotes/pull/63948/merge
---
2019-08-27T15:55:17.7731580Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-27T15:55:17.7731614Z 
2019-08-27T15:55:17.7731802Z   git checkout -b <new-branch-name>
2019-08-27T15:55:17.7731827Z 
2019-08-27T15:55:17.7731885Z HEAD is now at d487c7846 Merge d783ca3f5ff4f84fdf45f21357b7892704ab1704 into 7e0afdad28c4d1154176df6d35a14e738ec311af
2019-08-27T15:55:17.7883547Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-27T15:55:17.7886356Z ==============================================================================
2019-08-27T15:55:17.7886416Z Task         : Bash
2019-08-27T15:55:17.7886464Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-27T16:03:46.7658164Z     Checking rustc_plugin_impl v0.0.0 (/checkout/src/librustc_plugin)
2019-08-27T16:03:47.0377925Z     Checking rustc_resolve v0.0.0 (/checkout/src/librustc_resolve)
2019-08-27T16:03:49.5401825Z     Checking rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
2019-08-27T16:03:50.1115803Z     Checking rustc_codegen_ssa v0.0.0 (/checkout/src/librustc_codegen_ssa)
2019-08-27T16:03:52.7906052Z error[E0004]: non-exhaustive patterns: `NativeRawDylib` not covered
2019-08-27T16:03:52.7906438Z    --> src/librustc_codegen_ssa/back/link.rs:312:15
2019-08-27T16:03:52.7916232Z 312 |         match lib.kind {
2019-08-27T16:03:52.7916232Z 312 |         match lib.kind {
2019-08-27T16:03:52.7920602Z     |               ^^^^^^^^ pattern `NativeRawDylib` not covered
2019-08-27T16:03:52.7929838Z     |
2019-08-27T16:03:52.7944321Z     = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
2019-08-27T16:03:52.7944444Z 
2019-08-27T16:03:52.8003467Z error[E0004]: non-exhaustive patterns: `NativeRawDylib` not covered
2019-08-27T16:03:52.8003801Z    --> src/librustc_codegen_ssa/back/link.rs:859:19
2019-08-27T16:03:52.8004310Z 859 |             match lib.kind {
2019-08-27T16:03:52.8004310Z 859 |             match lib.kind {
2019-08-27T16:03:52.8004610Z     |                   ^^^^^^^^ pattern `NativeRawDylib` not covered
2019-08-27T16:03:52.8004839Z     |
2019-08-27T16:03:52.8005134Z     = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
2019-08-27T16:03:52.8005174Z 
2019-08-27T16:03:52.8074648Z error[E0004]: non-exhaustive patterns: `NativeRawDylib` not covered
2019-08-27T16:03:52.8074993Z     --> src/librustc_codegen_ssa/back/link.rs:1279:15
2019-08-27T16:03:52.8075459Z 1279 |         match lib.kind {
2019-08-27T16:03:52.8075459Z 1279 |         match lib.kind {
2019-08-27T16:03:52.8075759Z      |               ^^^^^^^^ pattern `NativeRawDylib` not covered
2019-08-27T16:03:52.8075955Z      |
2019-08-27T16:03:52.8076262Z      = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
2019-08-27T16:03:52.8076304Z 
2019-08-27T16:03:52.8135466Z error[E0004]: non-exhaustive patterns: `NativeRawDylib` not covered
2019-08-27T16:03:52.8135853Z     --> src/librustc_codegen_ssa/back/link.rs:1646:19
2019-08-27T16:03:52.8136348Z 1646 |             match lib.kind {
2019-08-27T16:03:52.8136348Z 1646 |             match lib.kind {
2019-08-27T16:03:52.8136652Z      |                   ^^^^^^^^ pattern `NativeRawDylib` not covered
2019-08-27T16:03:52.8136867Z      |
2019-08-27T16:03:52.8137190Z      = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
2019-08-27T16:03:54.2242656Z error: aborting due to 4 previous errors
2019-08-27T16:03:54.2242755Z 
2019-08-27T16:03:54.2243214Z For more information about this error, try `rustc --explain E0004`.
2019-08-27T16:03:54.2528082Z error: Could not compile `rustc_codegen_ssa`.
---
2019-08-27T16:04:01.5740980Z == clock drift check ==
2019-08-27T16:04:01.5754927Z   local time: Tue Aug 27 16:04:01 UTC 2019
2019-08-27T16:04:01.8626186Z   network time: Tue, 27 Aug 2019 16:04:01 GMT
2019-08-27T16:04:01.8628845Z == end clock drift check ==
2019-08-27T16:04:03.4180571Z ##[error]Bash exited with code '1'.
2019-08-27T16:04:03.4215255Z ##[section]Starting: Checkout
2019-08-27T16:04:03.4216656Z ==============================================================================
2019-08-27T16:04:03.4216700Z Task         : Get sources
2019-08-27T16:04:03.4216760Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
