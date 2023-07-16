plain
[00:07:59]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:08:19]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:15:06]    Compiling rustc_typeck v0.0.0 (file:///checkout/src/librustc_typeck)
[00:15:06]    Compiling rustc_mir v0.0.0 (file:///checkout/src/librustc_mir)
[00:15:08] error[E0425]: cannot find value `lint_root` in this scope
[00:15:08]   --> librustc_mir/interpret/step.rs:23:21
[00:15:08]    |
[00:15:08] 23 |                 .or(lint_root)
[00:15:08] 
[00:15:08] 
[00:15:19] error[E0609]: no field `lint_root` on type `&interpret::eval_context::Frame<'_, '_>`
[00:15:19]   --> librustc_mir/interpret/step.rs:21:43
[00:15:19]    |
[00:15:19] 21 |                 .filter_map(|frame| frame.lint_root)
[00:15:19] 
[00:15:19] 
[00:15:19] error[E0599]: no method named `lint_level_at_node` found for type `&mut interpret::eval_context::EvalContext<'a, 'mir, 'tcx, M>` in the current scope
[00:15:19]   --> librustc_mir/interpret/step.rs:31:21
[00:15:19]    |
[00:15:19] 31 |             if self.lint_level_at_node(lint, node_id).0 == lint::Level::Deny ||
[00:15:19] 
[00:15:19] 
[00:15:19] error[E0599]: no method named `lint_level_at_node` found for type `&mut interpret::eval_context::EvalContext<'a, 'mir, 'tcx, M>` in the current scope
[00:15:19]   --> librustc_mir/interpret/step.rs:32:22
[00:15:19]    |
[00:15:19] 32 |                 self.lint_level_at_node(lint, node_id).0 == lint::Level::Forbid {
[00:15:19] 
[00:15:19] 
[00:15:19] error[E0599]: no method named `report_as_err` found for type `rustc_errors::DiagnosticBuilder<'_>` in the current scope
[00:15:19]   --> librustc_mir/interpret/step.rs:33:25
[00:15:19]    |
[00:15:19] 33 |                     err.report_as_err(self.tcx, msg, node_id);
[00:15:19] 
[00:15:19] error[E0599]: no method named `report_as_lint` found for type `rustc_errors::DiagnosticBuilder<'_>` in the current scope
[00:15:19] error[E0599]: no method named `report_as_lint` found for type `rustc_errors::DiagnosticBuilder<'_>` in the current scope
[00:15:19]   --> librustc_mir/interpret/step.rs:35:25
[00:15:19]    |
[00:15:19] 35 |                     err.report_as_lint(self.tcx, msg);
[00:15:19] 
[00:15:20] error: aborting due to 6 previous errors
[00:15:20] 
[00:15:20] Some errors occurred: E0425, E0599, E0609.
[00:15:20] Some errors occurred: E0425, E0599, E0609.
[00:15:20] For more information about an error, try `rustc --explain E0425`.
[00:15:20] error: Could not compile `rustc_mir`.
[00:15:20] 
[00:15:20] Caused by:
[00:15:20]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_mir librustc_mir/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=1f35ea53eb60b9cb -C extra-filename=-1f35ea53eb60b9cb --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-670f6bac60d99b34.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-58741ed9de9aae4f.so --extern log_settings=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog_settings-f16e9eee09cda644.rlib --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-642a7efe79e24835.rlib --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-5073f1296cd24b67.rlib --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-6a2f0731783c2bd3.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-5d0a8a65bb9fe29f.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-5d0a8a65bb9fe29f.rlib --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-ccde2368d50449de.so --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-99b4534960f92449.rlib --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-c9d6678b1c0f0b46.so --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-01e98699f9876faa.so --extern either=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libeither-11fcd35bf55f48b3.rlib --extern polonius_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libpolonius_engine-6e0be5cf77966185.rlib --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-a7df2fc298b4fb06.so --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-be994de8b3050feb.so --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-78ebc049d4f53f46.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-88673787176f9d86/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-62d80197b9ec531e/out` (exit code: 101)
[00:17:19] error: build failed
[00:17:19] error: build failed
[00:17:19] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:17:19] expected success, got: exit code: 101
[00:17:19] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:17:19] travis_fold:end:stage0-rustc

[00:17:19] travis_time:end:stage0-rustc:start=1528711271095220488,finish=1528711992331609516,duration=721236389028


[00:17:19] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:17:19] Build completed unsuccessfully in 0:12:15
[00:17:19] Makefile:28: recipe for target 'all' failed
[00:17:19] make: *** [all] Error 1
397160 ./.git/objects
397120 ./.git/objects/pack
315012 ./src/llvm
249872 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib
---
147088 ./.git/modules/src
137992 ./obj/build/bootstrap/debug/incremental
132184 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc
123420 ./obj/build/bootstrap/debug/incremental/bootstrap-1r3bppl29tbrj
123416 ./obj/build/bootstrap/debug/incremental/bootstrap-1r3bppl29tbrj/s-f1vs6d7kmt-p9h87q-bfrkfwrvh0tv
89808 ./src/llvm/test/CodeGen
81432 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu
81428 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release
72480 ./.git/modules/src/tools
