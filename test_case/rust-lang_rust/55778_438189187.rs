plain
travis_time:end:0faa3490:start=1542097378742963785,finish=1542097435815265325,duration=57072301540
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:40:40]    Compiling parking_lot_core v0.3.0
[00:40:40]    Compiling tempfile v3.0.3
[00:40:41]    Compiling parking_lot v0.6.4
[00:40:42]    Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
[00:40:45] error[E0599]: no method named `clean` found for type `(&rustc::ty::Generics, &std::rc::Rc<rustc::ty::GenericPredicates<'_>>)` in the current scope
[00:40:45]     --> librustdoc/clean/mod.rs:1981:69
[00:40:45]      |
[00:40:45] 1981 |                                 &cx.tcx.predicates_of(self.def_id)).clean(cx);
[00:40:45]      |
[00:40:45]      = help: items from traits can only be used if the trait is implemented and in scope
[00:40:45]      = help: items from traits can only be used if the trait is implemented and in scope
[00:40:45]      = note: the following trait defines an item `clean`, perhaps you need to implement it:
[00:40:45]              candidate #1: `clean::Clean`
[00:40:45] 
[00:40:45] error[E0599]: no method named `clean` found for type `(&rustc::ty::Generics, &std::rc::Rc<rustc::ty::GenericPredicates<'_>>)` in the current scope
[00:40:45]     --> librustdoc/clean/mod.rs:2049:75
[00:40:45]      |
[00:40:45] 2049 |                     let generics = (cx.tcx.generics_of(did), &predicates).clean(cx);
[00:40:45]      |
[00:40:45]      = help: items from traits can only be used if the trait is implemented and in scope
[00:40:45]      = help: items from traits can only be used if the trait is implemented and in scope
[00:40:45]      = note: the following trait defines an item `clean`, perhaps you need to implement it:
[00:40:45]              candidate #1: `clean::Clean`
[00:40:45] 
[00:40:46] error[E0599]: no method named `clean` found for type `(&rustc::ty::Generics, &std::rc::Rc<rustc::ty::GenericPredicates<'_>>)` in the current scope
[00:40:46]    --> librustdoc/clean/inline.rs:191:59
[00:40:46]     |
[00:40:46] 191 |     let generics = (cx.tcx.generics_of(did), &predicates).clean(cx);
[00:40:46]     |
[00:40:46]     = help: items from traits can only be used if the trait is implemented and in scope
[00:40:46]     = help: items from traits can only be used if the trait is implemented and in scope
[00:40:46]     = note: the following trait defines an item `clean`, perhaps you need to implement it:
[00:40:46]             candidate #1: `clean::Clean`
[00:40:46] 
[00:40:46] error[E0599]: no method named `clean` found for type `(&rustc::ty::Generics, &std::rc::Rc<rustc::ty::GenericPredicates<'_>>)` in the current scope
[00:40:46]    --> librustdoc/clean/inline.rs:219:58
[00:40:46]     |
[00:40:46] 219 |         generics: (cx.tcx.generics_of(did), &predicates).clean(cx),
[00:40:46]     |
[00:40:46]     = help: items from traits can only be used if the trait is implemented and in scope
[00:40:46]     = help: items from traits can only be used if the trait is implemented and in scope
[00:40:46]     = note: the following trait defines an item `clean`, perhaps you need to implement it:
[00:40:46]             candidate #1: `clean::Clean`
[00:40:46] 
[00:40:46] error[E0599]: no method named `clean` found for type `(&rustc::ty::Generics, &std::rc::Rc<rustc::ty::GenericPredicates<'_>>)` in the current scope
[00:40:46]    --> librustdoc/clean/inline.rs:233:58
[00:40:46]     |
[00:40:46] 233 |         generics: (cx.tcx.generics_of(did), &predicates).clean(cx),
[00:40:46]     |
[00:40:46]     = help: items from traits can only be used if the trait is implemented and in scope
[00:40:46]     = help: items from traits can only be used if the trait is implemented and in scope
[00:40:46]     = note: the following trait defines an item `clean`, perhaps you need to implement it:
[00:40:46]             candidate #1: `clean::Clean`
[00:40:46] 
[00:40:46] error[E0599]: no method named `clean` found for type `(&rustc::ty::Generics, &std::rc::Rc<rustc::ty::GenericPredicates<'_>>)` in the current scope
[00:40:46]    --> librustdoc/clean/inline.rs:249:58
[00:40:46]     |
[00:40:46] 249 |         generics: (cx.tcx.generics_of(did), &predicates).clean(cx),
[00:40:46]     |
[00:40:46]     = help: items from traits can only be used if the trait is implemented and in scope
[00:40:46]     = help: items from traits can only be used if the trait is implemented and in scope
[00:40:46]     = note: the following trait defines an item `clean`, perhaps you need to implement it:
[00:40:46]             candidate #1: `clean::Clean`
[00:40:46] 
[00:40:46] error[E0599]: no method named `clean` found for type `(&rustc::ty::Generics, &std::rc::Rc<rustc::ty::GenericPredicates<'_>>)` in the current scope
[00:40:46]    --> librustdoc/clean/inline.rs:261:58
[00:40:46]     |
[00:40:46] 261 |         generics: (cx.tcx.generics_of(did), &predicates).clean(cx),
[00:40:46]     |
[00:40:46]     = help: items from traits can only be used if the trait is implemented and in scope
[00:40:46]     = help: items from traits can only be used if the trait is implemented and in scope
[00:40:46]     = note: the following trait defines an item `clean`, perhaps you need to implement it:
[00:40:46]             candidate #1: `clean::Clean`
[00:40:46] 
[00:40:46] error[E0599]: no method named `clean` found for type `(&rustc::ty::Generics, &std::rc::Rc<rustc::ty::GenericPredicates<'_>>)` in the current scope
[00:40:46]    --> librustdoc/clean/inline.rs:272:58
[00:40:46]     |
[00:40:46] 272 |         generics: (cx.tcx.generics_of(did), &predicates).clean(cx),
[00:40:46]     |
[00:40:46]     = help: items from traits can only be used if the trait is implemented and in scope
[00:40:46]     = help: items from traits can only be used if the trait is implemented and in scope
[00:40:46]     = note: the following trait defines an item `clean`, perhaps you need to implement it:
[00:40:46]             candidate #1: `clean::Clean`
[00:40:46] 
[00:40:46] error[E0599]: no method named `clean` found for type `(&rustc::ty::Generics, &std::rc::Rc<rustc::ty::GenericPredicates<'_>>)` in the current scope
[00:40:46]    --> librustdoc/clean/inline.rs:349:49
[00:40:46]     |
[00:40:46] 349 |             (tcx.generics_of(did), &predicates).clean(cx),
[00:40:46]     |
[00:40:46]     = help: items from traits can only be used if the trait is implemented and in scope
[00:40:46]     = help: items from traits can only be used if the trait is implemented and in scope
[00:40:46]     = note: the following trait defines an item `clean`, perhaps you need to implement it:
[00:40:46]             candidate #1: `clean::Clean`
[00:40:46] 
[00:40:46] error[E0277]: the size for values of type `[clean::Item]` cannot be known at compilation time
[00:40:46]    --> librustdoc/clean/inline.rs:328:10
[00:40:46]     |
[00:40:46] 328 |     let (trait_items, generics) = if let Some(nodeid) = tcx.hir.as_local_node_id(did) {
[00:40:46]     |
[00:40:46]     |
[00:40:46]     = help: the trait `std::marker::Sized` is not implemented for `[clean::Item]`
[00:40:46]     = note: all local variables must have a statically known size
[00:40:46]     = help: unsized locals are gated as an unstable feature
[00:40:46] 
[00:40:46] error[E0308]: mismatched types
[00:40:46] error[E0308]: mismatched types
[00:40:46]    --> librustdoc/clean/inline.rs:382:20
[00:40:46]     |
[00:40:46] 382 |             items: trait_items,
[00:40:46]     |                    ^^^^^^^^^^^
[00:40:46]     |                    |
[00:40:46]     |                    expected struct `std::vec::Vec`, found slice
[00:40:46]     |                    help: try using a conversion method: `trait_items.to_vec()`
[00:40:46]     |
[00:40:46]     = note: expected type `std::vec::Vec<clean::Item>`
[00:40:46]                found type `[clean::Item]`
[00:40:46] 
[00:40:46] error[E0599]: no method named `clean` found for type `(&rustc::ty::Generics, &std::rc::Rc<rustc::ty::GenericPredicates<'_>>)` in the current scope
[00:40:46]    --> librustdoc/clean/auto_trait.rs:577:27
[00:40:46]     |
[00:40:46] 577 |         } = full_generics.clean(self.cx);
[00:40:46]     |
[00:40:46]     = help: items from traits can only be used if the trait is implemented and in scope
[00:40:46]     = help: items from traits can only be used if the trait is implemented and in scope
[00:40:46]     = note: the following trait defines an item `clean`, perhaps you need to implement it:
[00:40:46]             candidate #1: `clean::Clean`
[00:40:46] 
[00:40:46] error[E0599]: no method named `clean` found for type `(&rustc::ty::Generics, &std::rc::Rc<rustc::ty::GenericPredicates<'_>>)` in the current scope
[00:40:46]    --> librustdoc/clean/blanket_impl.rs:158:69
[00:40:46]     |
[00:40:46] 158 |                                 generics: (t_generics, &predicates).clean(self.cx),
[00:40:46]     |
[00:40:46]     = help: items from traits can only be used if the trait is implemented and in scope
[00:40:46]     = help: items from traits can only be used if the trait is implemented and in scope
[00:40:46]     = note: the following trait defines an item `clean`, perhaps you need to implement it:
[00:40:46]             candidate #1: `clean::Clean`
3994148 .
1651492 ./obj
1651452 ./obj/build
1196984 ./.git
---
163296 ./.git/modules/src/tools/lldb/objects/pack
151412 ./src/tools/clang
150256 ./obj/build/bootstrap/debug/incremental
134668 ./obj/build/bootstrap/debug/incremental/bootstrap-zemjd6kcyh2u
134664 ./obj/build/bootstrap/debug/incremental/bootstrap-zemjd6kcyh2u/s-f6mlux815q-6qy1hk-2bn657r7yjulg
120932 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu
120928 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
118732 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
111088 ./src/llvm/test/CodeGen
---
travis_time:end:0273ebf2:start=1542099896355549577,finish=1542099896364651722,duration=9102145
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:23af2580
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2edfd8f0
$ cat ./o
