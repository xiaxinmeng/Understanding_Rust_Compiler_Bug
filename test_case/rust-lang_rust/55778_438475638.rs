plain
travis_time:end:09c06ae0:start=1542144737065003200,finish=1542144739396210925,duration=2331207725
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=mingw-check
---
[00:06:54]     Checking parking_lot_core v0.3.0
[00:06:54]     Checking tempfile v3.0.3
[00:06:55]     Checking parking_lot v0.6.4
[00:06:55]     Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
[00:06:57] error[E0599]: no method named `clean` found for type `(&rustc::ty::Generics, &std::sync::Arc<rustc::ty::GenericPredicates<'_>>)` in the current scope
[00:06:57]     --> librustdoc/clean/mod.rs:1984:69
[00:06:57]      |
[00:06:57] 1984 |                                 &cx.tcx.predicates_of(self.def_id)).clean(cx);
[00:06:57]      |
[00:06:57]      = help: items from traits can only be used if the trait is implemented and in scope
[00:06:57]      = help: items from traits can only be used if the trait is implemented and in scope
[00:06:57]      = note: the following trait defines an item `clean`, perhaps you need to implement it:
[00:06:57]              candidate #1: `clean::Clean`
[00:06:57] 
[00:06:57] error[E0599]: no method named `clean` found for type `(&rustc::ty::Generics, &std::sync::Arc<rustc::ty::GenericPredicates<'_>>)` in the current scope
[00:06:57]     --> librustdoc/clean/mod.rs:2052:75
[00:06:57]      |
[00:06:57] 2052 |                     let generics = (cx.tcx.generics_of(did), &predicates).clean(cx);
[00:06:57]      |
[00:06:57]      = help: items from traits can only be used if the trait is implemented and in scope
[00:06:57]      = help: items from traits can only be used if the trait is implemented and in scope
[00:06:57]      = note: the following trait defines an item `clean`, perhaps you need to implement it:
[00:06:57]              candidate #1: `clean::Clean`
[00:06:57] 
[00:06:57] error[E0599]: no method named `clean` found for type `(&rustc::ty::Generics, &std::sync::Arc<rustc::ty::GenericPredicates<'_>>)` in the current scope
[00:06:57]    --> librustdoc/clean/inline.rs:191:59
[00:06:57]     |
[00:06:57] 191 |     let generics = (cx.tcx.generics_of(did), &predicates).clean(cx);
[00:06:57]     |
[00:06:57]     = help: items from traits can only be used if the trait is implemented and in scope
[00:06:57]     = help: items from traits can only be used if the trait is implemented and in scope
[00:06:57]     = note: the following trait defines an item `clean`, perhaps you need to implement it:
[00:06:57]             candidate #1: `clean::Clean`
[00:06:57] 
[00:06:57] error[E0599]: no method named `clean` found for type `(&rustc::ty::Generics, &std::sync::Arc<rustc::ty::GenericPredicates<'_>>)` in the current scope
[00:06:57]    --> librustdoc/clean/inline.rs:219:58
[00:06:57]     |
[00:06:57] 219 |         generics: (cx.tcx.generics_of(did), &predicates).clean(cx),
[00:06:57]     |
[00:06:57]     = help: items from traits can only be used if the trait is implemented and in scope
[00:06:57]     = help: items from traits can only be used if the trait is implemented and in scope
[00:06:57]     = note: the following trait defines an item `clean`, perhaps you need to implement it:
[00:06:57]             candidate #1: `clean::Clean`
[00:06:57] 
[00:06:57] error[E0599]: no method named `clean` found for type `(&rustc::ty::Generics, &std::sync::Arc<rustc::ty::GenericPredicates<'_>>)` in the current scope
[00:06:57]    --> librustdoc/clean/inline.rs:233:58
[00:06:57]     |
[00:06:57] 233 |         generics: (cx.tcx.generics_of(did), &predicates).clean(cx),
[00:06:57]     |
[00:06:57]     = help: items from traits can only be used if the trait is implemented and in scope
[00:06:57]     = help: items from traits can only be used if the trait is implemented and in scope
[00:06:57]     = note: the following trait defines an item `clean`, perhaps you need to implement it:
[00:06:57]             candidate #1: `clean::Clean`
[00:06:57] 
[00:06:57] error[E0599]: no method named `clean` found for type `(&rustc::ty::Generics, &std::sync::Arc<rustc::ty::GenericPredicates<'_>>)` in the current scope
[00:06:57]    --> librustdoc/clean/inline.rs:249:58
[00:06:57]     |
[00:06:57] 249 |         generics: (cx.tcx.generics_of(did), &predicates).clean(cx),
[00:06:57]     |
[00:06:57]     = help: items from traits can only be used if the trait is implemented and in scope
[00:06:57]     = help: items from traits can only be used if the trait is implemented and in scope
[00:06:57]     = note: the following trait defines an item `clean`, perhaps you need to implement it:
[00:06:57]             candidate #1: `clean::Clean`
[00:06:57] 
[00:06:57] error[E0599]: no method named `clean` found for type `(&rustc::ty::Generics, &std::sync::Arc<rustc::ty::GenericPredicates<'_>>)` in the current scope
[00:06:57]    --> librustdoc/clean/inline.rs:261:58
[00:06:57]     |
[00:06:57] 261 |         generics: (cx.tcx.generics_of(did), &predicates).clean(cx),
[00:06:57]     |
[00:06:57]     = help: items from traits can only be used if the trait is implemented and in scope
[00:06:57]     = help: items from traits can only be used if the trait is implemented and in scope
[00:06:57]     = note: the following trait defines an item `clean`, perhaps you need to implement it:
[00:06:57]             candidate #1: `clean::Clean`
[00:06:57] 
[00:06:57] error[E0599]: no method named `clean` found for type `(&rustc::ty::Generics, &std::sync::Arc<rustc::ty::GenericPredicates<'_>>)` in the current scope
[00:06:57]    --> librustdoc/clean/inline.rs:272:58
[00:06:57]     |
[00:06:57] 272 |         generics: (cx.tcx.generics_of(did), &predicates).clean(cx),
[00:06:57]     |
[00:06:57]     = help: items from traits can only be used if the trait is implemented and in scope
[00:06:57]     = help: items from traits can only be used if the trait is implemented and in scope
[00:06:57]     = note: the following trait defines an item `clean`, perhaps you need to implement it:
[00:06:57]             candidate #1: `clean::Clean`
[00:06:57] 
[00:06:57] error[E0599]: no method named `clean` found for type `(&rustc::ty::Generics, &std::sync::Arc<rustc::ty::GenericPredicates<'_>>)` in the current scope
[00:06:57]    --> librustdoc/clean/inline.rs:349:49
[00:06:57]     |
[00:06:57] 349 |             (tcx.generics_of(did), &predicates).clean(cx),
[00:06:57]     |
[00:06:57]     = help: items from traits can only be used if the trait is implemented and in scope
[00:06:57]     = help: items from traits can only be used if the trait is implemented and in scope
[00:06:57]     = note: the following trait defines an item `clean`, perhaps you need to implement it:
[00:06:57]             candidate #1: `clean::Clean`
[00:06:57] 
[00:06:57] error[E0277]: the size for values of type `[clean::Item]` cannot be known at compilation time
[00:06:57]    --> librustdoc/clean/inline.rs:328:10
[00:06:57]     |
[00:06:57] 328 |     let (trait_items, generics) = if let Some(nodeid) = tcx.hir.as_local_node_id(did) {
[00:06:57]     |
[00:06:57]     |
[00:06:57]     = help: the trait `std::marker::Sized` is not implemented for `[clean::Item]`
[00:06:57]     = note: all local variables must have a statically known size
[00:06:57]     = help: unsized locals are gated as an unstable feature
[00:06:57] 
[00:06:57] error[E0308]: mismatched types
[00:06:57] error[E0308]: mismatched types
[00:06:57]    --> librustdoc/clean/inline.rs:382:20
[00:06:57]     |
[00:06:57] 382 |             items: trait_items,
[00:06:57]     |                    ^^^^^^^^^^^
[00:06:57]     |                    |
[00:06:57]     |                    expected struct `std::vec::Vec`, found slice
[00:06:57]     |                    help: try using a conversion method: `trait_items.to_vec()`
[00:06:57]     |
[00:06:57]     = note: expected type `std::vec::Vec<clean::Item>`
[00:06:57]                found type `[clean::Item]`
[00:06:57] 
[00:06:58] error[E0599]: no method named `clean` found for type `(&rustc::ty::Generics, &std::sync::Arc<rustc::ty::GenericPredicates<'_>>)` in the current scope
[00:06:58]    --> librustdoc/clean/auto_trait.rs:577:27
[00:06:58]     |
[00:06:58] 577 |         } = full_generics.clean(self.cx);
[00:06:58]     |
[00:06:58]     = help: items from traits can only be used if the trait is implemented and in scope
[00:06:58]     = help: items from traits can only be used if the trait is implemented and in scope
[00:06:58]     = note: the following trait defines an item `clean`, perhaps you need to implement it:
[00:06:58]             candidate #1: `clean::Clean`
[00:06:58] 
[00:06:58] error[E0599]: no method named `clean` found for type `(&rustc::ty::Generics, &std::sync::Arc<rustc::ty::GenericPredicates<'_>>)` in the current scope
[00:06:58]    --> librustdoc/clean/blanket_impl.rs:158:69
[00:06:58]     |
[00:06:58] 158 |                                 generics: (t_generics, &predicates).clean(self.cx),
[00:06:58]     |
[00:06:58]     = help: items from traits can only be used if the trait is implemented and in scope
[00:06:58]     = help: items from traits can only be used if the trait is implemented and in scope
[00:06:58]     = note: the following trait defines an item `clean`, perhaps you need to implement it:
[00:06:58]             candidate #1: `clean::Clean`
[00:06:59] error: aborting due to 13 previous errors
[00:06:59] 
[00:06:59] Some errors occurred: E0277, E0308, E0599.
[00:06:59] For more information about an error, try `rustc --explain E0277`.
[00:06:59] For more information about an error, try `rustc --explain E0277`.
[00:06:59] error: Could not compile `rustdoc`.
[00:06:59] 
[00:06:59] To learn more, run the command again with --verbose.
[00:06:59] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--color" "always" "--manifest-path" "/checkout/src/tools/rustdoc/Cargo.toml" "--message-format" "json"
[00:06:59] travis_fold:end:stage0-rustdoc

[00:06:59] travis_time:end:stage0-rustdoc:start=1542145158883809897,finish=1542145166934514893,duration=8050704996


[00:06:59] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1101:9
[00:06:59] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
[00:06:59] Build completed unsuccessfully in 0:04:55
travis_time:end:0e46d803:start=1542144747404107435,finish=1542145167168883933,duration=419764776498
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
---
travis_time:end:01ad7488:start=1542145167605219920,finish=1542145167609740192,duration=4520272
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0a38c3cb
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:014273af
travis_time:start:014273af
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:32597eb9
$ dmesg | grep -i kill
