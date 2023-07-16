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
Diff in /checkout/library/std/src/io/buffered/tests.rs at line 976:
     let writer = ProgrammableSink::default();
     let mut writer = LineWriter::new(writer);
 
-    fn write_vectored<'a, T: Into<IoSlice<'a>>, W: Write>(writer: &mut LineWriter<W>, content: T) -> usize {
+    fn write_vectored<'a, T: Into<IoSlice<'a>>, W: Write>(
+        writer: &mut LineWriter<W>,
+        content: T,
+    ) -> usize {
         writer.write_vectored(&[content.into()]).unwrap()
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/io/buffered/tests.rs" "/checkout/library/std/src/io/buffered/mod.rs" "/checkout/library/std/src/io/buffered/linewritershim.rs" "/checkout/library/std/src/io/tests.rs" "/checkout/library/std/src/io/error.rs" "/checkout/library/std/src/io/util/tests.rs" "/checkout/library/std/src/io/impls/tests.rs" "/checkout/library/std/src/io/buffered/bufwriter.rs"` failed.
Build completed unsuccessfully in 0:00:12
Build completed unsuccessfully in 0:00:12
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
