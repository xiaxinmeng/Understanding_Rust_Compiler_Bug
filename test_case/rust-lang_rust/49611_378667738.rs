plain
Resolving deltas: 100% (611426/611426), completed with 4858 local objects.
---
[00:00:46] configure: rust.quiet-tests     := True
---
[00:17:00] error[E0425]: cannot find function `is_reserved_ident` in module `token`
[00:17:00]   --> librustc_passes/ast_validation.rs:44:20
[00:17:00]    |
[00:17:00] 44 |             token::is_reserved_ident(lifetime.ident.without_first_quote()) {
[00:17:00]    |                    ^^^^^^^^^^^^^^^^^ not found in `token`
[00:17:00]
[00:17:00] error[E0425]: cannot find function `is_reserved_ident` in module `token`
[00:17:00]   --> librustc_passes/ast_validation.rs:50:19
[00:17:00]    |
[00:17:00] 50 |         if token::is_reserved_ident(label.without_first_quote()) {
[00:17:00]    |                   ^^^^^^^^^^^^^^^^^ not found in `token`
---
[00:17:00]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_passes librustc_passes/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=d6aeea102ad4d097 -C extra-filename=-d6aeea102ad4d097 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-b061046bb39a474d.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-069d1433ad059ff1.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-7eebd272275b441b.rlib --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-67023beb7edf290b.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-d823127e323fa243.so --extern rustc_const_math=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_const_math-2cbdd37d611532dc.so --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-1e725c22de2888b8.so --extern rustc_mir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_mir-1b4c3b4ba4ec5832.so -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-751752dec0960570/out/.libs -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-07c608814ba8d6ba/out` (exit code: 101)
[00:17:00] warning: build failed, waiting for other jobs to finish...
[00:17:13] error: build failed
[00:17:13] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1064:9
[00:17:13] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:17:13] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:17:13] expected success, got: exit code: 101
[00:17:13] travis_fold:end:stage0-rustc
[00:17:13] travis_time:end:stage0-rustc:start=1522859599377471700,finish=1522860352901180311,duration=753523708611
[00:17:13] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:17:13] Build completed unsuccessfully in 0:12:46
[00:17:13] Makefile:28: recipe for target 'all' failed
[00:17:13] make: *** [all] Error 1
---
$ dmesg | grep -i kill
[   10.155019] init: failsafe main process (1096) killed by TERM signal
