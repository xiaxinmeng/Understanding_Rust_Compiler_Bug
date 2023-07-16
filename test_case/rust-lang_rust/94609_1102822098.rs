plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
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
Diff in /checkout/library/std/src/sys/unix/fd.rs at line 11:
 
 use libc::{c_int, c_void};
 
+#[cfg(any(
+    target_os = "android",
+    target_os = "linux",
+    target_os = "emscripten",
+    target_os = "l4re"
+use libc::off64_t;
+use libc::off64_t;
 #[cfg(not(any(
     target_os = "linux",
     target_os = "emscripten",
Diff in /checkout/library/std/src/sys/unix/fd.rs at line 18:
     target_os = "android"
 )))]
 use libc::off_t as off64_t;
-#[cfg(any(target_os = "android", target_os = "linux", target_os = "emscripten", target_os = "l4re"))]
-use libc::off64_t;
 #[derive(Debug)]
 #[derive(Debug)]
 pub struct FileDesc(OwnedFd);
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/sys/unix/mod.rs" "/checkout/library/std/src/sys/unix/memchr.rs" "/checkout/library/std/src/sys/unix/thread.rs" "/checkout/library/std/src/sys/unix/path.rs" "/checkout/library/std/src/sys/unix/stdio.rs" "/checkout/library/std/src/sys/unix/weak.rs" "/checkout/library/std/src/sys/unix/fd.rs" "/checkout/library/std/src/sys/unix/os.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
