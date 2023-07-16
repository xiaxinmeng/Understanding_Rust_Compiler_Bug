plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/library/std/src/sys/unix/mod.rs at line 94:
             target_os = "horizon",
         )))]
         'poll: {
+            use crate::sys::os::errno;
             #[cfg(not(target_os = "linux"))]
             use libc::open as open64;
             #[cfg(target_os = "linux")]
Diff in /checkout/library/std/src/sys/unix/mod.rs at line 100:
             use libc::open64;
-            use crate::sys::os::errno;
             let pfds: &mut [_] = &mut [
                 libc::pollfd { fd: 0, events: 0, revents: 0 },
                 libc::pollfd { fd: 1, events: 0, revents: 0 },
Diff in /checkout/library/std/src/sys/unix/mod.rs at line 142:
             target_os = "horizon",
         )))]
+            use crate::sys::os::errno;
+            use crate::sys::os::errno;
             #[cfg(not(target_os = "linux"))]
             use libc::open as open64;
             #[cfg(target_os = "linux")]
Diff in /checkout/library/std/src/sys/unix/mod.rs at line 148:
             use libc::open64;
-            use crate::sys::os::errno;
             for fd in 0..3 {
                 if libc::fcntl(fd, libc::F_GETFD) == -1 && errno() == libc::EBADF {
                     if open64("/dev/null\0".as_ptr().cast(), libc::O_RDWR, 0) == -1 {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/sys/unix/kernel_copy.rs" "/checkout/library/std/src/sys/unix/memchr.rs" "/checkout/library/std/src/sys/unix/thread_local_key.rs" "/checkout/library/std/src/sys/unix/env.rs" "/checkout/library/std/src/sys/unix/mod.rs" "/checkout/library/std/src/sys/unix/l4re.rs" "/checkout/library/std/src/sys/unix/stdio.rs" "/checkout/library/std/src/sys/unix/fd/tests.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
