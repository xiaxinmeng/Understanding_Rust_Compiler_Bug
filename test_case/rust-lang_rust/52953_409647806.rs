plain
[00:07:26]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:07:35]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:13:46]    Compiling rustc_typeck v0.0.0 (file:///checkout/src/librustc_typeck)
[00:13:46]    Compiling rustc_mir v0.0.0 (file:///checkout/src/librustc_mir)
[00:13:52] error[E0599]: no method named `codemap` found for type `&rustc::session::Session` in the current scope
[00:13:52]    --> librustc_mir/borrow_check/mutability_errors.rs:426:25
[00:13:52]     |
[00:13:52] 426 |     (sp, match tcx.sess.codemap().span_to_snippet(sp) {
[00:13:52] 
[00:13:52] 
[00:13:52] error[E0599]: no method named `codemap` found for type `&rustc::session::Session` in the current scope
[00:13:52]    --> librustc_mir/borrow_check/mutability_errors.rs:489:31
[00:13:52]     |
[00:13:52] 489 |     if let Ok(src) = tcx.sess.codemap().span_to_snippet(highlight_span) {
[00:13:52] 
[00:14:01] error: aborting due to 2 previous errors
[00:14:01] 
[00:14:01] For more information about this error, try `rustc --explain E0599`.
[00:14:01] For more information about this error, try `rustc --explain E0599`.
[00:14:01] error: Could not compile `rustc_mir`.
[00:14:01] 
[00:14:01] Caused by:
[00:14:01]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_mir librustc_mir/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=5179475e4c9280ac -C extra-filename=-5179475e4c9280ac --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-148fd1c1391e24b6.so --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-1ae79b79f441d17a.rlib --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-9b67c77caff626b1.rlib --extern either=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libeither-21b97ae521a4c1d8.rlib --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-0959eff5b195ace2.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-eaf85f66e467efb7.rlib --extern log_settings=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog_settings-2a6ed09af6dbfac4.rlib --extern polonius_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libpolonius_engine-1027159c1712a408.rlib --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-427106130ab2ff56.so --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-8ccf59c5e9e172ee.rlib --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-3d8079c0b5c752f1.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-890d7a0f1ffe7f0b.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-1508bcc57003426f.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-a86c1222f09eecc5.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-a86c1222f09eecc5.rlib --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-1f3f13939c5ccaf0.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-881cb3e68ed15c2c.so -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-523855930bbd979d/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-298541be0c6807b6/out` (exit code: 101)
travis_time:end:1db0f6cc:start=1533142062019238770,finish=1533143009287909404,duration=947268670634

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:02f5e870
