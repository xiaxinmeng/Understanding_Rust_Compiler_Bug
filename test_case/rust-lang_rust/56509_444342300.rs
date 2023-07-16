plain
travis_time:end:01a3c964:start=1543978689339962560,finish=1543978745188258699,duration=55848296139
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:05:34]    Compiling arena v0.0.0 (/checkout/src/libarena)
[00:05:34]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:05:39]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:07:02]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:07:13] error[E0053]: method `compute` has an incompatible type for trait
[00:07:13]    --> src/librustc/ty/query/plumbing.rs:833:22
[00:07:13] 833 |                   key: &Self::Key
[00:07:13] 833 |                   key: &Self::Key
[00:07:13]     |                        ^^^^^^^^^^ expected struct `hir::def_id::DefId`, found reference
[00:07:13]     | 
[00:07:13]    ::: src/librustc/ty/query/config.rs:56:50
[00:07:13]     |
[00:07:13] 56  |       fn compute(tcx: TyCtxt<'_, 'tcx, 'tcx>, key: Self::Key) -> Self::Value;
[00:07:13]     |                                                    --------- type in trait
[00:07:13]     | 
[00:07:13]    ::: src/librustc/ty/query/mod.rs:106:1
[00:07:13]     |
[00:07:13] 106 | / define_queries! { <'tcx>
[00:07:13] 107 | |     Other {
[00:07:13] 108 | |         /// Records the type of every item.
[00:07:13] 109 | |         [] fn type_of: TypeOfItem(DefId) -> Ty<'tcx>,
[00:07:13] 698 | |     },
[00:07:13] 699 | | }
[00:07:13]     | |_- in this macro invocation
[00:07:13]     |
[00:07:13]     |
[00:07:13]     = note: expected type `fn(ty::context::TyCtxt<'_, 'tcx, 'tcx>, hir::def_id::DefId) -> &ty::TyS<'_>`
[00:07:13]                found type `fn(ty::context::TyCtxt<'_, 'tcx, 'tcx>, &hir::def_id::DefId) -> for<'r> fn(ty::context::TyCtxt<'r, 'tcx, 'tcx>, hir::def_id::DefId) -> &ty::TyS<'_>`
[00:07:13] error[E0053]: method `compute` has an incompatible type for trait
[00:07:13] error[E0053]: method `compute` has an incompatible type for trait
[00:07:13]    --> src/librustc/ty/query/plumbing.rs:833:22
[00:07:13] 833 |                   key: &Self::Key
[00:07:13] 833 |                   key: &Self::Key
[00:07:13]     |                        ^^^^^^^^^^ expected struct `hir::def_id::DefId`, found reference
[00:07:13]     | 
[00:07:13]    ::: src/librustc/ty/query/config.rs:56:50
[00:07:13]     |
[00:07:13] 56  |       fn compute(tcx: TyCtxt<'_, 'tcx, 'tcx>, key: Self::Key) -> Self::Value;
[00:07:13]     |                                                    --------- type in trait
[00:07:13]     | 
[00:07:13]    ::: src/librustc/ty/query/mod.rs:106:1
[00:07:13]     |
[00:07:13] 106 | / define_queries! { <'tcx>
[00:07:13] 107 | |     Other {
[00:07:13] 108 | |         /// Records the type of every item.
[00:07:13] 109 | |         [] fn type_of: TypeOfItem(DefId) -> Ty<'tcx>,
[00:07:13] 698 | |     },
[00:07:13] 699 | | }
[00:07:13]     | |_- in this macro invocation
[00:07:13]     |
[00:07:13]     |
[00:07:13]     = note: expected type `fn(ty::context::TyCtxt<'_, 'tcx, 'tcx>, hir::def_id::DefId) -> &ty::Generics`
[00:07:13]                found type `fn(ty::context::TyCtxt<'_, 'tcx, 'tcx>, &hir::def_id::DefId) -> for<'r> fn(ty::context::TyCtxt<'r, 'tcx, 'tcx>, hir::def_id::DefId) -> &ty::Generics`
[00:07:13] error[E0053]: method `compute` has an incompatible type for trait
[00:07:13] error[E0053]: method `compute` has an incompatible type for trait
[00:07:13]    --> src/librustc/ty/query/plumbing.rs:833:22
[00:07:13] 833 |                   key: &Self::Key
[00:07:13] 833 |                   key: &Self::Key
[00:07:13]     |                        ^^^^^^^^^^ expected struct `hir::def_id::DefId`, found reference
[00:07:13]     | 
[00:07:13]    ::: src/librustc/ty/query/config.rs:56:50
[00:07:13]     |
[00:07:13] 56  |       fn compute(tcx: TyCtxt<'_, 'tcx, 'tcx>, key: Self::Key) -> Self::Value;
[00:07:13]     |                                                    --------- type in trait
[00:07:13]     | 
[00:07:13]    ::: src/librustc/ty/query/mod.rs:106:1
[00:07:13]     |
[00:07:13] 106 | / define_queries! { <'tcx>
[00:07:13] 107 | |     Other {
[00:07:13] 108 | |         /// Records the type of every item.
[00:07:13] 109 | |         [] fn type_of: TypeOfItem(DefId) -> Ty<'tcx>,
[00:07:13] 698 | |     },
[00:07:13] 699 | | }
[00:07:13]     | |_- in this macro invocation
[00:07:13]     |
[00:07:13]     |
[00:07:13]     = note: expected type `fn(ty::context::TyCtxt<'_, 'tcx, 'tcx>, hir::def_id::DefId) -> std::rc::Rc<ty::GenericPredicates<'_>>`
[00:07:13]                found type `fn(ty::context::TyCtxt<'_, 'tcx, 'tcx>, &hir::def_id::DefId) -> for<'r> fn(ty::context::TyCtxt<'r, 'tcx, 'tcx>, hir::def_id::DefId) -> std::rc::Rc<ty::GenericPredicates<'_>>`
[00:07:13] error[E0053]: method `compute` has an incompatible type for trait
[00:07:13] error[E0053]: method `compute` has an incompatible type for trait
[00:07:13]    --> src/librustc/ty/query/plumbing.rs:833:22
[00:07:13] 833 |                   key: &Self::Key
[00:07:13] 833 |                   key: &Self::Key
[00:07:13]     |                        ^^^^^^^^^^ expected struct `hir::def_id::DefId`, found reference
[00:07:13]     | 
[00:07:13]    ::: src/librustc/ty/query/config.rs:56:50
[00:07:13]     |
[00:07:13] 56  |       fn compute(tcx: TyCtxt<'_, 'tcx, 'tcx>, key: Self::Key) -> Self::Value;
[00:07:13]     |                                                    --------- type in trait
[00:07:13]     | 
[00:07:13]    ::: src/librustc/ty/query/mod.rs:106:1
[00:07:13]     |
[00:07:13] 106 | / define_queries! { <'tcx>
[00:07:13] 107 | |     Other {
[00:07:13] 108 | |         /// Records the type of every item.
[00:07:13] 109 | |         [] fn type_of: TypeOfItem(DefId) -> Ty<'tcx>,
[00:07:13] 698 | |     },
[00:07:13] 699 | | }
[00:07:13]     | |_- in this macro invocation
[00:07:13]     |
[00:07:13]     |
[00:07:13]     = note: expected type `fn(ty::context::TyCtxt<'_, 'tcx, 'tcx>, hir::def_id::DefId) -> std::rc::Rc<std::vec::Vec<ty::Predicate<'_>>>`
[00:07:13]                found type `fn(ty::context::TyCtxt<'_, 'tcx, 'tcx>, &hir::def_id::DefId) -> for<'r> fn(ty::context::TyCtxt<'r, 'tcx, 'tcx>, hir::def_id::DefId) -> std::rc::Rc<std::vec::Vec<ty::Predicate<'_>>>`
[00:07:13] error[E0053]: method `compute` has an incompatible type for trait
[00:07:13] error[E0053]: method `compute` has an incompatible type for trait
[00:07:13]    --> src/librustc/ty/query/plumbing.rs:833:22
[00:07:13] 833 |                   key: &Self::Key
[00:07:13]     |                        ^^^^^^^^^^ expected tuple, found reference
[00:07:13]     | 
[00:07:13]     | 
[00:07:13]    ::: src/librustc/ty/query/config.rs:56:50
[00:07:13]     |
[00:07:13] 56  |       fn compute(tcx: TyCtxt<'_, 'tcx, 'tcx>, key: Self::Key) -> Self::Value;
[00:07:13]     |                                                    --------- type in trait
[00:07:13]     | 
[00:07:13]    ::: src/librustc/ty/query/mod.rs:106:1
[00:07:13]     |
[00:07:13] 106 | / define_queries! { <'tcx>
[00:07:13] 107 | |     Other {
[00:07:13] 108 | |         /// Records the type of every item.
[00:07:13] 109 | |         [] fn type_of: TypeOfItem(DefId) -> Ty<'tcx>,
[00:07:13] 698 | |     },
[00:07:13] 699 | | }
[00:07:13]     | |_- in this macro invocation
[00:07:13]     |
[00:07:13]     |
[00:07:13]     = note: expected type `fn(ty::context::TyCtxt<'_, 'tcx, 'tcx>, (hir::def_id::DefId, hir::def_id::DefId)) -> std::rc::Rc<ty::GenericPredicates<'_>>`
[00:07:13]                found type `fn(ty::context::TyCtxt<'_, 'tcx, 'tcx>, &(hir::def_id::DefId, hir::def_id::DefId)) -> for<'r> fn(ty::context::TyCtxt<'r, 'tcx, 'tcx>, (hir::def_id::DefId, hir::def_id::DefId)) -> std::rc::Rc<ty::GenericPredicates<'_>>`
[00:07:13] error[E0053]: method `compute` has an incompatible type for trait
[00:07:13] error[E0053]: method `compute` has an incompatible type for trait
[00:07:13]    --> src/librustc/ty/query/plumbing.rs:833:22
[00:07:13] 833 |                   key: &Self::Key
[00:07:13] 833 |                   key: &Self::Key
[00:07:13]     |                        ^^^^^^^^^^ expected struct `hir::def_id::DefId`, found reference
[00:07:13]     | 
[00:07:13]    ::: src/librustc/ty/query/config.rs:56:50
[00:07:13]     |
[00:07:13] 56  |       fn compute(tcx: TyCtxt<'_, 'tcx, 'tcx>, key: Self::Key) -> Self::Value;
[00:07:13]     |                                                    --------- type in trait
[00:07:13]     | 
[00:07:13]    ::: src/librustc/ty/query/mod.rs:106:1
[00:07:13]     |
[00:07:13] 106 | / define_queries! { <'tcx>
[00:07:13] 107 | |     Other {
[00:07:13] 108 | |         /// Records the type of every item.
[00:07:13] 109 | |         [] fn type_of: TypeOfItem(DefId) -> Ty<'tcx>,
[00:07:13] 698 | |     },
[00:07:13] 699 | | }
[00:07:13]     | |_- in this macro invocation
[00:07:13]     |
[00:07:13]     |
[00:07:13]     = note: expected type `fn(ty::context::TyCtxt<'_, 'tcx, 'tcx>, hir::def_id::DefId) -> &ty::trait_def::TraitDef`
[00:07:13]                found type `fn(ty::context::TyCtxt<'_, 'tcx, 'tcx>, &hir::def_id::DefId) -> for<'r> fn(ty::context::TyCtxt<'r, 'tcx, 'tcx>, hir::def_id::DefId) -> &ty::trait_def::TraitDef`
[00:07:13] error[E0053]: method `compute` has an incompatible type for trait
[00:07:13] error[E0053]: method `compute` has an incompatible type for trait
[00:07:13]    --> src/librustc/ty/query/plumbing.rs:833:22
[00:07:13] 833 |                   key: &Self::Key
[00:07:13] 833 |                   key: &Self::Key
[00:07:13]     |                        ^^^^^^^^^^ expected struct `hir::def_id::DefId`, found reference
[00:07:13]     | 
[00:07:13]    ::: src/librustc/ty/query/config.rs:56:50
[00:07:13]     |
[00:07:13] 56  |       fn compute(tcx: TyCtxt<'_, 'tcx, 'tcx>, key: Self::Key) -> Self::Value;
[00:07:13]     |                                                    --------- type in trait
[00:07:13]     | 
[00:07:13]    ::: src/librustc/ty/query/mod.rs:106:1
[00:07:13]     |
[00:07:13] 106 | / define_queries! { <'tcx>
[00:07:13] 107 | |     Other {
[00:07:13] 108 | |         /// Records the type of every item.
[00:07:13] 109 | |         [] fn type_of: TypeOfItem(DefId) -> Ty<'tcx>,
[00:07:13] 698 | |     },
[00:07:13] 699 | | }
[00:07:13]     | |_- in this macro invocation
[00:07:13]     |
[00:07:13]     |
[00:07:13]     = note: expected type `fn(ty::context::TyCtxt<'_, 'tcx, 'tcx>, hir::def_id::DefId) -> &ty::AdtDef`
[00:07:13]                found type `fn(ty::context::TyCtxt<'_, 'tcx, 'tcx>, &hir::def_id::DefId) -> for<'r> fn(ty::context::TyCtxt<'r, 'tcx, 'tcx>, hir::def_id::DefId) -> &ty::AdtDef`
[00:07:13] error[E0053]: method `compute` has an incompatible type for trait
[00:07:13] error[E0053]: method `compute` has an incompatible type for trait
[00:07:13]    --> src/librustc/ty/query/plumbing.rs:833:22
[00:07:13] 833 |                   key: &Self::Key
[00:07:13] 833 |                   key: &Self::Key
[00:07:13]     |                        ^^^^^^^^^^ expected struct `hir::def_id::DefId`, found reference
[00:07:13]     | 
[00:07:13]    ::: src/librustc/ty/query/config.rs:56:50
[00:07:13]     |
[00:07:13] 56  |       fn compute(tcx: TyCtxt<'_, 'tcx, 'tcx>, key: Self::Key) -> Self::Value;
[00:07:13]     |                                                    --------- type in trait
[00:07:13]     | 
[00:07:13]    ::: src/librustc/ty/query/mod.rs:106:1
[00:07:13]     |
[00:07:13] 106 | / define_queries! { <'tcx>
[00:07:13] 107 | |     Other {
[00:07:13] 108 | |         /// Records the type of every item.
[00:07:13] 109 | |         [] fn type_of: TypeOfItem(DefId) -> Ty<'tcx>,
[00:07:13] 698 | |     },
[00:07:13] 699 | | }
[00:07:13]     | |_- in this macro invocation
[00:07:13]     |
[00:07:13]     |
[00:07:13]     = note: expected type `fn(ty::context::TyCtxt<'_, 'tcx, 'tcx>, hir::def_id::DefId) -> std::option::Option<ty::Destructor>`
[00:07:13]                found type `fn(ty::context::TyCtxt<'_, 'tcx, 'tcx>, &hir::def_id::DefId) -> for<'r> fn(ty::context::TyCtxt<'r, 'tcx, 'tcx>, hir::def_id::DefId) -> std::option::Option<ty::Destructor>`
[00:07:13] error[E0053]: method `compute` has an incompatible type for trait
[00:07:13] error[E0053]: method `compute` has an incompatible type for trait
[00:07:13]    --> src/librustc/ty/query/plumbing.rs:833:22
[00:07:13] 833 |                   key: &Self::Key
[00:07:13] 833 |                   key: &Self::Key
[00:07:13]     |                        ^^^^^^^^^^ expected struct `hir::def_id::DefId`, found reference
[00:07:13]     | 
[00:07:13]    ::: src/librustc/ty/query/config.rs:56:50
[00:07:13]     |
[00:07:13] 56  |       fn compute(tcx: TyCtxt<'_, 'tcx, 'tcx>, key: Self::Key) -> Self::Value;
[00:07:13]     |                                                    --------- type in trait
[00:07:13]     | 
[00:07:13]    ::: src/librustc/ty/query/mod.rs:106:1
[00:07:13]     |
[00:07:13] 106 | / define_queries! { <'tcx>
[00:07:13] 107 | |     Other {
[00:07:13] 108 | |         /// Records the type of every item.
[00:07:13] 109 | |         [] fn type_of: TypeOfItem(DefId) -> Ty<'tcx>,
[00:07:13] 698 | |     },
[00:07:13] 699 | | }
[00:07:13]     | |_- in this macro invocation
[00:07:13]     |
[00:07:13]     |
[00:07:13]     = note: expected type `fn(ty::context::TyCtxt<'_, 'tcx, 'tcx>, hir::def_id::DefId) -> &[&ty::TyS<'_>]`
[00:07:13]                found type `fn(ty::context::TyCtxt<'_, 'tcx, 'tcx>, &hir::def_id::DefId) -> for<'r> fn(ty::context::TyCtxt<'r, 'tcx, 'tcx>, hir::def_id::DefId) -> &[&ty::TyS<'_>]`
[00:07:13] error[E0053]: method `compute` has an incompatible type for trait
[00:07:13] error[E0053]: method `compute` has an incompatible type for trait
[00:07:13]    --> src/librustc/ty/query/plumbing.rs:833:22
[00:07:13] 833 |                   key: &Self::Key
[00:07:13] 833 |                   key: &Self::Key
[00:07:13]     |                        ^^^^^^^^^^ expected struct `hir::def_id::DefId`, found reference
[00:07:13]     | 
[00:07:13]    ::: src/librustc/ty/query/config.rs:56:50
[00:07:13]     |
[00:07:13] 56  |       fn compute(tcx: TyCtxt<'_, 'tcx, 'tcx>, key: Self::Key) -> Self::Value;
[00:07:13]     |                                                    --------- type in trait
[00:07:13]     | 
[00:07:13]    ::: src/librustc/ty/query/mod.rs:106:1
[00:07:13]     |
[00:07:13] 106 | / define_queries! { <'tcx>
[00:07:13] 107 | |     Other {
[00:07:13] 108 | |         /// Records the type of every item.
[00:07:13] 109 | |         [] fn type_of: TypeOfItem(DefId) -> Ty<'tcx>,
[00:07:13] 698 | |     },
[00:07:13] 699 | | }
[00:07:13]     | |_- in this macro invocation
[00:07:13]     |
[00:07:13]     |
[00:07:13]     = note: expected type `fn(ty::context::TyCtxt<'_, 'tcx, 'tcx>, hir::def_id::DefId) -> std::result::Result<traits::query::dropck_outlives::DtorckConstraint<'_>, traits::query::NoSolution>`
[00:07:13]                found type `fn(ty::context::TyCtxt<'_, 'tcx, 'tcx>, &hir::def_id::DefId) -> for<'r> fn(ty::context::TyCtxt<'r, 'tcx, 'tcx>, hir::def_id::DefId) -> std::result::Result<traits::query::dropck_outlives::DtorckConstraint<'_>, traits::query::NoSolution>`
[00:07:13] error[E0053]: method `compute` has an incompatible type for trait
[00:07:13] error[E0053]: method `compute` has an incompatible type for trait
[00:07:13]    --> src/librustc/ty/query/plumbing.rs:833:22
[00:07:13] 833 |                   key: &Self::Key
[00:07:13] 833 |                   key: &Self::Key
[00:07:13]     |                        ^^^^^^^^^^ expected struct `hir::def_id::DefId`, found reference
[00:07:13]     | 
[00:07:13]    ::: src/librustc/ty/query/config.rs:56:50
[00:07:13]     |
[00:07:13] 56  |       fn compute(tcx: TyCtxt<'_, 'tcx, 'tcx>, key: Self::Key) -> Self::Value;
[00:07:13]     |                                                    --------- type in trait
[00:07:13]     | 
[00:07:13]    ::: src/librustc/ty/query/mod.rs:106:1
[00:07:13]     |
[00:07:13] 106 | / define_queries! { <'tcx>
[00:07:13] 107 | |     Other {
[00:07:13] 108 | |         /// Records the type of every item.
[00:07:13] 109 | |         [] fn type_of: TypeOfItem(DefId) -> Ty<'tcx>,
[00:07:13] 698 | |     },
[00:07:13] 699 | | }
[00:07:13]     | |_- in this macro invocation
[00:07:13]     |
[00:07:13]     |
[00:07:13]     = note: expected type `fn(ty::context::TyCtxt<'_, 'tcx, 'tcx>, hir::def_id::DefId) -> bool`
[00:07:13]                found type `fn(ty::context::TyCtxt<'_, 'tcx, 'tcx>, &hir::def_id::DefId) -> for<'r> fn(ty::context::TyCtxt<'r, 'tcx, 'tcx>, hir::def_id::DefId) -> bool`
[00:07:13] error[E0053]: method `compute` has an incompatible type for trait
[00:07:13] error[E0053]: method `compute` has an incompatible type for trait
[00:07:13]    --> src/librustc/ty/query/plumbing.rs:833:22
[00:07:13] 833 |                   key: &Self::Key
[00:07:13] 833 |                   key: &Self::Key
[00:07:13]     |                        ^^^^^^^^^^ expected enum `hir::def_id::CrateNum`, found reference
[00:07:13]     | 
[00:07:13]    ::: src/librustc/ty/query/config.rs:56:50
[00:07:13]     |
[00:07:13] 56  |       fn compute(tcx: TyCtxt<'_, 'tcx, 'tcx>, key: Self::Key) -> Self::Value;
[00:07:13]     |                                                    --------- type in trait
[00:07:13]     | 
[00:07:13]    ::: src/librustc/ty/query/mod.rs:106:1
[00:07:13]     |
[00:07:13] 106 | / define_queries! { <'tcx>
[00:07:13] 107 | |     Other {
[00:07:13] 108 | |         /// Records the type of every item.
[00:07:13] 109 | |         [] fn type_of: TypeOfItem(DefId) -> Ty<'tcx>,
[00:07:13] 698 | |     },
[00:07:13] 699 | | }
[00:07:13]     | |_- in this macro invocation
[00:07:13]     |
[00:07:13]     |
[00:07:13]     = note: expected type `fn(ty::context::TyCtxt<'_, 'tcx, 'tcx>, hir::def_id::CrateNum) -> std::rc::Rc<ty::CrateVariancesMap>`
[00:07:13]                found type `fn(ty::context::TyCtxt<'_, 'tcx, 'tcx>, &hir::def_id::CrateNum) -> for<'r> fn(ty::context::TyCtxt<'r, 'tcx, 'tcx>, hir::def_id::CrateNum) -> std::rc::Rc<ty::CrateVariancesMap>`
[00:07:13] error[E0053]: method `compute` has an incompatible type for trait
[00:07:13] error[E0053]: method `compute` has an incompatible type for trait
[00:07:13]    --> src/librustc/ty/query/plumbing.rs:833:22
[00:07:13] 833 |                   key: &Self::Key
[00:07:13] 833 |                   key: &Self::Key
[00:07:13]     |                        ^^^^^^^^^^ expected struct `hir::def_id::DefId`, found reference
[00:07:13]     | 
[00:07:13]    ::: src/librustc/ty/query/config.rs:56:50
[00:07:13]     |
[00:07:13] 56  |       fn compute(tcx: TyCtxt<'_, 'tcx, 'tcx>, key: Self::Key) -> Self::Value;
[00:07:13]     |                                                    --------- type in trait
[00:07:13]     | 
[00:07:13]    ::: src/librustc/ty/query/mod.rs:106:1
[00:07:13]     |
[00:07:13] 106 | / define_queries! { <'tcx>
[00:07:13] 107 | |     Other {
[00:07:13] 108 | |         /// Records the type of every item.
[00:07:13] 109 | |         [] fn type_of: TypeOfItem(DefId) -> Ty<'tcx>,
[00:07:13] 698 | |     },
[00:07:13] 699 | | }
[00:07:13]     | |_- in this macro invocation
[00:07:13]     |
[00:07:13]     |
[00:07:13]     = note: expected type `fn(ty::context::TyCtxt<'_, 'tcx, 'tcx>, hir::def_id::DefId) -> std::rc::Rc<std::vec::Vec<ty::Variance>>`
[00:07:13]                found type `fn(ty::context::TyCtxt<'_, 'tcx, 'tcx>, &hir::def_id::DefId) -> for<'r> fn(ty::context::TyCtxt<'r, 'tcx, 'tcx>, hir::def_id::DefId) -> std::rc::Rc<std::vec::Vec<ty::Variance>>`
[00:07:13] error[E0053]: method `compute` has an incompatible type for trait
[00:07:13] error[E0053]: method `compute` has an incompatible type for trait
[00:07:13]    --> src/librustc/ty/query/plumbing.rs:833:22
[00:07:13] 833 |                   key: &Self::Key
[00:07:13] 833 |                   key: &Self::Key
[00:07:13]     |                        ^^^^^^^^^^ expected enum `hir::def_id::CrateNum`, found reference
[00:07:13]     | 
[00:07:13]    ::: src/librustc/ty/query/config.rs:56:50
[00:07:13]     |
[00:07:13] 56  |       fn compute(tcx: TyCtxt<'_, 'tcx, 'tcx>, key: Self::Key) -> Self::Value;
[00:07:13]     |                                                    --------- type in trait
[00:07:13]     | 
[00:07:13]    ::: src/librustc/ty/query/mod.rs:106:1
[00:07:13]     |
[00:07:13] 106 | / define_queries! { <'tcx>
[00:07:13] 107 | |     Other {
[00:07:13] 108 | |         /// Records the type of every item.
[00:07:13] 109 | |         [] fn type_of: TypeOfItem(DefId) -> Ty<'tcx>,
[00:07:13] 698 | |     },
[00:07:13] 699 | | }
[00:07:13]     | |_- in this macro invocation
[00:07:13]     |
[00:07:13]     |
[00:07:13]     = note: expected type `fn(ty::context::TyCtxt<'_, 'tcx, 'tcx>, hir::def_id::CrateNum) -> std::rc::Rc<ty::CratePredicatesMap<'_>>`
[00:07:13]                found type `fn(ty::context::TyCtxt<'_, 'tcx, 'tcx>, &hir::def_id::CrateNum) -> for<'r> fn(ty::context::TyCtxt<'r, 'tcx, 'tcx>, hir::def_id::CrateNum) -> std::rc::Rc<ty::CratePredicatesMap<'_>>`
[00:07:13] error[E0053]: method `compute` has an incompatible type for trait
[00:07:13] error[E0053]: method `compute` has an incompatible type for trait
[00:07:13]    --> src/librustc/ty/query/plumbing.rs:833:22
[00:07:13] 833 |                   key: &Self::Key
[00:07:13] 833 |                   key: &Self::Key
[00:07:13]     |                        ^^^^^^^^^^ expected struct `hir::def_id::DefId`, found reference
[00:07:13]     | 
[00:07:13]    ::: src/librustc/ty/query/config.rs:56:50
[00:07:13]     |
[00:07:13] 56  |       fn compute(tcx: TyCtxt<'_, 'tcx, 'tcx>, key: Self::Key) -> Self::Value;
[00:07:13]     |                                                    --------- type in trait
[00:07:13]     | 
[00:07:13]    ::: src/librustc/ty/query/mod.rs:106:1
[00:07:13]     |
[00:07:13] 106 | / define_queries! { <'tcx>
[00:07:13] 107 | |     Other {
[00:07:13] 108 | |         /// Records the type of every item.
[00:07:13] 109 | |         [] fn type_of: TypeOfItem(DefId) -> Ty<'tcx>,
[00:07:13] 698 | |     },
[00:07:13] 699 | | }
[00:07:13]     | |_- in this macro invocation
[00:07:13]     |
[00:07:13]     |
[00:07:13]     = note: expected type `fn(ty::context::TyCtxt<'_, 'tcx, 'tcx>, hir::def_id::DefId) -> std::rc::Rc<std::vec::Vec<hir::def_id::DefId>>`
[00:07:13]                found type `fn(ty::context::TyCtxt<'_, 'tcx, 'tcx>, &hir::def_id::DefId) -> for<'r> fn(ty::context::TyCtxt<'r, 'tcx, 'tcx>, hir::def_id::DefId) -> std::rc::Rc<std::vec::Vec<hir::def_id::DefId>>`
[00:07:13] error[E0053]: method `compute` has an incompatible type for trait
[00:07:13] error[E0053]: method `compute` has an incompatible type for trait
[00:07:13]    --> src/librustc/ty/query/plumbing.rs:833:22
[00:07:13] 833 |                   key: &Self::Key
[00:07:13] 833 |                   key: &Self::Key
[00:07:13]     |                        ^^^^^^^^^^ expected struct `hir::def_id::DefId`, found reference
[00:07:13]     | 
[00:07:13]    ::: src/librustc/ty/query/config.rs:56:50
[00:07:13]     |
[00:07:13] 56  |       fn compute(tcx: TyCtxt<'_, 'tcx, 'tcx>, key: Self::Key) -> Self::Value;
[00:07:13]     |                                                    --------- type in trait
[00:07:13]     | 
[00:07:13]    ::: src/librustc/ty/query/mod.rs:106:1
[00:07:13]     |
[00:07:13] 106 | / define_queries! { <'tcx>
[00:07:13] 107 | |     Other {
[00:07:13] 108 | |         /// Records the type of every item.
[00:07:13] 109 | |         [] fn type_of: TypeOfItem(DefId) -> Ty<'tcx>,
[00:07:13] 698 | |     },
[00:07:13] 699 | | }
[00:07:13]     | |_- in this macro invocation
[00:07:13]     |
[00:07:13]     |
[00:07:13]     = note: expected type `fn(ty::context::TyCtxt<'_, 'tcx, 'tcx>, hir::def_id::DefId) -> ty::AssociatedItem`
[00:07:13]                found type `fn(ty::context::TyCtxt<'_, 'tcx, 'tcx>, &hir::def_id::DefId) -> for<'r> fn(ty::context::TyCtxt<'r, 'tcx, 'tcx>, hir::def_id::DefId) -> ty::AssociatedItem`
[00:07:13] error[E0053]: method `compute` has an incompatible type for trait
[00:07:13] error[E0053]: method `compute` has an incompatible type for trait
[00:07:13]    --> src/librustc/ty/query/plumbing.rs:833:22
[00:07:13] 833 |                   key: &Self::Key
[00:07:13] 833 |                   key: &Self::Key
[00:07:13]     |                        ^^^^^^^^^^ expected struct `hir::def_id::DefId`, found reference
[00:07:13]     | 
[00:07:13]    ::: src/librustc/ty/query/config.rs:56:50
[00:07:13]     |
[00:07:13] 56  |       fn compute(tcx: TyCtxt<'_, 'tcx, 'tcx>, key: Self::Key) -> Self::Value;
[00:07:13]     |                                                    --------- type in trait
[00:07:13]     | 
[00:07:13]    ::: src/librustc/ty/query/mod.rs:106:1
[00:07:13]     |
[00:07:13] 106 | / define_queries! { <'tcx>
[00:07:13] 107 | |     Other {
[00:07:13] 108 | |         /// Records the type of every item.
[00:07:13] 109 | |         [] fn type_of: TypeOfItem(DefId) -> Ty<'tcx>,
[00:07:13] 698 | |     },
[00:07:13] 699 | | }
[00:07:13]     | |_- in this macro invocation
[00:07:13]     |
[00:07:13]     |
[00:07:13]     = note: expected type `fn(ty::context::TyCtxt<'_, 'tcx, 'tcx>, hir::def_id::DefId) -> std::option::Option<ty::sty::TraitRef<'_>>`
[00:07:13]                found type `fn(ty::context::TyCtxt<'_, 'tcx, 'tcx>, &hir::def_id::DefId) -> for<'r> fn(ty::context::TyCtxt<'r, 'tcx, 'tcx>, hir::def_id::DefId) -> std::option::Option<ty::sty::TraitRef<'_>>`
[00:07:13] error[E0053]: method `compute` has an incompatible type for trait
[00:07:13] error[E0053]: method `compute` has an incompatible type for trait
[00:07:13]    --> src/librustc/ty/query/plumbing.rs:833:22
[00:07:13] 833 |                   key: &Self::Key
[00:07:13] 833 |                   key: &Self::Key
[00:07:13]     |                        ^^^^^^^^^^ expected struct `hir::def_id::DefId`, found reference
[00:07:13]     | 
[00:07:13]    ::: src/librustc/ty/query/config.rs:56:50
[00:07:13]     |
[00:07:13] 56  |       fn compute(tcx: TyCtxt<'_, 'tcx, 'tcx>, key: Self::Key) -> Self::Value;
[00:07:13]     |                                                    --------- type in trait
[00:07:13]     | 
[00:07:13]    ::: src/librustc/ty/query/mod.rs:106:1
[00:07:13]     |
[00:07:13] 106 | / define_queries! { <'tcx>
[00:07:13] 107 | |     Other {
[00:07:13] 108 | |         /// Records the type of every item.
[00:07:13] 109 | |         [] fn type_of: TypeOfItem(DefId) -> Ty<'tcx>,
[00:07:13] 698 | |     },
[00:07:13] 699 | | }
[00:07:13]     | |_- in this macro invocation
[00:07:13]     |
[00:07:13]     |
[00:07:13]     = note: expected type `fn(ty::context::TyCtxt<'_, 'tcx, 'tcx>, hir::def_id::DefId) -> hir::ImplPolarity`
[00:07:13]                found type `fn(ty::context::TyCtxt<'_, 'tcx, 'tcx>, &hir::def_id::DefId) -> for<'r> fn(ty::context::TyCtxt<'r, 'tcx, 'tcx>, hir::def_id::DefId) -> hir::ImplPolarity`
[00:07:13] error[E0053]: method `compute` has an incompatible type for trait
[00:07:13] error[E0053]: method `compute` has an incompatible type for trait
[00:07:13]    --> src/librustc/ty/query/plumbing.rs:833:22
[00:07:13] 833 |                   key: &Self::Key
[00:07:13] 833 |                   key: &Self::Key
[00:07:13]     |                        ^^^^^^^^^^ expected enum `hir::def_id::CrateNum`, found reference
[00:07:13]     | 
[00:07:13]    ::: src/librustc/ty/query/config.rs:56:50
[00:07:13]     |
[00:07:13] 56  |       fn compute(tcx: TyCtxt<'_, 'tcx, 'tcx>, key: Self::Key) -> Self::Value;
[00:07:13]     |                                                    --------- type in trait
[00:07:13]     | 
[00:07:13]    ::: src/librustc/ty/query/mod.rs:106:1
[00:07:13]     |
[00:07:13] 106 | / define_queries! { <'tcx>
[00:07:13] 107 | |     Other {
[00:07:13] 108 | |         /// Records the type of every item.
[00:07:13] 109 | |         [] fn type_of: TypeOfItem(DefId) -> Ty<'tcx>,
[00:07:13] 698 | |     },
[00:07:13] 699 | | }
[00:07:13]     | |_- in this macro invocation
[00:07:13]     |
[00:07:13]     |
[00:07:13]     = note: expected type `fn(ty::context::TyCtxt<'_, 'tcx, 'tcx>, hir::def_id::CrateNum) -> std::rc::Rc<std::collections::HashSet<hir::def_id::DefId, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>>`
[00:07:13]                found type `fn(ty::context::TyCtxt<'_, 'tcx, 'tcx>, &hir::def_id::CrateNum) -> for<'r> fn(ty::context::TyCtxt<'r, 'tcx, 'tcx>, hir::def_id::CrateNum) -> std::rc::Rc<std::collections::HashSet<hir::def_id::DefId, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>>`
[00:07:13] error[E0053]: method `compute` has an incompatible type for trait
[00:07:13] error[E0053]: method `compute` has an incompatible type for trait
[00:07:13]    --> src/librustc/ty/query/plumbing.rs:833:22
[00:07:13] 833 |                   key: &Self::Key
[00:07:13] 833 |                   key: &Self::Key
[00:07:13]     |                        ^^^^^^^^^^ expected struct `hir::def_id::DefId`, found reference
[00:07:13]     | 
[00:07:13]    ::: src/librustc/ty/query/config.rs:56:50
[00:07:13]     |
[00:07:13] 56  |       fn compute(tcx: TyCtxt<'_, 'tcx, 'tcx>, key: Self::Key) -> Self::Value;
[00:07:13]     |                                                    --------- type in trait
[00:07:13]     | 
[00:07:13]    ::: src/librustc/ty/query/mod.rs:106:1
[00:07:13]     |
[00:07:13] 106 | / define_queries! { <'tcx>
[00:07:13] 107 | |     Other {
[00:07:13] 108 | |         /// Records the type of every item.
[00:07:13] 109 | |         [] fn type_of: TypeOfItem(DefId) -> Ty<'tcx>,
[00:07:13] 698 | |     },
[00:07:13] 699 | | }
[00:07:13]     | |_- in this macro invocation
[00:07:13]     |
