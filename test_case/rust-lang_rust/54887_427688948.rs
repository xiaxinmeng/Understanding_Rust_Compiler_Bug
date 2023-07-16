plain
[00:53:16] ...................................................................................................i 2200/4567
[00:53:20] .................................................................................................... 2300/4567
[00:53:24] .................................................................................................... 2400/4567
[00:53:27] .................................................................................................... 2500/4567
[00:53:31] ............iiiiiiiii............................................................................... 2600/4567
[00:53:37] .................................................................................................... 2800/4567
[00:53:41] .................................................................................................... 2900/4567
[00:53:44] ................................i................................................................... 3000/4567
[00:53:47] ............................................................................................i.i..ii. 3100/4567
---
[01:21:21] .................................................................................................... 1700/2167
[01:21:30] .................................................................................................... 1800/2167
[01:21:38] .................................................................................................... 1900/2167
[01:21:48] .................................................................................................... 2000/2167
[01:21:59] ....F............FF................................................................i................ 2100/2167
[01:22:05] failures:
[01:22:05] 
[01:22:05] ---- slice/mod.rs - slice::[T]::split_at_mut (line 872) stdout ----
[01:22:05] ---- slice/mod.rs - slice::[T]::split_at_mut (line 872) stdout ----
[01:22:05] error[E0658]: use of unstable library feature 'try_split_at' (see issue #54886)
[01:22:05]  --> slice/mod.rs:876:27
[01:22:05]   |
[01:22:05] 7 |     let (left, right) = v.try_split_at_mut(2);
[01:22:05]   |
[01:22:05]   |
[01:22:05]   = help: add #![feature(try_split_at)] to the crate attributes to enable
[01:22:05] error[E0308]: mismatched types
[01:22:05]  --> slice/mod.rs:876:9
[01:22:05]   |
[01:22:05]   |
[01:22:05] 7 |     let (left, right) = v.try_split_at_mut(2);
[01:22:05]   |         ^^^^^^^^^^^^^ expected enum `std::option::Option`, found tuple
[01:22:05]   |
[01:22:05]   = note: expected type `std::option::Option<(&mut [{integer}], &mut [{integer}])>`
[01:22:05]              found type `(_, _)`
[01:22:05] thread 'slice/mod.rs - slice::[T]::split_at_mut (line 872)' panicked at 'couldn't compile the test', librustdoc/test.rs:333:13
[01:22:05] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:22:05] 
[01:22:05] ---- slice/mod.rs - slice::[T]::try_split_at_mut (line 908) stdout ----
[01:22:05] ---- slice/mod.rs - slice::[T]::try_split_at_mut (line 908) stdout ----
[01:22:05] error: this file contains an un-closed delimiter
[01:22:05]   --> slice/mod.rs:922:2
[01:22:05]    |
[01:22:05] 3  | fn main() {
[01:22:05]    |           - un-closed delimiter
[01:22:05] ...
[01:22:05] 7  |     match v.split_at_mut(2) {
[01:22:05]    |                             - this delimiter might not be properly closed...
[01:22:05] 15 | }
[01:22:05] 15 | }
[01:22:05]    | - ...as it matches this but it has different indentation
[01:22:05] 16 | assert!(v == [1, 2, 3, 4, 5, 6]);
[01:22:05] 17 | }
[01:22:05]    |  ^
[01:22:05] error[E0308]: mismatched types
[01:22:05]  --> slice/mod.rs:913:9
[01:22:05]   |
[01:22:05] 8 |         Some((left, right)) => {
[01:22:05] 8 |         Some((left, right)) => {
[01:22:05]   |         ^^^^^^^^^^^^^^^^^^^ expected tuple, found enum `std::option::Option`
[01:22:05]   |
[01:22:05]   = note: expected type `(&mut [{integer}], &mut [{integer}])`
[01:22:05] 
[01:22:05] error[E0308]: mismatched types
[01:22:05]   --> slice/mod.rs:919:9
[01:22:05]    |
[01:22:05]    |
[01:22:05] 14 |         None => assert!(false),
[01:22:05]    |         ^^^^ expected tuple, found enum `std::option::Option`
[01:22:05]    |
[01:22:05]    = note: expected type `(&mut [{integer}], &mut [{integer}])`
[01:22:05] 
[01:22:05] thread 'slice/mod.rs - slice::[T]::try_split_at_mut (line 908)' panicked at 'couldn't compile the test', librustdoc/test.rs:333:13
[01:22:05] 
[01:22:05] ---- slice/mod.rs - slice::[T]::try_split_at (line 817) stdout ----
[01:22:05] ---- slice/mod.rs - slice::[T]::try_split_at (line 817) stdout ----
[01:22:05] error[E0658]: use of unstable library feature 'try_split_at' (see issue #54886)
[01:22:05]  --> slice/mod.rs:821:13
[01:22:05]   |
[01:22:05] 7 |     match v.try_split_at(0) {
[01:22:05]   |
[01:22:05]   |
[01:22:05]   = help: add #![feature(try_split_at)] to the crate attributes to enable
[01:22:05] 
[01:22:05] error[E0658]: use of unstable library feature 'try_split_at' (see issue #54886)
[01:22:05]   --> slice/mod.rs:831:13
[01:22:05]    |
[01:22:05] 17 |     match v.try_split_at(2) {
[01:22:05]    |
[01:22:05]    |
[01:22:05]    = help: add #![feature(try_split_at)] to the crate attributes to enable
[01:22:05] 
[01:22:05] error[E0658]: use of unstable library feature 'try_split_at' (see issue #54886)
[01:22:05]   --> slice/mod.rs:841:13
[01:22:05]    |
[01:22:05] 27 |     match v.try_split_at(6) {
[01:22:05]    |
[01:22:05]    |
[01:22:05]    = help: add #![feature(try_split_at)] to the crate attributes to enable
[01:22:05] 
[01:22:05] error[E0599]: no method named `is_none` found for type `(&[{integer}], &[{integer}])` in the current scope
[01:22:05]   --> slice/mod.rs:851:27
[01:22:05]    |
[01:22:05] 37 |     assert!(v.split_at(7).is_none())
[01:22:05] 
[01:22:05] thread 'slice/mod.rs - slice::[T]::try_split_at (line 817)' panicked at 'couldn't compile the test', librustdoc/test.rs:333:13
[01:22:05] 
[01:22:05] 
---
[01:22:05] 
[01:22:05] error: test failed, to rerun pass '--doc'
[01:22:05] 
[01:22:05] 
[01:22:05] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:22:05] 
[01:22:05] 
[01:22:05] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:22:05] Build completed unsuccessfully in 0:33:31
[01:22:05] Build completed unsuccessfully in 0:33:31
[01:22:05] Makefile:58: recipe for target 'check' failed
[01:22:05] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0268906b
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
