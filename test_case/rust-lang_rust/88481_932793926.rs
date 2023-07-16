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
Diff in /checkout/compiler/rustc_index/src/vec.rs at line 594:
 
     #[inline]
     #[inline]
-    pub fn into_iter_enumerated(self) -> impl DoubleEndedIterator<Item = (I, T)> + ExactSizeIterator {
+        self,
+        self,
+    ) -> impl DoubleEndedIterator<Item = (I, T)> + ExactSizeIterator {
         self.raw.into_iter().enumerate().map(|(n, t)| (I::new(n), t))
 
 
Diff in /checkout/compiler/rustc_index/src/vec.rs at line 604:
 
     #[inline]
     #[inline]
-    pub fn iter_enumerated(&self) -> impl DoubleEndedIterator<Item = (I, &T)> + ExactSizeIterator + '_ {
+    pub fn iter_enumerated(
+        &self,
+    ) -> impl DoubleEndedIterator<Item = (I, &T)> + ExactSizeIterator + '_ {
         self.raw.iter().enumerate().map(|(n, t)| (I::new(n), t))
 
 
Diff in /checkout/compiler/rustc_index/src/vec.rs at line 619:
 
     #[inline]
     #[inline]
-    pub fn iter_enumerated_mut(&mut self) -> impl DoubleEndedIterator<Item = (I, &mut T)> + ExactSizeIterator + '_ {
+    pub fn iter_enumerated_mut(
+        &mut self,
+    ) -> impl DoubleEndedIterator<Item = (I, &mut T)> + ExactSizeIterator + '_ {
         self.raw.iter_mut().enumerate().map(|(n, t)| (I::new(n), t))
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_target/src/abi/call/nvptx64.rs" "/checkout/compiler/rustc_target/src/abi/call/x86_win64.rs" "/checkout/compiler/rustc_const_eval/src/interpret/machine.rs" "/checkout/compiler/rustc_const_eval/src/interpret/operand.rs" "/checkout/compiler/rustc_arena/src/tests.rs" "/checkout/compiler/rustc_ty_utils/src/ty.rs" "/checkout/compiler/rustc_index/src/vec.rs" "/checkout/compiler/rustc_target/src/abi/call/msp430.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
