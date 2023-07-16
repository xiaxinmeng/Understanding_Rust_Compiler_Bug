plain
[00:00:51] configure: rust.quiet-tests     := True
---
[00:20:45] error: use of deprecated item 'io::CharsError::Other::0': Use str::from_utf8 instead:
[00:20:45]     https://doc.rust-lang.org/nightly/std/str/struct.Utf8Error.html#examples
[00:20:45]     --> libstd/io/mod.rs:2037:11
[00:20:45]      |
[00:20:45] 2037 |     Other(Error),
[00:20:45]      |           ^^^^^^
[00:20:45]      |
[00:20:45]      = note: `-D deprecated` implied by `-D warnings`
---
[00:20:47]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name std libstd/lib.rs --color always --error-format json --crate-type dylib --crate-type rlib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 --cfg feature="alloc_jemalloc" --cfg feature="backtrace" --cfg feature="jemalloc" --cfg feature="panic-unwind" --cfg feature="panic_unwind" -C metadata=59f1365839a15336 -C extra-filename=-59f1365839a15336 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liblibc-22a7664bd150c80d.rlib --extern rustc_lsan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_lsan-dc103db2cc707fb1.rlib --extern alloc_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liballoc_system-a7af859ebb7ffd34.rlib --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libpanic_unwind-0d5ca310002751eb.rlib --extern alloc_jemalloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liballoc_jemalloc-2f9a4f2e814ed058.rlib --extern rustc_tsan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_tsan-ee8ce53bd96ab14e.rlib --extern rustc_msan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_msan-cd51e4d8e5ddddce.rlib --extern std_unicode=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libstd_unicode-f0c30809ac048014.rlib --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-d4e2de02cb1be3d7.rlib --extern rustc_asan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_asan-9ca7a6a39fc271b4.rlib --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libunwind-2c193ad75e997618.rlib --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcore-64878136ec7adadb.rlib --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liballoc-a1af8df7cb7f6288.rlib --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libpanic_abort-ae9922ea98a7d2f5.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/native/libbacktrace/.libs -l static=backtrace -l dl -l rt -l pthread -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/build/compile/rustlib/x86_64-unknown-linux-gnu
---
121760 ./obj/build/bootstrap/debug/incremental/bootstrap-351vorei3hhuv/s-f04dtyyciv-n1ddi7-3ssff2uugm5rt
---
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:00758524:start=1523744036964293721,finish=1523744036971887334,duration=7593613
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:059a5924
$ dmesg | grep -i kill
[   10.306026] init: failsafe main process (1093) killed by TERM signal
