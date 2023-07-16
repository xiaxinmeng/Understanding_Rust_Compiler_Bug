plain
[00:07:41]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:08:01]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:13:56]    Compiling rustc_typeck v0.0.0 (file:///checkout/src/librustc_typeck)
[00:13:56]    Compiling rustc_mir v0.0.0 (file:///checkout/src/librustc_mir)
[00:13:57] error[E0432]: unresolved import `rustc_data_structures::fx::FxHashxHasher`
[00:13:57]   --> librustc_mir/interpret/memory.rs:13:55
[00:13:57]    |
[00:13:57] 13 | use rustc_data_structures::fx::{FxHashSet, FxHashMap, FxHashxHasher};
[00:13:57]    |                                                       ^^^^^^^^^^^^^ no `FxHashxHasher` in `fx`
[00:13:57] 
[00:13:58] error[E0433]: failed to resolve. Use of undeclared type or module `FxHasher`
[00:13:58]    --> librustc_mir/interpret/memory.rs:104:29
[00:13:58]     |
[00:13:58] 104 |                 let mut h = FxHasher::default();
[00:13:58]     |                             ^^^^^^^^ Use of undeclared type or module `FxHasher`
[00:13:58] error: unused import: `ptr`
[00:13:58] error: unused import: `ptr`
[00:13:58]  --> librustc_mir/interpret/eval_context.rs:3:16
[00:13:58]   |
[00:13:58] 3 | use std::{mem, ptr};
[00:13:58]   |
[00:13:58]   = note: `-D unused-imports` implied by `-D warnings`
[00:13:58] 
[00:13:58] 
[00:13:58] error: unused import: `FxHashxHasher`
[00:13:58]   --> librustc_mir/interpret/memory.rs:13:55
[00:13:58]    |
[00:13:58] 13 | use rustc_data_structures::fx::{FxHashSet, FxHashMap, FxHashxHasher};
[00:13:58] 
[00:13:58] 
[00:14:08] error[E0026]: struct `interpret::memory::Memory` does not have a field named `uninitialized_statics`
[00:14:08]   --> librustc_mir/interpret/memory.rs:66:13
[00:14:08] 66 |             uninitialized_statics,
[00:14:08] 66 |             uninitialized_statics,
[00:14:08]    |             ^^^^^^^^^^^^^^^^^^^^^ struct `interpret::memory::Memory` does not have this field
[00:14:08] 
[00:14:08] error[E0609]: no field `uninitialized_statics` on type `&interpret::memory::Memory<'a, 'mir, 'tcx, M>`
[00:14:08]   --> librustc_mir/interpret/memory.rs:74:48
[00:14:08]    |
[00:14:08] 74 |             && *uninitialized_statics == other.uninitialized_statics
[00:14:08] 
[00:14:08] 
[00:14:08] error[E0026]: struct `interpret::memory::Memory` does not have a field named `uninitialized_statics`
[00:14:08]   --> librustc_mir/interpret/memory.rs:88:13
[00:14:08] 88 |             uninitialized_statics: _,
[00:14:08] 88 |             uninitialized_statics: _,
[00:14:08]    |             ^^^^^^^^^^^^^^^^^^^^^^^^ struct `interpret::memory::Memory` does not have this field
[00:14:10] error: aborting due to 7 previous errors
[00:14:10] 
[00:14:10] Some errors occurred: E0026, E0432, E0433, E0609.
[00:14:10] For more information about an error, try `rustc --explain E0026`.
[00:14:10] For more information about an error, try `rustc --explain E0026`.
[00:14:10] error: Could not compile `rustc_mir`.
[00:14:10] 
[00:14:10] Caused by:
[00:14:10]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_mir librustc_mir/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=8fc603afb8ea9b13 -C extra-filename=-8fc603afb8ea9b13 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-d45628fe21047b42.so --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-d8b3f1986e621085.rlib --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-09e9dbd1ef48ffa5.rlib --extern either=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libeither-b51deb005c05dd3b.rlib --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-6020508f01da724d.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-023d781fbd65d983.rlib --extern log_settings=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog_settings-9c861d36e123bec8.rlib --extern polonius_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libpolonius_engine-2e546cbdf217aece.rlib --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-1dddb0fa9d8a512f.so --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-be9737b074a8dae0.rlib --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-25582ce13d3618ea.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-e036f8f5b9204e52.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-066098b54e835a1f.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-05774896829b5d87.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-05774896829b5d87.rlib --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-a1b02e0d020520ac.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-f36f6cb494d373be.so -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-a59a4023b81a89b2/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-ea5907e223517cf2/out` (exit code: 101)
travis_time:end:05800cb3:start=1530736414669097161,finish=1530737361683726248,duration=947014629087

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:10f5fe94
---
travis_time:end:03cafb18:start=1530737362026617252,finish=1530737362033513137,duration=6895885
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1b2a13c8
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:07ae6e18
$ dmesg | grep -i kill
