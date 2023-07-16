plain
travis_time:end:0832e036:start=1544144633686904534,finish=1544144636566793490,duration=2879888956
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:05:38]    Compiling arena v0.0.0 (/checkout/src/libarena)
[00:05:38]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:05:43]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:07:08]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:07:08] error[E0252]: the name `hir` is defined multiple times
[00:07:08]   --> src/librustc/traits/util.rs:19:11
[00:07:08] 12 | use hir::{self};
[00:07:08] 12 | use hir::{self};
[00:07:08]    |           ---- previous import of the module `hir` here
[00:07:08] 19 | use hir::{self};
[00:07:08] 19 | use hir::{self};
[00:07:08]    |           ^^^^ `hir` reimported here
[00:07:08]    |
[00:07:08]    = note: `hir` must be defined only once in the type namespace of this module
[00:07:08] help: you can use `as` to change the binding name of the import
[00:07:08]    |
[00:07:08] 19 | use hir::{self as other_hir};
[00:07:08] 
[00:07:08] error[E0252]: the name `Debug` is defined multiple times
[00:07:08]   --> src/librustc/traits/mod.rs:50:5
[00:07:08]    |
[00:07:08]    |
[00:07:08] 47 | use std::fmt::Debug;
[00:07:08]    |     --------------- previous import of the trait `Debug` here
[00:07:08] 50 | use std::fmt::Debug;
[00:07:08] 50 | use std::fmt::Debug;
[00:07:08]    |     ^^^^^^^^^^^^^^^ `Debug` reimported here
[00:07:08]    |
[00:07:08]    = note: `Debug` must be defined only once in the type namespace of this module
[00:07:08] help: you can use `as` to change the binding name of the import
[00:07:08]    |
[00:07:08] 50 | use std::fmt::Debug as OtherDebug;
[00:07:08] 
[00:07:08] 
[00:07:08] error[E0252]: the name `Rc` is defined multiple times
[00:07:08]    |
[00:07:08] 48 | use std::rc::Rc;
[00:07:08] 48 | use std::rc::Rc;
[00:07:08]    |     ----------- previous import of the type `Rc` here
[00:07:08] 51 | use std::rc::Rc;
[00:07:08] 51 | use std::rc::Rc;
[00:07:08]    |     ^^^^^^^^^^^ `Rc` reimported here
[00:07:08]    |
[00:07:08]    = note: `Rc` must be defined only once in the type namespace of this module
[00:07:08] help: you can use `as` to change the binding name of the import
[00:07:08]    |
[00:07:08] 51 | use std::rc::Rc as OtherRc;
[00:07:08] 
[00:07:08] 
[00:07:10] error[E0432]: unresolved imports `self::util::expand_trait_refs`, `self::util::TraitRefExpander`, `self::util::TraitRefExpansionInfoDignosticBuilder`
[00:07:10]    |
[00:07:10]    |
[00:07:10] 74 | pub use self::util::{expand_trait_refs, TraitRefExpander, TraitRefExpansionInfoDignosticBuilder};
[00:07:10]    |                      ^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `TraitRefExpansionInfoDignosticBuilder` in `traits::util`
[00:07:10]    |                      |                  |
[00:07:10]    |                      |                  no `TraitRefExpander` in `traits::util`
[00:07:10]    |                      no `expand_trait_refs` in `traits::util`
[00:07:12] error[E0412]: cannot find type `Substs` in this scope
[00:07:12]    --> src/librustc/traits/util.rs:374:63
[00:07:12]     |
[00:07:12]     |
[00:07:12] 374 |                                                 impl_substs: &Substs<'tcx>)
[00:07:12] help: possible candidate is found in another module, you can import it into scope
[00:07:12]     |
[00:07:12] 11  | use ty::subst::Substs;
[00:07:12]     |
[00:07:12]     |
[00:07:12] 
[00:07:12] error[E0412]: cannot find type `Kind` in this scope
[00:07:12]    --> src/librustc/traits/util.rs:441:46
[00:07:12]     |
[00:07:12] 441 |                                    params: &[Kind<'tcx>])
[00:07:12] help: possible candidate is found in another module, you can import it into scope
[00:07:12]     |
[00:07:12] 11  | use ty::subst::Kind;
[00:07:12]     |
---
[00:07:13]    |
[00:07:13] 51 | use std::rc::Rc;
[00:07:13]    |     ^^^^^^^^^^^
[00:07:13] 
[00:07:36] error[E0599]: no method named `subst` found for type `ty::sty::TraitRef<'_>` in the current scope
[00:07:36]    --> src/librustc/traits/util.rs:381:24
[00:07:36]     |
[00:07:36] 381 |         impl_trait_ref.subst(selcx.tcx(), impl_substs);
[00:07:36]     | 
[00:07:36]     | 
[00:07:36]    ::: src/librustc/ty/sty.rs:649:1
[00:07:36] 649 | pub struct TraitRef<'tcx> {
[00:07:36] 649 | pub struct TraitRef<'tcx> {
[00:07:36]     | ------------------------- method `subst` not found for this
[00:07:36]     = help: items from traits can only be used if the trait is in scope
[00:07:36]     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[00:07:36]             `use ty::subst::Subst;`
[00:07:36] 
