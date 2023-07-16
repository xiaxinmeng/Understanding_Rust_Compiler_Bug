plain
[00:06:12]    Compiling syntax_pos v0.0.0 (file:///checkout/src/libsyntax_pos)
[00:06:16]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:08:02]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:08:23]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:08:53] error[E0609]: no field `0` on type `&ty::Slice<T>`
[00:08:53]    --> librustc/ty/mod.rs:651:37
[00:08:53]     |
[00:08:53] 651 |             <[T] as Ord>::cmp(&self.0, &other.0)
[00:08:53] 
[00:08:53] 
[00:08:53] error[E0609]: no field `0` on type `&ty::Slice<T>`
[00:08:53]    --> librustc/ty/mod.rs:651:47
[00:08:53]     |
[00:08:53] 651 |             <[T] as Ord>::cmp(&self.0, &other.0)
[00:08:53] 
[00:08:53] 
[00:08:53] error[E0609]: no field `0` on type `&ty::Slice<T>`
[00:08:53]    --> librustc/ty/mod.rs:659:52
[00:08:53]     |
[00:08:53] 659 |             <[T] as PartialOrd>::partial_cmp(&self.0, &other.0)
[00:08:53] 
[00:08:53] 
[00:08:53] error[E0609]: no field `0` on type `&ty::Slice<T>`
[00:08:53]    --> librustc/ty/mod.rs:659:62
[00:08:53]     |
[00:08:53] 659 |             <[T] as PartialOrd>::partial_cmp(&self.0, &other.0)
[00:08:53] 
[00:08:59] error: aborting due to 4 previous errors
[00:08:59] 
[00:08:59] For more information about this error, try `rustc --explain E0609`.
[00:08:59] For more information about this error, try `rustc --explain E0609`.
[00:08:59] error: Could not compile `rustc`.
[00:08:59] 
[00:08:59] Caused by:
[00:08:59]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc librustc/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=3 -C metadata=5b4f8bf37669326d -C extra-filename=-5b4f8bf37669326d --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-2bee8f0c1e719b79.so --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-3024734921a8d3c2.rlib --extern lazy_static=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblazy_static-e2bf728a74e7d80c.rlib --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-6f643d03661f77af.so --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflagx-gnu/release/deps/libsyntax-cfccf57e1967c508.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-bde077c9625bcd15.rlib --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-550225fbe3657ec3.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-5c21a695e1301573.so --extern tempdir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libtempdir-5940c2a57172dcdb.rlib --extern proc_macro=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libproc_macro-851dbea9607dbada.so --extern fmt_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libfmt_macros-c2f2c8c35b176352.so -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-f085762345e9053e/out/.libs -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-c0082fee642cc0bf/out` (exit code: 101)
[00:08:59] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:08:59] travis_fold:end:stage0-rustc

[00:08
