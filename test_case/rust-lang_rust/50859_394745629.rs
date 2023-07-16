plain
[00:07:45]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:08:05]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:14:33]    Compiling rustc_mir v0.0.0 (file:///checkout/src/librustc_mir)
[00:14:33]    Compiling rustc_typeck v0.0.0 (file:///checkout/src/librustc_typeck)
[00:14:34] error[E0423]: expected value, found macro `err`
[00:14:34]   --> librustc_mir/interpret/step.rs:15:30
[00:14:34]    |
[00:14:34] 15 |             self.report(&mut err, false, None, LintId::of(CONST_TIME_LIMIT));
[00:14:34]    |                              |
[00:14:34]    |                              did you mean `Err`?
[00:14:34]    |                              did you mean `Err`?
[00:14:34]    |                              did you mean `err!(...)`?
[00:14:34] 
[00:14:44] error[E0308]: if and else have incompatible types
[00:14:44]     --> librustc_mir/interpret/eval_context.rs:1702:27
[00:14:44]      |
[00:14:44] 1702 |               let mut err = if as_err {
[00:14:44]      |  ___________________________^
[00:14:44] 1703 | |                 ::rustc::middle::const_val::struct_error(*self.tcx, span, "constant evaluation error")
[00:14:44] 1704 | |             } else {
[00:14:44] 1705 | |                 let node_id = self
[00:14:44] 1729 | |                 }
[00:14:44] 1730 | |             };
[00:14:44] 1730 | |             };
[00:14:44]      | |_____________^ expected struct `rustc_errors::DiagnosticBuilder`, found ()
[00:14:44]      = note: expected type `rustc_errors::DiagnosticBuilder<'_>`
[00:14:44]                 found type `()`
[00:14:44] 
[00:14:46] error: aborting due to 2 previous errors
[00:14:46] error: aborting due to 2 previous errors
[00:14:46] 
[00:14:46] Some errors occurred: E0308, E0423.
[00:14:46] For more information about an error, try `rustc --explain E0308`.
[00:14:46] error: Could not compile `rustc_mir`.
[00:14:46] 
[00:14:46] Caused by:
[00:14:46]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_mir librustc_mir/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=b940bdad8ee7868b -C extra-filename=-b940bdad8ee7868b --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-58741ed9de9aae4f.so --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-622bce453bbffbd5.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-41b116eaee1e5535.so --extern log_settings=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog_settings-f16e9eee09cda644.rlib --extern either=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libeither-11fcd35bf55f48b3.rlib --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-9f4c7501a8934cc5.so --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-78ebc049d4f53f46.rlib --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-5073f1296cd24b67.rlib --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-05d59ed98ebd8949.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-5d0a8a65bb9fe29f.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-5d0a8a65bb9fe29f.rlib --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-0404335fb4ae3dc1.so --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-642a7efe79e24835.rlib --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-be994de8b3050feb.so --extern polonius_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libpolonius_engine-fea5947f694181d1.rlib --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-99b4534960f92449.rlib --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-4c5434c80172b18c.so -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-88673787176f9d86/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-62d80197b9ec531e/out` (exit code: 101)
[00:16:37] error: build failed
[00:16:37] travis_fold:end:stage0-rustc

[00:16:37] travis_time:end:stage0-rustc:start=1528210606919227673,finish=1528211289646633967,duration=682727406294
[00:16:37] travis_time:end:stage0-rustc:start=1528210606919227673,finish=1528211289646633967,duration=682727406294

[00:16:37] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:16:37] expected success, got: exit code: 101
[00:16:37] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:16:37] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:16:37] Build completed unsuccessfully in 0:11:35
[00:16:37] Build completed unsuccessfully in 0:11:35
[00:16:37] Makefile:28: recipe for target 'all' failed
[00:16:37] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0899f519
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
