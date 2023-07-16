plain
[00:05:59]    Compiling arena v0.0.0 (file:///checkout/src/libarena)
[00:06:07]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:07:47]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:08:07]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:08:07] error[E0433]: failed to resolve. Use of undeclared type or module `fmt`
[00:08:07]    --> libsyntax_ext/format_foreign.rs:992:57
[00:08:07]     |
[00:08:07] 992 |         fn fmt(&self, fmt: &mut std::fmt::Formatter) -> fmt::Result {
[00:08:07]     |                                                         ^^^ Use of undeclared type or module `fmt`
[00:08:08] error: aborting due to previous error
[00:08:08] 
[00:08:08] For more information about this error, try `rustc --explain E0433`.
[00:08:08] error: Could not compile `syntax_ext`.
[00:08:08] error: Could not compile `syntax_ext`.
[00:08:08] 
[00:08:08] Caused by:
[00:08:08]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name syntax_ext libsyntax_ext/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=3 -C metadata=4a747fc4f7c945c8 -C extra-filename=-4a747fc4f7c945c8 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-aed9d8ab86b35123.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-b789a86e1ab64d11.so --extern proc_macro=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libproc_macro-8d928be2ff984c7f.so --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-f456f53371aa074c.so --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-566a8d95e6a18781.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-b9901acb1e9e6766.so --extern fmt_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libfmt_macros-33787dcdac3a4dd2.so` (exit code: 101)
