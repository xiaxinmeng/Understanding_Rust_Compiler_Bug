diff
--- glacier.rs 2021-12-09 22:59:25.365148200 +0000
+++ test.rs 2021-12-09 23:02:06.135999800 +0000
@@ -26,9 +26,10 @@

 impl<T: Type> Spec1 for T where Predicate<{T::AT::C > 0}>: Satisfied {}

-trait Spec2: Spec1 {}
+trait Spec2 {}

-impl<T: Type + Spec1> Spec2 for T where Predicate<{T::AT::C > 1}>: Satisfied {}
+//impl<T: Type > Spec2 for T where Predicate<{T::AT::C > 1}>: Satisfied {}
+impl<T: Type > Spec2 for T where Predicate<true>: Satisfied {}

 //--------------------------------------------------
