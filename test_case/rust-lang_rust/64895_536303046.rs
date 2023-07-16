plain
2019-09-29T13:41:17.1276217Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-29T13:41:17.1476184Z ##[command]git config gc.auto 0
2019-09-29T13:41:17.1544349Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-29T13:41:17.1624608Z ##[command]git config --get-all http.proxy
2019-09-29T13:41:17.1782855Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64895/merge:refs/remotes/pull/64895/merge
---
2019-09-29T13:49:54.5009471Z     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
2019-09-29T13:50:13.4274786Z error[E0609]: no field `sty` on type `&ty::TyS<'_>`
2019-09-29T13:50:13.4275645Z     --> src/librustc/traits/error_reporting.rs:1658:74
2019-09-29T13:50:13.4275983Z      |
2019-09-29T13:50:13.4276515Z 1658 | ...                   derived_obligation.parent_trait_ref.self_ty().sty);
2019-09-29T13:50:13.4277069Z 
2019-09-29T13:50:13.4277382Z error[E0609]: no field `sty` on type `&ty::TyS<'_>`
2019-09-29T13:50:13.4277682Z     --> src/librustc/traits/error_reporting.rs:1659:73
2019-09-29T13:50:13.4277943Z      |
2019-09-29T13:50:13.4277943Z      |
2019-09-29T13:50:13.4278290Z 1659 |                     match derived_obligation.parent_trait_ref.self_ty().sty {
2019-09-29T13:50:13.4278723Z 
2019-09-29T13:50:13.4302770Z error[E0026]: struct `hir::Item` does not have a field named `node`
2019-09-29T13:50:13.4303605Z     --> src/librustc/traits/error_reporting.rs:1681:13
2019-09-29T13:50:13.4304154Z      |
---
2019-09-29T13:50:21.7088869Z == clock drift check ==
2019-09-29T13:50:21.7104654Z   local time: Sun Sep 29 13:50:21 UTC 2019
2019-09-29T13:50:21.7973981Z   network time: Sun, 29 Sep 2019 13:50:21 GMT
2019-09-29T13:50:21.7980669Z == end clock drift check ==
2019-09-29T13:50:22.3982398Z ##[error]Bash exited with code '1'.
2019-09-29T13:50:22.4063490Z ##[section]Starting: Checkout
2019-09-29T13:50:22.4066058Z ==============================================================================
2019-09-29T13:50:22.4066121Z Task         : Get sources
2019-09-29T13:50:22.4066176Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
