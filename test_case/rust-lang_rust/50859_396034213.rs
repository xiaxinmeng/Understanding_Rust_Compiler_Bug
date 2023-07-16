plain
[00:07:25]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:07:44]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:14:04]    Compiling rustc_typeck v0.0.0 (file:///checkout/src/librustc_typeck)
[00:14:04]    Compiling rustc_mir v0.0.0 (file:///checkout/src/librustc_mir)
[00:14:06] error[E0433]: failed to resolve. Use of undeclared type or module `lint`
[00:14:06]   --> librustc_mir/interpret/step.rs:30:60
[00:14:06]    |
[00:14:06] 30 |             if self.lint_level_at_node(lint, node_id).0 == lint::Level::Deny ||
[00:14:06]    |                                                            ^^^^ Use of undeclared type or module `lint`
[00:14:06] 
[00:14:06] error[E0433]: failed to resolve. Use of undeclared type or module `lint`
[00:14:06]   --> librustc_mir/interpret/step.rs:31:61
[00:14:06]    |
[00:14:06] 31 |                 self.lint_level_at_node(lint, node_id).0 == lint::Level::Forbid {
[00:14:06]    |                                                             ^^^^ Use of undeclared type or module `lint`
[00:14:06] 
[00:14:06] error[E0425]: cannot find value `frames` in this scope
[00:14:06]   --> librustc_mir/interpret/step.rs:17:27
[00:14:06]    |
[00:14:06] 17 |             let node_id = frames
[00:14:06] 
[00:14:06] 
[00:14:06] error[E0425]: cannot find value `lint_root` in this scope
[00:14:06]   --> librustc_mir/interpret/step.rs:22:21
[00:14:06]    |
[00:14:06] 22 |                 .or(lint_root)
[00:14:06] 
[00:14:06] 
[00:14:17] error[E0599]: no method named `lint_level_at_node` found for type `&mut interpret::eval_context::EvalContext<'a, 'mir, 'tcx, M>` in the current scope
[00:14:17]   --> librustc_mir/interpret/step.rs:30:21
[00:14:17]    |
[00:14:17] 30 |             if self.lint_level_at_node(lint, node_id).0 == lint::Level::Deny ||
[00:14:17] 
[00:14:17] 
[00:14:17] error[E0599]: no method named `lint_level_at_node` found for type `&mut interpret::eval_context::EvalContext<'a, 'mir, 'tcx, M>` in the current scope
[00:14:17]   --> librustc_mir/interpret/step.rs:31:22
[00:14:17]    |
[00:14:17] 31 |                 self.lint_level_at_node(lint, node_id).0 == lint::Level::Forbid {
[00:14:17] 
[00:14:17] 
[00:14:17] error[E0599]: no method named `report_as_err` found for type `rustc_errors::DiagnosticBuilder<'_>` in the current scope
[00:14:17]   --> librustc_mir/interpret/step.rs:32:25
[00:14:17]    |
[00:14:17] 32 |                     err.report_as_err(self.tcx, msg, node_id);
[00:14:17] 
[00:14:17] error[E0599]: no method named `report_as_lint` found for type `rustc_errors::DiagnosticBuilder<'_>` in the current scope
[00:14:17] error[E0599]: no method named `report_as_lint` found for type `rustc_errors::DiagnosticBuilder<'_>` in the current scope
[00:14:17]   --> librustc_mir/interpret/step.rs:34:25
[00:14:17]    |
[00:14:17] 34 |                     err.report_as_lint(self.tcx, msg);
[00:14:17] 
[00:14:18] error: aborting due to 8 previous errors
[00:14:18] 
[00:14:18] Some errors occurred: E0425, E0433, E0599.
[00:14:18] Some errors occurred: E0425, E0433, E0599.
[00:14:18] For more information about an error, try `rustc --explain E0425`.
[00:14:18] error: Could not compile `rustc_mir`.
[00:14:18] 
[00:14:18] Caused by:
[00:14:18]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_mir librustc_mir/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=1f35ea53eb60b9cb -C extra-filename=-1f35ea53eb60b9cb --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-a7df2fc298b4fb06.so --extern polonius_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libpolonius_engine-6e0be5cf77966185.rlib --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-be994de8b3050feb.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-670f6bac60d99b34.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-5073f1296cd24b67.rlib --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-58741ed9de9aae4f.so --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-78ebc049d4f53f46.rlib --extern either=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libeither-11fcd35bf55f48b3.rlib --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-642a7efe79e24835.rlib --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-01e98699f9876faa.so --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-99b4534960f92449.rlib --extern log_settings=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog_settings-f16e9eee09cda644.rlib --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-ccde2368d50449de.so --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-6a2f0731783c2bd3.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-5d0a8a65bb9fe29f.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-5d0a8a65bb9fe29f.rlib --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-c9d6678b1c0f0b46.so -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-88673787176f9d86/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-62d80197b9ec531e/out` (exit code: 101)
[00:16:17] error: build failed
[00:16:17] error: build failed
[00:16:17] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:16:17] expected success, got: exit code: 101
[00:16:17] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:16:17] travis_fold:end:stage0-rustc

[00:16:17] travis_time:end:stage0-rustc:start=1528621880545134295,finish=1528622564456564852,duration=683911430557


[00:16:17] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:16:17] Build completed unsuccessfully in 0:11:36
[00:16:17] Makefile:28: recipe for target 'all' failed
[00:16:17] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00d1a072
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
