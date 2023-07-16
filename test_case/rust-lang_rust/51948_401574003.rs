plain
[01:04:43]    Compiling panic_unwind v0.0.0 (file:///checkout/src/libpanic_unwind)
[01:04:43] [RUSTC-TIMING] panic_unwind test:false 0.233
[01:04:44] warning: dropping unsupported crate type `dylib` for target `aarch64-unknown-linux-musl`
[01:04:44] 
[01:04:45] error[E0425]: cannot find value `SO_RCVTIMEO` in module `c`
[01:04:45]    --> libstd/sys_common/net.rs:207:40
[01:04:45]     |
[01:04:45] 207 |         self.inner.set_timeout(dur, c::SO_RCVTIMEO)
[01:04:45] 
[01:04:45] 
[01:04:45] error[E0425]: cannot find value `SO_SNDTIMEO` in module `c`
[01:04:45]    --> libstd/sys_common/net.rs:211:40
[01:04:45]     |
[01:04:45] 211 |         self.inner.set_timeout(dur, c::SO_SNDTIMEO)
[01:04:45] 
[01:04:45] 
[01:04:45] error[E0425]: cannot find value `SO_RCVTIMEO` in module `c`
[01:04:45]    --> libstd/sys_common/net.rs:215:31
[01:04:45]     |
[01:04:45] 215 |         self.inner.timeout(c::SO_RCVTIMEO)
[01:04:45] 
[01:04:45] 
[01:04:45] error[E0425]: cannot find value `SO_SNDTIMEO` in module `c`
[01:04:45]    --> libstd/sys_common/net.rs:219:31
[01:04:45]     |
[01:04:45] 219 |         self.inner.timeout(c::SO_SNDTIMEO)
[01:04:45] 
[01:04:45] 
[01:04:45] error[E0425]: cannot find value `SO_RCVTIMEO` in module `c`
[01:04:45]    --> libstd/sys_common/net.rs:464:40
[01:04:45]     |
[01:04:45] 464 |         self.inner.set_timeout(dur, c::SO_RCVTIMEO)
[01:04:45] 
[01:04:45] 
[01:04:45] error[E0425]: cannot find value `SO_SNDTIMEO` in module `c`
[01:04:45]    --> libstd/sys_common/net.rs:468:40
[01:04:45]     |
[01:04:45] 468 |         self.inner.set_timeout(dur, c::SO_SNDTIMEO)
[01:04:45] 
[01:04:45] 
[01:04:45] error[E0425]: cannot find value `SO_RCVTIMEO` in module `c`
[01:04:45]    --> libstd/sys_common/net.rs:472:31
[01:04:45]     |
[01:04:45] 472 |         self.inner.timeout(c::SO_RCVTIMEO)
[01:04:45] 
[01:04:45] 
[01:04:45] error[E0425]: cannot find value `SO_SNDTIMEO` in module `c`
[01:04:45]    --> libstd/sys_common/net.rs:476:31
[01:04:45]     |
[01:04:45] 476 |         self.inner.timeout(c::SO_SNDTIMEO)
[01:04:45] 
[01:04:45] 
[01:04:45] error[E0425]: cannot find value `SO_RCVTIMEO` in module `libc`
[01:04:45]     |
[01:04:45]     |
[01:04:45] 424 |         self.0.set_timeout(timeout, libc::SO_RCVTIMEO)
[01:04:45]     |                                           ^^^^^^^^^^^ not found in `libc`
[01:04:45] 
[01:04:45] error[E0425]: cannot find value `SO_SNDTIMEO` in module `libc`
[01:04:45]     |
[01:04:45]     |
[01:04:45] 463 |         self.0.set_timeout(timeout, libc::SO_SNDTIMEO)
[01:04:45]     |                                           ^^^^^^^^^^^ not found in `libc`
[01:04:45] 
[01:04:45] error[E0425]: cannot find value `SO_RCVTIMEO` in module `libc`
[01:04:45]     |
[01:04:45]     |
[01:04:45] 480 |         self.0.timeout(libc::SO_RCVTIMEO)
[01:04:45]     |                              ^^^^^^^^^^^ not found in `libc`
[01:04:45] 
[01:04:45] error[E0425]: cannot find value `SO_SNDTIMEO` in module `libc`
[01:04:45]     |
[01:04:45]     |
[01:04:45] 497 |         self.0.timeout(libc::SO_SNDTIMEO)
[01:04:45]     |                              ^^^^^^^^^^^ not found in `libc`
[01:04:45] 
[01:04:45] error[E0425]: cannot find value `SO_RCVTIMEO` in module `libc`
[01:04:45]      |
[01:04:45]      |
[01:04:45] 1318 |         self.0.set_timeout(timeout, libc::SO_RCVTIMEO)
[01:04:45]      |                                           ^^^^^^^^^^^ not found in `libc`
[01:04:45] 
[01:04:45] error[E0425]: cannot find value `SO_SNDTIMEO` in module `libc`
[01:04:45]      |
[01:04:45]      |
[01:04:45] 1358 |         self.0.set_timeout(timeout, libc::SO_SNDTIMEO)
[01:04:45]      |                                           ^^^^^^^^^^^ not found in `libc`
[01:04:45] 
[01:04:45] error[E0425]: cannot find value `SO_RCVTIMEO` in module `libc`
[01:04:45]      |
[01:04:45]      |
[01:04:45] 1375 |         self.0.timeout(libc::SO_RCVTIMEO)
[01:04:45]      |                              ^^^^^^^^^^^ not found in `libc`
[01:04:45] 
[01:04:45] error[E0425]: cannot find value `SO_SNDTIMEO` in module `libc`
[01:04:45]      |
[01:04:45]      |
[01:04:45] 1393 |         self.0.timeout(libc::SO_SNDTIMEO)
[01:04:45]      |                              ^^^^^^^^^^^ not found in `libc`
[01:04:45] 
[01:04:45] error[E0425]: cannot find function `ioctl` in module `libc`
[01:04:45]    --> libstd/sys/unix/fd.rs:172:23
[01:04:45]     |
[01:04:45] 172 |             cvt(libc::ioctl(self.fd, libc::FIOCLEX))?;
[01:04:45]     |                       ^^^^^ not found in `libc`
[01:04:45] 
[01:04:45] error[E0425]: cannot find value `FIOCLEX` in module `libc`
[01:04:45]    --> libstd/sys/unix/fd.rs:172:44
[01:04:45]     |
[01:04:45] 172 |             cvt(libc::ioctl(self.fd, libc::FIOCLEX))?;
[01:04:45]     |                                            ^^^^^^^ not found in `libc`
[01:04:45] 
[01:04:45] error[E0425]: cannot find function `ioctl` in module `libc`
[01:04:45]    --> libstd/sys/unix/fd.rs:197:23
[01:04:45]     |
[01:04:45] 197 |             cvt(libc::ioctl(self.fd, libc::FIONBIO, &v))?;
[01:04:45]     |                       ^^^^^ not found in `libc`
[01:04:45] 
[01:04:45] error[E0425]: cannot find value `FIONBIO` in module `libc`
[01:04:45]    --> libstd/sys/unix/fd.rs:197:44
[01:04:45]     |
[01:04:45] 197 |             cvt(libc::ioctl(self.fd, libc::FIONBIO, &v))?;
[01:04:45]     |                                            ^^^^^^^ not found in `libc`
[01:04:45] 
[01:04:45] error[E0425]: cannot find function `ioctl` in module `libc`
[01:04:45]    --> libstd/sys/unix/net.rs:343:28
[01:04:45]     |
[01:04:45] 343 |         cvt(unsafe { libc::ioctl(*self.as_inner(), libc::FIONBIO, &mut nonblocking) }).map(|_| ())
[01:04:45]     |                            ^^^^^ not found in `libc`
[01:04:45] 
[01:04:45] error[E0425]: cannot find value `FIONBIO` in module `libc`
[01:04:45]    --> libstd/sys/unix/net.rs:343:58
[01:04:45]     |
[01:04:45] 343 |         cvt(unsafe { libc::ioctl(*self.as_inner(), libc::FIONBIO, &mut nonblocking) }).map(|_| ())
[01:04:45]     |                                                          ^^^^^^^ not found in `libc`
[01:04:45] 
[01:04:45] error[E0425]: cannot find value `EDEADLK` in module `libc`
[01:04:45]   --> libstd/sys/unix/rwlock.rs:55:30
[01:04:45]    |
[01:04:45] 55 |         } else if r == libc::EDEADLK || *self.write_locked.get() {
[01:04:45]    |                              ^^^^^^^ not found in `libc`
[01:04:45] 
[01:04:45] error[E0425]: cannot find value `EDEADLK` in module `libc`
[01:04:45]   --> libstd/sys/unix/rwlock.rs:85:23
[01:04:45]    |
[01:04:45] 85 |         if r == libc::EDEADLK || *self.write_locked.get() ||
[01:04:45]    |                       ^^^^^^^ not found in `libc`
[01:04:47] error: aborting due to 24 previous errors
[01:04:47] 
[01:04:47] For more information about this error, try `rustc --explain E0425`.
[01:04:47] [RUSTC-TIMING] std test:false 3.886
[01:04:47] [RUSTC-TIMING] std test:false 3.886
[01:04:47] error: Could not compile `std`.
[01:04:47] 
[01:04:47] Caused by:
[01:04:47]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name std libstd/lib.rs --color always --error-format json --crate-type dylib --crate-type rlib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 --cfg feature="alloc_jemalloc" --cfg feature="backtrace" --cfg feature="jemalloc" --cfg feature="panic-unwind" --cfg feature="panic_unwind" -C metadata=d8d39dc2e424c60b -C extra-filename=-d8d39dc2e424c60b --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/aarch64-unknown-linux-musl/release/deps --target aarch64-unknown-linux-musl -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/aarch64-unknown-linux-musl/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/deps --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/aarch64-unknown-linux-musl/release/deps/liballoc-f4b8d1ddf653b6ab.rlib --extern alloc_jemalloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/aarch64-unknown-linux-musl/release/deps/liballoc_jemalloc-0cc969fa3c931d08.rlib --extern alloc_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/aarch64-unknown-linux-musl/release/deps/liballoc_system-ad4e1eddf7f805d2.rlib --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/aarch64-unknown-linux-musl/release/deps/libcompiler_builtins-aa90108707aa1466.rlib --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/aarch64-unknown-linux-musl/release/deps/libcore-a7b4618666db2a6d.rlib --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/aarch64-unknown-linux-musl/release/deps/liblibc-27ec523020c77f60.rlib --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/aarch64-unknown-linux-musl/release/deps/libpanic_abort-56dcf1f890d89764.rlib --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/aarch64-unknown-linux-musl/release/deps/libpanic_unwind-7b045964361e5816.rlib --extern std_unicode=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/aarch64-unknown-linux-musl/release/deps/libstd_unicode-e25cac34856ede4e.rlib --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/aarch64-unknown-linux-musl/release/deps/libunwind-b069f975395737c0.rlib -L native=/checkout/obj/build/aarch64-unknown-linux-musl/native/libbacktrace/ -L native=/checkout/obj/build/aarch64-unknown-linux-musl/native/libbacktrace -l static=backtrace -l static=backtrace -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/aarch64-unknown-linux-musl/release/build/compiler_builtins-eca6f0a93909e857/out -L native=/checkout/obj/build/aarch64-unknown-linux-musl/native/jemalloc/lib` (exit code: 101)

[01:04:47] travis_time:end:stage2-std:start=1530403375861829004,finish=1530403426674951322,duration=50813122318


[01:04:47] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "aarch64-unknown-linux-musl" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[01:04:47] expected success, got: exit code: 101
[01:04:47] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[01:04:47] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --target asmjs-unknown-emscripten,wasm32-unknown-emscripten,x86_64-rumprun-netbsd,mips-unknown-linux-musl,mipsel-unknown-linux-musl,arm-unknown-linux-musleabi,arm-unknown-linux-musleabihf,armv5te-unknown-linux-gnueabi,armv5te-unknown-linux-musleabi,armv7-unknown-linux-musleabihf,aarch64-unknown-linux-musl,sparc64-unknown-linux-gnu,x86_64-unknown-redox,thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf
[01:04:47] Build completed unsuccessfully in 0:59:57
travis_time:end:13817db6:start=1530399538811577020,finish=1530403427016915530,duration=3888205338510


The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:1f121bf4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Jul  1 00:03:47 UTC 2018
Sun, 01 Jul 2018 00:03:47 GMT

The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
---
travis_time:end:0e6c5b20:start=1530403428731039638,finish=1530403428738706723,duration=7667085
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:07339a55
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:00369c6e
$ dmesg | grep -i kill
