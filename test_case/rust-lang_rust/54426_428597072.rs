plain
[00:12:40]    Compiling rustc_allocator v0.0.0 (/checkout/src/librustc_allocator)
[00:12:41] error[E0433]: failed to resolve. Use of undeclared type or module `PlaceBase`
[00:12:41]    --> librustc_mir/dataflow/impls/borrows.rs:263:29
[00:12:41]     |
[00:12:41] 263 |                 if let Some(PlaceBase::Local(local)) = neo_place.as_place_base() {
[00:12:41]     |                             ^^^^^^^^^ Use of undeclared type or module `PlaceBase`
[00:12:41] error[E0433]: failed to resolve. Use of undeclared type or module `PlaceBase`
[00:12:41]    --> librustc_mir/dataflow/impls/borrows.rs:328:37
[00:12:41]     |
[00:12:41]     |
[00:12:41] 328 |                         if let Some(PlaceBase::Local(local)) = neo_place.as_place_base() {
[00:12:41]     |                                     ^^^^^^^^^ Use of undeclared type or module `PlaceBase`
[00:12:42] error: unused import: `Place`
[00:12:42]   --> librustc_mir/dataflow/impls/borrows.rs:18:34
[00:12:42]    |
[00:12:42]    |
[00:12:42] 18 | use rustc::mir::{self, Location, Place, Mir};
[00:12:42]    |
[00:12:42]    = note: `-D unused-imports` implied by `-D warnings`
[00:12:42] 
[00:12:42] 
[00:12:48] error[E0599]: no method named `as_new_place` found for type `&rustc::mir::Place<'_>` in the current scope
[00:12:48]    --> librustc_mir/dataflow/impls/borrows.rs:262:37
[00:12:48]     |
[00:12:48] 262 |                 let neo_place = lhs.as_new_place(self.tcx);
[00:12:48] 
[00:12:48] 
[00:12:48] error[E0599]: no method named `as_new_place` found for type `&std::boxed::Box<[rustc::mir::Place<'_>]>` in the current scope
[00:12:48]    --> librustc_mir/dataflow/impls/borrows.rs:327:49
[00:12:48]     |
[00:12:48] 327 |                         let neo_place = outputs.as_new_place(self.tcx);
[00:12:48] 
[00:12:53] error: aborting due to 5 previous errors
[00:12:53] 
[00:12:53] Some errors occurred: E0433, E0599.
[00:12:53] Some errors occurred: E0433, E0599.
[00:12:53] For more information about an error, try `rustc --explain E0433`.
[00:12:53] error: Could not compile `rustc_mir`.
[00:12:53] warning: build failed, waiting for other jobs to finish...
[00:14:00] error: build failed
[00:14:00] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:14:00] expected success, got: exit code: 101
[00:14:00] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1112:9
[00:14:00] travis_fold:end:stage0-rustc

[00:14:00] travis_time:end:stage0-rustc:start=1539181631531061914,finish=1539182174242213151,duration=542711151237


[00:14:00] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:14:00] Build completed unsuccessfully in 0:09:57
[00:14:00] make: *** [all] Error 1
[00:14:00] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0640f5a0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:162903a4:start=1539182174919231485,finish=1539182174923720458,duration=4488973
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0f13d502
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:11ef20fc
travis_time:start:11ef20fc
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:07ed0de8
$ dmesg | grep -i kill
