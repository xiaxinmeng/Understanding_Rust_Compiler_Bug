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
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/library/std/src/io/impls.rs at line 377:
 
     #[inline]
     fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
-        let len = bufs
-            .iter()
-            .try_fold(0usize, |len, b| len.checked_add(b.len()))
-            .expect(OOM_ERROR_MSG);
+        let len =
+            bufs.iter().try_fold(0usize, |len, b| len.checked_add(b.len())).expect(OOM_ERROR_MSG);
         self.try_reserve(len).expect(OOM_ERROR_MSG);
         for buf in bufs {
             self.extend_from_slice(buf);
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/io/buffered/linewritershim.rs" "/checkout/library/std/src/io/cursor.rs" "/checkout/library/std/src/io/prelude.rs" "/checkout/library/std/src/panic.rs" "/checkout/library/std/src/io/impls.rs" "/checkout/library/std/src/io/buffered/tests.rs" "/checkout/library/std/src/time/tests.rs" "/checkout/library/std/src/io/buffered/bufreader.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:15
