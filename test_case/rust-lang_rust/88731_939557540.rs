plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/library/std/src/sys/unix/fs.rs at line 60:
     lstat as lstat64, off_t as off64_t, open as open64, stat as stat64,
 };
 #[cfg(any(target_os = "linux", target_os = "emscripten", target_os = "l4re"))]
-use libc::{
-    dirent64, fstat64, ftruncate64, lseek64, lstat64, off64_t, readdir64_r, stat64,
-};
+use libc::{dirent64, fstat64, ftruncate64, lseek64, lstat64, off64_t, readdir64_r, stat64};
 
 #[cfg(any(target_os = "linux", target_os = "android"))]
 mod dir_fd;
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/net/udp/tests.rs" "/checkout/library/std/src/sys/unix/os.rs" "/checkout/library/std/src/sys/unix/thread.rs" "/checkout/library/std/src/sys/unix/l4re.rs" "/checkout/library/std/src/sys/unix/memchr.rs" "/checkout/library/std/src/sys/unix/rand.rs" "/checkout/library/std/src/sys/unix/fs.rs" "/checkout/library/std/src/primitive_docs.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
