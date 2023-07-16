
     // Note: this is not intended to be stabilized in its current form. The
     // second element of the return value should instead be `&mut [T; {N - M}]`.
-    #[unstable(feature = "array_split_array", reason = "not intended for stabilization", issue = "74674")]
+    #[unstable(
+        feature = "array_split_array",
+        reason = "not intended for stabilization",
+        issue = "74674"
+    )]
     #[inline]
     pub fn split_array_mut<const M: usize>(&mut self) -> (&mut [T; M], &mut [T]) {
         self[..].split_array_mut::<M>()
Diff in /checkout/library/core/src/slice/mod.rs at line 1700:
     pub fn split_array_mut<const N: usize>(&mut self) -> (&mut [T; N], &mut [T]) {
         let (a, b) = self.split_at_mut(N);
         // SAFETY: a points to [T; N]? Yes it's [T] of length N (checked by split_at_mut)
-        unsafe { (&mut*(a.as_mut_ptr() as *mut [T; N]), b) }
+        unsafe { (&mut *(a.as_mut_ptr() as *mut [T; N]), b) }
 
 
     /// Returns an iterator over subslices separated by elements that match
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/core/src/ptr/metadata.rs" "/checkout/library/core/src/char/convert.rs" "/checkout/library/core/src/ptr/mut_ptr.rs" "/checkout/library/core/src/char/decode.rs" "/checkout/library/core/src/char/mod.rs" "/checkout/library/core/src/array/mod.rs" "/checkout/library/core/src/borrow.rs" "/checkout/library/core/src/char/methods.rs"` failed.
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/tidy
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
