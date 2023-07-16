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
Diff in /checkout/library/std/src/io/mod.rs at line 438:
     // To prevent extraneously checking the UTF-8-ness of the entire buffer
     // we pass it to our hardcoded `default_read_to_end` implementation which
     // we know is guaranteed to only read data into the end of the buffer.
-    unsafe {
-        append_to_string(buf, |b| default_read_to_end(r, b))
-    }
+    unsafe { append_to_string(buf, |b| default_read_to_end(r, b)) }
 
 
 pub(crate) fn default_read_vectored<F>(read: F, bufs: &mut [IoSliceMut<'_>]) -> Result<usize>
Diff in /checkout/library/std/src/io/mod.rs at line 2214:
         // Note that we are not calling the `.read_until` method here, but
         // rather our hardcoded implementation. For more details as to why, see
         // the comments in `read_to_end`.
-        unsafe {
-            append_to_string(buf, |b| read_until(self, b'\n', b))
-        }
+        unsafe { append_to_string(buf, |b| read_until(self, b'\n', b)) }
 
     /// Returns an iterator over the contents of this reader split on the byte
     /// Returns an iterator over the contents of this reader split on the byte
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/sync/condvar/tests.rs" "/checkout/library/std/src/io/buffered/bufwriter.rs" "/checkout/library/std/src/io/buffered/linewriter.rs" "/checkout/library/std/src/io/mod.rs" "/checkout/library/std/src/sync/once.rs" "/checkout/library/std/src/sync/mod.rs" "/checkout/library/std/src/io/buffered/linewritershim.rs" "/checkout/library/std/src/io/buffered/tests.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
