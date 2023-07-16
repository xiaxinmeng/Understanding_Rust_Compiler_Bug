plain
2019-10-09T17:36:30.4608175Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-09T17:36:30.5020317Z ##[command]git config gc.auto 0
2019-10-09T17:36:30.5102406Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-09T17:36:30.5155763Z ##[command]git config --get-all http.proxy
2019-10-09T17:36:30.5301685Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65244/merge:refs/remotes/pull/65244/merge
---
2019-10-09T17:45:07.2707898Z     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
2019-10-09T17:45:18.6635456Z error[E0308]: mismatched types
2019-10-09T17:45:18.6636899Z    --> src/librustc/hir/mod.rs:50:9
2019-10-09T17:45:18.6637532Z     |
2019-10-09T17:45:18.6638484Z 45  | / macro_rules! hir_vec {
2019-10-09T17:45:18.6639401Z 46  | |     ($elem:expr; $n:expr) => (
2019-10-09T17:45:18.6640102Z 47  | |         $crate::hir::HirVec::from(vec![$elem; $n])
2019-10-09T17:45:18.6640760Z 48  | |     );
2019-10-09T17:45:18.6641401Z 49  | |     ($($x:expr),*) => (
2019-10-09T17:45:18.6642055Z 50  | |         $crate::hir::HirVec::from(vec![$($x),*])
2019-10-09T17:45:18.6642897Z     | |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `hir::Expr`, found struct `hir::ptr::P`
2019-10-09T17:45:18.6644304Z 52  | | }
2019-10-09T17:45:18.6644304Z 52  | | }
2019-10-09T17:45:18.6644977Z     | |_- in this expansion of `hir_vec!`
2019-10-09T17:45:18.6646220Z    ::: src/librustc/hir/lowering/expr.rs:671:17
2019-10-09T17:45:18.6646813Z     |
2019-10-09T17:45:18.6646813Z     |
2019-10-09T17:45:18.6647458Z 671 |                   hir_vec![expr],
2019-10-09T17:45:18.6649063Z     |
2019-10-09T17:45:18.6649063Z     |
2019-10-09T17:45:18.6649751Z     = note: expected type `hir::ptr::P<[hir::Expr]>`
2019-10-09T17:45:18.6650378Z                found type `hir::ptr::P<[hir::ptr::P<hir::Expr>]>`
2019-10-09T17:45:32.6873956Z error: aborting due to previous error
2019-10-09T17:45:32.6874871Z 
2019-10-09T17:45:32.6875535Z For more information about this error, try `rustc --explain E0308`.
2019-10-09T17:45:32.8678751Z error: could not compile `rustc`.
---
2019-10-09T17:45:32.8762706Z == clock drift check ==
2019-10-09T17:45:32.8782984Z   local time: Wed Oct  9 17:45:32 UTC 2019
2019-10-09T17:45:33.0402787Z   network time: Wed, 09 Oct 2019 17:45:33 GMT
2019-10-09T17:45:33.0403229Z == end clock drift check ==
2019-10-09T17:45:33.5405743Z ##[error]Bash exited with code '1'.
2019-10-09T17:45:33.5440765Z ##[section]Starting: Checkout
2019-10-09T17:45:33.5442556Z ==============================================================================
2019-10-09T17:45:33.5442628Z Task         : Get sources
2019-10-09T17:45:33.5442692Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
