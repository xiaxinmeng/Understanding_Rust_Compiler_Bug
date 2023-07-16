plain
[00:25:21]    Compiling syntax_pos v0.0.0 (file:///checkout/src/libsyntax_pos)
[00:25:25]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:26:27]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:26:38]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:26:39] warning: `?` as a separator is deprecated and will be removed in an upcoming edition.
[00:26:39]    --> librustc/lint/mod.rs:111:55
[00:26:39]     |
[00:26:39] 111 |      $lint_edition: expr => $edition_level: ident $(,)?
[00:26:39] 
[00:26:39] 
[00:26:39] error: expected `*` or `+`
[00:26:39]    --> librustc/lint/mod.rs:111:55
[00:26:39]     |
[00:26:39] 111 |      $lint_edition: expr => $edition_level: ident $(,)?
[00:26:39] 
[00:26:39] 
[00:26:39] warning: `?` as a separator is deprecated and will be removed in an upcoming edition.
[00:26:39]    --> librustc/lint/mod.rs:125:28
[00:26:39]     |
[00:26:39] 125 |     ($( $lint:expr ),* $(,)?) => {{
[00:26:39] 
[00:26:39] 
[00:26:39] error: expected `*` or `+`
[00:26:39]    --> librustc/lint/mod.rs:125:28
[00:26:39]     |
[00:26:39] 125 |     ($( $lint:expr ),* $(,)?) => {{
[00:26:39] 
[00:27:13] error: aborting due to 2 previous errors
[00:27:13] 
[00:27:13] error: Could not compile `rustc`.
[00:27:13] error: Could not compile `rustc`.
[00:27:13] 
[00:27:13] Caused by:
[00:27:13]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc librustc/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=75c66186192fe9ef -C extra-filename=-75c66186192fe9ef --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-b06458981f2f8b08.so --extern flate2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libflate2-e0b46302cb6adfc1.rlib --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-da75a1391e0819dc.rlib --extern chalk_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libchalk_engine-ad6626ca6f9c7adf.rlib --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-c8132f366b701b34.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-c8132f366b701b34.rlib --extern tempdir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libtempdir-1f3205fe042ccbd0.rlib --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-c1fe07f143b7c210.so --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-5281aac7329915d9.rlib --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-688e8d8d7a3790aa.so --extern lazy_static=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblazy_static-f80cd5bfeff4f735.rlib --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-fb260ffdf5223bb7.so --extern proc_macro=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libproc_macro-5f271ace49e1c4e5.so --extern backtrace=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libbacktrace-a77c8710dcd8c364.rlib --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-a1814c1a386023f7.so --extern jobserver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libjobserver-7e250215e5863400.rlib --extern fmt_macros=/checkout/obj/build/x86_64-unknown-linux-gn86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release
13780 ./src/llvm/include/llvm
13476 ./.git/modules/src/stdsimd
13428 ./src/test/ui
travis_time:end:00ca13bb:start=1528080844921429757,finish=1528080845142746210,duration=221316453
