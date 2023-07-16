plain
[00:05:41]    Compiling syntax_pos v0.0.0 (file:///checkout/src/libsyntax_pos)
[00:05:44]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:07:12]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:07:30]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:08:49] error: constant item is never used: `EMPTY_SLICE`
[00:08:49]    --> librustc/ty/mod.rs:588:1
[00:08:49]     |
[00:08:49] 588 | const EMPTY_SLICE: usize = 1;
[00:08:49]     |
[00:08:49]     |
[00:08:49]     = note: `-D dead-code` implied by `-D warnings`
[00:08:51] error: aborting due to previous error
[00:08:51] 
[00:08:51] error: Could not compile `rustc`.
[00:08:51] 
[00:08:51] 
[00:08:51] Caused by:
[00:08:51]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc librustc/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=3 -C metadata=71e4694314b7f91c -C extra-filename=-71e4694314b7f91c --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-b295ea05411d660b.so --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-7575351e73c880a2.rlib --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-a7e6c70c1d8dd47c.so --extern proc_macro=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libproc_macro-e865be939b03ef41.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-a476bbed33563995.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-a476bbed33563995.rlib --extern jobserver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libjobserver-ae6ba972807bb99f.rlib --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-56d64b1c20d3e94a.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-f4ae3ca4254eee93.so --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-9a1651e7b43f567b.so --extern fmt_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libfmt_macros-c2f2c8c35b176352.so --extern tempdir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libtempdir-5940c2a57172dcdb.rlib --extern flate2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libflate2-7a064031021abb10.rlib --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-550225fbe3657ec3.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-bde077c9625bcd15.rlib --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-f223cc35df6c29ec.rlib --extern lazy_static=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86/release
48504 ./obj/build/x86_64-unknown-linux-gnu/stage0-std
47768 ./src/test
47328 ./obj/build/x86_64-unknown-linux-gnu/stage0-std/release
46352 ./src/llvm/test/MC
