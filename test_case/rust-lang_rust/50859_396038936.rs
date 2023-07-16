plain
[00:07:39]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:07:58]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:14:24]    Compiling rustc_mir v0.0.0 (file:///checkout/src/librustc_mir)
[00:14:24]    Compiling rustc_typeck v0.0.0 (file:///checkout/src/librustc_typeck)
[00:14:26] error[E0425]: cannot find value `frames` in this scope
[00:14:26]   --> librustc_mir/interpret/step.rs:18:27
[00:14:26]    |
[00:14:26] 18 |             let node_id = frames
[00:14:26] 
[00:14:26] 
[00:14:26] error[E0425]: cannot find value `lint_root` in this scope
[00:14:26]   --> librustc_mir/interpret/step.rs:23:21
[00:14:26]    |
[00:14:26] 23 |                 .or(lint_root)
[00:14:26] 
[00:14:26] 
[00:14:37] error[E0599]: no method named `lint_level_at_node` found for type `&mut interpret::eval_context::EvalContext<'a, 'mir, 'tcx, M>` in the current scope
[00:14:37]   --> librustc_mir/interpret/step.rs:31:21
[00:14:37]    |
[00:14:37] 31 |             if self.lint_level_at_node(lint, node_id).0 == lint::Level::Deny ||
[00:14:37] 
[00:14:37] 
[00:14:37] error[E0599]: no method named `lint_level_at_node` found for type `&mut interpret::eval_context::EvalContext<'a, 'mir, 'tcx, M>` in the current scope
[00:14:37]   --> librustc_mir/interpret/step.rs:32:22
[00:14:37]    |
[00:14:37] 32 |                 self.lint_level_at_node(lint, node_id).0 == lint::Level::Forbid {
[00:14:37] 
[00:14:37] 
[00:14:37] error[E0599]: no method named `report_as_err` found for type `rustc_errors::DiagnosticBuilder<'_>` in the current scope
[00:14:37]   --> librustc_mir/interpret/step.rs:33:25
[00:14:37]    |
[00:14:37] 33 |                     err.report_as_err(self.tcx, msg, node_id);
[00:14:37] 
[00:14:37] error[E0599]: no method named `report_as_lint` found for type `rustc_errors::DiagnosticBuilder<'_>` in the current scope
[00:14:37] error[E0599]: no method named `report_as_lint` found for type `rustc_errors::DiagnosticBuilder<'_>` in the current scope
[00:14:37]   --> librustc_mir/interpret/step.rs:35:25
[00:14:37]    |
[00:14:37] 35 |                     err.report_as_lint(self.tcx, msg);
[00:14:37] 
[00:14:38] error: aborting due to 6 previous errors
[00:14:38] 
[00:14:38] Some errors occurred: E0425, E0599.
[00:14:38] Some errors occurred: E0425, E0599.
[00:14:38] For more information about an error, try `rustc --explain E0425`.
[00:14:38] error: Could not compile `rustc_mir`.
[00:14:38] 
[00:14:38] Caused by:
[00:14:38]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_mir librustc_mir/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=1f35ea53eb60b9cb -C extra-filename=-1f35ea53eb60b9cb --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern log_settings=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog_settings-f16e9eee09cda644.rlib --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-be994de8b3050feb.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-5073f1296cd24b67.rlib --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-58741ed9de9aae4f.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-5d0a8a65bb9fe29f.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-5d0a8a65bb9fe29f.rlib --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-99b4534960f92449.rlib --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-ccde2368d50449de.so --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-01e98699f9876faa.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-670f6bac60d99b34.so --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-78ebc049d4f53f46.rlib --extern polonius_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libpolonius_engine-6e0be5cf77966185.rlib --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-6a2f0731783c2bd3.so --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-642a7efe79e24835.rlib --extern either=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libeither-11fcd35bf55f48b3.rlib --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-a7df2fc298b4fb06.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-c9d6678b1c0f0b46.so -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-88673787176f9d86/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-62d80197b9ec531e/out` (exit code: 101)
[00:16:34] error: build failed
[00:16:34] error: build failed
[00:16:34] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:16:34] expected success, got: exit code: 101
[00:16:34] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:16:34] travis_fold:end:stage0-rustc

[00:16:34] travis_time:end:stage0-rustc:start=1528626835023931596,finish=1528627525550085461,duration=690526153865


[00:16:34] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:16:34] Build completed unsuccessfully in 0:11:43
[00:16:34] make: *** [all] Error 1
[00:16:34] Makefile:28: recipe for target 'all' failed
23704 ./src/llvm/test/tools
22656 ./src/llvm-emscripten/test/MC
21136 ./src/llvm-emscripten/test/Transforms
20152 ./.git/modules/src/tools/cargo
---
travis_time:end:0826b336:start=1528627526136454245,finish=1528627526142669983,duration=6215738
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1bb37650
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:090071da
$ dmesg | grep -i kill
