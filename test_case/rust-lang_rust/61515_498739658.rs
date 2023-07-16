plain
travis_time:end:0edf11ba:start=1559660039446708615,finish=1559660223151861699,duration=183705153084
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:01:49] 
[01:01:49] running 144 tests
[01:01:51] i..iii.....iii..i.iii....i......................i..i.................i.....i..........ii.i..i..i.ii. 100/144
[01:01:53] test result: ok. 114 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[01:01:53] 
[01:01:53]  finished in 4.322
[01:01:53] travis_fold:end:test_codegen
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:01:55] 
[01:01:55] running 9 tests
[01:01:55] iiiiiiiii
[01:01:55] 
[01:01:55]  finished in 0.145
[01:01:55] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:02:10] 
[01:02:10] running 122 tests
[01:02:33] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:02:38] .i.i......iii.i.....ii
[01:02:38] 
[01:02:38]  finished in 28.008
[01:02:38] travis_fold:end:test_debuginfo

---
[01:14:14]    Compiling alloc v0.0.0 (/checkout/src/liballoc)
[01:14:16] error[E0412]: cannot find type `PlaceholderError` in module `crate::boxed`
[01:14:16]     --> src/liballoc/sync.rs:1793:44
[01:14:16]      |
[01:14:16] 1788 | / macro_rules! array_impls {
[01:14:16] 1789 | |     ($($N:expr)+) => {
[01:14:16] 1790 | |         $(
[01:14:16] 1791 | |             #[unstable(feature = "boxed_slice_try_from", issue = "0")]
[01:14:16] 1792 | |             impl<T> TryFrom<Arc<[T]>> for Arc<[T; $N]> {
[01:14:16] 1793 | |                 type Error = crate::boxed::PlaceholderError;
[01:14:16]      | |                                            ^^^^^^^^^^^^^^^^ not found in `crate::boxed`
[01:14:16] 1804 | |     }
[01:14:16] 1805 | | }
[01:14:16]      | |_- in this expansion of `array_impls!`
[01:14:16] 1806 | 
[01:14:16] 1806 | 
[01:14:16] 1807 | / array_impls! {
[01:14:16] 1809 | |     10 11 12 13 14 15 16 17 18 19
[01:14:16] 1810 | |     20 21 22 23 24 25 26 27 28 29
[01:14:16] 1811 | |     30 31 32
[01:14:16] 1812 | | }
---
[01:14:16] 
[01:14:16] error[E0425]: cannot find function `PlaceholderError` in module `crate::boxed`
[01:14:16]     --> src/liballoc/sync.rs:1799:43
[01:14:16]      |
[01:14:16] 1788 | / macro_rules! array_impls {
[01:14:16] 1789 | |     ($($N:expr)+) => {
[01:14:16] 1790 | |         $(
[01:14:16] 1791 | |             #[unstable(feature = "boxed_slice_try_from", issue = "0")]
[01:14:16] ...    |
[01:14:16] 1799 | |                         Err(crate::boxed::PlaceholderError(()))
[01:14:16]      | |                                           ^^^^^^^^^^^^^^^^ not found in `crate::boxed`
[01:14:16] 1804 | |     }
[01:14:16] 1805 | | }
[01:14:16]      | |_- in this expansion of `array_impls!`
[01:14:16] 1806 | 
[01:14:16] 1806 | 
[01:14:16] 1807 | / array_impls! {
[01:14:16] 1809 | |     10 11 12 13 14 15 16 17 18 19
[01:14:16] 1810 | |     20 21 22 23 24 25 26 27 28 29
[01:14:16] 1811 | |     30 31 32
[01:14:16] 1812 | | }
[01:14:16] 1812 | | }
[01:14:16]      | |_- in this macro invocation
[01:14:16] 
[01:14:16] error[E0412]: cannot find type `PlaceholderError` in module `crate::boxed`
[01:14:16]     --> src/liballoc/rc.rs:1221:44
[01:14:16]      |
[01:14:16] 1216 | / macro_rules! array_impls {
[01:14:16] 1217 | |     ($($N:expr)+) => {
[01:14:16] 1218 | |         $(
[01:14:16] 1219 | |             #[unstable(feature = "boxed_slice_try_from", issue = "0")]
[01:14:16] 1220 | |             impl<T> TryFrom<Rc<[T]>> for Rc<[T; $N]> {
[01:14:16] 1221 | |                 type Error = crate::boxed::PlaceholderError;
[01:14:16]      | |                                            ^^^^^^^^^^^^^^^^ not found in `crate::boxed`
[01:14:16] 1232 | |     }
[01:14:16] 1233 | | }
[01:14:16]      | |_- in this expansion of `array_impls!`
[01:14:16] 1234 | 
[01:14:16] 1234 | 
[01:14:16] 1235 | / array_impls! {
[01:14:16] 1237 | |     10 11 12 13 14 15 16 17 18 19
[01:14:16] 1238 | |     20 21 22 23 24 25 26 27 28 29
[01:14:16] 1239 | |     30 31 32
[01:14:16] 1240 | | }
---
[01:14:16] 
[01:14:16] error[E0425]: cannot find function `PlaceholderError` in module `crate::boxed`
[01:14:16]     --> src/liballoc/rc.rs:1227:43
[01:14:16]      |
[01:14:16] 1216 | / macro_rules! array_impls {
[01:14:16] 1217 | |     ($($N:expr)+) => {
[01:14:16] 1218 | |         $(
[01:14:16] 1219 | |             #[unstable(feature = "boxed_slice_try_from", issue = "0")]
[01:14:16] ...    |
[01:14:16] 1227 | |                         Err(crate::boxed::PlaceholderError(()))
[01:14:16]      | |                                           ^^^^^^^^^^^^^^^^ not found in `crate::boxed`
[01:14:16] 1232 | |     }
[01:14:16] 1233 | | }
[01:14:16]      | |_- in this expansion of `array_impls!`
[01:14:16] 1234 | 
[01:14:16] 1234 | 
[01:14:16] 1235 | / array_impls! {
[01:14:16] 1237 | |     10 11 12 13 14 15 16 17 18 19
[01:14:16] 1238 | |     20 21 22 23 24 25 26 27 28 29
[01:14:16] 1239 | |     30 31 32
[01:14:16] 1240 | | }
[01:14:16] 1240 | | }
[01:14:16]      | |_- in this macro invocation
[01:14:16] 
[01:14:18] error[E0308]: mismatched types
[01:14:18]    --> src/liballoc/boxed_test.rs:145:25
[01:14:18]     |
[01:14:18] 145 |     let r: Box<[u32]> = Box::from(v);
[01:14:18]     |                         ^^^^^^^^^^^^ expected slice, found struct `vec::Vec`
[01:14:18]     = note: expected type `std::boxed::Box<[u32]>`
[01:14:18]     = note: expected type `std::boxed::Box<[u32]>`
[01:14:18]                found type `std::boxed::Box<vec::Vec<{integer}>>`
[01:14:18] 
[01:14:18] error[E0599]: no method named `try_into` found for type `std::boxed::Box<[u32]>` in the current scope
[01:14:18]     |
[01:14:18]     |
[01:14:18] 147 |     let a: Result<Box<[u32; 3]>, _> = r.clone().try_into();
[01:14:18]     |
[01:14:18]     = help: items from traits can only be used if the trait is in scope
[01:14:18]     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[01:14:18]             `use core::convert::TryInto;`
[01:14:18]             `use core::convert::TryInto;`
[01:14:18] 
[01:14:18] error[E0599]: no method named `try_into` found for type `std::boxed::Box<[u32]>` in the current scope
[01:14:18]     |
[01:14:18]     |
[01:14:18] 150 |     let a: Result<Box<[u32; 2]>, _> = r.clone().try_into();
[01:14:18]     |
[01:14:18]     = help: items from traits can only be used if the trait is in scope
[01:14:18]     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[01:14:18]             `use core::convert::TryInto;`
[01:14:18]             `use core::convert::TryInto;`
[01:14:18] 
[01:14:19] error[E0599]: no method named `try_into` found for type `sync::Arc<[u32]>` in the current scope
[01:14:19]      |
[01:14:19]      |
[01:14:19] 193  | pub struct Arc<T: ?Sized> {
[01:14:19]      | ------------------------- method `try_into` not found for this
[01:14:19] ...
[01:14:19] 2301 |         let a: Result<Arc<[u32; 3]>, _> = r.clone().try_into();
[01:14:19]      |
[01:14:19]      = help: items from traits can only be used if the trait is in scope
[01:14:19]      = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[01:14:19]              `use core::convert::TryInto;`
[01:14:19]              `use core::convert::TryInto;`
[01:14:19] 
[01:14:19] error[E0599]: no method named `try_into` found for type `sync::Arc<[u32]>` in the current scope
[01:14:19]      |
[01:14:19]      |
[01:14:19] 193  | pub struct Arc<T: ?Sized> {
[01:14:19]      | ------------------------- method `try_into` not found for this
[01:14:19] ...
[01:14:19] 2304 |         let a: Result<Arc<[u32; 2]>, _> = r.clone().try_into();
[01:14:19]      |
[01:14:19]      = help: items from traits can only be used if the trait is in scope
[01:14:19]      = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[01:14:19]              `use core::convert::TryInto;`
[01:14:19]              `use core::convert::TryInto;`
[01:14:19] 
[01:14:19] error[E0599]: no method named `try_into` found for type `rc::Rc<[u32]>` in the current scope
[01:14:19]      |
[01:14:19]      |
[01:14:19] 273  | pub struct Rc<T: ?Sized> {
[01:14:19]      | ------------------------ method `try_into` not found for this
[01:14:19] ...
[01:14:19] 2177 |         let a: Result<Rc<[u32; 3]>, _> = r.clone().try_into();
[01:14:19]      |
[01:14:19]      = help: items from traits can only be used if the trait is in scope
[01:14:19]      = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[01:14:19]              `use core::convert::TryInto;`
[01:14:19]              `use core::convert::TryInto;`
[01:14:19] 
[01:14:19] error[E0599]: no method named `try_into` found for type `rc::Rc<[u32]>` in the current scope
[01:14:19]      |
[01:14:19]      |
[01:14:19] 273  | pub struct Rc<T: ?Sized> {
[01:14:19]      | ------------------------ method `try_into` not found for this
[01:14:19] ...
[01:14:19] 2180 |         let a: Result<Rc<[u32; 2]>, _> = r.clone().try_into();
[01:14:19]      |
[01:14:19]      = help: items from traits can only be used if the trait is in scope
[01:14:19]      = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[01:14:19]              `use core::convert::TryInto;`
---
[01:14:20] warning: build failed, waiting for other jobs to finish...
[01:15:28] error: build failed
[01:15:28] 
[01:15:28] 
[01:15:28] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "alloc" "--" "--quiet"
[01:15:28] 
[01:15:28] 
[01:15:28] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:15:28] Build completed unsuccessfully in 1:11:21
---
travis_time:end:3229e7e9:start=1559664762612361139,finish=1559664762618162945,duration=5801806
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2d1884a0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
