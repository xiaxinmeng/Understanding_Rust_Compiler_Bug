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
Diff in /checkout/library/core/src/iter/traits/iterator.rs at line 2237:
     #[inline]
     #[unstable(feature = "iter_at_least", reason = "new API", issue = "none")]
     fn at_least<F>(&mut self, n: usize, f: F) -> bool
-        where
-            Self: Sized,
-            F: FnMut(Self::Item) -> bool,
+    where
+        Self: Sized,
+        F: FnMut(Self::Item) -> bool,
     {
         #[inline]
         fn check<T>(
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/core/src/iter/traits/iterator.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Diff in /checkout/library/core/src/iter/traits/iterator.rs at line 2249:
             move |mut i, x| {
                 i += f(x) as usize;
-                if i < n {
-                    ControlFlow::Continue(i)
-                } else {
-                    ControlFlow::Break(i)
-                    ControlFlow::Break(i)
-                }
+                if i < n { ControlFlow::Continue(i) } else { ControlFlow::Break(i) }
         }
 
Diff in /checkout/library/core/src/iter/traits/iterator.rs at line 2306:
Diff in /checkout/library/core/src/iter/traits/iterator.rs at line 2306:
     #[inline]
     #[unstable(feature = "iter_at_most", reason = "new API", issue = "none")]
     fn at_most<F>(&mut self, n: usize, f: F) -> bool
-        where
-            Self: Sized,
-            F: FnMut(Self::Item) -> bool,
+    where
+        Self: Sized,
+        F: FnMut(Self::Item) -> bool,
     {
         #[inline]
         fn check<T>(
Diff in /checkout/library/core/src/iter/traits/iterator.rs at line 2318:
             move |mut i, x| {
                 i += f(x) as usize;
 
-                if i <= n {
-                    ControlFlow::Continue(i)
-                    ControlFlow::Break(i)
-                }
-                }
+                if i <= n { ControlFlow::Continue(i) } else { ControlFlow::Break(i) }
         }
 
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/tidy
Build completed unsuccessfully in 0:00:22
