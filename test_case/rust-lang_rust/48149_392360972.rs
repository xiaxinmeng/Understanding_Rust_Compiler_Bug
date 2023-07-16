plain
[00:07:49]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:08:09]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:14:38]    Compiling rustc_mir v0.0.0 (file:///checkout/src/librustc_mir)
[00:14:38]    Compiling rustc_typeck v0.0.0 (file:///checkout/src/librustc_typeck)
[00:14:39] error[E0599]: no method named `is_lt_parameterized` found for type `&rustc::hir::Generics` in the current scope
[00:14:39]    --> librustc_typeck/lib.rs:192:62
[00:14:39]     |
[00:14:39] 192 |                                 let param_type = if generics.is_lt_parameterized() {
[00:14:39] 
[00:14:46] error: aborting due to previous error
[00:14:46] 
[00:14:46] For more information about this error, try `rustc --explain E0599`.
[00:14:46] For more information about this error, try `rustc --explain E0599`.
[00:14:46] error: Could not compile `rustc_typeck`.
[00:14:46] 
[00:14:46] Caused by:
[00:14:46]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_typeck librustc_typeck/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=3 -C metadata=6ffecab30d161d45 -C extra-filename=-6ffecab30d161d45 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-6f643d03661f77af.so --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-cfccf57e1967c508.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-a7e6c70c1d8dd47c.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-5c21a695e1301573.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-bde077c9625bcd15.rlib --extern fmt_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libfmt_macros-c2f2c8c35b176352.so --extern rustc_platform_intrinsics=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_platform_intrinsics-a11aa962d27a56e4.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-2bee8f0c1e719b79.so --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-5b4f8bf37669326d.so --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-a8f5cae0510be7d3.so -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-f085762345e9053e/out/.libs -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-c0082fee642cc0bf/out` (exit code: 101)
eSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:0036f018:start=1527450098808194174,finish=1527450098814997050,duration=6802876
travis_fold:end:after_failure.3
