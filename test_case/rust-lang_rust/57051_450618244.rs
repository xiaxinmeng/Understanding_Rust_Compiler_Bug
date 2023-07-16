plain
[00:02:20]       Memory: 8 GB
[00:02:20]       Boot ROM Version: VMW71.00V.0.B64.1704110547
[00:02:20]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:02:20]       SMC Version (system): 2.8f0
[00:02:20]       Serial Number (system): VMVioWY30Bik
[00:02:20] 
[00:02:20] hw.ncpu: 4
[00:02:20] hw.byteorder: 1234
[00:02:20] hw.memsize: 8589934592
---
[01:11:06] [RUSTC-TIMING] racer test:false 27.283
[01:11:32] [RUSTC-TIMING] rustfmt_nightly test:false 53.050
[01:12:53] [RUSTC-TIMING] rls test:false 81.923
[01:12:53]     Finished release [optimized] target(s) in 6m 18s
[01:12:54] duplicate artifacts found when compiling a tool, this typically means that something was recompiled because a transitive dependency has different features activated than in a previous build:
[01:12:54] 
[01:12:54] the following dependencies are duplicated although they have the same features enabled:
[01:12:54]   crossbeam-channel 0.2.6 (registry+https://github.com/rust-lang/crates.io-index)
[01:12:54]     `rls` ("/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2-tools/x86_64-apple-darwin/release/deps/libcrossbeam_channel-16fb00439a89c4da.rlib")
[01:12:54]     `cargo` ("/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2-tools/x86_64-apple-darwin/release/deps/libcrossbeam_channel-6efa008a657362da.rlib")
[01:12:54]   ignore 0.4.4 (registry+https://github.com/rust-lang/crates.io-index)
[01:12:54]     `rls` ("/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2-tools/x86_64-apple-darwin/release/deps/libignore-9a961f29fd89f3ae.rlib")
[01:12:54]     `cargo` ("/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2-tools/x86_64-apple-darwin/release/deps/libignore-2bf832742f73f7f0.rlib")
[01:12:54] the following dependencies have different features:
[01:12:54]   parking_lot_core 0.3.0 (registry+https://github.com/rust-lang/crates.io-index)
[01:12:54]     `rls` additionally enabled features {"nightly"} at "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2-tools/x86_64-apple-darwin/release/deps/libparking_lot_core-8d1e5d3d6481b0c3.rlib"
[01:12:54]     `cargo` additionally enabled features {} at "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2-tools/x86_64-apple-darwin/release/deps/libparking_lot_core-2780374da10dcd7d.rlib"
[01:12:54]   parking_lot 0.6.4 (registry+https://github.com/rust-lang/crates.io-index)
[01:12:54]     `rls` additionally enabled features {"parking_lot_core", "nightly"} at "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2-tools/x86_64-apple-darwin/release/deps/libparking_lot-b2d458ca67492993.rlib"
[01:12:54]     `cargo` additionally enabled features {} at "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2-tools/x86_64-apple-darwin/release/deps/libparking_lot-13d4e3ef562ec70c.rlib"
[01:12:54] thread 'main' panicked at 'tools should not compile multiple copies of the same crate', src/bootstrap/tool.rs:169:13
[01:12:54] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:12:54] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:12:54] to fix this you will probably want to edit the local src/tools/rustc-workspace-hack/Cargo.toml crate, as that will update the dependency graph to ensure that these crates all share the same feature set
[01:12:54] make: *** [all] Error 1

[01:12:54] travis_time:end:stage2-rls:start=1546243149836140000,finish=1546243528112353000,duration=378276213000

[01:12:54] failed to run: /Users/travis/build/rust-lang/rust/build/bootstrap/debug/bootstrap build
---
travis_fold:start:after_failure.2
travis_time:start:20245e30
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 0
drwx------+ 15 travis  staff  510 Jan 25  2018 ..
drwx------   2 travis  staff   68 Dec  6  2017 .
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:00a63e3a
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_time:end:00a63e3a:start=1546243530662036000,finish=1546243530676692000,duration=14656000
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:04f45368
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:15cf2166
travis_time:start:15cf2166
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1eb0e992
$ dmesg | grep -i kill
$ dmesg | grep -i kill
Unable to obtain kernel buffer: Operation not permitted
usage: sudo dmesg
travis_fold:end:after_failure.6

Done. Your build exited with 1.
