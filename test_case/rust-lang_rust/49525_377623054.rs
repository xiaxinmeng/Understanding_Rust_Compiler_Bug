plain
Resolving deltas: 100% (613367/613367), completed with 4863 local objects.
---
[00:00:44] configure: rust.quiet-tests     := True
---
[00:29:35] error[E0658]: use of unstable library feature 'slice_sort_by_cached_key' (see issue #34447)
[00:29:35]    --> librustc_trans/base.rs:828:23
[00:29:35]     |
[00:29:35] 828 |         codegen_units.sort_by_cached_key(|cgu| usize::MAX - cgu.size_estimate());
[00:29:35]     |                       ^^^^^^^^^^^^^^^^^^
[00:29:35]     |
[00:29:35]     = help: add #![feature(slice_sort_by_cached_key)] to the crate attributes to enable
---
[00:29:38]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_trans librustc_trans/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 --cfg feature="jemalloc" --cfg feature="rustc_back" -C metadata=0a35af2040a3821a -C extra-filename=-0a35af2040a3821a --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern tempdir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libtempdir-c94e1f8066e974d5.rlib --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-2dfeda5511768b32.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-2dfeda5511768b32.rlib --extern rustc_back=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_back-4dabcc09012ea6c1.so --extern rustc_demangle=/checkout/obj/build/x86_64-unknown-linux-gnuheckout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-10ea99b4ea787d00/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-98efbe9a93844c35/out/.libs -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/rustc_llvm-9b5d9b4973b211b0/out -L native=/usr/lib/llvm-3.9/lib` (exit code: 101)
[00:29:38] travis_fold:start:stage1-rustc_trans
travis_time:start:stage1-rustc_trans
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/librustc_trans/Cargo.toml" "--features" " jemalloc" "--message-format" "json"
[00:29:38] expected success, got: exit code: 101
[00:29:38] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1064:9
[00:29:38] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:29:38] travis_fold:end:stage1-rustc_trans
[00:29:38] travis_time:end:stage1-rustc_trans:start=1522443852479970552,finish=1522443867435014761,duration=14955044209
[00:29:38] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:29:38] Build completed unsuccessfully in 0:25:19
[00:29:38] Makefile:28: recipe for target 'all' failed
[00:29:38] make: *** [all] Error 1
---
$ cat obj/tmp/sccache.log
travis_time:end:2073ad46:start=1522 1.
