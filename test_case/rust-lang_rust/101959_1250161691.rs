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
Diff in /checkout/library/core/src/num/f32.rs at line 1391:
     #[stable(feature = "clamp", since = "1.50.0")]
     #[inline]
     pub fn clamp(mut self, min: f32, max: f32) -> f32 {
-        assert!(min <= max, "min > max, or either was NaN. This is not allowed. min: {min:?}, max: {max:?}");
+        assert!(
+            min <= max,
+            "min > max, or either was NaN. This is not allowed. min: {min:?}, max: {max:?}"
+        );
         if self < min {
             self = min;
Diff in /checkout/library/core/src/num/f64.rs at line 1389:
Diff in /checkout/library/core/src/num/f64.rs at line 1389:
     #[stable(feature = "clamp", since = "1.50.0")]
     #[inline]
     pub fn clamp(mut self, min: f64, max: f64) -> f64 {
-        assert!(min <= max, "min > max, or either was NaN. This is not allowed. min: {min:?}, max: {max:?}");
+        assert!(
+            min <= max,
+            "min > max, or either was NaN. This is not allowed. min: {min:?}, max: {max:?}"
+        );
         if self < min {
             self = min;
         }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/core/src/hint.rs" "/checkout/library/core/tests/asserting.rs" "/checkout/library/core/src/num/f32.rs" "/checkout/library/core/src/future/into_future.rs" "/checkout/library/core/src/num/f64.rs" "/checkout/library/core/src/future/poll_fn.rs" "/checkout/library/core/src/num/error.rs" "/checkout/library/core/tests/iter/traits/accum.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
