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
Diff in /checkout/compiler/rustc_middle/src/mir/interpret/value.rs at line 317:
             Scalar::Int(int) => Ok(int),
             Scalar::Ptr(ptr, sz) => {
                 if Tag::OFFSET_IS_ADDR {
-                    Ok(
-                        ScalarInt::try_from_uint(ptr.offset.bytes(), Size::from_bytes(sz)).unwrap(),
-                    )
+                    Ok(ScalarInt::try_from_uint(ptr.offset.bytes(), Size::from_bytes(sz)).unwrap())
                 } else {
                     // We know `offset` is relative, since `OFFSET_IS_ADDR == false`.
                     let (tag, offset) = ptr.into_parts();
Diff in /checkout/compiler/rustc_middle/src/mir/interpret/value.rs at line 339:
     #[inline]
     pub fn to_bits(self, target_size: Size) -> InterpResult<'tcx, u128> {
         assert_ne!(target_size.bytes(), 0, "you should never look at the bits of a ZST");
-        self.try_to_int()
-            .map_err(|_| err_unsup!(ReadPointerAsBytes))?
-            .to_bits(target_size)
-            .map_err(|size| {
+        self.try_to_int().map_err(|_| err_unsup!(ReadPointerAsBytes))?.to_bits(target_size).map_err(
+            |size| {
                 err_ub!(ScalarSizeMismatch {
                     target_size: target_size.bytes(),
                     data_size: size.bytes(),
Diff in /checkout/compiler/rustc_middle/src/mir/interpret/value.rs at line 349:
                 .into()
-            })
+            },
+        )
+        )
     }
 
     #[inline(always)]
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_middle/src/ty/query/mod.rs" "/checkout/compiler/rustc_middle/src/mir/interpret/mod.rs" "/checkout/compiler/rustc_middle/src/mir/interpret/pointer.rs" "/checkout/compiler/rustc_middle/src/mir/interpret/queries.rs" "/checkout/compiler/rustc_middle/src/mir/interpret/allocation.rs" "/checkout/compiler/rustc_middle/src/mir/interpret/error.rs" "/checkout/compiler/rustc_middle/src/mir/predecessors.rs" "/checkout/compiler/rustc_middle/src/mir/interpret/value.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
