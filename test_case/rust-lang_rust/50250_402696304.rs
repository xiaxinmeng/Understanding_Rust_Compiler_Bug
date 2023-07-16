plain
[00:12:12]    Compiling rustc_typeck v0.0.0 (file:///checkout/src/librustc_typeck)
[00:14:26]    Compiling rustc_allocator v0.0.0 (file:///checkout/src/librustc_allocator)
[00:14:33]    Compiling rustc_resolve v0.0.0 (file:///checkout/src/librustc_resolve)
[00:16:00]    Compiling rustc_traits v0.0.0 (file:///checkout/src/librustc_traits)
[00:16:00] error[E0432]: unresolved import `rustc::traits::WhereClauseAtom`
[00:16:00]   --> librustc_traits/lowering.rs:16:71
[00:16:00]    |
[00:16:00] 16 |     Clause, Clauses, DomainGoal, Goal, PolyDomainGoal, ProgramClause, WhereClauseAtom,
[00:16:00]    |                                                                       ^^^^^^^^^^^^^^^ no `WhereClauseAtom` in `traits`. Did you mean to use `WhereClause`?
[00:16:00] 
[00:16:00] error[E0432]: unresolved import `self::WhereClause`
[00:16:00]    --> librustc_traits/lowering.rs:129:19
[00:16:00]     |
[00:16:00] 129 |         use self::WhereClause::*;
[00:16:00]     |                   ^^^^^^^^^^^ Could not find `WhereClause` in `self`
[00:16:00]    Compiling rustc_plugin v0.0.0 (file:///checkout/src/librustc_plugin)
[00:16:00]    Compiling rustc_plugin v0.0.0 (file:///checkout/src/librustc_plugin)
[00:16:00] error[E0433]: failed to resolve. Use of undeclared type or module `WhereClause`
[00:16:00]   --> librustc_traits/lowering.rs:50:9
[00:16:00]    |
[00:16:00] 50 |         WhereClause::Implemented(*self)
[00:16:00]    |         ^^^^^^^^^^^ Use of undeclared type or module `WhereClause`
[00:16:00] 
[00:16:00] error[E0433]: failed to resolve. Use of undeclared type or module `WhereClause`
[00:16:00]   --> librustc_traits/lowering.rs:56:9
[00:16:00]    |
[00:16:00] 56 |         WhereClause::ProjectionEq(*self)
[00:16:00]    |         ^^^^^^^^^^^ Use of undeclared type or module `WhereClause`
[00:16:00] 
[00:16:00] error[E0433]: failed to resolve. Use of undeclared type or module `WhereClause`
[00:16:00]   --> librustc_traits/lowering.rs:62:9
[00:16:00]    |
[00:16:00] 62 |         WhereClause::RegionOutlives(*self)
[00:16:00]    |         ^^^^^^^^^^^ Use of undeclared type or module `WhereClause`
[00:16:00] 
[00:16:00] error[E0433]: failed to resolve. Use of undeclared type or module `WhereClause`
[00:16:00]   --> librustc_traits/lowering.rs:68:9
[00:16:00]    |
[00:16:00] 68 |         WhereClause::TypeOutlives(*self)
[00:16:00]    |         ^^^^^^^^^^^ Use of undeclared type or module `WhereClause`
[00:16:00] 
[00:16:00] error[E0433]: failed to resolve. Use of undeclared type or module `WellFormed`
[00:16:00]    --> librustc_traits/lowering.rs:106:58
[00:16:00]     |
[00:16:00] 106 |                 ty::Binder::dummy(DomainGoal::WellFormed(WellFormed::Ty(*ty)))
[00:16:00]     |                                                          ^^^^^^^^^^ Use of undeclared type or module `WellFormed`
[00:16:00] error[E0433]: failed to resolve. Use of undeclared type or module `FromEnv`
[00:16:00]    --> librustc_traits/lowering.rs:133:37
[00:16:00]     |
[00:16:00]     |
[00:16:00] 133 |                 DomainGoal::FromEnv(FromEnv::Trait(trait_ref))
[00:16:00]     |                                     ^^^^^^^ Use of undeclared type or module `FromEnv`
[00:16:00] 
[00:16:00] error[E0433]: failed to resolve. Use of undeclared type or module `WellFormed`
[00:16:00]    --> librustc_traits/lowering.rs:375:38
[00:16:00]     |
[00:16:00] 375 |         goal: DomainGoal::WellFormed(WellFormed::Ty(ty)),
[00:16:00]     |                                      ^^^^^^^^^^ Use of undeclared type or module `WellFormed`
[00:16:00] error[E0433]: failed to resolve. Use of undeclared type or module `FromEnv`
[00:16:00]    --> librustc_traits/lowering.rs:396:45
[00:16:00]     |
[00:16:00]     |
[00:16:00] 396 |     let from_env_goal = DomainGoal::FromEnv(FromEnv::Ty(ty)).into_goal();
[00:16:00]     |                                             ^^^^^^^ Use of undeclared type or module `FromEnv`
[00:16:00] error[E0412]: cannot find type `Providers` in this scope
[00:16:00]   --> librustc_traits/lowering.rs:26:26
[00:16:00]    |
[00:16:00]    |
[00:16:00] 26 | crate fn provide(p: &mut Providers) {
[00:16:00] help: possible candidate is found in another module, you can import it into scope
[00:16:00]    |
[00:16:00]    |
[00:16:00] 11 | use rustc::ty::query::Providers;
[00:16:00] 
[00:16:00] error[E0422]: cannot find struct, variant or union type `Providers` in this scope
[00:16:00]   --> librustc_traits/lowering.rs:27:10
[00:16:00]    |
[00:16:00]    |
[00:16:00] 27 |     *p = Providers {
[00:16:00] help: possible candidate is found in another module, you can import it into scope
[00:16:00]    |
[00:16:00]    |
[00:16:00] 11 | use rustc::ty::query::Providers;
[00:16:00] 
[00:16:00] 
[00:16:00] error[E0412]: cannot find type `WhereClause` in this scope
[00:16:00]   --> librustc_traits/lowering.rs:48:18
[00:16:00]    |
[00:16:00] 48 | impl<'tcx> Lower<WhereClause<'tcx>> for ty::TraitPredicate<'tcx> {
[00:16:00] help: possible candidates are found in other modules, you can import them into scope
[00:16:00]    |
[00:16:00]    |
[00:16:00] 11 | use rustc::hir::WhereClause;
[00:16:00] 11 | use rustc::traits::WhereClause;
[00:16:00]    |
[00:16:00] 11 | use syntax::ast::WhereClause;
[00:16:00]    |
[00:16:00]    |
[00:16:00] 
[00:16:00] error[E0412]: cannot find type `WhereClause` in this scope
[00:16:00]   --> librustc_traits/lowering.rs:49:24
[00:16:00]    |
[00:16:00] 49 |     fn lower(&self) -> WhereClause<'tcx> {
[00:16:00] help: possible candidates are found in other modules, you can import them into scope
[00:16:00]    |
[00:16:00]    |
[00:16:00] 11 | use rustc::hir::WhereClause;
[00:16:00] 11 | use rustc::traits::WhereClause;
[00:16:00]    |
[00:16:00] 11 | use syntax::ast::WhereClause;
[00:16:00]    |
[00:16:00]    |
[00:16:00] 
[00:16:00] error[E0412]: cannot find type `WhereClause` in this scope
[00:16:00]   --> librustc_traits/lowering.rs:54:18
[00:16:00]    |
[00:16:00] 54 | impl<'tcx> Lower<WhereClause<'tcx>> for ty::ProjectionPredicate<'tcx> {
[00:16:00] help: possible candidates are found in other modules, you can import them into scope
[00:16:00]    |
[00:16:00]    |
[00:16:00] 11 | use rustc::hir::WhereClause;
[00:16:00] 11 | use rustc::traits::WhereClause;
[00:16:00]    |
[00:16:00] 11 | use syntax::ast::WhereClause;
[00:16:00]    |
[00:16:00]    |
[00:16:00] 
[00:16:00] error[E0412]: cannot find type `WhereClause` in this scope
[00:16:00]   --> librustc_traits/lowering.rs:55:24
[00:16:00]    |
[00:16:00] 55 |     fn lower(&self) -> WhereClause<'tcx> {
[00:16:00] help: possible candidates are found in other modules, you can import them into scope
[00:16:00]    |
[00:16:00]    |
[00:16:00] 11 | use rustc::hir::WhereClause;
[00:16:00] 11 | use rustc::traits::WhereClause;
[00:16:00]    |
[00:16:00] 11 | use syntax::ast::WhereClause;
[00:16:00]    |
[00:16:00]    |
[00:16:00] 
[00:16:00] error[E0412]: cannot find type `WhereClause` in this scope
[00:16:00]   --> librustc_traits/lowering.rs:60:18
[00:16:00]    |
[00:16:00] 60 | impl<'tcx> Lower<WhereClause<'tcx>> for ty::RegionOutlivesPredicate<'tcx> {
[00:16:00] help: possible candidates are found in other modules, you can import them into scope
[00:16:00]    |
[00:16:00]    |
[00:16:00] 11 | use rustc::hir::WhereClause;
[00:16:00] 11 | use rustc::traits::WhereClause;
[00:16:00]    |
[00:16:00] 11 | use syntax::ast::WhereClause;
[00:16:00]    |
[00:16:00]    |
[00:16:00] 
[00:16:00] error[E0412]: cannot find type `WhereClause` in this scope
[00:16:00]   --> librustc_traits/lowering.rs:61:24
[00:16:00]    |
[00:16:00] 61 |     fn lower(&self) -> WhereClause<'tcx> {
[00:16:00] help: possible candidates are found in other modules, you can import them into scope
[00:16:00]    |
[00:16:00]    |
[00:16:00] 11 | use rustc::hir::WhereClause;
[00:16:00] 11 | use rustc::traits::WhereClause;
[00:16:00]    |
[00:16:00] 11 | use syntax::ast::WhereClause;
[00:16:00]    |
[00:16:00]    |
[00:16:00] 
[00:16:00] error[E0412]: cannot find type `WhereClause` in this scope
[00:16:00]   --> librustc_traits/lowering.rs:66:18
[00:16:00]    |
[00:16:00] 66 | impl<'tcx> Lower<WhereClause<'tcx>> for ty::TypeOutlivesPredicate<'tcx> {
[00:16:00] help: possible candidates are found in other modules, you can import them into scope
[00:16:00]    |
[00:16:00]    |
[00:16:00] 11 | use rustc::hir::WhereClause;
[00:16:00] 11 | use rustc::traits::WhereClause;
[00:16:00]    |
[00:16:00] 11 | use syntax::ast::WhereClause;
[00:16:00]    |
[00:16:00]    |
[00:16:00] 
[00:16:00] error[E0412]: cannot find type `WhereClause` in this scope
[00:16:00]   --> librustc_traits/lowering.rs:67:24
[00:16:00]    |
[00:16:00] 67 |     fn lower(&self) -> WhereClause<'tcx> {
[00:16:00] help: possible candidates are found in other modules, you can import them into scope
[00:16:00]    |
[00:16:00]    |
[00:16:00] 11 | use rustc::hir::WhereClause;
[00:16:00] 11 | use rustc::traits::WhereClause;
[00:16:00]    |
[00:16:00] 11 | use syntax::ast::WhereClause;
[00:16:00]    |
[00:16:00]    |
[00:16:00] 
[00:16:00] error[E0412]: cannot find type `WhereClause` in this scope
[00:16:00]   --> librustc_traits/lowering.rs:74:14
[00:16:00]    |
[00:16:00] 74 |     T: Lower<WhereClause<'tcx>>,
[00:16:00] help: possible candidates are found in other modules, you can import them into scope
[00:16:00]    |
[00:16:00]    |
[00:16:00] 11 | use rustc::hir::WhereClause;
[00:16:00] 11 | use rustc::traits::WhereClause;
[00:16:00]    |
[00:16:00] 11 | use syntax::ast::WhereClause;
[00:16:00]    |
[00:16:00]    |
[00:16:00] 
[00:16:00] error[E0531]: cannot find tuple struct/variant `Implemented` in this scope
[00:16:00]    --> librustc_traits/lowering.rs:132:31
[00:16:00]     |
[00:16:00] 132 |             DomainGoal::Holds(Implemented(trait_ref)) => {
[00:16:00] help: possible candidate is found in another module, you can import it into scope
[00:16:00]     |
[00:16:00]     |
[00:16:00] 11  | use rustc::traits::WhereClause::Implemented;
[00:16:00] 
[00:16:00] 
[00:16:00] error[E0531]: cannot find tuple struct/variant `WellFormedTy` in this scope
[00:16:00]    --> librustc_traits/lowering.rs:145:44
[00:16:00]     |
[00:16:00] 145 |             WellFormed(..) | FromEnv(..) | WellFormedTy(..) | FromEnvTy(..) | Normalize(..)
[00:16:00]     |                                            ^^^^^^^^^^^^ did you mean `WellFormed`?
[00:16:00] 
[00:16:00] error[E0531]: cannot find tuple struct/variant `FromEnvTy` in this scope
[00:16:00]    --> librustc_traits/lowering.rs:145:63
[00:16:00]     |
[00:16:00] 145 |             WellFormed(..) | FromEnv(..) | WellFormedTy(..) | FromEnvTy(..) | Normalize(..)
[00:16:00]     |                                                               ^^^^^^^^^ did you mean `FromEnv`?
[00:16:00] 
[00:16:00] error[E0531]: cannot find tuple struct/variant `RegionOutlives` in this scope
[00:16:00]    --> librustc_traits/lowering.rs:146:15
[00:16:00]     |
[00:16:00] 146 |             | RegionOutlives(..) | TypeOutlives(..) => self,
[00:16:00] help: possible candidates are found in other modules, you can import them into scope
[00:16:00]     |
[00:16:00]     |
[00:16:00] 11  | use rustc::traits::WhereClause::RegionOutlives;
[00:16:00]     |
[00:16:00] 11  | use rustc::ty::Predicate::RegionOutlives;
[00:16:00] 
[00:16:00] 
[00:16:00] error[E0531]: cannot find tuple struct/variant `TypeOutlives` in this scope
[00:16:00]    --> librustc_traits/lowering.rs:146:36
[00:16:00]     |
[00:16:00] 146 |             | RegionOutlives(..) | TypeOutlives(..) => self,
[00:16:00] help: possible candidates are found in other modules, you can import them into scope
[00:16:00]     |
[00:16:00]     |
[00:16:00] 11  | use rustc::traits::WhereClause::TypeOutlives;
[00:16:00] 11  | use rustc::ty::Predicate::TypeOutlives;
[00:16:00]     |
[00:16:00] 
[00:16:00] 
[00:16:00] error[E0425]: cannot find function `implied_bound_from_trait` in this scope
[00:16:00]    --> librustc_traits/lowering.rs:279:19
[00:16:00]     |
[00:16:00] 279 |         .map(|wc| implied_bound_from_trait(tcx, trait_pred, wc));
[00:16:00] 
[00:16:00] error[E0425]: cannot find value `where_clause` in this scope
[00:16:00]    --> librustc_traits/lowering.rs:301:25
[00:16:00]     |
[00:16:00]     |
[00:16:00] 301 |     let wellformed_wc = where_clause
[00:16:00]     |                         ^^^^^^^^^^^^ did you mean `where_clauses`?
[00:16:00] 
[00:16:00] error[E0425]: cannot find value `wellformed_wcs` in this scope
[00:16:00]    --> librustc_traits/lowering.rs:306:16
[00:16:00]     |
[00:16:00] 306 |     wcs.extend(wellformed_wcs);
[00:16:00]     |                ^^^^^^^^^^^^^^ did you mean `wellformed_wc`?
[00:16:00] error: unused import: `rustc::ty::subst::Substs`
[00:16:00]   --> librustc_traits/lowering.rs:18:5
[00:16:00]    |
[00:16:00] 18 | use rustc::ty::subst::Substs;
[00:16:00] 18 | use rustc::ty::subst::Substs;
[00:16:00]    |     ^^^^^^^^^^^^^^^^^^^^^^^^
[00:16:00]    |
[00:16:00]    = note: `-D unused-imports` implied by `-D warnings`
[00:16:00] 
[00:16:00] error: unused import: `self::WhereClause::*`
[00:16:00]    --> librustc_traits/lowering.rs:129:13
[00:16:00]     |
[00:16:00] 129 |         use self::WhereClause::*;
[00:16:00] 
[00:16:00]    Compiling rustc_privacy v0.0.0 (file:///checkout/src/librustc_privacy)
[00:16:00]    Compiling rustc_privacy v0.0.0 (file:///checkout/src/librustc_privacy)
[00:16:00] error[E0119]: conflicting implementations of trait `lowering::Lower<[type error]>` for type `rustc::ty::TraitPredicate<'_>`:
[00:16:00]   --> librustc_traits/lowering.rs:72:1
[00:16:00]    |
[00:16:00] 48 |   impl<'tcx> Lower<WhereClause<'tcx>> for ty::TraitPredicate<'tcx> {
[00:16:00]    |   ---------------------------------------------------------------- first implementation here
[00:16:00] ...
[00:16:00] 72 | / impl<'tcx, T> Lower<DomainGoal<'tcx>> for T
[00:16:00] 73 | | where
[00:16:00] 74 | |     T: Lower<WhereClause<'tcx>>,
[00:16:00] 75 | | {
[00:16:00] 78 | |     }
[00:16:00] 79 | | }
[00:16:00] 79 | | }
[00:16:00]    | |_^ conflicting implementation for `rustc::ty::TraitPredicate<'_>`
[00:16:00] error: aborting due to 32 previous errors
[00:16:00] 
[00:16:00] Some errors occurred: E0119, E0412, E0422, E0425, E0432, E0433, E0531.
[00:16:00] For more information about an error, try `rustc --explain E0119`.
[00:16:00] For more information about an error, try `rustc --explain E0119`.
[00:16:00] error: Could not compile `rustc_traits`.
[00:16:00] 
[00:16:00] Caused by:
[00:16:00]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_traits librustc_traits/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=c7bf1f19f47cd687 -C extra-filename=-c7bf1f19f47cd687 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-d8b3f1986e621085.rlib --extern chalk_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libchalk_engine-415e59373305337c.rlib --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-6020508f01da724d.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-023d781fbd65d983.rlib --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-1dddb0fa9d8a512f.so --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-25582ce13d3618ea.so --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-a1b02e0d020520ac.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-f36f6cb494d373be.so -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-a59a4023b81a89b2/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-ea5907e223517cf2/out` (exit code: 101)
[00:16:10] error: build failed
[00:16:10] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:16:10] expected success, got: exit code: 101
[00:16:10] expected success, got: exit code: 101
[00:16:10] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1117:9
[00:16:10] travis_fold:end:stage0-rustc

[00:16:10] travis_time:end:stage0-rustc:start=1530790513706575563,finish=1530791217503302495,duration=703796726932


[00:16:10] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:16:10] Build completed unsuccessfully in 0:11:55
[00:16:10] Makefile:28: recipe for target 'all' failed
[00:16:10] make: *** [all] Error 1
315044 ./src/llvm
241180 ./src/llvm-emscripten
210224 ./src/llvm/test
192708 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib
---
149116 ./src/llvm-emscripten/test
144328 ./obj/build/bootstrap/debug/incremental
133076 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc
129816 ./obj/build/bootstrap/debug/incremental/bootstrap-146vjsckowoo9
129812 ./obj/build/bootstrap/debug/incremental/bootstrap-146vjsckowoo9/s-f2mbdtyrln-j0orfq-2tbvj9xpkkcf2
94948 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu
94944 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release
93020 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps
89816 ./src/llvm/test/CodeGen
