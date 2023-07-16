plain
[00:06:21]    Compiling rustc_data_structures v0.0.0 (file:///checkout/src/librustc_data_structures)
[00:06:26]    Compiling arena v0.0.0 (file:///checkout/src/libarena)
[00:06:26]    Compiling syntax_pos v0.0.0 (file:///checkout/src/libsyntax_pos)
[00:06:33]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:06:41] error[E0425]: cannot find function `contains_novel_literal` in this scope
[00:06:41]     --> libsyntax/feature_gate.rs:1525:47
[00:06:41]      |
[00:06:41] 1525 |                     if !allow_attr_literal && contains_novel_literal(&meta) {
[00:06:41] 
[00:06:41] 
[00:06:44] error[E0609]: no field `attr_literals` on type `&feature_gate::Features`
[00:06:44]     --> libsyntax/feature_gate.rs:1526:51
[00:06:44]      |
[00:06:44] 1526 |                         gate_feature_post!(&self, attr_literals, attr.span,
[00:06:44] 
[00:06:47] error: aborting due to 2 previous errors
[00:06:47] 
[00:06:47] Some errors occurred: E0425, E0609.
[00:06:47] Some errors occurred: E0425, E0609.
[00:06:47] For more information about an error, try `rustc --explain E0425`.
[00:06:47] error: Could not compile `syntax`.
[00:06:47] 
[00:06:47] Caused by:
[00:06:47]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name syntax libsyntax/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=87a5880d376ced18 -C extra-filename=-87a5880d376ced18 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-3907cba388d41ef0.rlib --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-b6c566856a1e65b9.rlib --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-f0f8a6e97a1485be.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-b3c36f4b5b85fd64.so --extern rustc_target=/checkout/obj/build/x86_64-unknThu, 23 Aug 2018 23:09:32 GMT

The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
