plain
[00:06:38]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:07:01]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:13:00]    Compiling rustc_typeck v0.0.0 (file:///checkout/src/librustc_typeck)
[00:13:00]    Compiling rustc_mir v0.0.0 (file:///checkout/src/librustc_mir)
[00:13:08] error[E0308]: match arms have incompatible types
[00:13:08]    --> librustc_mir/hair/cx/expr.rs:646:21
[00:13:08] 646 | /                     match did {
[00:13:08] 646 | /                     match did {
[00:13:08] 647 | |                         Some(did) => {
[00:13:08] 648 | |                             // in case we are offsetting from a computed discriminant
[00:13:08] 649 | |                             // and not the beginning of discriminants (which is always `0`)
[00:13:08] ...   |
[00:13:08] 664 | |                         None => offset,
[00:13:08]     | |                                 ------ match arm with an incompatible type
[00:13:08] 665 | |                     }
[00:13:08]     | |_____________________^ expected struct `hair::Expr`, found enum `hair::ExprRef`
[00:13:08]     |
[00:13:08]     = note: expected type `hair::Expr<'_>`
[00:13:08]                found type `hair::ExprRef<'_>`
[00:13:13] error: aborting due to previous error
[00:13:13] 
[00:13:13] For more information about this error, try `rustc --explain E0308`.
[00:13:13] error: Could not compile `rustc_mir`.
[00:13:13] error: Could not compile `rustc_mir`.
[00:13:13] 
[00:13:13] Caused by:
[00:13:13]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_mir librustc_mir/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=809b6518a044b653 -C extra-filename=-809b6518a044b653 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-3e9ac89279d9d9fd.so --extern log_settings=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog_settings-70f7ed359bd54256.rlib --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-ca61dfec7c40c4d4.rlib --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-93cb1ddd29ab61a4.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-c04ded78717d5d67.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-c04ded78717d5d67.rlib --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-9f3518d56a01456f.so --extern rustc_const_math=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_const_math-895f7ddc4467bb8d.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-609e2421d03f9c9a.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-3e35efb9e5bc7a9a.rlib --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-9866e194db82a141.rlib --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-1299638d641ea770.rlib --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-d3b6fcf798f7d22a.so --extern rustc_back=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_back-861dba9fe03aa669.so --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-0bde40de32995f14.so --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-dd80e21b25e89693.so -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-22398a187b4139a2/out/.libs -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-4a7fffed6b170d5b/out` (exit code: 101)
[00:14:50] error: build failed
[00:14:50] error: build failed
[00:14:50] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:14:50] expected success, got: exit code: 101
[00:14:50] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:14:50] travis_fold:end:stage0-rustc

[00:14:50] travis_time:end:stage0-rustc:start=1524651203586423494,finish=1524651814846263294,duration=611259839800


[00:14:50] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:14:50] Build completed unsuccessfully in 0:10:23
[00:14:50] Makefile:28: recipe for target 'all' failed
[00:14:50] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0a482a92
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
