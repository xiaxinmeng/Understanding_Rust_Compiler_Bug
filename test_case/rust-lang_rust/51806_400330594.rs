plain
[00:06:09]    Compiling syntax_pos v0.0.0 (file:///checkout/src/libsyntax_pos)
[00:06:13]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:07:50]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:08:14]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:08:17] error[E0425]: cannot find value `closure_id` in this scope
[00:08:17]     --> librustc/hir/lowering.rs:3637:49
[00:08:17]      |
[00:08:17] 3637 |                                 capture_clause, closure_id, async_ret_ty,
[00:08:17] 
[00:08:17] 
[00:08:29] error[E0026]: variant `syntax::ast::IsAsync::Async` does not have a field named `async_closure_node_id`
[00:08:29]     --> librustc/hir/lowering.rs:3601:41
[00:08:29]      |
[00:08:29] 3601 |                 if let IsAsync::Async { async_closure_node_id, .. } = asyncness {
[00:08:29]      |                                         ^^^^^^^^^^^^^^^^^^^^^ variant `syntax::ast::IsAsync::Async` does not have this field
[00:08:49] error: aborting due to 2 previous errors
[00:08:49] 
[00:08:49] Some errors occurred: E0026, E0425.
[00:08:49] For more information about an error, try `rustc --explain E0026`.
[00:08:49] For more information about an error, try `rustc --explain E0026`.
[00:08:49] error: Could not compile `rustc`.
[00:08:49] 
[00:08:49] Caused by:
[00:08:49]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc librustc/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=f81250751e0ebed7 -C extra-filename=-f81250751e0ebed7 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-58741ed9de9aae4f.so --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-99b4534960f92449.rlib --extern fmt_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libfmt_macros-a1ad98118554e7d4.so --extern flate2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libflate2-5eb12204825df3be.rlib --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stagown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libproc_macro-524e4d04204f0089.so --extern rustc_rayon_core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon_core-6f8a066c77c355d9.rlib --extern polonius_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libpolonius_engine-6e0be5cf77966185.rlib --extern chalk_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libchalk_engine-dc5a2a01279da0f2.rlib --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-670f6bac60d99b34.so --extern rustc_rayon=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon-dc5c45209eae7e7b.rlib --extern parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libparking_lot-210745eb40d9c073.rlib --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-5073f1296cd24b67.rlib --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-be994de8b3050feb.so --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-ccde2368d50449de.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-c9d6678b1c0f0b46.so -L native=/checkout/obj/build/x86_64Tue, 26 Jun 2018 14:31:17 GMT

The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
