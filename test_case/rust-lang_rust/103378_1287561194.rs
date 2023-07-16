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
Diff in /checkout/library/core/tests/ptr.rs at line 458:
 #[test]
 fn align_offset_issue_103361() {
     #[cfg(target_pointer_width = "64")]
-    const SIZE: usize = 1<<47;
+    const SIZE: usize = 1 << 47;
     #[cfg(target_pointer_width = "32")]
-    const SIZE: usize = 1<<30;
+    const SIZE: usize = 1 << 30;
     #[cfg(target_pointer_width = "16")]
-    const SIZE: usize = 1<<13;
+    const SIZE: usize = 1 << 13;
     struct HugeSize([u8; SIZE - 1]);
     let _ = (SIZE as *const HugeSize).align_offset(SIZE);
Diff in /checkout/library/core/tests/ptr.rs at line 469:
-
 
 #[test]
 #[test]
 fn offset_from() {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/core/tests/time.rs" "/checkout/library/core/src/cell.rs" "/checkout/library/core/tests/ptr.rs" "/checkout/library/core/src/time.rs" "/checkout/library/core/tests/num/i64.rs" "/checkout/library/core/src/unicode/unicode_data.rs" "/checkout/library/core/tests/num/ieee754.rs" "/checkout/library/core/src/prelude/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Diff in /checkout/library/core/src/ptr/mod.rs at line 1571:
Diff in /checkout/library/core/src/ptr/mod.rs at line 1571:
     // FIXME(#75598): Direct use of these intrinsics improves codegen significantly at opt-level <=
     // 1, where the method versions of these operations are not inlined.
     use intrinsics::{
-        cttz_nonzero, exact_div, unchecked_rem, unchecked_shl, unchecked_shr, unchecked_sub,
-        wrapping_add, wrapping_mul, wrapping_sub, mul_with_overflow
+        cttz_nonzero, exact_div, mul_with_overflow, unchecked_rem, unchecked_shl, unchecked_shr,
+        unchecked_sub, wrapping_add, wrapping_mul, wrapping_sub,
 
 
     /// Calculate multiplicative modular inverse of `x` modulo `m`.
