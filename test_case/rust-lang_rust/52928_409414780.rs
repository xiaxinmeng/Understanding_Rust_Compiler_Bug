plain
[00:17:00]    Compiling rustc_codegen_utils v0.0.0 (file:///checkout/src/librustc_codegen_utils)
[00:17:21]    Compiling rustc_lint v0.0.0 (file:///checkout/src/librustc_lint)
[00:17:21]    Compiling rustc_borrowck v0.0.0 (file:///checkout/src/librustc_borrowck)
[00:17:21]    Compiling rustc_passes v0.0.0 (file:///checkout/src/librustc_passes)
[00:17:23] error: field is never used: `param_env`
[00:17:23]   --> librustc_borrowck/borrowck/check_loans.rs:94:5
[00:17:23]    |
[00:17:23] 94 |     param_env: ty::ParamEnv<'tcx>,
[00:17:23]    |
[00:17:23]    = note: `-D dead-code` implied by `-D warnings`
[00:17:23] 
[00:17:23] error: aborting due to previous error
[00:17:23] error: aborting due to previous error
[00:17:23] 
[00:17:23] error: Could not compile `rustc_borrowck`.
[00:17:23] 
[00:17:23] Caused by:
[00:17:23]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_borrowck librustc_borrowck/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=59b76794ea6e6838 -C extra-filename=-59b76794ea6e6838 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-0959eff5b195ace2.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-eaf85f66e467efb7.rlib --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-427106130ab2ff56.so --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-3d8079c0b5c752f1.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-890d7a0f1ffe7f0b.so --extern rustc_mir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_mir-5179475e4c9280ac.so --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-1f3f13939c5ccaf0.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-881cb3e68ed15c2c.so -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-523855930bbd979d/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-298541be0c6807b6/out` (exit code: 101)
