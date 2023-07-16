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
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/library/std/src/io/readbuf.rs at line 207:
     #[inline]
     pub fn init_mut(&mut self) -> &mut [u8] {
         // SAFETY: We only slice the initialized part of the buffer, which is always valid
-        unsafe {
-            self.buf.buf[self.buf.filled..self.buf.init].assume_init_mut()
-        }
+        unsafe { self.buf.buf[self.buf.filled..self.buf.init].assume_init_mut() }
 
     /// Returns a mutable reference to the uninitialized part of the cursor.
     /// Returns a mutable reference to the uninitialized part of the cursor.
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/io/error/tests.rs" "/checkout/library/std/src/io/impls/tests.rs" "/checkout/library/std/src/io/error.rs" "/checkout/library/std/src/io/readbuf.rs" "/checkout/library/std/src/io/readbuf/tests.rs" "/checkout/library/std/src/io/cursor/tests.rs" "/checkout/library/std/src/io/buffered/bufreader/buffer.rs" "/checkout/library/std/src/io/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
