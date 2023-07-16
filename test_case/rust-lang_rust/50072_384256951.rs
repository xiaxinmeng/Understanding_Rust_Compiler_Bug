plain
[00:06:18]    Compiling rustc_allocator v0.0.0 (file:///checkout/src/librustc_allocator)
[00:06:18]    Compiling rustc_resolve v0.0.0 (file:///checkout/src/librustc_resolve)
[00:06:21]    Compiling rustc_traits v0.0.0 (file:///checkout/src/librustc_traits)
[00:06:23]    Compiling rustc_plugin v0.0.0 (file:///checkout/src/librustc_plugin)
[00:06:24] error[E0308]: match arms have incompatible types
[00:06:24]    --> librustc_mir/hair/cx/expr.rs:646:21
[00:06:24] 646 | /                     match did {
[00:06:24] 646 | /                     match did {
[00:06:24] 647 | |                         Some(did) => {
[00:06:24] 648 | |                             // in case we are offsetting from a computed discriminant
[00:06:24] 649 | |                             // and not the beginning of discriminants (which is always `0`)
[00:06:24] ...   |
[00:06:24] 664 | |                         None => offset,
[00:06:24]     | |                                 ------ match arm with an incompatible type
[00:06:24] 665 | |                     }
[00:06:24]     | |_____________________^ expected struct `hair::Expr`, found enum `hair::ExprRef`
[00:06:24]     |
[00:06:24]     = note: expected type `hair::Expr<'_>`
[00:06:24]                found type `hair::ExprRef<'_>`
[00:06:28] error: aborting due to previous error
[00:06:28] 
[00:06:28] For more information about this error, try `rustc --explain E0308`.
[00:06:28] error: Could not compile `rustc_mir`.
[00:06:28] error: Could not compile `rustc_mir`.
[00:06:28] 
[00:06:28] Caused by:
[00:06:28]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_mir librustc_mir/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,metadata -C prefer-dynamic -C debug-assertions=off -C overflow-checks=on -C metadata=4a095d98bf09b735 -C extra-filename=-4a095d98bf09b735 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -C incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/incremental -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-44651fb323089e23.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-c43c75e6fa38c52c.rmeta --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-4779ef091a9cf2d0.rmeta --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-5726b7c8b62bc86c.rmeta --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-558590efb5478820.rmeta --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-42e2b3d79b612245.rmeta --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-b90d1b4d1642a393.rmeta --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-e790766762a27d5b.rmeta --extern rustc_const_math=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_const_math-2bde2a5e13686daa.rmeta --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-75f2b25c19b6a07d.rmeta --extern rustc_back=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_back-dab4dcedeb876e0f.rmeta --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-90283f9e250487de.rmeta --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-ee064adeca477563.rmeta --extern log_settings=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog_settings-1641cbd4836aff89.rmeta --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-1dffe97d0651361b.rmeta -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-22398a187b4139a2/out/.libs -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-4a7fffed6b170d5b/out` (exit code: 101)
[00:06:34] error: build failed
[00:06:34] error: build failed
[00:06:34] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:06:34] expected success, got: exit code: 101
[00:06:34] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:06:34] travis_fold:end:stage0-rustc

[00:06:34] travis_time:end:stage0-rustc:start=1524656124701754019,finish=1524656303500906554,duration=178799152535

