diff
modified   src/test/ui/impl-trait/bound-normalization-pass.rs
@@ -43,12 +43,14 @@ mod lifetimes {
     }

     /// Like above.
-    fn foo2_pass<'a, T: Trait<'a, Assoc=()> + 'a>() -> impl FooLike<Output=T::Assoc> + 'a {
+    fn foo2_pass<'a, T: Trait<'a, Assoc=()> + 'a>(
+    ) -> impl FooLike<Output=<T as Trait<'a>>::Assoc> + 'a {
         Foo(())
     }

     /// Normalization to type containing bound region.
-    fn foo2_pass2<'a, T: Trait<'a, Assoc=&'a ()> + 'a>() -> impl FooLike<Output=T::Assoc> + 'a {
+    fn foo2_pass2<'a, T: Trait<'a, Assoc=&'a ()> + 'a>(
+    ) -> impl FooLike<Output=<T as Trait<'a>>::Assoc> + 'a {
         Foo(&())
     }
 }
