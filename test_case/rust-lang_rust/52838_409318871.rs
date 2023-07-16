plain
[00:05:35]    Compiling syntax_pos v0.0.0 (file:///checkout/src/libsyntax_pos)
[00:05:39]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:06:58]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:07:07]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:07:08] error[E0658]: use of unstable library feature 'int_to_from_bytes' (see issue #49792)
[00:07:08]   --> libsyntax_ext/concat.rs:57:50
[00:07:08]    |
[00:07:08] 57 |                     unified_accumulator.extend(i.to_bytes().iter());
[00:07:08]    |
[00:07:08]    = help: add #![feature(int_to_from_bytes)] to the crate attributes to enable
[00:07:08] 
[00:07:08] error: aborting due to previous error
[00:07:08] error: aborting due to previous error
[00:07:08] 
[00:07:08] For more information about this error, try `rustc --explain E0658`.
[00:07:08] error: Could not compile `syntax_ext`.
[00:07:08] 
[00:07:08] Caused by:
[00:07:08]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name syntax_ext libsyntax_ext/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=6a9676b2920a8fc7 -C extra-filename=-6a9676b2920a8fc7 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern fmt_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libfmt_macros-12d96abeeae9fd70.so --extern proc_macro=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libproc_macro-571fcf63af2b690f.so --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-3d8079c0b5c752f1.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-890d7a0f1ffe7f0b.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-1508bcc57003426f.so --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-1f3f13939c5ccaf0.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-881cb3e68ed15c2c.so` (exit code: 101)
