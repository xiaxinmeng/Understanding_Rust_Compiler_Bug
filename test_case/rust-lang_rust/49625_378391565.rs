plain
Resolving deltas: 100% (594355/594355), completed with 4534 local objects.
---
[00:00:43] configure: rust.quiet-tests     := True
---
[00:13:44] 331 |     pub fn get_alloc_kind(&self, id: AllocId) -> Option<MemoryKind<M::MemoryKinds>> {
[00:13:44]     |                                                  ---------------------------------- expected `std::option::Option<interpret::memory::MemoryKind<<M as interpret::machine::Machine<'mir, 'tcx>>::MemoryKinds>>` because of return type
[00:13:44] 332 |         self.alloc_kind.get(&id)
[00:13:44]     |         ^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `interpret::memory::MemoryKind`, found reference
[00:13:44]     |
[00:13:44]     = note: expected type `std::option::Option<interpret::memory::MemoryKind<<M as interpret::machine::Machine<'mir, 'tcx>>::MemoryKinds>>`
[00:13:44]                found type `std::option::Option<&interpret::memory::MemoryKind<<M as interpret::machine::Machine<'mir, 'tcx>>::MemoryKinds>>`
---
[00:13:46]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_mir librustc_mir/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=1b4c3b4ba4ec5832 -C extra-filename=-1b4c3b4ba4ec5832 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-2e4ce590106ede04.so --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-01fc77d929e8bdd6.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-069d1433ad059ff1.so --extern rustc_const_math=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_const_math-2cbdd37d611532dc.so --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-e01ce88b04783514.rlib --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-f27fded04dd31022.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-f27fded04dd31022.rlib --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-d823127e323fa243.so --extern rustc_back=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_back-c188df8980be19c3.so --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-f04ca02424a68c4e.rlib --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-1e725c22de2888b8.so --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-b061046bb39a474d.so --extern log_settings=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog_settings-6f97f4efb9094c9d.rlib --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-b23276248c684a7d.rlib --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-67023beb7edf290b.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-7eebd272275b441b.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-751752dec0960570/out/.libs -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-07c608814ba8d6ba/out` (exit code: 101)
[00:13:46] warning: build failed, waiting for other jobs to finish...
[00:15:16] error: build failed
[00:15:16] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1064:9
[00:15:16] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:15:16] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:15:16] expected success, got: exit code: 101
[00:15:16] travis_fold:end:stage0-rustc
[00:15:16] travis_time:end:stage0-rustc:start=1522787333683584109,finish=1522787975501252380,duration=641817668271
[00:15:16] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:15:16] Build completed unsuccessfully in 0:10:55
[00:15:16] Makefile:28: recipe for target 'all' failed
[00:15:16] make: *** [all] Error 1
---
$ cat obj/tmp/sccache.log
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:0e4d1a94:start=1522787976031665769,finish=1522787976038449042,duration=6783273
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:29c351e8
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:29c351e8:start=1522787976044380783,finish=1522787976050803416,duration=6422633
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0a0b281c
$ dmesg | grep -i kill
[   10.386606] init: failsafe main process (1093) killed by TERM signal
