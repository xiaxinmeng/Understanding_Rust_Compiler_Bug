plain
[00:03:48]    Compiling syntax_pos v0.0.0 (file:///checkout/src/libsyntax_pos)
[00:03:49]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:03:51]    Compiling backtrace v0.3.6
[00:04:03]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:04:03] error[E0425]: cannot find function `is_path_segment_keyword` in module `token`
[00:04:03]    --> libproc_macro/lib.rs:821:19
[00:04:03]     |
[00:04:03] 821 |            token::is_path_segment_keyword(ast::Ident::with_empty_ctxt(ident.sym)) {
[00:04:03]     |                   ^^^^^^^^^^^^^^^^^^^^^^^ not found in `token`
[00:04:04] error: aborting due to previous error
[00:04:04] 
[00:04:04] For more information about this error, try `rustc --explain E0425`.
[00:04:04] error: Could not compile `proc_macro`.
[00:04:04] error: Could not compile `proc_macro`.
[00:04:04] 
[00:04:04] Caused by:
[00:04:04]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name proc_macro libproc_macro/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,metadata -C prefer-dynamic -C debug-assertions=off -C overflow-checks=on -C metadata=f65e917d1a6aeb8a -C extra-filename=-f65e917d1a6aeb8a --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-24d16f00de0b3e02.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-750c848698d19171.rmeta --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-54e4cced589011ca.rmeta --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-b878b60a3b79382b.rmeta` (exit code: 101)
[00:04:04] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:04:04] expected success, got: exit code: 101
[00:04:04] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:04:04] travis_fold:end:stage0-rustc

[00:04:04] travis_time:end:stage0-rustc:start=1526541407644825260,finish=1526541444689757796,duration=37044932536

