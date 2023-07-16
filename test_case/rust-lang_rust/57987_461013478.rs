plain
[00:02:49]       Memory: 8 GB
[00:02:49]       Boot ROM Version: VMW71.00V.7581552.B64.1801142334
[00:02:49]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:02:49]       SMC Version (system): 2.8f0
[00:02:49]       Serial Number (system): VMwtjts8S2li
[00:02:49] 
[00:02:49] hw.ncpu: 4
[00:02:49] hw.byteorder: 1234
[00:02:49] hw.memsize: 8589934592
---
[01:05:56]    Compiling cc v1.0.28
[01:05:56]    Compiling core v0.0.0 (/Users/travis/build/rust-lang/rust/src/libcore)
[01:05:56]    Compiling libc v0.2.46
[01:05:56]    Compiling unwind v0.0.0 (/Users/travis/build/rust-lang/rust/src/libunwind)
[01:05:58] error[E0428]: the name `VaListImpl` is defined multiple times
[01:05:58]   --> src/libcore/ffi.rs:81:1
[01:05:58] 57 |     type VaListImpl;
[01:05:58] 57 |     type VaListImpl;
[01:05:58]    |     ---------------- previous definition of the type `VaListImpl` here
[01:05:58] 81 | struct VaListImpl {
[01:05:58] 81 | struct VaListImpl {
[01:05:58]    | ^^^^^^^^^^^^^^^^^ `VaListImpl` redefined here
[01:05:58]    |
[01:05:58]    = note: `VaListImpl` must be defined only once in the type namespace of this module
[01:05:58] 
[01:05:59] error[E0574]: expected struct, variant or union type, found foreign type `VaListImpl`
[01:06:03]    Compiling compiler_builtins v0.1.5
[01:06:03]    Compiling backtrace-sys v0.1.27
[01:06:03]    Compiling profiler_builtins v0.0.0 (/Users/travis/build/rust-lang/rust/src/libprofiler_builtins)
[01:06:04]    Compiling std v0.0.0 (/Users/travis/build/rust-lang/rust/src/libstd)
[01:06:04]    Compiling std v0.0.0 (/Users/travis/build/rust-lang/rust/src/libstd)
[01:06:09] error[E0308]: intrinsic has wrong type
[01:06:09]    --> src/libcore/ffi.rs:223:5
[01:06:09]     |
[01:06:09] 223 |     fn va_copy(src: &VaList) -> VaListImpl;
[01:06:09]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected extern type `ffi::::VaListImpl`, found struct `ffi::VaList`
[01:06:09]     |
[01:06:09]     = note: expected type `for<'r, 's> unsafe extern "rust-intrinsic" fn(&'r ffi::VaList<'s>) -> ffi::::VaListImpl`
[01:06:09]                found type `for<'r, 's> unsafe extern "rust-intrinsic" fn(&'r ffi::VaList<'s>) -> ffi::VaList<'s>`
[01:06:16] thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', src/libcore/option.rs:345:21
[01:06:16] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:06:19] error: aborting due to 3 previous errors
[01:06:19] 
---
[01:06:19] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:06:19] 
[01:06:19] note: rustc 1.34.0-nightly (348d3f927 2019-02-06) running on x86_64-apple-darwin
[01:06:19] 
[01:06:19] note: compiler flags: -Z save-analysis -Z osx-rpath-install-name -Z force-unstable-if-unmarked -C opt-level=2 -C prefer-dynamic -C linker=/Users/travis/build/rust-lang/rust/clang+llvm-7.0.0-x86_64-apple-darwin/bin/clang -C debuginfo=1 -C debug-assertions=n -C codegen-units=1 -C link-args=-Wl,-rpath,@loader_path/../lib --crate-type lib
[01:06:19] note: some of the compiler flags provided by cargo are hidden
[01:06:19] 
[01:06:19] [RUSTC-TIMING] core test:false 23.014
[01:06:19] error: Could not compile `core`.
[01:06:19] error: Could not compile `core`.
[01:06:19] warning: build failed, waiting for other jobs to finish...
[01:07:17] error: build failed
[01:07:17] command did not execute successfully: "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage0/bin/cargo" "build" "--target" "aarch64-apple-ios" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace profiler" "--manifest-path" "/Users/travis/build/rust-lang/rust/src/libstd/Cargo.toml" "--message-format" "json"
[01:07:17] expected success, got: exit code: 101
[01:07:17] failed to run: /Users/travis/build/rust-lang/rust/build/bootstrap/debug/bootstrap build
[01:07:17] Build completed unsuccessfully in 1:03:21
[01:07:17] make: *** [all] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:128b722f
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Feb  6 12:54:36 GMT 2019
---
travis_fold:start:after_failure.2
travis_time:start:0c1b3c0c
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 0
drwx------+ 15 travis  staff  510 Jan 25  2018 ..
drwx------   2 travis  staff   68 Dec  6  2017 .
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:01400dd4
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_time:end:01400dd4:start=1549457683992447000,finish=1549457684027873000,duration=35426000
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:282d1a42
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0ce090b4
travis_time:start:0ce090b4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:13a62f28
$ dmesg | grep -i kill
$ dmesg | grep -i kill
Unable to obtain kernel buffer: Operation not permitted
usage: sudo dmesg
travis_fold:end:after_failure.6

Done. Your build exited with 1.
