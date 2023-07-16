plain
travis_time:end:09a8471e:start=1541664697556926973,finish=1541664750763816100,duration=53206889127
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:41:43]    Compiling parking_lot_core v0.3.0
[00:41:43]    Compiling tempfile v3.0.3
[00:41:44]    Compiling parking_lot v0.6.4
[00:41:45]    Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
[00:41:48] error[E0599]: no method named `clean` found for type `(&rustc::ty::Generics, &std::rc::Rc<rustc::ty::GenericPredicates<'_>>)` in the current scope
[00:41:48]     --> librustdoc/clean/mod.rs:1981:69
[00:41:48]      |
[00:41:48] 1981 |                                 &cx.tcx.predicates_of(self.def_id)).clean(cx);
[00:41:48]      |
[00:41:48]      = help: items from traits can only be used if the trait is implemented and in scope
[00:41:48]      = help: items from traits can only be used if the trait is implemented and in scope
[00:41:48]      = note: the following trait defines an item `clean`, perhaps you need to implement it:
[00:41:48]              candidate #1: `clean::Clean`
[00:41:48] 
[00:41:48] error[E0599]: no method named `clean` found for type `(&rustc::ty::Generics, &std::rc::Rc<rustc::ty::GenericPredicates<'_>>)` in the current scope
[00:41:48]     --> librustdoc/clean/mod.rs:2049:75
[00:41:48]      |
[00:41:48] 2049 |                     let generics = (cx.tcx.generics_of(did), &predicates).clean(cx);
[00:41:48]      |
[00:41:48]      = help: items from traits can only be used if the trait is implemented and in scope
[00:41:48]      = help: items from traits can only be used if the trait is implemented and in scope
[00:41:48]      = note: the following trait defines an item `clean`, perhaps you need to implement it:
[00:41:48]              candidate #1: `clean::Clean`
^^
[00:41:49]     |
[00:41:49]     = help: items from traits can only be used if the trait is implemented and in scope
[00:41:49]     = help: items from traits can only be used if the trait is implemented and in scope
[00:41:49]     = note: the following trait defines an item `clean`, perhaps you need to implement it:
[00:41:49]             candidate #1: `clean::Clean`
[00:41:49] 
[00:41:49] error[E0599]: no method named `clean` found for type `(&rustc::ty::Generics, &std::rc::Rc<rustc::ty::GenericPredicates<'_>>)` in the current scope
[00:41:49]    --> librustdoc/clean/inline.rs:233:58
[00:41:49]     |
[00:41:49] 233 |         generics: (cx.tcx.generics_of(did), &predicates).clean(cx),
[00:41:49]     |
[00:41:49]     = help: items from traits can only be used if the trait is implemented and in scope
[00:41:49]     = help: items from traits can only be used if the trait is implemented and in scope
[00:41:49]     = note: the following trait defines an item `clean`, perhaps you need to implement it:
[00:41:49]             candidate #1: `clean::Clean`
[00:41:49] 
[00:41:49] error[E0599]: no method named `clean` found for type `(&rustc::ty::Generics, &std::rc::Rc<rustc::ty::GenericPredicates<'_>>)` in the curreemented and in scope
[00:41:49]     = note: the following trait defines an item `clean`, perhaps you need to implement it:
[00:41:49]             candidate #1: `clean::Clean`
[00:41:49] 
[00:41:49] error[E0599]: no method named `clean` found for type `(&rustc::ty::Generics, &std::rc::Rc<rustc::ty::GenericPredicates<'_>>)` in the current scope
[00:41:49]    --> librustdoc/clean/inline.rs:272:58
[00:41:49]     |
[00:41:49] 272 |         generics: (cx.tcx.generics_of(did), &predicates).clean(cx),
[00:41:49]     |
[00:41:49]     = help: items from traits can only be used if the trait is implemented and in scope
[00:41:49]     = help: items from traits can only be used if the trait is implemented and in scope
[00:41:49]     = note: the following trait defines an item `clean`, perhaps you need to implement it:
[00:41:49]             candidate #1: `clean::Clean`
[00:41:49] 
[00:41:49] error[E0599]: no method named `clean` found for type `(&rustc::ty::Generics, &std::rc::Rc<rustc::ty::GenericPredicates<'_>>)` in the current scope
[00:41:49]    --> librustdoc/clean/inline.rs:349:49
[00:41:49]     |
[00:41:49] 349 |             (tcx.generics_of(did), &predicates).clean(cx),
[00:41:49]     |
[00:41:49]     = help: items from traits can only be used if the trait is implemented and in scope
[00:41:49]     = help: items from traits can only be used if the trait is implemented and in scope
[00:41:49]     = note: the following trait defines an item `clean`, perhaps you need to implement it:
[00:41:49]             candidate #1: `clean::Clean`
[00:41:49] 
[00:41:49] error[E0277]: the size for values of type `[clean::Item]` cannot be known at compilation time
[00:41:49]    --> librustdoc/clean/inline.rs:328:10
[00:41:49]     |
[00:41:49] 328 |     let (trait_items, generics) = if let Some(nodeid) = tcx.hir.as_local_node_id(did) {
[00:41:49]     |
[00:41:49]     |
[00:41:49]     = help: the trait `std::marker::Sized` is not implemented for `[clean::Item]`
[00:41:49]     = note: all local variables must have a statically known size
[00:41:49]     = help: unsized locals are gated as an unstable feature
[00:41:49] 
[00:41:49] error[E0308]: mismatched types
[00:41:49] error[E0308]: mismatched types
[00:41:49]    --> librustdoc/clean/inline.rs:382:20
[00:41:49]     |
[00:41:49] 382 |             items: trait_items,
[00:41:49]     |                    ^^^^^^^^^^^
[00:41:49]     |                    |
[00:41:49]     |                    expected struct `std::vec::Vec`, found slice
[00:41:49]     |                    help: try using a conversion method: `trait_items.to_vec()`
[00:41:49]     |
[00:41:49]     = note: expected type `std::vec::Vec<clean::Item>`
[00:41:49]                found type `[clean::Item]`
[00:41:49] 
[00:41:49] error[E0599]: no method named `clean` found for type `(&rustc::ty::Generics, &std::rc::Rc<rustc::ty::GenericPredicates<'_>>)` in the current scope
[00:41:49]    --> librustdoc/clean/auto_trait.rs:577:27
[00:41:49]     |
[00:41:49] 577 |         } = full_generics.clean(self.cx);
[00:41:49]     |
[00:41:49]     = help: items from traits can only be used if the trait is implemented and in scope
[00:41:49]     = help: items from traits can only be used if the trait is implemented and in scope
[00:41:49]     = note: the following trait defines an item `clean`, perhaps you need to implement it:
[00:41:49]             candidate #1: `clean::Clean`
[00:41:49] 
[00:41:49] error[E0599]: no method named `clean` found for type `(&rustc::ty::Generics, &std::rc::Rc<rustc::ty::GenericPredicates<'_>>)` in the current scope
[00:41:49]    --> librustdoc/clean/blanket_impl.rs:158:69
[00:41:49]     |
[00:41:49] 158 |                                 generics: (t_generics, &predicates).clean(self.cx),
[00:41:49]     |
[00:41:49]     = help: items from traits can only be used if the trait is implemented and in scope
[00:41:49]     = help: items from traits can only be used if the trait is implemented and in scope
[00:41:49]     = note: the following trait defines an item `clean`, perhaps you need to implement it:
[00:41:49]             candidate #1: `clean::Clean`
73608919831,duration=525918928
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:2454ae50
---
travis_time:end:01c2bf8f:start=1541667273625624969,finish=1541667273630198488,duration=4573519
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:034d09c0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:
