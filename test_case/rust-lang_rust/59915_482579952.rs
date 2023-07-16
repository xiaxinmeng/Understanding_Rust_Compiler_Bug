plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:0102f92c
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:06:05]   Downloaded macro-utils v0.1.2
[00:06:05]   Downloaded rustc-ap-rustc_cratesio_shim v407.0.0
[00:06:05]   Downloaded dlmalloc v0.1.3
[00:06:05]   Downloaded byte-tools v0.2.0
[00:06:05]   Downloaded measureme v0.2.0
[00:06:06]   Downloaded url_serde v0.2.0
[00:06:06]   Downloaded rustc-ap-rustc_errors v407.0.0
[00:06:06]   Downloaded idna v0.1.5
[00:06:06]   Downloaded rustc-ap-graphviz v407.0.0
---
[00:46:36]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:46:37]    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:47:20] thread 'rustc' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 30, kind: Other, message: "Read-only file system" }', src/libcore/result.rs:997:5
[00:47:20] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:47:20] thread 'rustc' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 30, kind: Other, message: "Read-only file system" }', src/libcore/result.rs:997:5
[00:47:20] stack backtrace:
[00:47:20]    0:     0x7fbf496c05d3 - std::sys::unix::backtrace::tracing::imp::unwind_backtrace::h5ee153a5708476ff
[00:47:20]                                at src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:39
[00:47:20]    1:     0x7fbf496b845b - std::sys_common::backtrace::_print::h37f7cb19e8925dd7
[00:47:20]                                at src/libstd/sys_common/backtrace.rs:71
[00:47:20]    2:     0x7fbf496bc836 - std::panicking::default_hook::{{closure}}::h71d76408e7ea4aca
[00:47:20]                                at src/libstd/sys_common/backtrace.rs:59
[00:47:20]                                at src/libstd/panicking.rs:197
[00:47:20]    3:     0x7fbf496bc5c9 - std::panicking::default_hook::hba712c5213afcfac
[00:47:20]                                at src/libstd/panicking.rs:211
[00:47:20]    4:     0x7fbf46cfa6a0 - rustc::util::common::panic_hook::h51146904b6d9df8c
[00:47:20]    5:     0x7fbf496bd028 - std::panicking::rust_panic_with_hook::h7c9d916ecd443dd9
[00:47:20]                                at src/libstd/panicking.rs:478
[00:47:20]    6:     0x7fbf496bcac1 - std::panicking::continue_panic_fmt::hfc217f9901983873
[00:47:20]                                at src/libstd/panicking.rs:381
[00:47:20]    7:     0x7fbf496bc9a5 - rust_begin_unwind
[00:47:20]                                at src/libstd/panicking.rs:308
[00:47:20]    8:     0x7fbf496e585c - core::panicking::panic_fmt::h22fbbae22ebba188
[00:47:20]                                at src/libcore/panicking.rs:85
[00:47:20]    9:     0x7fbf46e22d3d - core::result::unwrap_failed::h79406567cea1657e
[00:47:20]   10:     0x7fbf46e256ec - <measureme::mmap_serialization_sink::MmapSerializationSink as core::ops::drop::Drop>::drop::h40893ebac3478089
[00:47:20]   11:     0x7fbf499e0937 - <alloc::sync::Arc<T>>::drop_slow::hb4d08bb4dfa64c02
[00:47:20]   12:     0x7fbf499e45db - core::ptr::real_drop_in_place::h73d4610e58258978
[00:47:20]   13:     0x7fbf499e0c01 - <alloc::sync::Arc<T>>::drop_slow::hd297c215ac02409b
[00:47:20]   14:     0x7fbf499fbb7b - <alloc::rc::Rc<T> as core::ops::drop::Drop>::drop::h95229cc39ffaece8
[00:47:20]   15:     0x7fbf499b287b - core::ptr::real_drop_in_place::ha484dce8506c85f2
[00:47:20]   16:     0x7fbf499ac91e - rustc_interface::interface::run_compiler_in_existing_thread_pool::had57937cf0490b4d
[00:47:20]   17:     0x7fbf49978bd5 - <std::thread::local::LocalKey<T>>::with::h99363ecc34303cb6
[00:47:20]   18:     0x7fbf499e1894 - <scoped_tls::ScopedKey<T>>::set::h9bc93ac0f84ec0ec
[00:47:20]   19:     0x7fbf49a13b91 - syntax::with_globals::hb9fcbdbd3e7f1ce2
[00:47:20]   20:     0x7fbf4997a6d4 - std::sys_common::backtrace::__rust_begin_short_backtrace::hbf97b5db70026ac3
[00:47:20]   21:     0x7fbf496cdd29 - __rust_maybe_catch_panic
[00:47:20]                                at src/libpanic_unwind/lib.rs:87
[00:47:20]   22:     0x7fbf4997c878 - core::ops::function::FnOnce::call_once::{{vtable.shim}}::hfb1cfde5deacd290
[00:47:20]   23:     0x7fbf4969f77e - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h3b71c972b364448f
[00:47:20]                                at /rustc/9dd2a9910ec8832159fe9b23a1d08545aa453df3/src/liballoc/boxed.rs:702
[00:47:20]   24:     0x7fbf496cca8f - std::sys::unix::thread::Thread::new::thread_start::hc1cf3874635d723f
[00:47:20]                                at /rustc/9dd2a9910ec8832159fe9b23a1d08545aa453df3/src/liballoc/boxed.rs:702
[00:47:20]                                at src/libstd/sys_common/thread.rs:14
[00:47:20]                                at src/libstd/sys/unix/thread.rs:80
[00:47:20]   25:     0x7fbf4943e83c - start_thread
[00:47:20]   26:     0x7fbf48da6fdc - clone
[00:47:20] thread panicked while panicking. aborting.
[00:47:21] warning:   flock(fd, LOCK_EX);
[00:47:21] warning:   ^
[00:47:21] warning: /cargo/registry/src/github.com-1ecc6299db9ec823/compiler_builtins-0.1.10/compiler-rt/lib/profile/InstrProfilingUtil.c:153:3: warning: implicit declaration of function 'flock' is invalid in C99 [-Wimplicit-function-declaration]
[00:47:21] warning:   flock(fd, LOCK_UN);
[00:47:21] warning:   flock(fd, LOCK_UN);
[00:47:21] warning:   ^
[00:47:21] warning: 2 warnings generated.
[00:48:12] error: Could not compile `core`.
[00:48:12] 
[00:48:12] Caused by:
[00:48:12]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core src/libcore/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=a767b18e8ef14871 -C extra-filename=-a767b18e8ef14871 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -C linker=clang -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps` (signal: 4, SIGILL: illegal instruction)
[00:48:13] error: build failed
[00:48:13] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace profiler" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:48:13] expected success, got: exit code: 101
[00:48:13] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
---
travis_time:end:07d6d405:start=1555076585473368337,finish=1555076585484230647,duration=10862310
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2dc810f0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:start:crashlog
obj/cores/core.4698.!checkout!obj!build!x86_64-unknown-linux-gnu!stage1!bin!rustc
[New LWP 4703]
[New LWP 4698]
warning: Could not load shared library symbols for 7 libraries, e.g. /lib64/libpthread.so.0.
Use the "info sharedlibrary" command to see the complete listing.
Do you need "set solib-search-path" or "set sysroot"?
Core was generated by `/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/rustc --crate-name core'.
Program terminated with signal SIGILL, Illegal instruction.
#0  0x00007fbf496bce4a in rust_panic_with_hook () at /rustc/9dd2a9910ec8832159fe9b23a1d08545aa453df3/src/libcore/fmt/mod.rs:316
316 /rustc/9dd2a9910ec8832159fe9b23a1d08545aa453df3/src/libcore/fmt/mod.rs: No such file or directory.
[Current thread is 1 (LWP 4703)]
#0  0x00007fbf496bce4a in rust_panic_with_hook () at /rustc/9dd2a9910ec8832159fe9b23a1d08545aa453df3/src/libcore/fmt/mod.rs:316
#1  0x00007fbf496bcac2 in continue_panic_fmt () at src/libstd/panicking.rs:381
#2  0x00007fbf496bc9a6 in rust_begin_unwind () at src/libstd/panicking.rs:308
#3  0x00007fbf496e585d in panic_fmt () at src/libcore/panicking.rs:85
#4  0x00007fbf46e22d3e in core::result::unwrap_failed::h79406567cea1657e () from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/../lib/librustc-a993507cc2ce9b19.so
#5  0x00007fbf46e256ed in _$LT$measureme..mmap_serialization_sink..MmapSerializationSink$u20$as$u20$core..ops..drop..Drop$GT$::drop::h40893ebac3478089 () from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/../lib/librustc-a993507cc2ce9b19.so
#6  0x00007fbf499e0938 in _$LT$alloc..sync..Arc$LT$T$GT$$GT$::drop_slow::hb4d08bb4dfa64c02 () from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_driver-e0faff33e0439a04.so
#7  0x00007fbf499e45dc in core::ptr::real_drop_in_place::h73d4610e58258978 () from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_driver-e0faff33e0439a04.so
#8  0x00007fbf499e0c02 in _$LT$alloc..sync..Arc$LT$T$GT$$GT$::drop_slow::hd297c215ac02409b () from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_driver-e0faff33e0439a04.so
#9  0x00007fbf499fbb7c in _$LT$alloc..rc..Rc$LT$T$GT$$u20$as$u20$core..ops..drop..Drop$GT$::drop::h95229cc39ffaece8 () from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_driver-e0faff33e0439a04.so
#10 0x00007fbf499b287c in core::ptr::real_drop_in_place::ha484dce8506c85f2 () from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_driver-e0faff33e0439a04.so
#11 0x00007fbf499ac91f in rustc_interface::interface::run_compiler_in_existing_thread_pool::had57937cf0490b4d () from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_driver-e0faff33e0439a04.so
#12 0x00007fbf49978bd6 in _$LT$std..thread..local..LocalKey$LT$T$GT$$GT$::with::h99363ecc34303cb6 () from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_driver-e0faff33e0439a04.so
#13 0x00007fbf499e1895 in _$LT$scoped_tls..ScopedKey$LT$T$GT$$GT$::set::h9bc93ac0f84ec0ec () from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_driver-e0faff33e0439a04.so
#14 0x00007fbf49a13b92 in syntax::with_globals::hb9fcbdbd3e7f1ce2 () from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_driver-e0faff33e0439a04.so
#15 0x00007fbf4997a6d5 in std::sys_common::backtrace::__rust_begin_short_backtrace::hbf97b5db70026ac3 () from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_driver-e0faff33e0439a04.so
#16 0x00007fbf496cdd2a in __rust_maybe_catch_panic () at src/libpanic_unwind/lib.rs:87
#17 0x00007fbf4997c879 in core::ops::function::FnOnce::call_once::_$u7b$$u7b$vtable.shim$u7d$$u7d$::hfb1cfde5deacd290 () from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_driver-e0faff33e0439a04.so
#18 0x00007fbf4969f77f in call_once<(),FnBox<()>> () at /rustc/9dd2a9910ec8832159fe9b23a1d08545aa453df3/src/liballoc/boxed.rs:702
#19 0x00007fbf496cca90 in call_once<(),alloc::boxed::Box<FnBox<()>>> () at /rustc/9dd2a9910ec8832159fe9b23a1d08545aa453df3/src/liballoc/boxed.rs:702
#20 start_thread () at src/libstd/sys_common/thread.rs:14
#21 thread_start () at src/libstd/sys/unix/thread.rs:80
#22 0x00007fbf4943e83d in ?? ()
#23 0x0000000000000000 in ?? ()
travis_time:end:2dc810f0:start=1555076585490218113,finish=1555076587876852922,duration=2386634809
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:040c612e
travis_time:start:040c612e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:018cb784
$ dmesg | grep -i kill
