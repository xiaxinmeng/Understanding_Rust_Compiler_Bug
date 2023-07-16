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
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/library/alloc/src/rc.rs at line 1434:
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
Diff in /checkout/library/alloc/src/rc.rs at line 2790:
     /// If `self` was created using [`Weak::new`], this will return 0.
     #[stable(feature = "weak_counts", since = "1.41.0")]
     pub fn strong_count(&self) -> usize {
-        if let Some(inner) = self.inner() {
-            inner.strong()
-            0
-        }
-        }
+        if let Some(inner) = self.inner() { inner.strong() } else { 0 }
 
 
     /// Gets the number of `Weak` pointers pointing to this allocation.
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/alloc/src/prelude/mod.rs" "/checkout/library/alloc/src/raw_vec.rs" "/checkout/library/alloc/src/alloc/tests.rs" "/checkout/library/alloc/src/rc/tests.rs" "/checkout/library/alloc/src/rc.rs" "/checkout/library/alloc/src/alloc.rs" "/checkout/library/alloc/src/tests.rs" "/checkout/library/alloc/src/sync/tests.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
