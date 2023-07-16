plain
[00:21:55]    Compiling syntax_pos v0.0.0 (file:///checkout/src/libsyntax_pos)
[00:21:58]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:22:57]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:23:07]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:23:37] thread 'main' panicked at 'attempt to subtract with overflow', librustc_typeck/check/mod.rs:4922:41
[00:23:37] 
[00:23:37] error: internal compiler error: unexpected panic
[00:23:37] 
[00:23:37] 
[00:23:37] note: the compiler unexpectedly panicked. this is a bug.
[00:23:37] 
[00:23:37] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:23:37] note: rustc 1.29.0-dev running on x86_64-unknown-linux-gnu
[00:23:37] 
[00:23:37] 
[00:23:37] note: compiler flags: -Z force-unstable-if-unmarked -C prefer-dynamic -C opt-level=2 -C prefer-dynamic -C debug-assertions=y -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type dylib
[00:23:37] 
[00:23:37] note: some of the compiler flags provided by cargo are hidden
[00:23:37] 
/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libfmt_macros-ea6e9b45f399b67b.so --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-272a56f8aad7f7e9.so --extern jobserver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libjobserver-c8cb2b8c1baf0a6d.rlib --extern lazy_static=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblazy_static-f27533b38a757c6d.rlib --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-babaf616bce40eec.rlib --extern parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libparking_lot-ccb3d88d9f914fd9.rlib --extern polonius_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libpolonius_engine-c2d87a6b8c6962fe.rlib --extern proc_macro=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libproc_macro-7bc63fa4315e1858.so --extern rustc_rayon=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon-348a65d11d8a9aac.rlib --extern rustc_rayon_core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon_core-fa41d79ed310ac32.rlib --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-f063b3fff3eb1317.rlib --h; du . | sort -nr | head -n100
