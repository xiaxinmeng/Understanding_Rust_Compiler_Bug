plain
[00:49:43] [RUSTC-TIMING] alloc_system test:false 0.137
[00:49:47] [RUSTC-TIMING] alloc test:false 5.682
[00:49:47]    Compiling panic_unwind v0.0.0 (file:///checkout/src/libpanic_unwind)
[00:49:48] [RUSTC-TIMING] panic_unwind test:false 0.263
[00:49:50] error[E0560]: struct `libc::in6_addr` has no field named `s6_addr`
[00:49:50]     |
[00:49:50]     |
[00:49:50] 870 |                 s6_addr: [
[00:49:50]     |                 ^^^^^^^ `libc::in6_addr` does not have this field
[00:49:50]     |
[00:49:50]     = note: available fields are: `__in6_union`
[00:49:50] 
[00:49:50] error[E0609]: no field `s6_addr` on type `libc::in6_addr`
[00:49:50]     |
[00:49:50]     |
[00:49:50] 923 |         let arr = &self.inner.s6_addr;
[00:49:50]     |
[00:49:50]     |
[00:49:50]     = note: available fields are: `__in6_union`
[00:49:50] 
[00:49:50] error[E0609]: no field `s6_addr` on type `libc::in6_addr`
[00:49:50]      |
[00:49:50] 1228 |         self.inner.s6_addr
[00:49:50]      |                    ^^^^^^^ unknown field
[00:49:50]      |
[00:49:50]      |
[00:49:50]      = note: available fields are: `__in6_union`
[00:49:50] 
[00:49:50] error[E0609]: no field `s6_addr` on type `libc::in6_addr`
[00:49:50]      |
[00:49:50]      |
[00:49:50] 1319 |         self.inner.s6_addr == other.inner.s6_addr
[00:49:50]      |
[00:49:50]      |
[00:49:50]      = note: available fields are: `__in6_union`
[00:49:50] 
[00:49:50] error[E0609]: no field `s6_addr` on type `libc::in6_addr`
[00:49:50]      |
[00:49:50]      |
[00:49:50] 1319 |         self.inner.s6_addr == other.inner.s6_addr
[00:49:50]      |
[00:49:50]      |
[00:49:50]      = note: available fields are: `__in6_union`
[00:49:50] 
[00:49:50] error[E0609]: no field `s6_addr` on type `libc::in6_addr`
[00:49:50]      |
[00:49:50]      |
[00:49:50] 1349 |         self.inner.s6_addr.hash(s)
[00:49:50]      |
[00:49:50]      |
[00:49:50]      = note: available fields are: `__in6_union`
[00:49:50] 
[00:49:50] error[E0560]: struct `libc::in6_addr` has no field named `s6_addr`
[00:49:50]      |
[00:49:50]      |
[00:49:50] 1419 |         let inner = c::in6_addr { s6_addr: octets };
[00:49:50]      |                                   ^^^^^^^ `libc::in6_addr` does not have this field
[00:49:50]      |
[00:49:50]      = note: available fields are: `__in6_union`
[00:49:50] 
[00:49:50] error[E0609]: no field `s6_addr` on type `libc::in6_addr`
[00:49:50]     |
[00:49:50]     |
[00:49:50] 647 |             self.inner.sin6_addr.s6_addr == other.inner.sin6_addr.s6_addr &&
[00:49:50]     |
[00:49:50]     |
[00:49:50]     = note: available fields are: `__in6_union`
[00:49:50] 
[00:49:50] error[E0609]: no field `s6_addr` on type `libc::in6_addr`
[00:49:50]     |
[00:49:50]     |
[00:49:50] 647 |             self.inner.sin6_addr.s6_addr == other.inner.sin6_addr.s6_addr &&
[00:49:50]     |
[00:49:50]     |
[00:49:50]     = note: available fields are: `__in6_union`
[00:49:50] 
[00:49:50] error[E0609]: no field `s6_addr` on type `libc::in6_addr`
[00:49:50]     |
[00:49:50]     |
[00:49:50] 666 |         (self.inner.sin6_port, &self.inner.sin6_addr.s6_addr,
[00:49:50]     |
[00:49:50]     |
[00:49:50]     = note: available fields are: `__in6_union`
[00:49:50] 
[00:49:51] thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', libcore/option.rs:345:21
[00:49:52] error: aborting due to 10 previous errors
[00:49:52] 
[00:49:52] Some errors occurred: E0560, E0609.
[00:49:52] For more information about an error, try `rustc --explain E0560`.
[00:49:52] For more information about an error, try `rustc --explain E0560`.
[00:49:52] 
[00:49:52] error: internal compiler error: unexpected panic
[00:49:52] 
[00:49:52] note: the compiler unexpectedly panicked. this is a bug.
[00:49:52] 
[00:49:52] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:49:52] 
[00:49:52] note: rustc 1.30.0-nightly (5d1dccdcf 2018-08-24) running on x86_64-unknown-linux-gnu
[00:49:52] 
[00:49:52] note: compiler flags: -Z save-analysis -Z force-unstable-if-unmarked -C prefer-dynamic -C opt-level=2 -C prefer-dynamic -C linker=x86_64-fuchsia-clang -C debuginfo=1 -C debug-assertions=n -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type dylib --crate-type rlib
[00:49:52] note: some of the compiler flags provided by cargo are hidden
[00:49:52] 
[00:49:52] [RUSTC-TIMING] std test:false 3.896
[00:49:52] error: Could not compile `std`.
[00:49:52] error: Could not compile `std`.
[00:49:52] 
[00:49:52] Caused by:
[00:49:52]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name std libstd/lib.rs --color always --error-format json --crate-type dylib --crate-type rlib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 --cfg 'feature="alloc_jemalloc"' --cfg 'feature="backtrace"' --cfg 'feature="jemalloc"' --cfg 'feature="panic-unwind"' --cfg 'feature="panic_unwind"' -C metadata=664d6dd020593d31 -C extra-filename=-664d6dd020593d31 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps --target x86_64-fuchsia -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/deps --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/liballoc-ee8860eea972243c.rlib --extern alloc_jemalloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/liballoc_jemalloc-87f6a0ed946066dd.rlib --extern alloc_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/liballoc_system-3a847a39a085f9e4.rlib --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/libcompiler_builtins-e88df711517fea58.rlib --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/libcore-c47404d7be466a0f.rlib --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/liblibc-d1e9039f186c4ca1.rlib --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/libpanic_abort-7df37ac10d919f07.rlib --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/libpanic_unwind-8478637bad004b0f.rlib --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/libunwind-7bc77397dde4d7b3.rlib -L native=/checkout/obj/build/x86_64-fuchsia/native/libbacktrace/ -L native=/checkout/obj/build/x86_64-fuchsia/native/libbacktrace -l static=backtrace -l static=backtrace -l zircon -l fdio -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-1146e4d2060601c0/out` (exit code: 101)

[00:49:52] travis_time:end:stage2-std:start=1535107079856864558,finish=1535107126731120745,duration=46874256187

[00:49:52] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-fuchsia" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:49:52] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-fuchsia" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:49:52] expected success, got: exit code: 101
[00:49:52] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1119:9
[00:49:52] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --target x86_64-fuchsia,aarch64-fuchsia,sparcv9-sun-solaris,wasm32-unknown-unknown,x86_64-sun-solaris,x86_64-unknown-linux-gnux32,x86_64-unknown-cloudabi
[00:49:52] Build completed unsuccessfully in 0:42:07
travis_time:end:09752313:start=1535104134659239887,finish=1535107127007349731,duration=2992348109844

---
travis_time:end:0f7fbc00:start=1535107127871488807,finish=1535107127889983654,duration=18494847
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:08e36964
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:07b5e87e
travis_time:start:07b5e87e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:09bcdb1b
$ dmesg | grep -i kill
