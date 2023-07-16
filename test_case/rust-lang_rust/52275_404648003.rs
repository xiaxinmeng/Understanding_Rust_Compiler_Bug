plain
[00:07:04]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:07:04] error: unknown start of token: `
[00:07:04]    --> librustc/lint/builtin.rs:333:29
[00:07:04]     |
[00:07:04] 333 | >>>>>>> rustc: Lint against `#[macro_use]` in 2018 idioms
[00:07:04] help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
[00:07:04]     |
[00:07:04]     |
[00:07:04] 333 | >>>>>>> rustc: Lint against '#[macro_use]` in 2018 idioms
[00:07:04] 
[00:07:04] error: aborting due to previous error
[00:07:04] 
[00:07:04] error: Could not compile `rustc`.
[00:07:04] error: Could not compile `rustc`.
[00:07:04] 
[00:07:04] Caused by:
[00:07:04]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc librustc/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=f3f1ff1495349b05 -C extra-filename=-f3f1ff1495349b05 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-ebe584ed738e3de8.so --extern backtrace=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbacktrace-fe47afe24276c7d2.rlib --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-c8771f0802893cf2.rlib --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-2a1f5ee3e102fe42.rlib --extern chalk_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libchalk_engine-b0c146f265c1c79a.rlib --extern flate2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libflate2-994b41a3641758fe.rlib --extern fmt_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libfmt_macros-6ad9c4f0e3eb0853.so --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-6020508f01da724d.so --extern jobserver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libjobserver-788cf68a3284443c.rlib --extern lazy_static=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblazy_static-8be422c91f409734.rlib --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-30c8a625cec7af60.rlib --extern parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libparking_lot-092cb0eb406f49a4.rlib --extern polonius_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libpolonius_engine-7d678b2486a5f5d9.rlib --extern proc_macro=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libproc_macro-cf4549c1ea81a6c6.so --extern rustc_rayon=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon-7fe26042bddd389a.rlib --extern rustc_rayon_core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon_core-48fc88bb94c74c6d.rlib --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-0b0f824eb96efcb5.rlib --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-dfab6c84d2674220.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-fb4c86e007981750.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-e528c05031478194.so --extern scoped_tls=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libscoped_tls-6bc8a5c455fbc99f.rlib --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-05774896829b5d87.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-05774896829b5d87.rlib --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-5f603e9854c9c328.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-786292eb849f05d6.so --extern tempfile=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libtempfile-e07eba65ba38a675.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-e7051d4409bf3a37/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-49fbbb5cce716fd4/out` (exit code: 101)
[00:07:24] error: build failed
[00:07:24] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:07:24] expected success, got: exit code: 101
[00:07:24] expected success, got: exit code: 101
[00:07:24] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1117:9
[00:07:24] travis_fold:end:stage0-rustc

[00:07:24] travis_time:end:stage0-rustc:start=1531428811756417066,finish=1531428976472512746,duration=164716095680


[00:07:24] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:07:24] Build completed unsuccessfully in 0:03:42
[00:07:24] Makefile:28: recipe for target 'all' failed
[00:07:24] make: *** [all] Error 1
251928 ./src/llvm/test
241180 ./src/llvm-emscripten
192708 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib
177612 ./obj/build/bootstrap/debug/deps
---
159808 ./.git/modules/src
149116 ./src/llvm-emscripten/test
144876 ./obj/build/bootstrap/debug/incremental
130360 ./obj/build/bootstrap/debug/incremental/bootstrap-2evv84e4ca5z
130356 ./obj/build/bootstrap/debug/incremental/bootstrap-2evv84e4ca5z/s-f2uglys491-1e8vlot-3b6w870is5kd0
97524 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
79448 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc
76364 ./.git/modules/src/tools
71508 ./src/llvm/lib
