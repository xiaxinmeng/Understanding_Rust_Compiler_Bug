plain
[00:13:16]    Compiling rustc_allocator v0.0.0 (file:///checkout/src/librustc_allocator)
[00:13:16] error[E0433]: failed to resolve. Use of undeclared type or module `PlaceBase`
[00:13:16]    --> librustc_mir/borrow_check/borrow_set.rs:379:13
[00:13:16]     |
[00:13:16] 379 |             PlaceBase::Local(temp) if neo_place.has_no_projection() => temp,
[00:13:16]     |             ^^^^^^^^^ Use of undeclared type or module `PlaceBase`
[00:13:16] 
[00:13:19] error[E0616]: field `base` of struct `rustc::mir::NeoPlace` is private
[00:13:19]    --> librustc_mir/borrow_check/borrow_set.rs:260:38
[00:13:19]     |
[00:13:19] 260 |         if let Place::Local(temp) = &neo_place.base {
[00:13:19] 
[00:13:19] error[E0308]: mismatched types
[00:13:19]    --> librustc_mir/borrow_check/borrow_set.rs:260:16
[00:13:19]     |
[00:13:19]     |
[00:13:19] 260 |         if let Place::Local(temp) = &neo_place.base {
[00:13:19]     |                ^^^^^^^^^^^^^^^^^^ expected enum `rustc::mir::PlaceBase`, found enum `rustc::mir::Place`
[00:13:19]     |
[00:13:19]     = note: expected type `rustc::mir::PlaceBase<'_>`
[00:13:19]                found type `rustc::mir::Place<'_>`
[00:13:19] 
[00:13:19] error[E0616]: field `base` of struct `rustc::mir::NeoPlace` is private
[00:13:19]    --> librustc_mir/borrow_check/borrow_set.rs:378:27
[00:13:19]     |
[00:13:19] 378 |         let temp = match &neo_place.base {
[00:13:19] 
[00:13:27] error: aborting due to 4 previous errors
[00:13:27] 
[00:13:27] Some errors occurred: E0308, E0433, E0616.
[00:13:27] Some errors occurred: E0308, E0433, E0616.
[00:13:27] For more information about an error, try `rustc --explain E0308`.
[00:13:27] error: Could not compile `rustc_mir`.
[00:13:27] warning: build failed, waiting for other jobs to finish...
[00:14:38] error: build failed
[00:14:38] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:14:38] expected success, got: exit code: 101
[00:14:38] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1112:9
[00:14:38] travis_fold:end:stage0-rustc

[00:14:38] travis_time:end:stage0-rustc:start=1537712381058209369,finish=1537712936652559162,duration=555594349793


[00:14:38] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:14:38] Build completed unsuccessfully in 0:10:10
[00:14:38] Makefile:28: recipe for target 'all' failed
[00:14:38] make: *** [all] Error 1
