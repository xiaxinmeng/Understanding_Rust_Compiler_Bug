plain
[00:05:47]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:07:19]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:07:19]    Compiling rustc_const_math v0.0.0 (file:///checkout/src/librustc_const_math)
[00:07:44]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:09:29] error[E0597]: borrowed value does not live long enough
[00:09:29]     --> librustc/session/config.rs:1646:14
[00:09:29]      |
[00:09:29] 1646 |             &edition_name_list(),
[00:09:29]      |              ^^^^^^^^^^^^^^^^^^^ temporary value does not live long enough
[00:09:29] 1654 |     ]);
[00:09:29] 1654 |     ]);
[00:09:29]      |       - temporary value only lives until here
[00:09:29]      |
[00:09:29]      = note: borrowed value must be valid for the static lifetime...
[00:09:29]      = note: consider using a `let` binding to increase its lifetime
[00:09:37] error: aborting due to previous error
[00:09:37] 
[00:09:37] For more information about this error, try `rustc --explain E0597`.
[00:09:37] error: Could not compile `rustc`.
[00:09:37] error: Could not compile `rustc`.
[00:09:37] 
[00:09:37] Caused by:
[00:09:37]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc librustc/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=ba75de4e9c022817 -C extra-filename=-ba75de4e9c022817 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-1299638d641ea770.rlib --extern tempdir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libtempdir-65dde0349e75e965.rlib --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-d3b6fcf798f7d22a.so --extern rustc_const_math=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_const_math-895f7ddc4467bb8d.so --extern flate2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libflate2-3c8223b0152f22a5.rlib --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-c04ded78717d5d67.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-c04ded78717d5d67.rlib --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-9f3518d56a01456f.so --extern backtrace=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbacktrace-109fed6e8125a798.rlib --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-ca61dfec7c40c4d4.rlib --extern jobserver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libjobserver-5bcc8c1ccd509892.rlib --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-609e2421d03f9c9a.so --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-93cb1ddd29ab61a4.so --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-0bde40de32995f14.so --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-9866e194db82a141.rlib --extern fmt_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libfmt_macros-3706e912fdb98df1.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-3e35efb9e5bc7a9a.rlib --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-dd80e21b25e89693.so --extern proc_macro=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libproc_macro-6e4119b5ec8457a3.so --extern lazy_static=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblazy_static-2803512e03c19cf7.rlib --extern rustc_back=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_back-861dba9fe03aa669.so -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-8b35e3c2ea935fab/out/.libs -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-63734d0048644b22/out` (exit code: 101)
[00:09:37] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:09:37] expected success, got: exit code: 101
[00:09:37] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:09:37] travis_fold:end:stage0-rustc

[00:09:37] travis_time:end:stage0-rustc:start=1524174208667859168,finish=1524174477877419023,duration=269209559855


[00:09:37] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:09:37] Build completed unsuccessfully in 0:04:42
[00:09:37] make: *** [all] Error 1
[00:09:37] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:02a9df86
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
