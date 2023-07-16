plain
[00:03:24]       Memory: 8 GB
[00:03:24]       Boot ROM Version: VMW71.00V.0.B64.1704110547
[00:03:24]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:03:24]       SMC Version (system): 2.8f0
[00:03:24]       Serial Number (system): VM9EoW7iZl+4
[00:03:24] 
[00:03:24] hw.ncpu: 4
[00:03:24] hw.byteorder: 1234
[00:03:24] hw.memsize: 8589934592
---
[01:02:29] [RUSTC-TIMING] clippy_lints test:false 82.838
[01:02:52] [RUSTC-TIMING] cargo_clippy test:false 1.193
[01:02:57] [RUSTC-TIMING] clippy_driver test:false 6.262
[01:02:57]     Finished release [optimized] target(s) in 2m 57s
[01:02:57] duplicate artfacts found when compiling a tool, this typically means that something was recompiled because a transitive dependency has different features activated than in a previous build:
[01:02:57] 
[01:02:57] the following dependencies are duplicated although they have the same features enabled:
[01:02:57] the following dependencies have different features:
[01:02:57]   serde_json 1.0.31 (registry+https://github.com/rust-lang/crates.io-index)
[01:02:57]     `clippy-driver` additionally enabled features {} at "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2-tools/x86_64-apple-darwin/release/deps/libserde_json-866c5e2383ccaafb.rlib"
[01:02:57]     `cargo` additionally enabled features {"raw_value"} at "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2-tools/x86_64-apple-darwin/release/deps/libserde_json-a09a736f2b1f9335.rlib"
[01:02:57] 
[01:02:57] to fix this you will probably want to edit the local src/tools/rustc-workspace-hack/Cargo.toml crate, as that will update the dependency graph to ensure that these crates all share the same feature set
[01:02:57] thread 'main' panicked at 'tools should not compile multiple copies of the same crate', bootstrap/tool.rs:178:13
[01:02:57] travis_fold:end:stage2-clippy-driver

[01:02:57] travis_time:end:stage2-clippy-driver:start=1538576470476599000,finish=1538576647726045000,duration=177249446000


[01:02:57] failed to run: /Users/travis/build/rust-lang/rust/build/bootstrap/debug/bootstrap build
[01:02:57] Build completed unsuccessfully in 0:58:29
[01:02:58] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:04a100af
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_fold:start:after_failure.2
travis_time:start:0c48a29f
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 0
drwx------+ 15 travis  staff  510 Jan 25  2018 ..
drwx------   2 travis  staff   68 Dec  6  2017 .
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:12be6d3e
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_time:end:12be6d3e:start=1538576653093985000,finish=1538576653113528000,duration=19543000
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1a85320a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:05759572
travis_time:start:05759572
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:09e88677
$ dmesg | grep -i kill
$ dmesg | grep -i kill
Unable to obtain kernel buffer: Operation not permitted
usage: sudo dmesg
travis_fold:end:after_failure.6

Done. Your build exited with 1.
