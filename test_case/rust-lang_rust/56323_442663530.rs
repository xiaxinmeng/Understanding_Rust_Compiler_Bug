plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:093839a1
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:06:52]    Compiling lock_api v0.1.3
[00:06:52]    Compiling polonius-engine v0.5.0
[00:06:53]    Compiling chalk-engine v0.8.0
[00:06:53]    Compiling env_logger v0.5.12
[00:06:55]    Compiling rustc_ezilaires v0.0.0 (/checkout/src/librustc_ezilaires)
[00:06:56]    Compiling parking_lot_core v0.3.0
[00:06:58]    Compiling tempfile v3.0.3
[00:06:58]    Compiling parking_lot v0.6.4
[00:07:00]    Compiling flate2 v1.0.3
---
[00:56:13]    Compiling chalk-engine v0.8.0
[00:56:13]    Compiling env_logger v0.5.12
[00:56:13]    Compiling tempfile v3.0.3
[00:56:15]    Compiling parking_lot_core v0.3.0
[00:56:15]    Compiling rustc_ezilaires v0.0.0 (/checkout/src/librustc_ezilaires)
[00:56:18]    Compiling parking_lot v0.6.4
[00:56:20]    Compiling crossbeam-epoch v0.3.1
[00:56:20]    Compiling log_settings v0.1.2
[00:56:20]    Compiling flate2 v1.0.3
---
[01:29:02]     Checking ena v0.11.0
[01:29:03]     Checking rustc_cratesio_shim v0.0.0 (/checkout/src/librustc_cratesio_shim)
[01:29:03]     Checking lock_api v0.1.3
[01:29:03]     Checking crossbeam-epoch v0.3.1
[01:29:03]     Checking rustc_ezilaires v0.0.0 (/checkout/src/librustc_ezilaires)
[01:29:04]     Checking rustc-rayon-core v0.1.1
[01:29:05]     Checking parking_lot_core v0.3.0
[01:29:05]     Checking rustc-rayon v0.1.1
[01:29:05]     Checking parking_lot v0.6.4
---
[01:29:38]     Checking tempfile v3.0.3
[01:29:38]     Checking polonius-engine v0.5.0
[01:29:38]     Checking chalk-engine v0.8.0
[01:29:39]     Checking rustc_apfloat v0.0.0 (/checkout/src/librustc_apfloat)
[01:29:39]  Documenting rustc_ezilaires v0.0.0 (/checkout/src/librustc_ezilaires)
[01:29:40]     Checking env_logger v0.5.12
[01:29:41]     Checking miniz-sys v0.1.10
[01:29:41]     Checking backtrace-sys v0.1.24
[01:29:41] warning: `[3]` cannot be resolved, ignoring it...
[01:29:41] warning: `[3]` cannot be resolved, ignoring it...
[01:29:41]     --> src/librustc_ezilaires/json.rs:1275:25
[01:29:41] 1275 | /// For example foo.bar[3].x
[01:29:41]      |                         ^ cannot be resolved, ignoring
[01:29:41]      |
[01:29:41]      = note: #[warn(intra_doc_link_resolution_failure)] on by default
[01:29:41]      = note: #[warn(intra_doc_link_resolution_failure)] on by default
[01:29:41]      = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:29:41] 
[01:29:41] warning: `[3]` cannot be resolved, ignoring it...
[01:29:41]     --> src/librustc_ezilaires/json.rs:1284:65
[01:29:41] 1284 | /// StackElements compositing the stack that represents foo.bar[3].x
[01:29:41]      |                                                                 ^ cannot be resolved, ignoring
[01:29:41]      |
[01:29:41]      = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
---
[01:36:40] Dist rustc stage2 (x86_64-unknown-linux-gnu)
[01:40:01] Dist std stage2 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:41:42] Dist analysis
[01:41:42] image_src: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/save-analysis", dst: "/checkout/obj/build/tmp/dist/rust-analysis-nightly-x86_64-unknown-linux-gnu-image/lib/rustlib/x86_64-unknown-linux-gnu/analysis"
[01:41:42] thread 'main' panicked at 'fs::read_dir(src) failed with No such file or directory (os error 2)', src/bootstrap/lib.rs:1211:18
[01:41:42] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
[01:41:42] Build completed unsuccessfully in 1:36:27
travis_time:end:113c1278:start=1543446544835532491,finish=1543452647841834946,duration=6103006302455
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
---
travis_time:end:1159bbe1:start=1543452650628138481,finish=1543452650641649722,duration=13511241
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:05495880
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2d424140
travis_time:start:2d424140
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0551d49c
$ dmesg | grep -i kill
