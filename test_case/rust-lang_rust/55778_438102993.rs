plain
travis_time:end:1dad8e69:start=1542069042721364227,finish=1542069100510171992,duration=57788807765
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=mingw-check
---
[00:07:01]     Checking parking_lot_core v0.3.0
[00:07:01]     Checking tempfile v3.0.3
[00:07:01]     Checking parking_lot v0.6.4
[00:07:02]     Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
[00:07:04] error[E0599]: no method named `clean` found for type `(&rustc::ty::Generics, &std::sync::Arc<rustc::ty::GenericPredicates<'_>>)` in the current scope
[00:07:04]     --> librustdoc/clean/mod.rs:1981:69
[00:07:04]      |
[00:07:04] 1981 |                                 &cx.tcx.predicates_of(self.def_id)).clean(cx);
[00:07:04]      |
[00:07:04]      = help: items from traits can only be used if the trait is implemented and in scope
[00:07:04]      = help: items from traits can only be used if the trait is implemented and in scope
[00:07:04]      = note: the following trait defines an item `clean`, perhaps you need to implement it:
[00:07:04]              candidate #1: `clean::Clean`
[00:07:04] 
[00:07:04] error[E0599]: no method named `clean` found for type `(&rustc::ty::Generics, &std::sync::Arc<rustc::ty::GenericPredicates<'_>>)` in the current scope
[00:07:04]     --> librustdoc/clean/mod.rs:2049:75
[00:07:04]      |
[00:07:04] 2049 |                     let generics = (cx.tcx.generics_of(did), &predicates).clean(cx);
[00:07:04]      |
[00:07:04]      = help: items from traits can only be used if the trait is implemented and in scope
[00:07:04]      = help: items from traits can only be used if the trait is implemented and in scope
[00:07:04]      = note: the following trait defines an item `clean`, perhaps you need to implement it:
[00:07:04]              candidate #1: `clean::Clean`
[00:07:04] 
[00:07:04] error[E0599]: no method named `clean` found for type `(&rustc::ty::Generics, &std::sync::Arc<rustc::ty::GenericPredicates<'_>>)` in the current scope
[00:07:04]    --> librustdoc/clean/inline.rs:191:59
[00:07:04]     |
[00:07:04] 191 |     let generics = (cx.tcx.generics_of(did), &predicates).clean(cx);
[00:07:04]     |
[00:07:04]     = help: items from traits can only be used if the trait is implemented and in scope
[00:07:04]     = help: items from traits can only be used if the trait is implemented and in scope
[00:07:04]     = note: the following trait defines an item `clean`, perhaps you need to implement it:
[00:07:04]             candidate #1: `clean::Clean`
[00:07:04] 
[00:07:04] error[E0599]: no method named `clean` found for type `(&rustc::ty::Generics, &std::sync::Arc<rustc::ty::GenericPredicates<'_>>)` in the current scope
[00:07:04]    --> librustdoc/clean/inline.rs:219:58
[00:07:04]     |
[00:07:04] 219 |         generics: (cx.tcx.generics_of(did), &predicates).clean(cx),
[00:07:04]     |
[00:07:04]     = help: items from traits can only be used if the trait is implemented and in scope
[00:07:04]     = help: items from traits can only be used if the trait is implemented and in scope
[00:07:04]     = note: the following trait defines an item `clean`, perhaps you need to implement it:
[00:07:04]             candidate #1: `clean::Clean`
[00:07:04] 
[00:07:04] error[E0599]: no method named `clean` found for type `(&rustc::ty::Generics, &std::sync::Arc<rustc::ty::GenericPredicates<'_>>)` in the current scope
[00:07:04]    --> librustdoc/clean/inline.rs:233:58
[00:07:04]     |
[00:07:04] 233 |         generics: (cx.tcx.generics_of(did), &predicates).clean(cx),
[00:07:04]     |
[00:07:04]     = help: items from traits can only be used if the trait is implemented and in scope
[00:07:04]     = help: items from traits can only be used if the trait is implemented and in scope
[00:07:04]     = note: the following trait defines an item `clean`, perhaps you need to implement it:
[00:07:04]             candidate #1: `clean::Clean`
[00:07:04] 
[00:07:04] error[E0599]: no method named `clean` found for type `(&rustc::ty::Generics, &std::sync::Arc<rustc::ty::GenericPredicates<'_>>)` in the current scope
[00:07:04]    --> librustdoc/clean/inline.rs:249:58
[00:07:04]     |
[00:07:04] 249 |         generics: (cx.tcx.generics_of(did), &predicates).clean(cx),
[00:07:04]     |
[00:07:04]     = help: items from traits can only be used if the trait is implemented and in scope
[00:07:04]     = help: items from traits can only be used if the trait is implemented and in scope
[00:07:04]     = note: the following trait defines an item `clean`, perhaps you need to implement it:
[00:07:04]             candidate #1: `clean::Clean`
[00:07:04] 
[00:07:04] error[E0599]: no method named `clean` found for type `(&rustc::ty::Generics, &std::sync::Arc<rustc::ty::GenericPredicates<'_>>)` in the current scope
[00:07:04]    --> librustdoc/clean/inline.rs:261:58
[00:07:04]     |
[00:07:04] 261 |         generics: (cx.tcx.generics_of(did), &predicates).clean(cx),
[00:07:04]     |
[00:07:04]     = help: items from traits can only be used if the trait is implemented and in scope
[00:07:04]     = help: items from traits can only be used if the trait is implemented and in scope
[00:07:04]     = note: the following trait defines an item `clean`, perhaps you need to implement it:
[00:07:04]             candidate #1: `clean::Clean`
[00:07:04] 
[00:07:04] error[E0599]: no method named `clean` found for type `(&rustc::ty::Generics, &std::sync::Arc<rustc::ty::GenericPredicates<'_>>)` in the current scope
[00:07:04]    --> librustdoc/clean/inline.rs:272:58
[00:07:04]     |
[00:07:04] 272 |         generics: (cx.tcx.generics_of(did), &predicates).clean(cx),
[00:07:04]     |
[00:07:04]     = help: items from traits can only be used if the trait is implemented and in scope
[00:07:04]     = help: items from traits can only be used if the trait is implemented and in scope
[00:07:04]     = note: the following trait defines an item `clean`, perhaps you need to implement it:
[00:07:04]             candidate #1: `clean::Clean`
[00:07:04] 
[00:07:04] error[E0599]: no method named `clean` found for type `(&rustc::ty::Generics, &std::sync::Arc<rustc::ty::GenericPredicates<'_>>)` in the current scope
[00:07:04]    --> librustdoc/clean/inline.rs:349:49
[00:07:04]     |
[00:07:04] 349 |             (tcx.generics_of(did), &predicates).clean(cx),
[00:07:04]     |
[00:07:04]     = help: items from traits can only be used if the trait is implemented and in scope
[00:07:04]     = help: items from traits can only be used if the trait is implemented and in scope
[00:07:04]     = note: the following trait defines an item `clean`, perhaps you need to implement it:
[00:07:04]             candidate #1: `clean::Clean`
[00:07:04] 
[00:07:04] error[E0277]: the size for values of type `[clean::Item]` cannot be known at compilation time
[00:07:04]    --> librustdoc/clean/inline.rs:328:10
[00:07:04]     |
[00:07:04] 328 |     let (trait_items, generics) = if let Some(nodeid) = tcx.hir.as_local_node_id(did) {
[00:07:04]     |
[00:07:04]     |
[00:07:04]     = help: the trait `std::marker::Sized` is not implemented for `[clean::Item]`
[00:07:04]     = note: all local variables must have a statically known size
[00:07:04]     = help: unsized locals are gated as an unstable feature
[00:07:04] 
[00:07:04] error[E0308]: mismatched types
[00:07:04] error[E0308]: mismatched types
[00:07:04]    --> librustdoc/clean/inline.rs:382:20
[00:07:04]     |
[00:07:04] 382 |             items: trait_items,
[00:07:04]     |                    ^^^^^^^^^^^
[00:07:04]     |                    |
[00:07:04]     |                    expected struct `std::vec::Vec`, found slice
[00:07:04]     |                    help: try using a conversion method: `trait_items.to_vec()`
[00:07:04]     |
[00:07:04]     = note: expected type `std::vec::Vec<clean::Item>`
[00:07:04]                found type `[clean::Item]`
[00:07:04] 
[00:07:05] error[E0599]: no method named `clean` found for type `(&rustc::ty::Generics, &std::sync::Arc<rustc::ty::GenericPredicates<'_>>)` in the current scope
[00:07:05]    --> librustdoc/clean/auto_trait.rs:577:27
[00:07:05]     |
[00:07:05] 577 |         } = full_generics.clean(self.cx);
[00:07:05]     |
[00:07:05]     = help: items from traits can only be used if the trait is implemented and in scope
[00:07:05]     = help: items from traits can only be used if the trait is implemented and in scope
[00:07:05]     = note: the following trait defines an item `clean`, perhaps you need to implement it:
[00:07:05]             candidate #1: `clean::Clean`
[00:07:05] 
[00:07:05] error[E0599]: no method named `clean` found for type `(&rustc::ty::Generics, &std::sync::Arc<rustc::ty::GenericPredicates<'_>>)` in the current scope
[00:07:05]    --> librustdoc/clean/blanket_impl.rs:158:69
[00:07:05]     |
[00:07:05] 158 |                                 generics: (t_generics, &predicates).clean(self.cx),
[00:07:05]     |
[00:07:05]     = help: items from traits can only be used if the trait is implemented and in scope
[00:07:05]     = help: items from traits can only be used if the trait is implemented and in scope
[00:07:05]     = note: the following trait defines an item `clean`, perhaps you need to implement it:
[00:07:05]             candidate #1: `clean::Clean`
[00:07:06] error: aborting due to 13 previous errors
[00:07:06] 
[00:07:06] Some errors occurred: E0277, E0308, E0599.
[00:07:06] For more information about an error, try `rustc --explain E0277`.
[00:07:06] For more information about an error, try `rustc --explain E0277`.
[00:07:06] error: Could not compile `rustdoc`.
[00:07:06] 
[00:07:06] To learn more, run the command again with --verbose.
[00:07:06] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--color" "always" "--manifest-path" "/checkout/src/tools/rustdoc/Cargo.toml" "--message-format" "json"
[00:07:06] expected success, got: exit code: 101
[00:07:06] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1101:9
[00:07:06] travis_fold:end:stage0-rustdoc

[00:07:06] travis_time:end:stage0-rustdoc:start=1542069526863098628,finish=1542069535977155251,duration=9114056623

---
travis_time:end:0341ec0b:start=1542069536698251311,finish=1542069536703661699,duration=5410388
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1ca64540
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1326a6b4
travis_time:start:1326a6b4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0e5cdbba
$ dmesg | grep -i kill
