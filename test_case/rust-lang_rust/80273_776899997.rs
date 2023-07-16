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
Diff in /checkout/library/alloc/src/sync.rs at line 915:
         let cnt = this.inner().meta.weak.load(SeqCst);
         // If the weak count is currently locked, the value of the
         // count was 0 just before taking the lock.
-        if cnt == usize::MAX {
-        } else {
-            cnt - 1
-        }
-        }
+        if cnt == usize::MAX { 0 } else { cnt - 1 }
 
 
     /// Gets the number of strong (`Arc`) pointers to this allocation.
Diff in /checkout/library/alloc/src/sync.rs at line 1861:
     /// If `self` was created using [`Weak::new`], this will return 0.
     #[stable(feature = "weak_counts", since = "1.41.0")]
     pub fn strong_count(&self) -> usize {
-        if let Some(inner) = self.inner() {
-            inner.strong.load(SeqCst)
-            0
-        }
-        }
+        if let Some(inner) = self.inner() { inner.strong.load(SeqCst) } else { 0 }
 
 
     /// Gets an approximation of the number of `Weak` pointers pointing to this
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/alloc/src/sync.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:18
