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
Diff in /checkout/library/core/src/cmp.rs at line 1130:
     #[must_use]
     #[stable(feature = "rust1", since = "1.0.0")]
     fn lt(&self, other: &Rhs) -> bool {
-        if let Some(ordering) = self.partial_cmp(other) {
-            ordering.is_lt()
-            false
-        }
-        }
+        if let Some(ordering) = self.partial_cmp(other) { ordering.is_lt() } else { false }
 
 
     /// This method tests less than or equal to (for `self` and `other`) and is used by the `<=`
Diff in /checkout/library/core/src/cmp.rs at line 1153:
     #[must_use]
     #[stable(feature = "rust1", since = "1.0.0")]
     fn le(&self, other: &Rhs) -> bool {
-        if let Some(ordering) = self.partial_cmp(other) {
-            ordering.is_le()
-            false
-        }
-        }
+        if let Some(ordering) = self.partial_cmp(other) { ordering.is_le() } else { false }
 
 
     /// This method tests greater than (for `self` and `other`) and is used by the `>` operator.
Diff in /checkout/library/core/src/cmp.rs at line 1175:
     #[must_use]
     #[stable(feature = "rust1", since = "1.0.0")]
     fn gt(&self, other: &Rhs) -> bool {
-        if let Some(ordering) = self.partial_cmp(other) {
-            ordering.is_gt()
-            false
-        }
-        }
+        if let Some(ordering) = self.partial_cmp(other) { ordering.is_gt() } else { false }
 
 
     /// This method tests greater than or equal to (for `self` and `other`) and is used by the `>=`
Diff in /checkout/library/core/src/cmp.rs at line 1198:
     #[must_use]
     #[stable(feature = "rust1", since = "1.0.0")]
     fn ge(&self, other: &Rhs) -> bool {
-        if let Some(ordering) = self.partial_cmp(other) {
-            ordering.is_ge()
-            false
-        }
-        }
+        if let Some(ordering) = self.partial_cmp(other) { ordering.is_ge() } else { false }
 }
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/core/src/ffi/c_str.rs" "/checkout/library/core/src/ffi/mod.rs" "/checkout/library/core/src/cmp.rs" "/checkout/library/core/src/lib.rs" "/checkout/library/core/src/const_closure.rs" "/checkout/library/core/src/macros/mod.rs" "/checkout/library/core/src/ptr/mod.rs" "/checkout/library/core/src/arch.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
