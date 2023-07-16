plain
Resolving deltas: 100% (604721/604721), completed with 4781 local objects.
---
[00:01:59] configure: rust.quiet-tests     := True
---
[00:15:13] 299 |         self.cancel_if_wrong_origin(err, o)
[00:15:13]     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ lifetime mismatch
[00:15:13]     |
[00:15:13]     = note: expected type `rustc_errors::DiagnosticBuilder<'_>`
[00:15:13]                found type `rustc_errors::DiagnosticBuilder<'cx>`
[00:15:13] note: the lifetime 'cx as defined on the trait at 55:1...
[00:15:13]    --> librustc_mir/util/borrowck_errors.rs:55:1
[00:15:13]     |
[00:15:13] 55  | pub trait BorrowckErrors<'cx>: Sized + Copy {
[00:15:13]     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:15:13] note: ...does not necessarily outlive the anonymous lifetime #1 defined on the method body at 287:5
[00:15:13]    --> librustc_mir/util/borrowck_errors.rs:287:5
[00:15:13]     |
[00:15:13] 287 | /     fn cannot_assign(&self, span: Span, desc: &str, o: Origin, is_reference:bool)
[00:15:13] 288 | |                      -> DiagnosticBuilder
[00:15:13] 289 | |     {
[00:15:13] 290 | |         let msg = if is_reference {
[00:15:13] ...   |
[00:15:13] 299 | |         self.cancel_if_wrong_origin(err, o)
[00:15:13] 300 | |     }
[00:15:13]     | |_____^
[00:15:13]
[00:15:13] error[E0309]: the parameter type `Self` may not live long enough
[00:15:13]    --> librustc_mir/util/borrowck_errors.rs:305:9
[00:15:13]     |
[00:15:13] 305 |         self.cannot_assign(span, &format!("immutable static item `{}`", desc), o, false)
[00:15:13]     |         ^^^^
[00:15:13]     |
[00:15:13]     = help: consider adding an explicit lifetime bound `Self: 'cx`...
[00:15:13] note: ...so that the type `Self` is not borrowed for too long
[00:15:13]    --> librustc_mir/util/borrowck_errors.rs:305:9
[00:15:13]     |
[00:15:13] 305 |         self.cannot_assign(span, &format!("immutable static item `{}`", desc), o, false)
[00:15:13]     |         ^^^^
[00:15:13]
[00:15:13] error[E0309]: the parameter type `Self` may not live long enough
[00:15:13]    --> librustc_mir/util/borrowck_errors.rs:305:14
[00:15:13]     |
[00:15:13] 305 |         self.cannot_assign(span, &format!("immutable static item `{}`", desc), o, false)
[00:15:13]     |              ^^^^^^^^^^^^^
[00:15:13]     |
[00:15:13]     = help: consider adding an explicit lifetime bound `Self: 'cx`...
[00:15:13] note: ...so that the reference type `&Self` does not outlive the data it points at
[00:15:13]    --> librustc_mir/util/borrowck_errors.rs:305:14
[00:15:13]     |
[00:15:13] 305 |         self.cannot_assign(span, &format!("immutable static item `{}`", desc), o, false)
---
[00:15:16]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_mir librustc_mir/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=1b4c3b4ba4ec5832 -C extra-filename=-1b4c3b4ba4ec5832 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-b23276248c684a7d.rlib --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-069d1433ad059ff1.so --extern log_settings=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog_settings-6f97f4efb9094c9d.rlib --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-f04ca02424a68c4e.rlib --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-01fc77d929e8bdd6.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-f27fded04dd31022.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-f27fded04dd31022.rlib --extern rustc_const_math=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_const_math-2cbdd37d611532dc.so --extern rustc_back=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_back-c188df8980be19c3.so --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-67023beb7edf290b.so --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-b061046bb39a474d.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-d823127e323fa243.so --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-2e4ce590106ede04.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-7eebd272275b441b.rlib --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-e01ce88b04783514.rlib --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-1e725c22de2888b8.so -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-751752dec0960570/out/.libs -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-07c608814ba8d6ba/out` (exit code: 101)
[00:15:16] warning: build failed, waiting for other jobs to finish...
[00:16:04] error: build failed
[00:16:04] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:16:04] expected success, got: exit code: 101
[00:16:04] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1085:9
[00:16:04] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:16:04] travis_fold:end:stage0-rustc
[00:16:04] travis_time:end:stage0-rustc:start=1522949892723702712,finish=1522950514379066873,duration=621655364161
[00:16:04] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:16:04] Build completed unsuccessfully in 0:10:34
[00:16:04] Makefile:28: recipe for target 'all' failed
[00:16:04] make: *** [all] Error 1
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:06721130:start=1522950515012849782,finish=1522950515018970371,duration=6120589
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:109f6e6c
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis
