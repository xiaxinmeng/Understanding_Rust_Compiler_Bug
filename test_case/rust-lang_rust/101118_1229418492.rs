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
Diff in /checkout/library/std/src/sys/unix/fs.rs at line 1260:
             None
         }
 
-        #[cfg(any(target_os = "linux", target_os = "macos", target_os = "freebsd", target_os = "netbsd", target_os = "openbsd", target_os = "vxworks"))]
+        #[cfg(any(
+            target_os = "linux",
+            target_os = "macos",
+            target_os = "freebsd",
+            target_os = "netbsd",
+            target_os = "openbsd",
+            target_os = "vxworks"
+        ))]
         fn get_mode(fd: c_int) -> Option<(bool, bool)> {
             let mode = unsafe { libc::fcntl(fd, libc::F_GETFL) };
             if mode == -1 {
Diff in /checkout/library/std/src/sys/unix/fs.rs at line 1274:
         }
 
 
-        #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "freebsd", target_os = "netbsd", target_os = "openbsd", target_os = "vxworks")))]
+        #[cfg(not(any(
+            target_os = "linux",
+            target_os = "macos",
+            target_os = "freebsd",
+            target_os = "netbsd",
+            target_os = "openbsd",
+            target_os = "vxworks"
+        )))]
         fn get_mode(_fd: c_int) -> Option<(bool, bool)> {
             // FIXME(#24570): implement this for other Unix platforms
             None
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/sys/unix/path.rs" "/checkout/library/std/src/sys/unix/locks/mod.rs" "/checkout/library/std/src/sys/unix/fs.rs" "/checkout/library/std/src/sys/unix/locks/pthread_mutex.rs" "/checkout/library/std/src/sys/unix/env.rs" "/checkout/library/std/src/sys/unix/locks/futex_condvar.rs" "/checkout/library/std/src/sys/unix/mod.rs" "/checkout/library/std/src/sys/unix/thread_parker.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
