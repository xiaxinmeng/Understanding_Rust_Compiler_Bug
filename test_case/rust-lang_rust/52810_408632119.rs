plain
[00:06:59]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:07:10]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:12:42]    Compiling rustc_mir v0.0.0 (file:///checkout/src/librustc_mir)
[00:12:42]    Compiling rustc_typeck v0.0.0 (file:///checkout/src/librustc_typeck)
[00:12:48] error[E0599]: no method named `apush` found for type `rustc_data_structures::indexed_vec::IndexVec<rustc::mir::Local, rustc::mir::LocalDecl<'tcx>>` in the current scope
[00:12:48]     --> librustc_mir/build/matches/mod.rs:1216:58
[00:12:48]      |
[00:12:48] 1216 |                 let val_for_guard_idx = self.local_decls.apush(LocalDecl {
[00:12:48]      |
[00:12:48]      |
[00:12:48]      = help: did you mean `push`?
[00:12:54] error: aborting due to previous error
[00:12:54] 
[00:12:54] For more information about this error, try `rustc --explain E0599`.
[00:12:54] error: Could not compile `rustc_mir`.
[00:12:54] error: Could not compile `rustc_mir`.
[00:12:54] 
[00:12:54] Caused by:
[00:12:54]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_mir librustc_mir/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=c9be4d191c952b3c -C extra-filename=-c9be4d191c952b3c --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-793da96b213d7844.so --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-1ae79b79f441d17a.rlib --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-9b67c77caff626b1.rlib --extern either=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libeither-21b97ae521a4c1d8.rlib --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-0959eff5b195ace2.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-eaf85f66e467efb7.rlib --extern log_settings=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog_settings-2a6ed09af6dbfac4.rlib --extern polonius_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libpolonius_engine-1027159c1712a408.rlib --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-09d905e44a1d835c.so --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-8ccf59c5e9e172ee.rlib --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-3f4afd8f1fa86b47.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-c56afaa02a608b32.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-1508bcc57003426f.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-a86c1222f09eecc5.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-a86c1222f09eecc5.rlib --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-ccb7d1024b71ac73.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-555c0a62ece4d8d0.so -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-523855930bbd979d/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-298541be0c6807b6/out` (exit code: 101)
[00:14:24] error: build failed
[00:14:24] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:14:24] expected success, got: exit code: 101
[00:14:24] expected success, got: exit code: 101
[00:14:24] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1119:9
[00:14:24] travis_fold:end:stage0-rustc

[00:14:24] travis_time:end:stage0-rustc:start=1532808051079801009,finish=1532808619891015961,duration=568811214952


[00:14:24] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:14:24] Build completed unsuccessfully in 0:10:26
[00:14:24] Makefile:28: recipe for target 'all' failed
[00:14:24] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:004fb3dd
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:3b20611c:start=1532808620549644941,finish=1532808620558089950,duration=8445009
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:34976c5c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:032ee949
$ cat ./obj/build/x86_64-unknown-linux-
