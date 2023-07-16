plain
[00:06:21]    Compiling syntax_pos v0.0.0 (file:///checkout/src/libsyntax_pos)
[00:06:25]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:08:02]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:08:21]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:08:23] error[E0432]: unresolved import `mir::interpret::ConstVal`
[00:08:23]   --> librustc/traits/query/normalize.rs:17:32
[00:08:23]    |
[00:08:23] 17 | use mir::interpret::{GlobalId, ConstVal};
[00:08:23]    |                                ^^^^^^^^ no `ConstVal` in `mir::interpret`. Did you mean to use `ConstValue`?
[00:08:24] error[E0433]: failed to resolve. Use of undeclared type or module `ConstValue`
[00:08:24] error[E0433]: failed to resolve. Use of undeclared type or module `ConstValue`
[00:08:24]    --> librustc/traits/query/normalize.rs:197:16
[00:08:24] 197 |         if let ConstValue::Unevaluated(def_id, substs) = constant.val {
[00:08:24]     |                ^^^^^^^^^^ Use of undeclared type or module `ConstValue`
[00:08:24] 
[00:08:25] error: unused import: `ConstVal`
[00:08:25] error: unused import: `ConstVal`
[00:08:25]   --> librustc/traits/query/normalize.rs:17:32
[00:08:25]    |
[00:08:25] 17 | use mir::interpret::{GlobalId, ConstVal};
[00:08:25]    |
[00:08:25]    = note: `-D unused-imports` implied by `-D warnings`
[00:08:25] 
[00:08:25] 
age0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-78ebc049d4f53f46.rlib --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-a7df2fc298b4fb06.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-670f6bac60d99b34.so --extern scoped_tls=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libscoped_tls-b76c070114255d98.rlib --extern backtrace=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbacktrace-f3a2fb1d767a0bf7.rlib --extern proc_macro=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libproc_macro-524e4d04204f0089.so --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-642a7efe79e24835.rlib --extern rustc_rayon=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon-dc5c45209eae7e7b.rlib --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-ccde2368d50449de.so --extern tempfile=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libtempfile-e2afe7409f33879a.rlib --extern parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libparking_lot-210745eb40d9c073.rlib --extern flate2=/checkout/obj/build/x86_6
