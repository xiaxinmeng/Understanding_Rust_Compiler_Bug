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
Diff in /checkout/library/core/src/option.rs at line 1498:
     #[must_use = "`self` will be dropped if the result is not used"]
     #[stable(feature = "rust1", since = "1.0.0")]
     #[rustc_const_unstable(feature = "const_option_cloned", issue = "none")]
-    pub const fn cloned(self) -> Option<T> where T: ~const Clone {
+    pub const fn cloned(self) -> Option<T>
+    where
+        T: ~const Clone,
         match self {
         match self {
             Some(t) => Some(t.clone()),
             None => None,
Diff in /checkout/library/core/src/option.rs at line 1522:
     #[must_use = "`self` will be dropped if the result is not used"]
     #[stable(since = "1.26.0", feature = "option_ref_mut_cloned")]
     #[rustc_const_unstable(feature = "const_option_cloned", issue = "none")]
-    pub const fn cloned(self) -> Option<T> where T: ~const Clone {
+    pub const fn cloned(self) -> Option<T>
+    where
+        T: ~const Clone,
         match self {
         match self {
             Some(t) => Some(t.clone()),
             None => None,
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/core/src/array/iter.rs" "/checkout/library/core/src/array/equality.rs" "/checkout/library/core/src/option.rs" "/checkout/library/core/src/primitive_docs.rs" "/checkout/library/core/src/array/mod.rs" "/checkout/library/core/src/lazy.rs" "/checkout/library/core/src/ffi.rs" "/checkout/library/core/src/hint.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
