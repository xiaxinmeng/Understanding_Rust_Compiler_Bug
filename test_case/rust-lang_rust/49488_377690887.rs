plain
[01:12:24] error[E0432]: unresolved import `sys::stdio::stderr_prints_nothing`
[01:12:24]   --> libstd/sys_common/util.rs:13:26
[01:12:24]    |
[01:12:24] 13 | use sys::stdio::{Stderr, stderr_prints_nothing};
[01:12:24]    |                          ^^^^^^^^^^^^^^^^^^^^^ no `stderr_prints_nothing` in `sys::cloudabi::stdio`
[01:12:24]
[01:12:24] error[E0432]: unresolved import `sys::stdio::stderr_prints_nothing`
[01:12:24]   --> libstd/panicking.rs:32:26
[01:12:24]    |
[01:12:24] 32 | use sys::stdio::{Stderr, stderr_prints_nothing};
[01:12:24]    |                          ^^^^^^^^^^^^^^^^^^^^^ no `stderr_prints_nothing` in `sys::cloudabi::stdio`
---
[01:12:30]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name std libstd/lib.rs --color always --error-format json --crate-type dylib --crate-type rlib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 --cfg feature="alloc_jemalloc" --cfg feature="backtrace" --cfg feature="jemalloc" --cfg feature="panic-unwind" --cfg feature="panic_unwind" -C metadata=1c2dc0c5e8fbedb0 -C extra-filename=-1c2dc0c5e8fbedb0 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-cloudabi/release/deps --target x86_64-unknown-cloudabi -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-cloudabi/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/deps --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-cloudabi/release/deps/libpanic_abort-bb4be34ec6f53c39.rlib --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-cloudabi/release/deps/liblibc-dcd42166dcc16a4f.rlib --extern std_unicode=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-cloudabi/release/deps/libstd_unicode-cde857d49c4f18a7.rlib --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-cloudabi/release/deps/libpanic_unwind-6a83003f49c04647.rlib --extern alloc_jemalloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-cloudabi/release/deps/liballoc_jemalloc-4bc326ec04f87560.rlib --extern alloc_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-cloudabi/release/deps/liballoc_system-775483055fc5de65.rlib --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-cloudabi/release/deps/libunwind-b71ab6ab8da4cde8.rlib --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-cloudabi/release/deps/libcompiler_builtins-2e5b17011c0292ac.rlib --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-cloudabi/release/deps/liballoc-d3b7072e474960a6.rlib --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-cloudabi/release/deps/libcore-bb0a4d66dfb91f38.rlib -l unwind -l c -l compiler_rt -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-cloudabi/release/build/compiler_builtins-2bbde421be80adc0/out` (exit code: 101)
[01:12:30] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1064:9
[01:12:30] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:12:30] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-cloudabi" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
---
$ cat obj/tmp/sccache.log
WARN:sccache::cache::s3: Got AWS error: Error(BadHTTPStatus(NotFound), State { next_error: None })
WARN:sccache::cache::s3: Got AWS error: Error(BadHTTPStatus(NotFound), State { next_error: None })
WARN:sccache::cache::s3: Got AWS error: Error(BadHTTPStatus(NotFound), State { next_error: None })
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:003e6d92:start=1522500557044270928,finish=1522500557051619305,duration=7348377
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2903804b
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:2903804b:start=1522500557056963373,finish=1522500557063343256,duration=6379883
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1c20411c
$ dmesg | grep -i kill
[   10.769531] init: failsafe main process (1092) killed by TERM signal
