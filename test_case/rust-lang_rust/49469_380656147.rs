plain
Resolving deltas: 100% (611746/611746), completed with 4887 local objects.
---
[00:00:46] configure: rust.quiet-tests     := True
---
[00:14:44] error[E0616]: field `features` of struct `rustc::session::Session` is private
[00:14:44]    --> librustc_mir/hair/pattern/check_match.rs:370:32
[00:14:44]     |
[00:14:44] 370 |                             if cx.tcx.sess.features.borrow().irrefutable_let_pattern {
[00:14:44]     |                                ^^^^^^^^^^^^^^^^^^^^
[00:14:44]
[00:14:44] error[E0609]: no field `irrefutable_let_pattern` on type `std::cell::Ref<'_, std::option::Option<syntax::feature_gate::Features>>`
[00:14:44]    --> librustc_mir/hair/pattern/check_match.rs:370:62
[00:14:44]     |
[00:14:44] 370 |                             if cx.tcx.sess.features.borrow().irrefutable_let_pattern {
[00:14:44]     |                                                              ^^^^^^^^^^^^^^^^^^^^^^^ unknown field
[00:14:44]
[00:14:44] error[E0616]: field `features` of struct `rustc::session::Session` is private
[00:14:44]    --> librustc_mir/hair/pattern/check_match.rs:410:40
[00:14:44]     |
[00:14:44] 410 |                                     if cx.tcx.sess.features.borrow().irrefutable_let_pattern {
[00:14:44]     |                                        ^^^^^^^^^^^^^^^^^^^^
[00:14:44]
[00:14:44] error[E0609]: no field `irrefutable_let_pattern` on type `std::cell::Ref<'_, std::option::Option<syntax::feature_gate::Features>>`
[00:14:44]    --> librustc_mir/hair/pattern/check_match.rs:410:70
[00:14:44]     |
[00:14:44] 410 |                                     if cx.tcx.sess.features.borrow().irrefutable_let_pattern {
---
[00:14:49]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_mir librustc_mir/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=97e644ee6c469e67 -C extra-filename=-97e644ee6c469e67 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern rustc_back=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_back-861dba9fe03aa669.so --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-23f5dcecdda8feb4.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-9f3518d56a01456f.so --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-0bde40de32995f14.so --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-ca61dfec7c40c4d4.rlib --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-f2778bd6cd1c5bdd.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-609e2421d03f9c9a.so --extern log_settings=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog_settings-70f7ed359bd54256.rlib --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-93cb1ddd29ab61a4.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-3e35efb9e5bc7a9a.rlib --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-1299638d641ea770.rlib --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-d3b6fcf798f7d22a.so --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-9866e194db82a141.rlib --extern rustc_const_math=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_const_math-895f7ddc4467bb8d.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-c04ded78717d5d67.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-c04ded78717d5d67.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-8b35e3c2ea935fab/out/.libs -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-63734d0048644b22/out` (exit code: 101)
