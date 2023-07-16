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
Diff in /checkout/library/core/src/iter/traits/double_ended.rs at line 1:
+use super::super::{TrustedLen, TrustedRandomAccess};
 use crate::ops::{ControlFlow, Try};
-use super::super::{TrustedRandomAccess, TrustedLen};
 
 /// An iterator able to yield elements from both ends.
Diff in /checkout/library/core/src/iter/traits/double_ended.rs at line 359:
     }
 }
 
 
-
-
 trait SpecIteratorDefaultImpls: DoubleEndedIterator {
     fn rfold_spec<B, F>(self, init: B, f: F) -> B
-        where
-            Self: Sized,
-            F: FnMut(B, Self::Item) -> B;
+    where
+        Self: Sized,
+        F: FnMut(B, Self::Item) -> B;
 
 
-impl<T> SpecIteratorDefaultImpls for T where T: DoubleEndedIterator {
+impl<T> SpecIteratorDefaultImpls for T
+where
+    T: DoubleEndedIterator,
     #[inline]
     #[inline]
     default fn rfold_spec<B, F>(mut self, init: B, mut f: F) -> B
-        where
-            Self: Sized,
-            F: FnMut(B, Self::Item) -> B,
+    where
+        Self: Sized,
+        F: FnMut(B, Self::Item) -> B,
         let mut accum = init;
         let mut accum = init;
         while let Some(x) = self.next_back() {
Diff in /checkout/library/core/src/iter/traits/double_ended.rs at line 383:
 }
 
 
-impl<T> SpecIteratorDefaultImpls for T where T: DoubleEndedIterator + Sized + TrustedLen {
+impl<T> SpecIteratorDefaultImpls for T
+where
+    T: DoubleEndedIterator + Sized + TrustedLen,
     #[inline]
     #[inline]
     default fn rfold_spec<B, F>(mut self, init: B, mut f: F) -> B
-        where
-            Self: Sized,
-            F: FnMut(B, Self::Item) -> B,
+    where
+        Self: Sized,
+        F: FnMut(B, Self::Item) -> B,
         let mut accum = init;
         let mut accum = init;
         while self.size_hint().1.is_none() {
Diff in /checkout/library/core/src/iter/traits/double_ended.rs at line 399:
         accum
     }
-
 }
 }
 
-impl<T> SpecIteratorDefaultImpls for T where T: DoubleEndedIterator + Sized + TrustedLen + TrustedRandomAccess {
+impl<T> SpecIteratorDefaultImpls for T
+where
+    T: DoubleEndedIterator + Sized + TrustedLen + TrustedRandomAccess,
     #[inline]
     #[inline]
     fn rfold_spec<B, F>(mut self, init: B, mut f: F) -> B
-        where
-            Self: Sized,
-            F: FnMut(B, Self::Item) -> B,
+    where
+        Self: Sized,
+        F: FnMut(B, Self::Item) -> B,
         let mut accum = init;
         let mut accum = init;
         // SAFETY: every element is only read once, as required by the
Diff in /checkout/library/core/src/iter/traits/iterator.rs at line 5:
 use crate::cmp::{self, Ordering};
 use crate::ops::{ControlFlow, Try};
 
-use super::super::{TrustedRandomAccess, TrustedLen};
 use super::super::{Chain, Cloned, Copied, Cycle, Enumerate, Filter, FilterMap, Fuse};
 use super::super::{FlatMap, Flatten};
 use super::super::{FromIterator, Intersperse, IntersperseWith, Product, Sum, Zip};
Diff in /checkout/library/core/src/iter/traits/iterator.rs at line 12:
 use super::super::{
     Inspect, Map, MapWhile, Peekable, Rev, Scan, Skip, SkipWhile, StepBy, Take, TakeWhile,
 };
+use super::super::{TrustedLen, TrustedRandomAccess};
 
 fn _assert_is_object_safe(_: &dyn Iterator<Item = ()>) {}
Diff in /checkout/library/core/src/iter/traits/iterator.rs at line 3417:
     }
 }
 
 
-
 trait SpecIteratorDefaultImpls: Iterator {
     fn fold_spec<B, F>(self, init: B, f: F) -> B
-        where
-            Self: Sized,
-            F: FnMut(B, Self::Item) -> B;
+    where
+        Self: Sized,
+        F: FnMut(B, Self::Item) -> B;
 
 
-impl<T> SpecIteratorDefaultImpls for T where T: Iterator {
+impl<T> SpecIteratorDefaultImpls for T
+where
+    T: Iterator,
     #[inline]
     #[inline]
     default fn fold_spec<B, F>(mut self, init: B, mut f: F) -> B
-        where
-            Self: Sized,
-            F: FnMut(B, Self::Item) -> B
+    where
+        Self: Sized,
+        F: FnMut(B, Self::Item) -> B,
-
         let mut accum = init;
         let mut accum = init;
         while let Some(x) = self.next() {
             accum = f(accum, x);
Diff in /checkout/library/core/src/iter/traits/iterator.rs at line 3441:
 }
 
 
-impl<T> SpecIteratorDefaultImpls for T where T: Iterator + Sized + TrustedLen {
+impl<T> SpecIteratorDefaultImpls for T
+where
+    T: Iterator + Sized + TrustedLen,
     #[inline]
     #[inline]
     default fn fold_spec<B, F>(mut self, init: B, mut f: F) -> B
-        where
-            Self: Sized,
-            F: FnMut(B, Self::Item) -> B
+    where
+        Self: Sized,
+        F: FnMut(B, Self::Item) -> B,
         let mut accum = init;
         let mut accum = init;
         while self.size_hint().1.is_none() {
Diff in /checkout/library/core/src/iter/traits/iterator.rs at line 3455:
         for _ in 0..self.size_hint().1.unwrap() {
             accum = f(accum, self.next().unwrap())
-       accum
+        accum
     }
 }
 }
 
Diff in /checkout/library/core/src/iter/traits/iterator.rs at line 3462:
-impl<T> SpecIteratorDefaultImpls for T where T: Iterator + Sized + TrustedLen + TrustedRandomAccess {
+impl<T> SpecIteratorDefaultImpls for T
+where
+    T: Iterator + Sized + TrustedLen + TrustedRandomAccess,
+{
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/core/src/iter/traits/mod.rs" "/checkout/library/core/src/iter/traits/accum.rs" "/checkout/library/core/src/iter/traits/marker.rs" "/checkout/library/core/src/iter/traits/double_ended.rs" "/checkout/library/core/src/iter/traits/collect.rs" "/checkout/library/core/src/iter/traits/exact_size.rs" "/checkout/library/core/src/iter/traits/iterator.rs" "/checkout/library/core/src/task/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
     #[inline]
     fn fold_spec<B, F>(mut self, init: B, mut f: F) -> B
-        where
-            Self: Sized,
-            F: FnMut(B, Self::Item) -> B
+    where
+        Self: Sized,
+        F: FnMut(B, Self::Item) -> B,
         let mut accum = init;
         let mut accum = init;
         // SAFETY: every element is only read once, as required by the
Build completed unsuccessfully in 0:00:16
