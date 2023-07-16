plain
[01:08:33]    Compiling panic_unwind v0.0.0 (file:///checkout/src/libpanic_unwind)
[01:08:33] [RUSTC-TIMING] panic_unwind test:false 0.250
[01:08:34] warning: dropping unsupported crate type `dylib` for target `aarch64-unknown-linux-musl`
[01:08:34] 
[01:08:35] error[E0425]: cannot find value `SO_RCVTIMEO` in module `c`
[01:08:35]    --> libstd/sys_common/net.rs:207:40
[01:08:35]     |
[01:08:35] 207 |         self.inner.set_timeout(dur, c::SO_RCVTIMEO)
[01:08:35] 
[01:08:35] 
[01:08:35] error[E0425]: cannot find value `SO_SNDTIMEO` in module `c`
[01:08:35]    --> libstd/sys_common/net.rs:211:40
[01:08:35]     |
[01:08:35] 211 |         self.inner.set_timeout(dur, c::SO_SNDTIMEO)
[01:08:35] 
[01:08:35] 
[01:08:35] error[E0425]: cannot find value `SO_RCVTIMEO` in module `c`
[01:08:35]    --> libstd/sys_common/net.rs:215:31
[01:08:35]     |
[01:08:35] 215 |         self.inner.timeout(c::SO_RCVTIMEO)
[01:08:35] 
[01:08:35] 
[01:08:35] error[E0425]: cannot find value `SO_SNDTIMEO` in module `c`
[01:08:35]    --> libstd/sys_common/net.rs:219:31
[01:08:35]     |
[01:08:35] 219 |         self.inner.timeout(c::SO_SNDTIMEO)
[01:08:35] 
[01:08:35] 
[01:08:35] error[E0425]: cannot find value `SO_RCVTIMEO` in module `c`
[01:08:35]    --> libstd/sys_common/net.rs:464:40
[01:08:35]     |
[01:08:35] 464 |         self.inner.set_timeout(dur, c::SO_RCVTIMEO)
[01:08:35] 
[01:08:35] 
[01:08:35] error[E0425]: cannot find value `SO_SNDTIMEO` in module `c`
[01:08:35]    --> libstd/sys_common/net.rs:468:40
[01:08:35]     |
[01:08:35] 468 |         self.inner.set_timeout(dur, c::SO_SNDTIMEO)
[01:08:35] 
[01:08:35] 
[01:08:35] error[E0425]: cannot find value `SO_RCVTIMEO` in module `c`
[01:08:35]    --> libstd/sys_common/net.rs:472:31
[01:08:35]     |
[01:08:35] 472 |         self.inner.timeout(c::SO_RCVTIMEO)
[01:08:35] 
[01:08:35] 
[01:08:35] error[E0425]: cannot find value `SO_SNDTIMEO` in module `c`
[01:08:35]    --> libstd/sys_common/net.rs:476:31
[01:08:35]     |
[01:08:35] 476 |         self.inner.timeout(c::SO_SNDTIMEO)
[01:08:35] 
[01:08:35] 
[01:08:35] error[E0425]: cannot find value `SO_RCVTIMEO` in module `libc`
[01:08:35]     |
[01:08:35]     |
[01:08:35] 424 |         self.0.set_timeout(timeout, libc::SO_RCVTIMEO)
[01:08:35]     |                                           ^^^^^^^^^^^ not found in `libc`
[01:08:35] 
[01:08:35] error[E0425]: cannot find value `SO_SNDTIMEO` in module `libc`
[01:08:35]     |
[01:08:35]     |
[01:08:35] 463 |         self.0.set_timeout(timeout, libc::SO_SNDTIMEO)
[01:08:35]     |                                           ^^^^^^^^^^^ not found in `libc`
[01:08:35] 
[01:08:35] error[E0425]: cannot find value `SO_RCVTIMEO` in module `libc`
[01:08:35]     |
[01:08:35]     |
[01:08:35] 480 |         self.0.timeout(libc::SO_RCVTIMEO)
[01:08:35]     |                              ^^^^^^^^^^^ not found in `libc`
[01:08:35] 
[01:08:35] error[E0425]: cannot find value `SO_SNDTIMEO` in module `libc`
[01:08:35]     |
[01:08:35]     |
[01:08:35] 497 |         self.0.timeout(libc::SO_SNDTIMEO)
[01:08:35]     |                              ^^^^^^^^^^^ not found in `libc`
[01:08:35] 
[01:08:35] error[E0425]: cannot find value `SO_RCVTIMEO` in module `libc`
[01:08:35]      |
[01:08:35]      |
[01:08:35] 1318 |         self.0.set_timeout(timeout, libc::SO_RCVTIMEO)
[01:08:35]      |                                           ^^^^^^^^^^^ not found in `libc`
[01:08:35] 
[01:08:35] error[E0425]: cannot find value `SO_SNDTIMEO` in module `libc`
[01:08:35]      |
[01:08:35]      |
[01:08:35] 1358 |         self.0.set_timeout(timeout, libc::SO_SNDTIMEO)
[01:08:35]      |                                           ^^^^^^^^^^^ not found in `libc`
[01:08:35] 
[01:08:35] error[E0425]: cannot find value `SO_RCVTIMEO` in module `libc`
[01:08:35]      |
[01:08:35]      |
[01:08:35] 1375 |         self.0.timeout(libc::SO_RCVTIMEO)
[01:08:35]      |                              ^^^^^^^^^^^ not found in `libc`
[01:08:35] 
[01:08:35] error[E0425]: cannot find value `SO_SNDTIMEO` in module `libc`
[01:08:35]      |
[01:08:35]      |
[01:08:35] 1393 |         self.0.timeout(libc::SO_SNDTIMEO)
[01:08:35]      |                              ^^^^^^^^^^^ not found in `libc`
[01:08:35] 
[01:08:35] error[E0425]: cannot find function `ioctl` in module `libc`
[01:08:35]    --> libstd/sys/unix/fd.rs:172:23
[01:08:35]     |
[01:08:35] 172 |             cvt(libc::ioctl(self.fd, libc::FIOCLEX))?;
[01:08:35]     |                       ^^^^^ not found in `libc`
[01:08:35] 
[01:08:35] error[E0425]: cannot find value `FIOCLEX` in module `libc`
[01:08:35]    --> libstd/sys/unix/fd.rs:172:44
[01:08:35]     |
[01:08:35] 172 |             cvt(libc::ioctl(self.fd, libc::FIOCLEX))?;
[01:08:35]     |                                            ^^^^^^^ not found in `libc`
[01:08:35] 
[01:08:35] error[E0425]: cannot find function `ioctl` in module `libc`
[01:08:35]    --> libstd/sys/unix/fd.rs:197:23
[01:08:35]     |
[01:08:35] 197 |             cvt(libc::ioctl(self.fd, libc::FIONBIO, &v))?;
[01:08:35]     |                       ^^^^^ not found in `libc`
[01:08:35] 
[01:08:35] error[E0425]: cannot find value `FIONBIO` in module `libc`
[01:08:35]    --> libstd/sys/unix/fd.rs:197:44
[01:08:35]     |
[01:08:35] 197 |             cvt(libc::ioctl(self.fd, libc::FIONBIO, &v))?;
[01:08:35]     |                                            ^^^^^^^ not found in `libc`
[01:08:35] 
[01:08:35] error[E0425]: cannot find function `ioctl` in module `libc`
[01:08:35]    --> libstd/sys/unix/net.rs:343:28
[01:08:35]     |
[01:08:35] 343 |         cvt(unsafe { libc::ioctl(*self.as_inner(), libc::FIONBIO, &mut nonblocking) }).map(|_| ())
[01:08:35]     |                            ^^^^^ not found in `libc`
[01:08:35] 
[01:08:35] error[E0425]: cannot find value `FIONBIO` in module `libc`
[01:08:35]    --> libstd/sys/unix/net.rs:343:58
[01:08:35]     |
[01:08:35] 343 |         cvt(unsafe { libc::ioctl(*self.as_inner(), libc::FIONBIO, &mut nonblocking) }).map(|_| ())
[01:08:35]     |                                                          ^^^^^^^ not found in `libc`
[01:08:35] 
[01:08:35] error[E0425]: cannot find value `EDEADLK` in module `libc`
[01:08:35]   --> libstd/sys/unix/rwlock.rs:55:30
[01:08:35]    |
[01:08:35] 55 |         } else if r == libc::EDEADLK || *self.write_locked.get() {
[01:08:35]    |                              ^^^^^^^ not found in `libc`
[01:08:35] 
[01:08:35] error[E0425]: cannot find value `EDEADLK` in module `libc`
[01:08:35]   --> libstd/sys/unix/rwlock.rs:85:23
[01:08:35]    |
[01:08:35] 85 |         if r == libc::EDEADLK || *self.write_locked.get() ||
[01:08:35]    |                       ^^^^^^^ not found in `libc`
[01:08:38] error: aborting due to 24 previous errors
[01:08:38] 
[01:08:38] For more information about this error, try `rustc --explain E0425`.
[01:08:38] [RUSTC-TIMING] std test:false 4.175
[01:08:38] [RUSTC-TIMING] std test:false 4.175
[01:08:38] error: Could not compile `std`.
[01:08:38] 
[01:08:38] Caused by:
[01:08:38]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name std libstd/lib.rs --color always --error-format json --crate-type dylib --crate-type rlib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 --cfg feature="alloc_jemalloc" --cfg feature="backtrace" --cfg feature="jemalloc" --cfg feature="panic-unwind" --cfg feature="panic_unwind" -C metadata=8ecd72a8635f5f48 -C extra-filename=-8ecd72a8635f5f48 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/aarch64-unknown-linux-musl/release/deps --target aarch64-unknown-linux-musl -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/aarch64-unknown-linux-musl/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/deps --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/aarch64-unknown-linux-musl/release/deps/liballoc-2538efb8522c896b.rlib --extern alloc_jemalloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/aarch64-unknown-linux-musl/release/deps/liballoc_jemalloc-dd30d28b8f1d14a9.rlib --extern alloc_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/aarch64-unknown-linux-musl/release/deps/liballoc_system-3474249dfd3c4c58.rlib --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/aarch64-unknown-linux-musl/release/deps/libcompiler_builtins-734f92474a6cc952.rlib --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/aarch64-unknown-linux-musl/release/deps/libcore-1dcda79bae251168.rlib --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/aarch64-unknown-linux-musl/release/deps/liblibc-ce7dab0934a9b53a.rlib --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/aarch64-unknown-linux-musl/release/deps/libpanic_abort-40a7ce45480295bf.rlib --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/aarch64-unknown-linux-musl/release/deps/libpanic_unwind-709dada791787b4c.rlib --extern std_unicode=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/aarch64-unknown-linux-musl/release/deps/libstd_unicode-9fe0942f24dee1ce.rlib --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/aarch64-unknown-linux-musl/release/deps/libunwind-cbdda41b07e5aa27.rlib -L native=/checkout/obj/build/aarch64-unknown-linux-musl/native/libbacktrace/ -L native=/checkout/obj/build/aarch64-unknown-linux-musl/native/libbacktrace -l static=backtrace -l static=backtrace -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/aarch64-unknown-linux-musl/release/build/compiler_builtins-aa3c08f519bcf6c6/out -L native=/checkout/obj/build/aarch64-unknown-linux-musl/native/jemalloc/lib` (exit code: 101)
[01:08:38] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "aarch64-unknown-linux-musl" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[01:08:38] expected success, got: exit code: 101
[01:08:38] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[01:08:38] travis_fold:end:stage2-std

[01:08:38] travis_time:end:stage2-std:start=1530415115222550213,finish=1530415169223333303,duration=54000783090

---
travis_time:end:1202ff70:start=1530415171368422462,finish=1530415171384517591,duration=16095129
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00481e3a
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:04381820
$ dmesg | grep -i kill
