plain
[00:48:34] ................................................................................................i... 2200/4563
[00:48:38] .................................................................................................... 2300/4563
[00:48:42] .................................................................................................... 2400/4563
[00:48:45] .................................................................................................... 2500/4563
[00:48:49] ........iiiiiiiii................................................................................... 2600/4563
[00:48:55] .................................................................................................... 2800/4563
[00:48:59] .................................................................................................... 2900/4563
[00:49:02] ............................i....................................................................... 3000/4563
[00:49:05] ........................................................................................i.i..ii..... 3100/4563
---
[01:13:25] .................................................................................................... 1700/2167
[01:13:32] .................................................................................................... 1800/2167
[01:13:39] .................................................................................................... 1900/2167
[01:13:48] .................................................................................................... 2000/2167
[01:13:57] ................FF.................................................................i................ 2100/2167
[01:14:02] failures:
[01:14:02] 
[01:14:02] ---- slice/mod.rs - slice::[T]::try_split_at (line 817) stdout ----
[01:14:02] error[E0308]: mismatched types
[01:14:02] error[E0308]: mismatched types
[01:14:02]  --> slice/mod.rs:822:9
[01:14:02]   |
[01:14:02] 8 |         Some((left, right)) => {
[01:14:02]   |         ^^^^^^^^^^^^^^^^^^^ expected tuple, found enum `std::option::Option`
[01:14:02]   |
[01:14:02]   = note: expected type `(&[{integer}], &[{integer}])`
[01:14:02] 
[01:14:02] error[E0308]: mismatched types
[01:14:02]   --> slice/mod.rs:826:9
[01:14:02]    |
[01:14:02]    |
[01:14:02] 12 |         None => assert!(false),
[01:14:02]    |         ^^^^ expected tuple, found enum `std::option::Option`
[01:14:02]    |
[01:14:02]    = note: expected type `(&[{integer}], &[{integer}])`
[01:14:02] 
[01:14:02] error[E0308]: mismatched types
[01:14:02]   --> slice/mod.rs:832:9
[01:14:02]    |
[01:14:02]    |
[01:14:02] 18 |         Some((left, right)) => {
[01:14:02]    |         ^^^^^^^^^^^^^^^^^^^ expected tuple, found enum `std::option::Option`
[01:14:02]    |
[01:14:02]    = note: expected type `(&[{integer}], &[{integer}])`
[01:14:02] 
[01:14:02] error[E0308]: mismatched types
[01:14:02]   --> slice/mod.rs:836:9
[01:14:02]    |
[01:14:02]    |
[01:14:02] 22 |         None => assert!(false),
[01:14:02]    |         ^^^^ expected tuple, found enum `std::option::Option`
[01:14:02]    |
[01:14:02]    = note: expected type  - un-closed delimiter
[01:14:02] 7  |     match v.split_at_mut(2) {
[01:14:02] 7  |     match v.split_at_mut(2) {
[01:14:02]    |                             - this delimiter might not be properly closed...
[01:14:02] 15 | }
[01:14:02] 15 | }
[01:14:02]    | - ...as it matches this but it has different indentation
[01:14:02] 16 | assert!(v == [1, 2, 3, 4, 5, 6]);
[01:14:02] 17 | }
[01:14:02]    |  ^
[01:14:02] error[E0308]: mismatched types
[01:14:02]  --> slice/mod.rs:915:9
[01:14:02]   |
[01:14:02] 8 |         Some((left, right)) => {
[01:14:02] 8 |         Some((left, right)) => {
[01:14:02]   |         ^^^^^^^^^^^^^^^^^^^ expected tuple, found enum `std::option::Option`
[01:14:02]   |
[01:14:02]   = note: expected type `(&mut [{integer}], &mut [{integer}])`
[01:14:02] 
[01:14:02] error[E0308]: mismatched types
[01:14:02]   --> slice/mod.rs:921:9
[01:14:02]    |
[01:14:02]    |
[01:14:02] 14 |         None => assert!(false),
[01:14:02]    |         ^^^^ expected tuple, found enum `std::option::Option`
[01:14:02]    |
[01:14:02]    = note: expected type `(&mut [{integer}], &mut [{integer}])`
[01:14:02] 
[01:14:02] thread 'slice/mod.rs - slice::[T]::try_split_at_mut (line 910)' panicked at 'couldn't compile the test', librustdoc/test.rs:333:13
[01:14:02] 
[01:14:02] 
---
[01:14:02] 
[01:14:02] error: test failed, to rerun pass '--doc'
[01:14:02] 
[01:14:02] 
[01:14:02] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:14:02] 
[01:14:02] 
[01:14:02] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:14:02] Build completed unsuccessfully in 0:29:51
[01:14:02] Build completed unsuccessfully in 0:29:51
[01:14:02] Makefile:58: recipe for target 'check' failed
[01:14:02] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:102fa390
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
