plain
2019-10-18T22:31:05.4768731Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-18T22:31:05.4959956Z ##[command]git config gc.auto 0
2019-10-18T22:31:05.5042902Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-18T22:31:05.5102688Z ##[command]git config --get-all http.proxy
2019-10-18T22:31:05.5266970Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/59789/merge:refs/remotes/pull/59789/merge
---
2019-10-18T23:19:44.1605605Z    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2019-10-18T23:19:46.8975377Z error[E0478]: lifetime bound not satisfied
2019-10-18T23:19:46.8975840Z   --> src/librustc_mir/borrow_check/nll/constraint_generation.rs:44:5
2019-10-18T23:19:46.8976087Z    |
2019-10-18T23:19:46.8976431Z 44 |     infcx: &'cg InferCtxt<'cx, 'tcx>,
2019-10-18T23:19:46.8977280Z    |
2019-10-18T23:19:46.8977605Z note: lifetime parameter instantiated with the lifetime `'tcx` as defined on the struct at 43:39
2019-10-18T23:19:46.8977904Z   --> src/librustc_mir/borrow_check/nll/constraint_generation.rs:43:39
2019-10-18T23:19:46.8978125Z    |
2019-10-18T23:19:46.8978125Z    |
2019-10-18T23:19:46.8978426Z 43 | struct ConstraintGeneration<'cg, 'cx, 'tcx> {
2019-10-18T23:19:46.8978718Z    |                                       ^^^^
2019-10-18T23:19:46.8979022Z note: but lifetime parameter must outlive the lifetime `'cx` as defined on the struct at 43:34
2019-10-18T23:19:46.8979569Z   --> src/librustc_mir/borrow_check/nll/constraint_generation.rs:43:34
2019-10-18T23:19:46.8979856Z    |
2019-10-18T23:19:46.8980159Z 43 | struct ConstraintGeneration<'cg, 'cx, 'tcx> {
2019-10-18T23:19:46.8980511Z 
2019-10-18T23:19:46.9097998Z error[E0478]: lifetime bound not satisfied
2019-10-18T23:19:46.9098878Z   --> src/librustc_mir/borrow_check/nll/region_infer/error_reporting/mod.rs:63:5
2019-10-18T23:19:46.9099501Z    |
2019-10-18T23:19:46.9099501Z    |
2019-10-18T23:19:46.9100121Z 63 |     infcx: &'b InferCtxt<'a, 'tcx>,
2019-10-18T23:19:46.9101339Z    |
2019-10-18T23:19:46.9101978Z note: lifetime parameter instantiated with the lifetime `'tcx` as defined on the struct at 57:38
2019-10-18T23:19:46.9102624Z   --> src/librustc_mir/borrow_check/nll/region_infer/error_reporting/mod.rs:57:38
2019-10-18T23:19:46.9103169Z    |
2019-10-18T23:19:46.9103169Z    |
2019-10-18T23:19:46.9103801Z 57 | pub struct ErrorReportingCtx<'a, 'b, 'tcx> {
2019-10-18T23:19:46.9104402Z    |                                      ^^^^
2019-10-18T23:19:46.9105506Z note: but lifetime parameter must outlive the lifetime `'a` as defined on the struct at 57:30
2019-10-18T23:19:46.9106217Z   --> src/librustc_mir/borrow_check/nll/region_infer/error_reporting/mod.rs:57:30
2019-10-18T23:19:46.9106802Z    |
2019-10-18T23:19:46.9107423Z 57 | pub struct ErrorReportingCtx<'a, 'b, 'tcx> {
2019-10-18T23:19:46.9108422Z 
2019-10-18T23:19:47.8474016Z error: aborting due to 2 previous errors
2019-10-18T23:19:47.8474131Z 
2019-10-18T23:19:47.8479790Z For more information about this error, try `rustc --explain E0478`.
---
2019-10-18T23:21:18.1856506Z   local time: Fri Oct 18 23:21:18 UTC 2019
2019-10-18T23:21:18.4515005Z   network time: Fri, 18 Oct 2019 23:21:18 GMT
2019-10-18T23:21:18.4516047Z == end clock drift check ==
2019-10-18T23:21:21.4096263Z 
2019-10-18T23:21:21.4169020Z ##[error]Bash exited with code '1'.
2019-10-18T23:21:21.4253536Z ##[section]Starting: Checkout
2019-10-18T23:21:21.4259064Z ==============================================================================
2019-10-18T23:21:21.4259168Z Task         : Get sources
2019-10-18T23:21:21.4259221Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
