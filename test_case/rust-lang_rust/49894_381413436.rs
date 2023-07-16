plain
[00:00:47] configure: rust.quiet-tests     := True
---
serialize-c04ded78717d5d67.rlib --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-9f3518d56a01456f.so --extern fmt_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libfmt_macros-3706e912fdb98df1.so --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-9866e194db82a141.rlib --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-0bde40de32995f14.so --extern flate2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libflate2-3c8223b0152f22a5.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-8b35e3c2ea935fab/out/.libs -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-63734d0048644b22/out` (exit code: 101)
[00:07:49] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:07:49] expected success, got: exit code: 101
[00:07:49] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1085:9
[00:07:49] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:07:49] travis_fold:end:stage0-rustc
[00:07:49] travis_time:end:stage0-rustc:start=1523804678821502182,finish=1523804855296100416,duration=176474598234
[00:07:49] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:07:49] Build completed unsuccessfully in 0:03:08
[00:07:49] Makefile:28: recipe for target 'all' failed
[00:07:49] make: *** [all] Error 1
---
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:151da37b:start=1523804855738312853,finish=1523804855744662139,duration=6349286
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:173af718
$ dmesg | grep -i kill
[   10.535064] init: failsafe main process (1094) killed by TERM signal
