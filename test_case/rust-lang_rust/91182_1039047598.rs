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
Diff in /checkout/library/std/src/sys/windows/process.rs at line 488:
 /// Check if a file exists without following symlinks.
 fn program_exists(path: &Path) -> bool {
     unsafe {
-        to_u16s(path).map(|path| {
-            // Getting attributes using `GetFileAttributesW` does not follow symlinks
-            // and it will almost always be successful if the link exists.
-            // There are some exceptions for special system files (e.g. the pagefile)
-            // but these are not executable.
-            c::GetFileAttributesW(path.as_ptr()) != c::INVALID_FILE_ATTRIBUTES
-        }).unwrap_or(false)
+        to_u16s(path)
+            .map(|path| {
+                // Getting attributes using `GetFileAttributesW` does not follow symlinks
+                // and it will almost always be successful if the link exists.
+                // There are some exceptions for special system files (e.g. the pagefile)
+                // but these are not executable.
+                c::GetFileAttributesW(path.as_ptr()) != c::INVALID_FILE_ATTRIBUTES
+            })
+            .unwrap_or(false)
 }
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/sys/windows/process.rs" "/checkout/library/std/src/sys/windows/cmath.rs" "/checkout/library/std/src/sys/windows/rand.rs" "/checkout/library/std/src/sys/windows/thread_local_key.rs" "/checkout/library/std/src/sys/windows/stack_overflow_uwp.rs" "/checkout/library/std/src/sys/windows/os_str.rs" "/checkout/library/std/src/sys/windows/fs.rs" "/checkout/library/std/src/io/error/repr_bitpacked.rs"` failed.
Build completed unsuccessfully in 0:00:13
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
