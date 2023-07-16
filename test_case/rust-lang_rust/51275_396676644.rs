plain
[00:09:02]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:09:21]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:15:39]    Compiling rustc_mir v0.0.0 (file:///checkout/src/librustc_mir)
[00:15:39]    Compiling rustc_typeck v0.0.0 (file:///checkout/src/librustc_typeck)
[00:15:41] error[E0425]: cannot find value `syntactic_scope` in this scope
[00:15:41]     --> librustc_mir/build/matches/mod.rs:1160:17
[00:15:41]      |
[00:15:41] 1160 |                 syntactic_scope,
[00:15:41] 
[00:15:41] 
[00:15:45] error[E0560]: struct `rustc::mir::LocalDecl<'tcx>` has no field named `syntactic_scope`
[00:15:45]     --> librustc_mir/build/matches/mod.rs:1160:17
[00:15:45]      |
[00:15:45] 1160 |                 syntactic_scope,
[00:15:45]      |                 ^^^^^^^^^^^^^^^ `rustc::mir::LocalDecl<'tcx>` does not have this field
[00:15:45]      |
[00:15:45]      = note: available fields are: `mutability`, `is_user_variable`, `internal`, `ty`, `name` ... and 2 others
[00:15:52] error: aborting due to 2 previous errors
[00:15:52] 
[00:15:52] Some errors occurred: E0425, E0560.
[00:15:52] For more information about an error, try `rustc --explain E0425`.
[00:15:52] For more information about an error, try `rustc --explain E0425`.
[00:15:52] error: Could not compile `rustc_mir`.
[00:15:52] 
[00:15:52] Caused by:
[00:15:52]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_mir librustc_mir/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=1f35ea53eb60b9cb -C extra-filename=-1f35ea53eb60b9cb --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-99b4534960f92449.rlib --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-78ebc049d4f53f46.rlib --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-5d0a8a65bb9fe29f.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-5d0a8a65bb9fe29f.rlib --extern log_settings=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog_settings-f16e9eee09cda644.rlib --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-ccde2368d50449de.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-5073f1296cd24b67.rlib --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-a7df2fc298b4fb06.so --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-01e98699f9876faa.so --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-be994de8b3050feb.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-670f6bac60d99b34.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-c9d6678b1c0f0b46.so --extern polonius_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libpolonius_engine-6e0be5cf77966185.rlib --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-642a7efe79e24835.rlib --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-6a2f0731783c2bd3.so --extern either=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libeither-11fcd35bf55f48b3.rlib --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-58741ed9de9aae4f.so -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-88673787176f9d86/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-62d80197b9ec531e/out` (exit code: 101)
rustlib/x86_64-unknown-linux-gnu
315008 ./src/llvm
249872 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib
241180 ./src/llvm-emscripten
---
147592 ./.git/modules/src
137992 ./obj/build/bootstrap/debug/incremental
132184 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc
123420 ./obj/build/bootstrap/debug/incremental/bootstrap-1r3bppl29tbrj
123416 ./obj/build/bootstrap/debug/incremental/bootstrap-1r3bppl29tbrj/s-f1x8fd6lok-6wniw6-bfrkfwrvh0tv
89804 ./src/llvm/test/CodeGen
81432 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu
81428 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release
72472 ./.git/modules/src/tools
---
travis_time:end:0589127d:start=1528825746652078033,finish=1528825746658101010,duration=6022977
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:12589860
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0bb7026b
$ dmesg | grep -i kill
