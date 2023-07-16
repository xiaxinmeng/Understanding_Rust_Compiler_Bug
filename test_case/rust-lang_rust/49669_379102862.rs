plain
Receiving objects: 100% (326/326), 49.86 KiB | 4.16 MiB/s, done.
---
Resolving deltas: 100% (269/269), completed with 79 local objects.
---
[00:00:00] Attempting with retry: sh -c rm -f download-src-llvm.tar.gz &&         curl -sSL -o download-src-llvm.tar.gz https://github.com/rust-lang/llvm/archive/7243155b1c3da0a980c868a87adebf00e0b33989.tar.gz
---
[00:00:35] configure: rust.quiet-tests     := True
---
[00:03:19]       Serial Number (system): VMA55upmXQOD
---
[00:04:03] error[E0425]: cannot find value `AllocErr` in this scope
[00:04:03]    --> liballoc_system/lib.rs:157:36
[00:04:03]     |
[00:04:03] 157 |                         return Err(AllocErr)
[00:04:03]     |                                    ^^^^^^^^ not found in this scope
[00:04:03] help: possible candidates are found in other modules, you can import them into scope
[00:04:03]     |
[00:04:03] 141 |     use core::alloc::AllocErr;
[00:04:03]     |
[00:04:03] 141 |     use core::heap::AllocErr;
[00:04:03]     |
[00:04:03] 141 |     use core::heap::CollectionAllocErr::AllocErr;
[00:04:03]     |
[00:04:03]
[00:04:03] error[E0308]: mismatched types
[00:04:03]    --> liballoc_system/lib.rs:157:32
[00:04:03]     |
[00:04:03] 157 |                         return Err(AllocErr)
[00:04:03]     |                                ^^^^^^^^^^^^^ expected *-ptr, found enum `core::result::Result`
[00:04:03]     |
[00:04:03]     = note: expected type `*mut core::alloc::Void`
[00:04:03]                found type `core::result::Result<_, _>`
[00:04:03]
[00:04:03] error: aborting due to 2 previous errors
[00:04:03]
[00:04:03] error: Could not compile `alloc_system`.
[00:04:03]
[00:04:03] Caused by:
[00:04:03]   process didn't exit successfully: `/Users/travis/build/rust-lang/rust/build/bootstrap/debug/rustc --crate-name alloc_system liballoc_system/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=45bb03660d87a9c8 -C extra-filename=-45bb03660d87a9c8 --out-dir /Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage0-std/x86_64-apple-darwin/release/deps --target x86_64-apple-darwin -L dependency=/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage0-std/x86_64-apple-darwin/release/deps -L dependency=/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage0-std/release/deps --extern libc=/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage0-std/x86_64-apple-darwin/release/deps/liblibc-073fa19f0830bb93.rlib --extern core=/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage0-std/x86_64-apple-darwin/release/deps/libcore-eeb4fd07b4fe8223.rlib` (exit code: 101)
[00:04:03] warning: build failed, waiting for other jobs to finish...
[00:04:14] error: build failed
[00:04:14] command did not execute successfully: "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage0/bin/cargo" "build" "--target" "x86_64-apple-darwin" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace profiler" "--manifest-path" "/Users/travis/build/rust-lang/rust/src/libstd/Cargo.toml" "--message-format" "json"
[00:04:14] expected success, got: exit code: 101
[00:04:14] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1085:9
[00:04:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:04:14] travis_fold:end:stage0-std
[00:04:14] travis_time:end:stage0-std:start=1522956799127143000,finish=1522956851754829000,duration=52627686000
[00:04:14] failed to run: /Users/travis/build/rust-lang/rust/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:14] Build completed unsuccessfully in 0:00:54
[00:04:14] make: *** [tidy] Error 1
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 0
drwx------+ 15 travis  staff  510 Jan 25 19:20 ..
drwx------   2 travis  staff   68 Dec  6 11:00 .
travis_time:end:044f4ef0:start=1522956852523598000,finish=1522956852554172000,duration=30574000
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:0ee898ca
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_time:end:0ee898ca:start=1522956852576186000,finish=1522956852601689000,duration=25503000
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1d296c48
$ dmesg | grep -i kill
Unable to obtain kernel buffer: Operation not permitted
usage: sudo dmesg
