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
Diff in /checkout/library/core/src/iter/adapters/zip.rs at line 126:
     where
         A: DoubleEndedIterator + ExactSizeIterator,
         B: DoubleEndedIterator + ExactSizeIterator;
 }
 
 
 // Work around limitations of specialization, requiring `default` impls to be repeated
Diff in /checkout/library/core/src/iter/adapters/zip.rs at line 134:
 macro_rules! zip_impl_general_defaults {
     () => {
         default fn new(a: A, b: B) -> Self {
-            Zip {
-                a,
-                b,
-            }
+            Zip { a, b }
 
         #[inline]
Diff in /checkout/library/core/src/iter/adapters/zip.rs at line 176:
                     }
                     }
                 }
             }
-            match (self.b.next_back(), self.a.next_back(), ) {
+            match (self.b.next_back(), self.a.next_back()) {
                 (Some(y), Some(x)) => Some((x, y)),
                 (None, None) => None,
                 _ => unreachable!(),
Diff in /checkout/library/core/src/iter/adapters/zip.rs at line 212:
         (lower, upper)
     }
-
 }
 }
 
-
 #[stable(feature = "rust1", since = "1.0.0")]
 impl<A, B> ExactSizeIterator for Zip<A, B>
Diff in /checkout/library/core/src/iter/adapters/zip.rs at line 224:
 {
 }
 
 
-
 #[stable(feature = "fused", since = "1.26.0")]
 impl<A, B> FusedIterator for Zip<A, B>
Diff in /checkout/library/core/src/iter/adapters/zip.rs at line 277:
Diff in /checkout/library/core/src/iter/adapters/zip.rs at line 277:
         f.debug_struct("Zip").field("a", &self.a).field("b", &self.b).finish()
 }
-
 
 
 /// An iterator whose items are random-accessible efficiently
 ///
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/core/src/iter/adapters/zip.rs" "/checkout/library/core/src/iter/adapters/filter.rs" "/checkout/library/core/src/iter/adapters/inspect.rs" "/checkout/library/core/src/iter/adapters/skip_while.rs" "/checkout/library/core/src/iter/traits/iterator.rs" "/checkout/library/core/src/iter/adapters/take_while.rs" "/checkout/library/core/src/iter/traits/exact_size.rs" "/checkout/library/core/src/iter/traits/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
