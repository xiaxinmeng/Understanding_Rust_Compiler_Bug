plain
2019-09-17T08:54:16.4179301Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-17T08:54:16.4370554Z ##[command]git config gc.auto 0
2019-09-17T08:54:16.4422163Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-17T08:54:16.4481784Z ##[command]git config --get-all http.proxy
2019-09-17T08:54:16.4661926Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64525/merge:refs/remotes/pull/64525/merge
---
2019-09-17T09:02:07.1232465Z     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
2019-09-17T09:02:17.2358966Z error[E0308]: mismatched types
2019-09-17T09:02:17.2360290Z     --> src/librustc/hir/lowering.rs:2714:9
2019-09-17T09:02:17.2360870Z      |
2019-09-17T09:02:17.2361334Z 2713 |     fn lower_block_expr(&mut self, b: &Block) -> P<hir::Expr> {
2019-09-17T09:02:17.2361887Z      |                                                  ------------ expected `hir::ptr::P<hir::Expr>` because of return type
2019-09-17T09:02:17.2362390Z 2714 |         self.expr_block(self.lower_block(b, false), ThinVec::new())
2019-09-17T09:02:17.2362941Z      |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `hir::ptr::P`, found struct `hir::Expr`
2019-09-17T09:02:17.2364324Z      = note: expected type `hir::ptr::P<hir::Expr>`
2019-09-17T09:02:17.2364834Z                 found type `hir::Expr`
2019-09-17T09:02:17.2365094Z 
2019-09-17T09:02:17.3800894Z error[E0308]: mismatched types
2019-09-17T09:02:17.3800894Z error[E0308]: mismatched types
2019-09-17T09:02:17.3801168Z   --> src/librustc/hir/lowering/expr.rs:93:21
2019-09-17T09:02:17.3801334Z    |
2019-09-17T09:02:17.3801595Z 93 |                     this.with_new_scopes(|this| this.lower_block_expr(block))
2019-09-17T09:02:17.3802292Z    |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `hir::Expr`, found struct `hir::ptr::P`
2019-09-17T09:02:17.3802738Z    = note: expected type `hir::Expr`
2019-09-17T09:02:17.3802957Z               found type `hir::ptr::P<hir::Expr>`
2019-09-17T09:02:17.3802990Z 
2019-09-17T09:02:17.4770011Z error[E0308]: mismatched types
2019-09-17T09:02:17.4770011Z error[E0308]: mismatched types
2019-09-17T09:02:17.4770467Z    --> src/librustc/hir/lowering/expr.rs:309:45
2019-09-17T09:02:17.4770660Z     |
2019-09-17T09:02:17.4770882Z 309 |         let then_arm = self.arm(then_pat, P(then_expr));
2019-09-17T09:02:17.4771161Z     |                                             ^^^^^^^^^ expected struct `hir::Expr`, found struct `hir::ptr::P`
2019-09-17T09:02:17.4771546Z     = note: expected type `hir::Expr`
2019-09-17T09:02:17.4771765Z                found type `hir::ptr::P<hir::Expr>`
2019-09-17T09:02:17.4771797Z 
2019-09-17T09:02:17.5715279Z error[E0308]: mismatched types
2019-09-17T09:02:17.5715279Z error[E0308]: mismatched types
2019-09-17T09:02:17.5715684Z    --> src/librustc/hir/lowering/expr.rs:377:45
2019-09-17T09:02:17.5715912Z     |
2019-09-17T09:02:17.5716208Z 377 |         let then_arm = self.arm(then_pat, P(then_expr));
2019-09-17T09:02:17.5716729Z     |                                             ^^^^^^^^^ expected struct `hir::Expr`, found struct `hir::ptr::P`
2019-09-17T09:02:17.5717353Z     = note: expected type `hir::Expr`
2019-09-17T09:02:17.5717635Z                found type `hir::ptr::P<hir::Expr>`
2019-09-17T09:02:17.5717670Z 
2019-09-17T09:02:17.7278751Z error[E0061]: this function takes 1 parameter but 2 parameters were supplied
2019-09-17T09:02:17.7278751Z error[E0061]: this function takes 1 parameter but 2 parameters were supplied
2019-09-17T09:02:17.7279051Z     --> src/librustc/hir/lowering/item.rs:1074:46
2019-09-17T09:02:17.7279212Z      |
2019-09-17T09:02:17.7279469Z 1074 |         self.lower_fn_body(decl, |this| this.lower_block_expr(body, false))
2019-09-17T09:02:17.7280109Z      | 
2019-09-17T09:02:17.7280313Z     ::: src/librustc/hir/lowering.rs:2713:5
2019-09-17T09:02:17.7280474Z      |
2019-09-17T09:02:17.7280474Z      |
2019-09-17T09:02:17.7280711Z 2713 |     fn lower_block_expr(&mut self, b: &Block) -> P<hir::Expr> {
2019-09-17T09:02:17.7281011Z 
2019-09-17T09:02:17.7281208Z error[E0308]: mismatched types
2019-09-17T09:02:17.7281398Z     --> src/librustc/hir/lowering/item.rs:1074:41
2019-09-17T09:02:17.7281569Z      |
2019-09-17T09:02:17.7281569Z      |
2019-09-17T09:02:17.7281804Z 1074 |         self.lower_fn_body(decl, |this| this.lower_block_expr(body, false))
2019-09-17T09:02:17.7282130Z      |                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `hir::Expr`, found struct `hir::ptr::P`
2019-09-17T09:02:17.7282539Z      = note: expected type `hir::Expr`
2019-09-17T09:02:17.7282986Z                 found type `hir::ptr::P<hir::Expr>`
2019-09-17T09:02:17.7283020Z 
2019-09-17T09:02:17.8255493Z error[E0308]: mismatched types
2019-09-17T09:02:17.8255493Z error[E0308]: mismatched types
2019-09-17T09:02:17.8256583Z     --> src/librustc/hir/lowering/item.rs:1237:27
2019-09-17T09:02:17.8257079Z      |
2019-09-17T09:02:17.8258343Z 1237 |                         P(user_body),
2019-09-17T09:02:17.8258937Z      |                           ^^^^^^^^^ expected struct `hir::Expr`, found struct `hir::ptr::P`
2019-09-17T09:02:17.8259854Z      = note: expected type `hir::Expr`
2019-09-17T09:02:17.8260343Z                 found type `hir::ptr::P<hir::Expr>`
2019-09-17T09:02:17.8260543Z 
2019-09-17T09:02:30.6044370Z error: aborting due to 7 previous errors
---
2019-09-17T09:02:30.7929633Z == clock drift check ==
2019-09-17T09:02:30.7946076Z   local time: Tue Sep 17 09:02:30 UTC 2019
2019-09-17T09:02:30.9552375Z   network time: Tue, 17 Sep 2019 09:02:30 GMT
2019-09-17T09:02:30.9559748Z == end clock drift check ==
2019-09-17T09:02:31.7950353Z ##[error]Bash exited with code '1'.
2019-09-17T09:02:31.7993678Z ##[section]Starting: Checkout
2019-09-17T09:02:31.7995803Z ==============================================================================
2019-09-17T09:02:31.7995879Z Task         : Get sources
2019-09-17T09:02:31.7995928Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
