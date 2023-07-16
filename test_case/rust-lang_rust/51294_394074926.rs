plain
[00:08:01] For more information about this error, try `rustc --explain E0308`.
[00:08:01] error: Could not compile `rustc`.
[00:08:01] 
[00:08:01] Caused by:
[00:08:01]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc librustc/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=622bce453bbffbd5 -C extra-filename=-622bce453bbffbd5 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-642a7efe79e24835.rlib --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-78ebc049d4f53f46.rlib --extern backtrace=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbacktrace-f3a2fb1d767a0bf7.rlib --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-99b4534960f92449.rlib --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-5073f1296cd24b67.rlib --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-4c5434c80172b18c.so --extern proc_macro=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libproc_macro-f53f15d4555417c6.so --extern polonius_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libpolonius_engine-fea5947f694181d1.rlib --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-9f4c7501a8934cc5.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-58741ed9de9aae4f.so --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-05d59ed98ebd8949.so --extern chalk_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libchalk_engine-dc5a2a01279da0f2.rlib --extern jobserver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libjobserver-e8a1df2f3ad631bc.rlib --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-be994de8b3050feb.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-5d0a8a65bb9fe29f.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/ bootstrap/compile.rs:1091:9
[00:08:01] travis_fold:end:stage0-rustc

[00:08:01] travis_time:end:stage0-rustc:start=1527933190270699318,finish=1527933382861014433,duration=192590315115


[00:08:01] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:08:01] Build completed unsuccessfully in 0:03:25
[00:08:01] make: *** [all] Error 1
[00:08:01] Makefile:28: recipe for target 'all' failed
315008 ./src/llvm
249872 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib
241184 ./src/llvm-emscripten
210196 ./src/llvm/test
---
146444 ./.git/modules
146440 ./.git/modules/src
137732 ./obj/build/bootstrap/debug/incremental
123180 ./obj/build/bootstrap/debug/incremental/bootstrap-1r3bppl29tbrj
123176 ./obj/build/bootstrap/debug/incremental/bootstrap-1r3bppl29tbrj/s-f1luqjpdec-wnllba-2rxy29zuufd4c
92436 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc
89804 ./src/llvm/test/CodeGen
71572 ./.git/modules/src/tools
70500 ./obj/build/x86_64-unknown-linux-gnu/native
---
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:14f5e734
$ dmesg | grep -i kill
[    3.332454] misc rfkill: hash matches
travis_time:end:14f5e734:start=1527933383433054192,finish=1527933383446434817,duration=13380625
travis_fold:end:after_failure.4

Done. Your build exited with 1.
