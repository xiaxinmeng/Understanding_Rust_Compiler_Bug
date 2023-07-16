plain
[00:05:27]    Compiling backtrace v0.3.6
[00:05:35]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:07:01]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:07:20]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:07:23] error[E0433]: failed to resolve. Use of undeclared type or module `iter`
[00:07:23]    --> librustc/ty/layout.rs:222:61
[00:07:23]     |
[00:07:23] 222 |     pub fn index_by_increasing_offset<'a>(&'a self) -> impl iter::Iterator<Item=usize>+'a {
[00:07:23]     |                                                             ^^^^ Use of undeclared type or module `iter`
[00:07:24] error[E0412]: cannot find type `RangeInclusive` in this scope
[00:07:24]    --> librustc/ty/layout.rs:320:25
[00:07:24]     |
[00:07:24]     |
[00:07:24] 320 |         niche_variants: RangeInclusive<usize>,
[00:07:24]     |                         ^^^^^^^^^^^^^^ not found in this scope
[00:07:24] help: possible candidate is found in another module, you can import it into scope
[00:07:24] 11  | use std::ops::RangeInclusive;
[00:07:24]     |
[00:07:24] 
[00:07:24] 
[00:07:24] error[E0425]: cannot find value `discr` in this scope
[00:07:24]     --> librustc/ty/layout.rs:1800:65
[00:07:24]      |
[00:07:24] 1800 |                         let layout = LayoutDetails::scalar(tcx, discr.clone());
[00:07:24] 
[00:07:24] 
[00:07:24] error[E0425]: cannot find value `discr` in this scope
[00:07:24]     --> librustc/ty/layout.rs:1803:33
[00:07:24] 1803 |                             ty: discr.value.to_ty(tcx)
[00:07:24]      |                                 ^^^^^ not found in this scope
[00:07:24] 
[00:07:28] error: aborting due to 4 previous errors
[00:07:28] error: aborting due to 4 previous errors
[00:07:28] 
[00:07:28] Some errors occurred: E0412, E0425, E0433.
[00:07:28] For more information about an error, try `rustc --explain E0412`.
[00:07:28] error: Could not compile `rustc`.
[00:07:28] 
[00:07:28] Caused by:
[00:07:28]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc librustc/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=49eb0ce85cccebe8 -C extra-filename=-49eb0ce85cccebe8 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-40a8b14ac2445c60.so --extern flate2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libflate2-1a0253e187e6500f.rlib --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-3780f5cb4ab85083.rlib --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-9d60fa98a5a10c1e.rlib --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-6870b7044c859a70.rlib --extern jobserver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libjobserver-ab2cf718bbbe7ba7.rlib --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-24d121bd290f1f21.so --extern proc_macro=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libproc_macro-c159e07ec5770fc7.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-af41b93aec4c1e93.rlib --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-7cdf848b4ea36a34.so --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-19bf33dd406ed70a.so --extern backtrace=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbacktrace-9432588caf307c31.rlib --extern fmt_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libfmt_macros-bca5fb1f27226f8d.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-5a636a569660b030.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-cf2c0cb22fec89b8.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-cf2c0cb22fec89b8.rlib --extern tempdir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libtempdir-0c7cf905e0cd6081.rlib --extern lazy_static=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblazy_static-3087f9759716c87e.rlib --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-7c5db1f16c17a1cb.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-6ed50b2d1f28e8a9.so -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-838d7b7b46636e17/out/.libs -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-c12600360c64db1e/out` (exit code: 101)
ustlib/x86_64-unknown-linux-gnu
314852 ./src/llvm
256636 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib
241192 ./src/llvm-emscripten
---
149616 ./.git/modules/src
149128 ./src/llvm-emscripten/test
144684 ./obj/build/bootstrap/debug/incremental
123716 ./obj/build/bootstrap/debug/incremental/bootstrap-1wl4zjaz72e5d
123712 ./obj/build/bootstrap/debug/incremental/bootstrap-1wl4zjaz72e5d/s-f0ngxlpdqp-5w7v7g-1ujokh614sjd2
89692 ./src/llvm/test/CodeGen
82788 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc
71000 ./.git/modules/src/tools
70944 ./obj/build/x86_64-unknown-linux-gnu/native
