plain
[00:47:46] .........................................................i..........................................
[00:47:51] .............................................................................ii.....................
[00:47:57] ....................................................................................................
[00:48:03] .......................................................................................i............
[00:48:05] .....iiiiiiiii...................................................
[00:48:05] 
[00:48:05] travis_fold:start:test_ui_nll
travis_time:start:test_ui_nll
Check compiletest suite=ui mode=ui compare_mode=nll (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
[00:48:53] .........................................................i..........................................
[00:48:58] .............................................................................ii.....................
[00:49:03] ....................................................................................................
[00:49:08] .......................................................................................i............
[00:49:11] .....iiiiiiiii...................................................
[00:49:11] 
[00:49:11]  finished in 65.386
[00:49:11] travis_fold:end:test_ui_nll

---
[01:17:14] travis_fold:start:test_stage1-std
travis_time:start:test_stage1-std
Testing std stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:17:15]    Compiling std v0.0.0 (file:///checkout/src/libstd)
[01:17:21] error[E0689]: can't call method `hash` on ambiguous numeric type `{integer}`
[01:17:21]      |
[01:17:21]      |
[01:17:21] 4266 |             1.hash(&mut h);
[01:17:21]      |               ^^^^
[01:17:21] help: you must specify a concrete type for this numeric value, like `i32`
[01:17:21]      |
[01:17:21] 4266 |             1_i32.hash(&mut h);
[01:17:21] 
[01:17:21] 
[01:17:21] error[E0599]: no method named `finish` found for type `collections::hash::map::DefaultHasher` in the current scope
[01:17:21]      |
[01:17:21]      |
[01:17:21] 3194 | pub struct DefaultHasher(SipHasher13);
[01:17:21]      | -------------------------------------- method `finish` not found for this
[01:17:21] ...
[01:17:21] 4267 |             h.finish()
[01:17:21]      |
[01:17:21]      = help: items from traits can only be used if the trait is in scope
[01:17:21]      = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[01:17:21]              candidate #1: `use core::hash::Hasher;`
[01:17:21]              candidate #1: `use core::hash::Hasher;`
[01:17:21] 
[01:17:21] error[E0689]: can't call method `hash` on ambiguous numeric type `{integer}`
[01:17:21]      |
[01:17:21]      |
[01:17:21] 4287 |             3.hash(&mut h);
[01:17:21]      |               ^^^^
[01:17:21] help: you must specify a concrete type for this numeric value, like `i32`
[01:17:21]      |
[01:17:21] 4287 |             3_i32.hash(&mut h);
[01:17:21] 
[01:17:21] 
[01:17:21] error[E0599]: no method named `finish` found for type `collections::hash::map::DefaultHasher` in the current scope
[01:17:21]      |
[01:17:21]      |
[01:17:21] 3194 | pub struct DefaultHasher(SipHasher13);
[01:17:21]      | -------------------------------------- method `finish` not found for this
[01:17:21] ...
[01:17:21] 4288 |             h.finish()
[01:17:21]      |
[01:17:21]      = help: items from traits can only be used if the trait is in scope
[01:17:21]      = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[01:17:21]              candidate #1: `use core::hash::Hasher;`
[01:17:21]              candidate #1: `use core::hash::Hasher;`
[01:17:21] 
[01:17:21] error[E0599]: no method named `remove_kv` found for type `collections::hash::map::RawOccupiedEntry<'_, {integer}, {integer}>` in the current scope
[01:17:21]      |
[01:17:21]      |
[01:17:21] 1866 | pub struct RawOccupiedEntry<'a, K: 'a, V: 'a> {
[01:17:21]      | --------------------------------------------- method `remove_kv` not found for this
[01:17:21] ...
[01:17:21] 4292 |                 assert_eq!(view.remove_kv(), (3, 30));
[01:17:21]      |
[01:17:21]      |
[01:17:21]      = help: did you mean `remove`?
[01:17:30] error: aborting due to 5 previous errors
[01:17:30] 
[01:17:30] Some errors occurred: E0599, E0689.
[01:17:30] For more information about an error, try `rustc --explain E0599`.
[01:17:30] For more information about an error, try `rustc --explain E0599`.
[01:17:30] error: Could not compile `std`.
[01:17:30] 
[01:17:30] To learn more, run the command again with --verbose.
[01:17:30] 
[01:17:30] 
[01:17:30] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "std" "--" "--quiet"
[01:17:30] 
[01:17:30] 
[01:17:30] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:17:30] Build completed unsuccessfully in 0:32:02
[01:17:30] Build completed unsuccessfully in 0:32:02
[01:17:30] Makefile:58: recipe for target 'check' failed
[01:17:30] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:32bee10a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
