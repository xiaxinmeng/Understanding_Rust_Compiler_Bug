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
Diff in /checkout/library/alloc/src/sync.rs at line 978:
         let cnt = this.inner().weak.load(SeqCst);
         // If the weak count is currently locked, the value of the
         // count was 0 just before taking the lock.
-        if cnt == usize::MAX {
-        } else {
-            cnt - 1
-        }
-        }
+        if cnt == usize::MAX { 0 } else { cnt - 1 }
 
 
     /// Gets the number of strong (`Arc`) pointers to this allocation.
Diff in /checkout/library/alloc/src/sync.rs at line 1935:
     #[must_use]
     #[stable(feature = "weak_counts", since = "1.41.0")]
     pub fn strong_count(&self) -> usize {
-        if let Some(inner) = self.inner() {
-            inner.strong.load(SeqCst)
-            0
-        }
-        }
+        if let Some(inner) = self.inner() { inner.strong.load(SeqCst) } else { 0 }
 
 
     /// Gets an approximation of the number of `Weak` pointers pointing to this
Diff in /checkout/library/alloc/src/rc.rs at line 1063:
     #[inline]
     #[stable(feature = "rc_unique", since = "1.4.0")]
     pub fn get_mut(this: &mut Self) -> Option<&mut T> {
-        if Rc::is_unique(this) {
-            unsafe { Some(Rc::get_mut_unchecked(this)) }
-            None
-        }
-        }
+        if Rc::is_unique(this) { unsafe { Some(Rc::get_mut_unchecked(this)) } } else { None }
 
 
     /// Returns a mutable reference into the given `Rc`,
Diff in /checkout/library/alloc/src/rc.rs at line 2289:
     #[must_use]
     #[stable(feature = "weak_counts", since = "1.41.0")]
     pub fn strong_count(&self) -> usize {
-        if let Some(inner) = self.inner() {
-            inner.strong()
-            0
-        }
-        }
+        if let Some(inner) = self.inner() { inner.strong() } else { 0 }
 
 
     /// Gets the number of `Weak` pointers pointing to this allocation.
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/alloc/src/macros.rs" "/checkout/library/alloc/src/collections/linked_list/tests.rs" "/checkout/library/alloc/src/rc.rs" "/checkout/library/alloc/src/lib.rs" "/checkout/library/alloc/src/rc/tests.rs" "/checkout/library/alloc/src/raw_vec/tests.rs" "/checkout/library/alloc/src/borrow.rs" "/checkout/library/alloc/src/alloc.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
