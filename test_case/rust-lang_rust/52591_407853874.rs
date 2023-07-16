plain
[00:05:12]    Compiling syntax_pos v0.0.0 (file:///checkout/src/libsyntax_pos)
[00:05:16]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:06:25]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:06:33]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:06:50] error[E0277]: the trait bound `rustc_data_structures::unify::UnificationTable<rustc_data_structures::unify::InPlace<ty::sty::RegionVid>>: std::default::Default` is not satisfied
[00:06:50]   --> librustc/infer/region_constraints/mod.rs:72:5
[00:06:50]    |
[00:06:50] 72 |     unification_table: ut::UnificationTable<ut::InPlace<ty::RegionVid>>,
[00:06:50]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::default::Default` is not implemented for `rustc_data_structures::unify::UnificationTable<rustc_data_structures::unify::InPlace<ty::sty::RegionVid>>`
[00:06:50]    = note: required by `std::default::Default::default`
[00:06:50] 
[00:07:00] error: aborting due to previous error
[00:07:00] 
[00:07:00] 
[00:07:00] For more information about this error, try `rustc --explain E0277`.
[00:07:00] error: Could not compile `rustc`.
[00:07:00] 
[00:07:00] Caused by:
[00:07:00]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc librustc/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=3969fd5fc27e256c -C extra-filename=-3969fd5fc27e256c --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-5d5d2990686c15ff.so --extern backtrace=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbacktrace-38d27ec0a56fa3cb.rlib --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-693c666d349ae1b6.rlib --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-35cd99699a8927a8.rlib --extern chalk_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libchalk_engine-6fb61f83cb288550.rlib --extern flate2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libflate2-2b513184d9d612c2.rlib --extern fmt_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libfmt_macros-67c8aaeb97e8d843.so --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-09a8465c475acbfe.so --extern jobserver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libjobserver-d405c21c3677962b.rlib --extern lazy_static=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblazy_static-a185b6f37f2c1490.rlib --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-96863e4e4a7d8742.rlib --extern parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libparking_lot-1436fcd917ac8cb6.rlib --extern polonius_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libpolonius_engine-7f18c5c76091a313.rlib --extern proc_macro=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libproc_macro-3856cda38a76ff6d.so --extern rustc_rayon=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon-a4d022068ecc6660.rlib --extern rustc_rayon_core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayonase/build/backtrace-sys-7205e6adb9d66e6b/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-27571ee36438df44/out` (exit code: 101)
[00:07:00] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:07:00] expected success, got: exit code: 101
[00:07:00] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1119:9
[00:07:00] travis_fold:end:stage0-rustc

[00:07:00] travis_time:end:stage0-rustc:start=1532543677279519739,finish=1532543826088657084,duration=148809137345


[00:07:00] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:07:00] Build completed unsuccessfully in 0:03:21
[00:07:00] Makefile:28: recipe for target 'all' failed
[00:07:00] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:08e49b6b
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
