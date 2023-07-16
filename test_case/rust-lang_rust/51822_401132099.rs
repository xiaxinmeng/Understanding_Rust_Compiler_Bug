plain
[00:14:13]    Compiling rustc_mir v0.0.0 (file:///checkout/src/librustc_mir)
[00:14:17] error[E0609]: no field `name` on type `&rustc::ty::AssociatedItem`
[00:14:17]     --> librustc_typeck/check/mod.rs:4562:83
[00:14:17]      |
[00:14:17] 4562 |                     .map(|(receiver, method)| format!("{}.{}()", receiver, method.name))
[00:14:17] 
[00:14:22] error: aborting due to previous error
[00:14:22] 
[00:14:22] For more information about this error, try `rustc --explain E0609`.
[00:14:22] For more information about this error, try `rustc --explain E0609`.
[00:14:22] error: Could not compile `rustc_typeck`.
[00:14:22] 
[00:14:22] Caused by:
[00:14:22]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_typeck librustc_typeck/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=c999c5a50c9e4e61 -C extra-filename=-c999c5a50c9e4e61 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-58741ed9de9aae4f.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-c73ed1030caf17c3.so --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-a7df2fc298b4fb06.so --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-b9c6a6d70ded088a.so --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-f0ddda7cc3e4e4d5.so --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-ccde2368d50449de.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-5073f1296cd24b67.rlib --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-fcb7f2ed7949c0c2.so --extern rustc_platform_intrinsics=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_platform_intrinsics-584193bd2f2f0208.so -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-88673787176f9d86/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-62d80197b9ec531e/out` (exit code: 101)
