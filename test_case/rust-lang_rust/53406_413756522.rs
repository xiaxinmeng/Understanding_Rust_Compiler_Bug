plain
[00:03:36]       Memory: 8 GB
[00:03:36]       Boot ROM Version: VMW71.00V.0.B64.1704110547
[00:03:36]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:03:36]       SMC Version (system): 2.8f0
[00:03:36]       Serial Number (system): VMGBAFC1Oux9
[00:03:36] 
[00:03:36] hw.ncpu: 4
[00:03:36] hw.byteorder: 1234
[00:03:36] hw.memsize: 8589934592
---

[01:10:03] [TIMING] ToolBuild { compiler: Compiler { stage: 2, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin", tool: "miri", path: "src/tools/miri", mode: ToolRustc, is_optional_tool: true, source_type: Submodule, extra_features: [] } -- 10.411
[01:10:03] Build completed successfully in 1:05:24
[01:10:04]     Finished dev [unoptimized] target(s) in 0.48s
[01:10:06] thread 'main' panicked at 'fs::read_dir(builder.src.join("src/doc/book/redirects")) failed with No such file or directory (os error 2)', bootstrap/doc.rs:294:21
[01:10:06] failed to run: /Users/travis/build/rust-lang/rust/build/bootstrap/debug/bootstrap doc
[01:10:06] Build completed unsuccessfully in 0:00:02
[01:10:06] Build completed unsuccessfully in 0:00:02
[01:10:06] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0c21b1d0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_fold:start:after_failure.2
travis_time:start:008fa861
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 0
drwx------+ 15 travis  staff  510 Jan 25  2018 ..
drwx------   2 travis  staff   68 Dec  6  2017 .
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:01e34da0
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_time:end:01e34da0:start=1534481166385602000,finish=1534481166409093000,duration=23491000
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:076eed20
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0abbdf3e
travis_time:start:0abbdf3e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:028ccd01
$ dmesg | grep -i kill
$ dmesg | grep -i kill
Unable to obtain kernel buffer: Operation not permitted
usage: sudo dmesg
travis_fold:end:after_failure.6

Done. Your build exited with 1.
