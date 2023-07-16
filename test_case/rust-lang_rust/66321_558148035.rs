plain
2019-11-25T13:01:14.7197506Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-25T13:01:14.7211609Z ##[command]git config gc.auto 0
2019-11-25T13:01:15.7186308Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-25T13:01:15.7232305Z ##[command]git config --get-all http.proxy
2019-11-25T13:01:15.7235260Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66321/merge:refs/remotes/pull/66321/merge
---
2019-11-25T13:09:27.8806178Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-11-25T13:09:42.1907955Z error[E0433]: failed to resolve: use of undeclared type or module `hir`
2019-11-25T13:09:42.1909234Z    --> src/librustc/mir/mod.rs:419:20
2019-11-25T13:09:42.1909911Z     |
2019-11-25T13:09:42.1910306Z 419 |     ExplicitUnsafe(hir::HirId),
2019-11-25T13:09:42.1914048Z 
2019-11-25T13:09:42.1930529Z error[E0433]: failed to resolve: use of undeclared type or module `hir`
2019-11-25T13:09:42.1931016Z    --> src/librustc/mir/mod.rs:480:27
2019-11-25T13:09:42.1931374Z     |
2019-11-25T13:09:42.1931374Z     |
2019-11-25T13:09:42.1931787Z 480 | impl From<Mutability> for hir::Mutability {
2019-11-25T13:09:42.1932395Z 
2019-11-25T13:09:42.1937301Z error[E0433]: failed to resolve: use of undeclared type or module `hir`
2019-11-25T13:09:42.1937762Z    --> src/librustc/mir/mod.rs:483:32
2019-11-25T13:09:42.1938144Z     |
---
2019-11-25T13:09:42.1941653Z 
2019-11-25T13:09:42.2170840Z error[E0433]: failed to resolve: use of undeclared type or module `hir`
2019-11-25T13:09:42.2171463Z     --> src/librustc/mir/mod.rs:1688:14
2019-11-25T13:09:42.2171847Z      |
2019-11-25T13:09:42.2172278Z 1688 |     pub asm: hir::InlineAsmInner,
2019-11-25T13:09:42.2173172Z 
2019-11-25T13:09:42.2308914Z error[E0433]: failed to resolve: use of undeclared type or module `hir`
2019-11-25T13:09:42.2309182Z     --> src/librustc/mir/mod.rs:2049:20
2019-11-25T13:09:42.2309395Z      |
2019-11-25T13:09:42.2309395Z      |
2019-11-25T13:09:42.2309618Z 2049 |     pub lint_root: hir::HirId,
2019-11-25T13:09:42.2309948Z 
2019-11-25T13:09:42.2376083Z error[E0433]: failed to resolve: use of undeclared type or module `hir`
2019-11-25T13:09:42.2376355Z     --> src/librustc/mir/mod.rs:2174:39
2019-11-25T13:09:42.2376561Z      |
2019-11-25T13:09:42.2376561Z      |
2019-11-25T13:09:42.2377017Z 2174 |     Generator(DefId, SubstsRef<'tcx>, hir::Movability),
2019-11-25T13:09:42.2377359Z 
2019-11-25T13:09:42.2480938Z error[E0433]: failed to resolve: use of undeclared type or module `hir`
2019-11-25T13:09:42.2481227Z     --> src/librustc/mir/mod.rs:2726:18
2019-11-25T13:09:42.2481412Z      |
2019-11-25T13:09:42.2481412Z      |
2019-11-25T13:09:42.2481632Z 2726 |     BorrowPacked(hir::HirId),
2019-11-25T13:09:42.2481987Z 
2019-11-25T13:09:42.2498836Z error[E0433]: failed to resolve: use of undeclared type or module `hir`
2019-11-25T13:09:42.2499108Z     --> src/librustc/mir/mod.rs:2743:30
2019-11-25T13:09:42.2499291Z      |
2019-11-25T13:09:42.2499291Z      |
2019-11-25T13:09:42.2499526Z 2743 |     pub unsafe_blocks: Lrc<[(hir::HirId, bool)]>,
2019-11-25T13:09:42.2499860Z 
2019-11-25T13:09:51.0012831Z error[E0277]: the trait bound `hir::GeneratorKind: ty::fold::TypeFoldable<'_>` is not satisfied
2019-11-25T13:09:51.0013775Z   --> src/librustc/mir/mod.rs:90:68
2019-11-25T13:09:51.0014257Z    |
2019-11-25T13:09:51.0014257Z    |
2019-11-25T13:09:51.0014796Z 90 |   #[derive(Clone, RustcEncodable, RustcDecodable, Debug, HashStable, TypeFoldable)]
2019-11-25T13:09:51.0015779Z    |                                                                      |
2019-11-25T13:09:51.0016312Z    |                                                                      the trait `ty::fold::TypeFoldable<'_>` is not implemented for `hir::GeneratorKind`
2019-11-25T13:09:51.0016829Z    |                                                                      in this macro invocation
2019-11-25T13:09:51.0017383Z    | 
2019-11-25T13:09:51.0017383Z    | 
2019-11-25T13:09:51.0017899Z   ::: src/librustc/ty/fold.rs:47:5
2019-11-25T13:09:51.0018303Z    |
2019-11-25T13:09:51.0018772Z 47 |       fn fold_with<F: TypeFolder<'tcx>>(&self, folder: &mut F) -> Self {
2019-11-25T13:09:51.0019322Z    |       ---------------------------------------------------------------- required by `ty::fold::TypeFoldable::fold_with`
2019-11-25T13:09:51.0019722Z    |
2019-11-25T13:09:51.0020246Z    = note: required because of the requirements on the impl of `ty::fold::TypeFoldable<'_>` for `std::option::Option<hir::GeneratorKind>`
2019-11-25T13:09:51.0116359Z error[E0277]: the trait bound `hir::GeneratorKind: ty::fold::TypeFoldable<'_>` is not satisfied
2019-11-25T13:09:51.0117254Z   --> src/librustc/mir/mod.rs:90:68
2019-11-25T13:09:51.0117687Z    |
2019-11-25T13:09:51.0117687Z    |
2019-11-25T13:09:51.0118213Z 90 |   #[derive(Clone, RustcEncodable, RustcDecodable, Debug, HashStable, TypeFoldable)]
2019-11-25T13:09:51.0119196Z    |                                                                      |
2019-11-25T13:09:51.0119742Z    |                                                                      the trait `ty::fold::TypeFoldable<'_>` is not implemented for `hir::GeneratorKind`
2019-11-25T13:09:51.0120291Z    |                                                                      in this macro invocation
2019-11-25T13:09:51.0121225Z    | 
2019-11-25T13:09:51.0121225Z    | 
2019-11-25T13:09:51.0121742Z   ::: src/librustc/ty/fold.rs:52:5
2019-11-25T13:09:51.0122070Z    |
2019-11-25T13:09:51.0122453Z 52 |       fn visit_with<V: TypeVisitor<'tcx>>(&self, visitor: &mut V) -> bool {
2019-11-25T13:09:51.0122921Z    |       ------------------------------------------------------------------- required by `ty::fold::TypeFoldable::visit_with`
2019-11-25T13:09:51.0123232Z    |
2019-11-25T13:09:51.0123672Z    = note: required because of the requirements on the impl of `ty::fold::TypeFoldable<'_>` for `std::option::Option<hir::GeneratorKind>`
2019-11-25T13:09:51.0123827Z 
2019-11-25T13:09:51.5069278Z error[E0283]: type annotations needed: cannot resolve `std::boxed::Box<mir::AggregateKind<'_>>: serialize::serialize::Decodable`
2019-11-25T13:09:51.5069782Z     ::: /checkout/src/libserialize/serialize.rs:304:18
2019-11-25T13:09:51.5069962Z      |
2019-11-25T13:09:51.5069962Z      |
2019-11-25T13:09:51.5070205Z 304  |     fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error>;
2019-11-25T13:09:51.5070517Z      |                  ------- required by this bound in `serialize::serialize::Decodable::decode`
2019-11-25T13:09:51.5071082Z     ::: <::core::macros::builtin::RustcDecodable macros>:1:1
2019-11-25T13:09:51.5071485Z      |
2019-11-25T13:09:51.5071485Z      |
2019-11-25T13:09:51.5071714Z 1    | ($ item : item) => { }
2019-11-25T13:09:51.5071981Z      | ---------------------- in this expansion of `#[derive(RustcDecodable)]`
2019-11-25T13:09:51.5072382Z      |
2019-11-25T13:09:51.5072633Z 2118 | #[derive(Clone, RustcEncodable, RustcDecodable, HashStable, PartialEq)]
2019-11-25T13:09:51.5072934Z      |                                 ^^^^^^^^^^^^^^ in this macro invocation
2019-11-25T13:09:51.5072968Z 
---
2019-11-25T13:10:00.3998295Z   local time: Mon Nov 25 13:10:00 UTC 2019
2019-11-25T13:10:00.9185552Z   network time: Mon, 25 Nov 2019 13:10:00 GMT
2019-11-25T13:10:00.9187037Z == end clock drift check ==
2019-11-25T13:10:02.0136505Z 
2019-11-25T13:10:02.0189568Z ##[error]Bash exited with code '1'.
2019-11-25T13:10:02.0211226Z ##[section]Starting: Checkout
2019-11-25T13:10:02.0212609Z ==============================================================================
2019-11-25T13:10:02.0212669Z Task         : Get sources
2019-11-25T13:10:02.0212704Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
