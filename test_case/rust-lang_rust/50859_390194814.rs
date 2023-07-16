plain
[00:09:26]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:09:49]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:17:46]    Compiling rustc_typeck v0.0.0 (file:///checkout/src/librustc_typeck)
[00:17:46]    Compiling rustc_mir v0.0.0 (file:///checkout/src/librustc_mir)
[00:18:39] error: unused `rustc_errors::DiagnosticBuilder` which must be used
[00:18:39]   --> librustc_mir/interpret/step.rs:22:13
[00:18:39]    |
[00:18:39] 22 | /             self.tcx.struct_span_lint_node(
[00:18:39] 23 | |                 ::rustc::lint::builtin::CONST_TIME_LIMIT,
[00:18:39] 24 | |                 node_id,
[00:18:39] 25 | |                 self.frame().span,
[00:18:39] 26 | |                 "constant evaluating a complex constant, this might take some time"
[00:18:39] 27 | |             );
[00:18:39]    |
[00:18:39]    = note: `-D unused-must-use` implied by `-D warnings`
[00:18:39] 
[00:18:39] error: aborting due to previous error
[00:18:39] error: aborting due to previous error
[00:18:39] 
[00:18:39] error: Could not compile `rustc_mir`.
[00:18:39] 
[00:18:39] Caused by:
[00:18:39]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_mir librustc_mir/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=3 -C metadata=8c428dcf5a112e65 -C extra-filename=-8c428dcf5a112e65 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-a7e6c70c1d8dd47c.so --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-ef6ca4ebda266b83.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-a476bbed33563995.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-a476bbed33563995.rlib --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-01d5fdf06e96971b.so --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-949c6ac74d89c4f0.so --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-550225fbe3657ec3.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-bde077c9625bcd15.rlib --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-3024734921a8d3c2.rlib --extern log_settings=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog_settings-67c14357c369f5d3.rlib --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-b6248f90fefe05c1.so --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-7575351e73c880a2.rlib --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-f223cc35df6c29ec.rlib --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-fd56861614616157.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-029c79b35b3d2152.so -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-f085762345e9053e/out/.libs -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-c0082fee642cc0bf/out` (exit code: 101)
unknown-linux-gnu/release/deps
68788 ./src/llvm/lib
65420 ./src/llvm-emscripten/test/CodeGen
60840 ./src/llvm-emscripten/lib
