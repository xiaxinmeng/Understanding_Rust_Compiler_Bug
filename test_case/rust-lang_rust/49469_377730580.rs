plain
Resolving deltas: 100% (613609/613609), completed with 4861 local objects.
---
[00:00:41] configure: rust.quiet-tests     := True
---
[00:13:54] error[E0616]: field `features` of struct `rustc::session::Session` is private
[00:13:54]    --> librustc_mir/hair/pattern/check_match.rs:370:32
[00:13:54]     |
[00:13:54] 370 |                             if cx.tcx.sess.features.borrow().irrefutable_let_pattern {
[00:13:54]     |                                ^^^^^^^^^^^^^^^^^^^^
[00:13:54]
[00:13:54] error[E0609]: no field `irrefutable_let_pattern` on type `std::cell::Ref<'_, std::option::Option<syntax::feature_gate::Features>>`
[00:13:54]    --> librustc_mir/hair/pattern/check_match.rs:370:62
[00:13:54]     |
[00:13:54] 370 |                             if cx.tcx.sess.features.borrow().irrefutable_let_pattern {
[00:13:54]     |                                                              ^^^^^^^^^^^^^^^^^^^^^^^ unknown field
[00:13:54]
[00:13:54] error[E0616]: field `features` of struct `rustc::session::Session` is private
[00:13:54]    --> librustc_mir/hair/pattern/check_match.rs:410:40
[00:13:54]     |
[00:13:54] 410 |                                     if cx.tcx.sess.features.borrow().irrefutable_let_pattern {
[00:13:54]     |                                        ^^^^^^^^^^^^^^^^^^^^
[00:13:54]
[00:13:54] error[E0609]: no field `irrefutable_let_pattern` on type `std::cell::Ref<'_, std::option::Option<syntax::feature_gate::Features>>`
[00:13:54]    --> librustc_mir/hair/pattern/check_match.rs:410:70
[00:13:54]     |
[00:13:54] 410 |                                     if cx.tcx.sess.features.borrow().irrefutable_let_pattern {
[00:13:54]     |                                                                      ^^^^^^^^^^^^^^^^^^^^^^^ unknown field
---
[00:13:59]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_mir librustc_mir/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=02ca437d4346e274 -C extra-filename=-02ca437d4346e274 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-2e4ce590106ede04.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-e8abef14b8b443aa.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-7eebd272275b441b.rlib --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-f27fded04dd31022.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-f27fded04dd31022.rlib --extern rustc_back=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_back-078085824a3af3a6.so --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-e01ce88b04783514.rlib --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-b23276248c684a7d.rlib --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-505a23f516928b0b.so --extern log_settings=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog_settings-6f97f4efb9094c9d.rlib --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-01f70e5b8d09b4d5.rlib --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-7437cee1018df6d3.so --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-01fc77d929e8bdd6.so --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-50848c58fc469522.so --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-7eb604ad0d7f2749.so --extern rustc_const_math=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_const_math-9784fba5d291c443.so -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-751752dec0960570/out/.libs -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-07c608814ba8d6ba/out` (exit code: 101)
[00:13:59] warning: build failed, waiting for other jobs to finish...
[00:14:35] error: build failed
[00:14:35] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1064:9
[00:14:35] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:14:35] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:14:35] expected success, got: exit code: 101
[00:14:35] travis_fold:end:stage0-rustc
[00:14:35] travis_time:end:stage0-rustc:start=1522538058280003672,finish=1522538633576054366,duration=575296050694
[00:14:35] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:14:35] Build completed unsuccessfully in 0:09:48
[00:14:35] make: *** [all] Error 1
[00:14:35] Makefile:28: recipe for target 'all' failed
