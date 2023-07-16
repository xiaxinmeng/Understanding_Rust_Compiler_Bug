plain
Successfully built 2667f75f2d9d
Successfully tagged rust-ci:latest
Built container sha256:2667f75f2d9df56cf3b62701e271ff64f241e2081a72a41e451392acd9ea5ae4
Uploading finished image to https://ci-caches.rust-lang.org/docker/58bd69d3781a382eb5941028d9596b84692bd50bf9ab53a2d16b350022deca32f83037b6d64da3bfdf69e2b275e0db2ddd3212390c690549c0b3382192828b45
upload failed: - to s3://rust-lang-ci-sccache2/docker/58bd69d3781a382eb5941028d9596b84692bd50bf9ab53a2d16b350022deca32f83037b6d64da3bfdf69e2b275e0db2ddd3212390c690549c0b3382192828b45 Unable to locate credentials
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
---
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
Diff in /checkout/library/std/src/sys/unix/fs.rs at line 45:
     all(target_os = "linux", target_env = "gnu")
 use libc::c_char;
 use libc::c_char;
-#[cfg(any(all(target_os = "linux", not(target_env = "musl")), target_os = "emscripten", target_os = "android"))]
+#[cfg(any(
+    all(target_os = "linux", not(target_env = "musl")),
+    target_os = "emscripten",
+    target_os = "android"
 use libc::dirfd;
 use libc::dirfd;
 #[cfg(any(not(target_env = "musl"), target_os = "emscripten"))]
Diff in /checkout/library/std/src/sys/unix/fs.rs at line 81:
Diff in /checkout/library/std/src/sys/unix/fs.rs at line 81:
 #[cfg(target_env = "musl")]
 use libc::{
     dirent as dirent64, fstat as fstat64, ftruncate as ftruncate64, lseek as lseek64,
-    lstat as lstat64, off_t as off64_t, stat as stat64, open as open64,
+    lstat as lstat64, off_t as off64_t, open as open64, stat as stat64,
 };
 #[cfg(not(any(
     target_env = "musl",
Diff in /checkout/library/std/src/sys/unix/fd.rs at line 115:
 
 
     pub fn read_at(&self, buf: &mut [u8], offset: u64) -> io::Result<usize> {
-        #[cfg(not(any(all(target_os = "linux", not(target_env = "musl")) , target_os = "android")))]
+        #[cfg(not(any(
+            all(target_os = "linux", not(target_env = "musl")),
+            target_os = "android"
+        )))]
         use libc::pread as pread64;
         #[cfg(any(all(target_os = "linux", not(target_env = "musl")), target_os = "android"))]
         use libc::pread64;
Diff in /checkout/library/std/src/sys/unix/fd.rs at line 181:
 
 
     pub fn write_at(&self, buf: &[u8], offset: u64) -> io::Result<usize> {
-        #[cfg(not(any(all(target_os = "linux", not(target_env = "musl")) , target_os = "android")))]
+        #[cfg(not(any(
+            all(target_os = "linux", not(target_env = "musl")),
+            target_os = "android"
+        )))]
         use libc::pwrite as pwrite64;
         #[cfg(any(all(target_os = "linux", not(target_env = "musl")), target_os = "android"))]
         use libc::pwrite64;
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/sys/unix/l4re.rs" "/checkout/library/std/src/sys/unix/cmath.rs" "/checkout/library/std/src/sys/unix/io.rs" "/checkout/library/std/src/sys/unix/stdio.rs" "/checkout/library/std/src/sys/unix/path.rs" "/checkout/library/std/src/sys/unix/thread_parker/pthread.rs" "/checkout/library/std/src/sys/unix/os_str.rs" "/checkout/library/std/src/sys/unix/fs.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
