plain
Resolving deltas: 100% (612186/612186), completed with 4851 local objects.
---
[00:01:40] configure: rust.quiet-tests     := True
---
[00:14:21] error[E0614]: type `bool` cannot be dereferenced
[00:14:21]    --> librustc_metadata/encoder.rs:490:36
[00:14:21]     |
[00:14:21] 490 |         let has_global_allocator = *tcx.sess.has_global_allocator.get();
---
[00:14:22]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_metadata librustc_metadata/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=b15820c823700759 -C extra-filename=-b15820c823700759 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-77cf6eeaff5de3d1.so --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-16698affcdf451b7.so --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-5946a6bf92640d0b.so --extern rustc_back=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_back-44fe32f4b60902ad.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-1ccff5f8476ad1eb.rlib --extern syntax_ext=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_ext-f1a44c1ba4676215.so --extern flate2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libflate2-94bdd789b09700ba.rlib --extern proc_macro=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libproc_macro-ef56643d34591e45.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-6b77f29b6ee45775.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-6b77f29b6ee45775.rlib --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-3fc1984c0aad3c41.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-a1057361f09ea8a5.so -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-2911c7b5ca977aa1/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-5891bf0101f32727/out/.libs` (exit code: 101)
[00:14:22] warning: build failed, waiting for other jobs to finish...
[00:16:27] error: build failed
[00:16:27] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1064:9
[00:16:27] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:16:27] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:16:27] expected success, got: exit code: 101
[00:16:27] travis_fold:end:stage0-rustc
[00:16:27] travis_time:end:stage0-rustc:start=1523973005055276700,finish=1523973666904269328,duration=661848992628
[00:16:27] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:16:27] Build completed unsuccessfully in 0:11:15
[00:16:27] Makefile:28: recipe for target 'all' failed
[00:16:27] make: *** [all] Error 1
---
$ cat obj/tmp/sccache.log
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:270908e2:start=1523973667472867745,finish=1523973667478417517,duration=5549772
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:03f2b24b
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:03f2b24b:start=1523973667483649024,finish=1523973667489635929,duration=5986905
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:13750275
$ dmesg | grep -i kill
[   10.224154] init: failsafe main process (1094) killed by TERM signal
