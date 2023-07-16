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
Diff in /checkout/library/alloc/src/vec/source_iter_marker.rs at line 103:
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/alloc/src/vec/source_iter_marker.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
 
 
 impl<T, I> SpecInplace<T, I> for I
-    where
-        I: Iterator<Item = T>,
+where
+    I: Iterator<Item = T>,
 {
     #[inline]
     default fn collect_in_place(&mut self, dst_buf: *mut T, end: *const T) -> usize {
Diff in /checkout/library/alloc/src/vec/source_iter_marker.rs at line 121:
 
 
 impl<T, I> SpecInplace<T, I> for I
-    where
-        I: Iterator<Item = T> + TrustedRandomAccess,
+where
+    I: Iterator<Item = T> + TrustedRandomAccess,
 {
     #[inline]
     fn collect_in_place(&mut self, dst_buf: *mut T, end: *const T) -> usize {
Build completed unsuccessfully in 0:00:23
