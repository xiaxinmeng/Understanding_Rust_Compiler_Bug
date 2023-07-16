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
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/library/alloc/src/raw_vec.rs at line 229:
     /// This will always be `usize::MAX` if `T` is zero-sized.
     #[inline(always)]
     pub fn capacity(&self) -> usize {
-        if mem::size_of::<T>() == 0 {
-        } else {
-            self.cap
-        }
-        }
+        if mem::size_of::<T>() == 0 { usize::MAX } else { self.cap }
 
 
     /// Returns a shared reference to the allocator backing this `RawVec`.
Diff in /checkout/library/alloc/src/raw_vec.rs at line 339:
         len: usize,
         additional: usize,
     ) -> Result<(), TryReserveError> {
-        if self.needs_to_grow(len, additional) {
-            self.grow_exact(len, additional)
-            Ok(())
-        }
-        }
+        if self.needs_to_grow(len, additional) { self.grow_exact(len, additional) } else { Ok(()) }
 
 
     /// Shrinks the buffer down to the specified capacity. If the given amount
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/alloc/src/raw_vec.rs" "/checkout/library/alloc/benches/binary_heap.rs" "/checkout/library/alloc/src/rc/tests.rs" "/checkout/library/alloc/src/borrow.rs" "/checkout/library/alloc/src/sync/tests.rs" "/checkout/library/alloc/src/raw_vec/tests.rs" "/checkout/library/alloc/src/alloc.rs" "/checkout/library/alloc/src/macros.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
