plain
[00:07:11]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:07:25]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:12:37]    Compiling rustc_mir v0.0.0 (file:///checkout/src/librustc_mir)
[00:12:37]    Compiling rustc_typeck v0.0.0 (file:///checkout/src/librustc_typeck)
[00:12:43] error[E0277]: the trait bound `rustc_data_structures::indexed_set::IdxSet<_>: std::clone::Clone` is not satisfied
[00:12:43]    --> librustc_mir/dataflow/mod.rs:244:17
[00:12:43]     |
[00:12:43] 244 |                 IdxSet::clone_from(in_out, sets.on_entry);
[00:12:43]     |                 ^^^^^^^^^^^^^^^^^^ the trait `std::clone::Clone` is not implemented for `rustc_data_structures::indexed_set::IdxSet<_>`
[00:12:43]     = note: required by `std::clone::Clone::clone_from`
[00:12:43] 
[00:12:43] 
[00:12:43] error[E0277]: the size for value values of type `[usize]` cannot be known at compilation time
[00:12:43]    --> librustc_mir/dataflow/mod.rs:244:17
[00:12:43]     |
[00:12:43] 244 |                 IdxSet::clone_from(in_out, sets.on_entry);
[00:12:43]     |                 ^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
[00:12:43]     |
[00:12:43]     = help: within `rustc_data_structures::indexed_set::IdxSet<_>`, the trait `std::marker::Sized` is not implemented for `[usize]`
[00:12:43]     = note: to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types--sized>
[00:12:43]     = note: required because it appears within the type `rustc_data_structures::indexed_set::IdxSet<_>`
[00:12:43]     = note: required by `std::clone::Clone::clone_from`
[00:12:48] error: aborting due to 2 previous errors
[00:12:48] 
[00:12:48] For more information about this error, try `rustc --explain E0277`.
[00:12:48] error: Could not compile `rustc_mir`.
[00:12:48] error: Could not compile `rustc_mir`.
[00:12:48] 
[00:12:48] Caused by:
[00:12:48]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_mir librustc_mir/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=8fc603afb8ea9b13 -C extra-filename=-8fc603afb8ea9b13 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-d45628fe21047b42.so --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-d8b3f1986e621085.rlib --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-09e9dbd1ef48ffa5.rlib --extern either=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libeither-b51deb005c05dd3b.rlib --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-6020508f01da724d.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-023d781fbd65d983.rlib --extern log_settings=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog_settings-9c861d36e123bec8.rlib --extern polonius_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libpolonius_engine-2e546cbdf217aece.rlib --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-1dddb0fa9d8a512f.so --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-be9737b074a8dae0.rlib --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-25582ce13d3618ea.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-e036f8f5b9204e52.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-066098b54e835a1f.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-05774896829b5d87.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-05774896829b5d87.rlib --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-a1b02e0d020520ac.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-f36f6cb494d373be.so -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-a59a4023b81a89b2/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-ea5907e223517cf2/out` (exit code: 101)
2644264 .
1353080 ./obj
1353048 ./obj/build
758764 ./obj/build/x86_64-unknown-linux-gnu
---
158412 ./.git/modules/src
149112 ./src/llvm-emscripten/test
144408 ./obj/build/bootstrap/debug/incremental
129896 ./obj/build/bootstrap/debug/incremental/bootstrap-146vjsckowoo9
129892 ./obj/build/bootstrap/debug/incremental/bootstrap-146vjsckowoo9/s-f2jbfurakr-15xzs3l-3ah8kbogwyk4h
97524 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
89808 ./src/llvm/test/CodeGen
80576 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu
80572 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release
