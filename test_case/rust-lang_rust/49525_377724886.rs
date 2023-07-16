plain
Resolving deltas: 100% (613419/613419), completed with 4860 local objects.
---
[00:00:57] configure: rust.quiet-tests     := True
---
[00:12:13] error[E0433]: failed to resolve. Use of undeclared type or module `std`
[00:12:13]    --> librustc_mir/monomorphize/partitioning.rs:453:48
[00:12:13]     |
[00:12:13] 453 |         codegen_units.sort_by_cached_key(|cgu| std::cmp::Reverse(cgu.size_estimate()));
[00:12:13]     |                                                ^^^ Use of undeclared type or module `std`
---
[00:12:24]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_mir librustc_mir/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=02ca437d4346e274 -C extra-filename=-02ca437d4346e274 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-505a23f516928b0b.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-e8abef14b8b443aa.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-7eebd272275b441b.rlib --extern log_settings=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog_settings-6f97f4efb9094c9d.rlib --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-7eb604ad0d7f2749.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-f27fded04dd31022.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-f27fded04dd31022.rlib --extern rustc_back=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_back-078085824a3af3a6.so --extern rustc_const_math=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_const_math-9784fba5d291c443.so --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-01f70e5b8d09b4d5.rlib --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-50848c58fc469522.so --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-b23276248c684a7d.rlib --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-2e4ce590106ede04.so --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-e01ce88b04783514.rlib --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-7437cee1018df6d3.so --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-01fc77d929e8bdd6.so -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-751752dec0960570/out/.libs -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-07c608814ba8d6ba/out` (exit code: 101)
[00:12:24] warning: build failed, waiting for other jobs to finish...
[00:13:56] error: build failed
[00:13:56] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:13:56] expected success, got: exit code: 101
[00:13:56] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1064:9
[00:13:56] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:13:56] travis_fold:end:stage0-rustc
[00:13:56] travis_time:end:stage0-rustc:start=1522531312438553593,finish=1522531862907214082,duration=550468660489
[00:13:56] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:13:56] Build completed unsuccessfully in 0:09:23
[00:13:56] make: *** [all] Error 1
[00:13:56] Makefile:28: recipe for target 'all' failed
---
$ cat obj/tmp/sccache.log
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:120c88f0:start=1522531863441729109,finish=1522531863447461641,duration=5732532
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:280d4a34
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:280d4a34:start=1522531863452695620,finish=1522531863458333331,duration=5637711
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:031ff788
$ dmesg | grep -i kill
[   10.710712] init: failsafe main process (1094) killed by TERM signal
