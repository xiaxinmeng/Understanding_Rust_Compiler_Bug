plain
2020-01-18T23:03:16.2757214Z    Compiling rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-01-18T23:03:30.4906778Z error: Fallback to `!` may introduce undefined behavior
2020-01-18T23:03:30.4907499Z     --> src/librustc/ty/context.rs:205:13
2020-01-18T23:03:30.4938604Z      |
2020-01-18T23:03:30.4939743Z 205  | /             ty::tls::with(|tcx| {
2020-01-18T23:03:30.4940159Z 206  | |                 bug!(
2020-01-18T23:03:30.4940535Z 207  | |                     "node {} with HirId::owner {:?} cannot be placed in \
2020-01-18T23:03:30.4940946Z 208  | |                         TypeckTables with local_id_root {:?}",
2020-01-18T23:03:30.4943018Z 212  | |                 )
2020-01-18T23:03:30.4943495Z 213  | |             });
2020-01-18T23:03:30.4943903Z      | |______________^
2020-01-18T23:03:30.4944399Z      |
2020-01-18T23:03:30.4944399Z      |
2020-01-18T23:03:30.4944862Z note: the type parameter R here was inferred to `!`
2020-01-18T23:03:30.4945237Z     --> src/librustc/ty/context.rs:205:13
2020-01-18T23:03:30.4945582Z      |
2020-01-18T23:03:30.4945975Z 205  |             ty::tls::with(|tcx| {
2020-01-18T23:03:30.4946743Z note: (type parameter defined here)
2020-01-18T23:03:30.4947123Z     --> src/librustc/ty/context.rs:1792:20
2020-01-18T23:03:30.4947474Z      |
2020-01-18T23:03:30.4947474Z      |
2020-01-18T23:03:30.4947868Z 1792 |     pub fn with<F, R>(f: F) -> R
2020-01-18T23:03:30.4948647Z note: ... due to this expression evaluating to `!`
2020-01-18T23:03:30.4949023Z     --> src/librustc/macros.rs:42:9
2020-01-18T23:03:30.4949346Z      |
2020-01-18T23:03:30.4949782Z 39   | / macro_rules! bug {
2020-01-18T23:03:30.4949782Z 39   | / macro_rules! bug {
2020-01-18T23:03:30.4950231Z 40   | |     () => ( bug!("impossible case reached") );
2020-01-18T23:03:30.4950660Z 41   | |     ($($message:tt)*) => ({
2020-01-18T23:03:30.4951339Z 42   | |         $crate::util::bug::bug_fmt(file!(), line!(), format_args!($($message)*))
2020-01-18T23:03:30.4952250Z 43   | |     })
2020-01-18T23:03:30.4952671Z 44   | | }
2020-01-18T23:03:30.4953097Z      | |_- in this expansion of `bug!`
2020-01-18T23:03:30.4953436Z      | 
2020-01-18T23:03:30.4953436Z      | 
2020-01-18T23:03:30.4953822Z     ::: src/librustc/ty/context.rs:206:17
2020-01-18T23:03:30.4954164Z      |
2020-01-18T23:03:30.4954579Z 206  | /                 bug!(
2020-01-18T23:03:30.4955065Z 207  | |                     "node {} with HirId::owner {:?} cannot be placed in \
2020-01-18T23:03:30.4955531Z 208  | |                         TypeckTables with local_id_root {:?}",
2020-01-18T23:03:30.4956303Z 209  | |                     tcx.hir().node_to_string(hir_id),
2020-01-18T23:03:30.4956884Z 210  | |                     DefId::local(hir_id.owner),
2020-01-18T23:03:30.4957345Z 211  | |                     local_id_root
2020-01-18T23:03:30.4959429Z      | |_________________- in this macro invocation
2020-01-18T23:03:30.4960012Z      = note: If you want the `!` type to be used here, add explicit type annotations
2020-01-18T23:03:30.4960252Z 
2020-01-18T23:03:31.6504050Z error: Fallback to `!` may introduce undefined behavior
2020-01-18T23:03:31.6504050Z error: Fallback to `!` may introduce undefined behavior
2020-01-18T23:03:31.6505038Z     --> src/librustc/util/bug.rs:32:5
2020-01-18T23:03:31.6505561Z      |
2020-01-18T23:03:31.6506098Z 32   | /     tls::with_opt(move |tcx| {
2020-01-18T23:03:31.6506725Z 33   | |         let msg = format!("{}:{}: {}", file, line, args);
2020-01-18T23:03:31.6507310Z 34   | |         match (tcx, span) {
2020-01-18T23:03:31.6507903Z 35   | |             (Some(tcx), Some(span)) => tcx.sess.diagnostic().span_bug(span, &msg),
2020-01-18T23:03:31.6508919Z 38   | |         }
2020-01-18T23:03:31.6509436Z 39   | |     });
2020-01-18T23:03:31.6510166Z      | |______^
2020-01-18T23:03:31.6510626Z      |
2020-01-18T23:03:31.6510626Z      |
2020-01-18T23:03:31.6511094Z note: the type parameter R here was inferred to `!`
2020-01-18T23:03:31.6511591Z     --> src/librustc/util/bug.rs:32:5
2020-01-18T23:03:31.6512026Z      |
2020-01-18T23:03:31.6512545Z 32   |     tls::with_opt(move |tcx| {
2020-01-18T23:03:31.6513509Z note: (type parameter defined here)
2020-01-18T23:03:31.6514005Z     --> src/librustc/ty/context.rs:1802:24
2020-01-18T23:03:31.6514459Z      |
2020-01-18T23:03:31.6514459Z      |
2020-01-18T23:03:31.6514948Z 1802 |     pub fn with_opt<F, R>(f: F) -> R
2020-01-18T23:03:31.6515950Z note: ... due to this expression evaluating to `!`
2020-01-18T23:03:31.6516452Z     --> src/librustc/util/bug.rs:35:40
2020-01-18T23:03:31.6516910Z      |
2020-01-18T23:03:31.6516910Z      |
2020-01-18T23:03:31.6517438Z 35   |             (Some(tcx), Some(span)) => tcx.sess.diagnostic().span_bug(span, &msg),
2020-01-18T23:03:31.6518590Z      = note: If you want the `!` type to be used here, add explicit type annotations
2020-01-18T23:03:31.6518837Z 
2020-01-18T23:03:31.8755953Z error: aborting due to 2 previous errors
2020-01-18T23:03:31.8756720Z 
---
2020-01-18T23:03:52.9619768Z   local time: Sat Jan 18 23:03:52 UTC 2020
2020-01-18T23:03:53.6003281Z   network time: Sat, 18 Jan 2020 23:03:53 GMT
2020-01-18T23:03:53.6003876Z == end clock drift check ==
2020-01-18T23:03:53.9235535Z 
2020-01-18T23:03:53.9312614Z ##[error]Bash exited with code '1'.
2020-01-18T23:03:53.9346058Z ##[section]Starting: Checkout rust-lang/rust@try to s
2020-01-18T23:03:53.9347780Z ==============================================================================
2020-01-18T23:03:53.9347866Z Task         : Get sources
2020-01-18T23:03:53.9347928Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
