plain
[00:03:27]       Memory: 8 GB
[00:03:27]       Boot ROM Version: VMW71.00V.0.B64.1704110547
[00:03:27]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:03:27]       SMC Version (system): 2.8f0
[00:03:27]       Serial Number (system): VMHVZnaZ/gwW
[00:03:27] 
[00:03:27] hw.ncpu: 4
[00:03:27] hw.byteorder: 1234
[00:03:27] hw.memsize: 8589934592
---
[01:06:37] [RUSTC-TIMING] racer test:false 25.597
[01:07:00] [RUSTC-TIMING] rustfmt_nightly test:false 48.970
[01:08:05] [RUSTC-TIMING] rls test:false 64.613
[01:08:05]     Finished release [optimized] target(s) in 5m 29s
[01:08:05] duplicate artfacts found when compiling a tool, this typically means that something was recompiled because a transitive dependency has different features activated than in a previous build:
[01:08:05] 
[01:08:05] the following dependencies are duplicated although they have the same features enabled:
[01:08:05]   tempfile 3.0.3 (registry+https://github.com/rust-lang/crates.io-index)
[01:08:05]     `rls` ("/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2-tools/x86_64-apple-darwin/release/deps/libtempfile-f5f1cfa5ccd89317.rlib")
[01:08:05]     `cargo` ("/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2-tools/x86_64-apple-darwin/release/deps/libtempfile-0ee2c9a6386ea9fc.rlib")
[01:08:05] the following dependencies have different features:
[01:08:05]   rand 0.5.5 (registry+https://github.com/rust-lang/crates.io-index)
[01:08:05]     `rls` additionally enabled features {} at "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2-tools/x86_64-apple-darwin/release/deps/librand-3bfb134e2a7f77c2.rlib"
[01:08:05]     `cargo` additionally enabled features {"i128_support"} at "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2-tools/x86_64-apple-darwin/release/deps/librand-942306da3e39e7b2.rlib"
[01:08:05] 
[01:08:05] to fix this you will probably want to edit the local src/tools/rustc-workspace-hack/Cargo.toml crate, as that will update the dependency graph to ensure that these crates all share the same feature set
[01:08:05] thread 'main' panicked at 'tools should not compile multiple copies of the same crate', bootstrap/tool.rs:178:13
[01:08:05] travis_fold:end:stage2-rls

[01:08:05] travis_time:end:stage2-rls:start=1538589889136717000,finish=1538590219047987000,duration=329911270000


[01:08:05] failed to run: /Users/travis/build/rust-lang/rust/build/bootstrap/debug/bootstrap build
[01:08:05] Build completed unsuccessfully in 1:03:31
[01:08:05] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:285b9e0e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_fold:start:after_failure.2
travis_time:start:1df7402f
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 0
drwx------+ 15 travis  staff  510 Jan 25  2018 ..
drwx------   2 travis  staff   68 Dec  6  2017 .
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:2784ea18
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_time:end:2784ea18:start=1538590222637828000,finish=1538590222653227000,duration=15399000
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:01722258
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:11d1f024
travis_time:start:11d1f024
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:195d021e
$ dmesg | grep -i kill
$ dmesg | grep -i kill
Unable to obtain kernel buffer: Operation not permitted
usage: sudo dmesg
travis_fold:end:after_failure.6

Done. Your build exited with 1.
