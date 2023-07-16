plain
[00:02:55]       Memory: 8 GB
[00:02:55]       Boot ROM Version: VMW71.00V.7581552.B64.1801142334
[00:02:55]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:02:55]       SMC Version (system): 2.8f0
[00:02:55]       Serial Number (system): VMnfZUOSYAVd
[00:02:55] 
[00:02:55] hw.ncpu: 4
[00:02:55] hw.byteorder: 1234
[00:02:55] hw.memsize: 8589934592
---
[01:21:46] [RUSTC-TIMING] miri test:false 48.415
[01:21:52] [RUSTC-TIMING] cargo_miri test:false 5.421
[01:21:53] [RUSTC-TIMING] miri test:false 6.830
[01:21:53]     Finished release [optimized] target(s) in 1m 11s
[01:21:53] duplicate artifacts found when compiling a tool, this typically means that something was recompiled because a transitive dependency has different features activated than in a previous build:
[01:21:53] the following dependencies are duplicated although they have the same features enabled:
[01:21:53] the following dependencies are duplicated although they have the same features enabled:
[01:21:53]   parking_lot_core 0.4.0 (registry+https://github.com/rust-lang/crates.io-index)
[01:21:53]     `miri` ("/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2-tools/x86_64-apple-darwin/release/deps/libparking_lot_core-307f981d52c7dc33.rlib")
[01:21:53]     `cargo` ("/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2-tools/x86_64-apple-darwin/release/deps/libparking_lot_core-c4581a9e11c9cf36.rlib")
[01:21:53]   parking_lot 0.7.1 (registry+https://github.com/rust-lang/crates.io-index)
[01:21:53]     `miri` ("/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2-tools/x86_64-apple-darwin/release/deps/libparking_lot-b6aa07150686efcf.rlib")
[01:21:53]     `cargo` ("/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2-tools/x86_64-apple-darwin/release/deps/libparking_lot-f8d432f17a0258a9.rlib")
[01:21:53]   rustc-workspace-hack 1.0.0 (path+file:///Users/travis/build/rust-lang/rust/src/tools/rustc-workspace-hack)
[01:21:53]     `miri` ("/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2-tools/x86_64-apple-darwin/release/deps/librustc_workspace_hack-dd86f1f5fcac4965.rlib")
[01:21:53]     `cargo` ("/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2-tools/x86_64-apple-darwin/release/deps/librustc_workspace_hack-3ac1deb2455741a2.rlib")
[01:21:53] the following dependencies have different features:
[01:21:53]   rand 0.6.1 (registry+https://github.com/rust-lang/crates.io-index)
[01:21:53]     `miri` additionally enabled features {} at "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2-tools/x86_64-apple-darwin/release/deps/librand-bb9ec2d01a61cf1d.rlib"
[01:21:53]     `cargo` additionally enabled features {"i128_support"} at "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2-tools/x86_64-apple-darwin/release/deps/librand-ff2bb5dd6d03d558.rlib"
[01:21:53] 
[01:21:53] to fix this you will probably want to edit the local src/tools/rustc-workspace-hack/Cargo.toml crate, as that will update the dependency graph to ensure that these crates all share the same feature set
[01:21:53] thread 'main' panicked at 'tools should not compile multiple copies of the same crate', src/bootstrap/tool.rs:170:13
[01:21:53] travis_fold:end:stage2-miri

[01:21:53] travis_time:end:stage2-miri:start=1554029079322022000,finish=1554029150999016000,duration=71676994000

---
travis_fold:start:after_failure.2
travis_time:start:02a79c14
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 0
drwx------+ 15 travis  staff  510 Jan 25  2018 ..
drwx------   2 travis  staff   68 Dec  6  2017 .
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:05fce5a8
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_time:end:05fce5a8:start=1554029156923222000,finish=1554029156957274000,duration=34052000
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0aa2be6c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:154e3e04
travis_time:start:154e3e04
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1d760024
$ dmesg | grep -i kill
$ dmesg | grep -i kill
Unable to obtain kernel buffer: Operation not permitted
usage: sudo dmesg
travis_fold:end:after_failure.6

Done. Your build exited with 1.
