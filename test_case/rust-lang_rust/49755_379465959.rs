plain
Resolving deltas: 100% (612958/612958), completed with 4866 local objects.
---
[00:00:43] configure: rust.quiet-tests     := True
---
[00:07:13] error[E0432]: unresolved import `codemap`
[00:07:13]   --> librustc/lint/mod.rs:57:5
[00:07:13]    |
[00:07:13] 57 | use codemap::{ExpnFormat, ExpnInfo, Span };
[00:07:13]    |     ^^^^^^^ Did you mean `syntax::codemap`?
[00:07:13]
[00:07:14] error[E0433]: failed to resolve. Use of undeclared type or module `lint`
[00:07:14]    --> librustc/lint/context.rs:469:13
[00:07:14]     |
[00:07:14] 469 |         if !lint::in_external_macro(lint, span) {
[00:07:14]     |             ^^^^ Use of undeclared type or module `lint`
[00:07:14]
[00:07:17] error: unused import: `Span`
[00:07:17]   --> librustc/lint/mod.rs:57:37
[00:07:17]    |
[00:07:17] 57 | use codemap::{ExpnFormat, ExpnInfo, Span };
---
[00:07:17]    = note: #[deny(unused_imports)] implied by #[deny(warnings)]
[00:07:17]
gnu/release/deps/libbitflags-e01ce88b04783514.rlib --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-f27fded04dd31022.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-f27fded04dd31022.rlib --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-7eebd272275b441b.rlib --extern fmt_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libfmt_macros-5106459147843225.so --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-f04ca02424a68c4e.rlib --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-1e725c22de2888b8.so --extern backtrace=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbacktrace-a7efadae36f54a3c.rlib --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-b23276248c684a7d.rlib --extern flate2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libflate2-0249ed74490015f9.rlib --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-069d1433ad059ff1.so --extern lazy_static=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblazy_static-3c1d922a28d91411.rlib --extern jobserver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libjobserver-3c385cb05f9c08fa.rlib --extern tempdir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libtempdir-728fcbc86125f341.rlib --extern proc_macro=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libproc_macro-414d7e12269f768c.so --extern rustc_const_math=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_const_math-2cbdd37d611532dc.so -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-751752dec0960570/out/.libs -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-07c608814ba8d6ba/out` (exit code: 101)
[00:07:52] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:07:52] expected success, got: exit code: 101
[00:07:52] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1085:9
[00:07:52] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:07:52] travis_fold:end:stage0-rustc
[00:07:52] travis_time:end:stage0-rustc:start=1523104033367956872,finish=1523104230584479106,duration=197216522234
[00:07:52] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:07:52] Build completed unsuccessfully in 0:03:30
[00:07:52] Makefile:28: recipe for target 'all' failed
[00:07:52] make: *** [all] Error 1
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or di
