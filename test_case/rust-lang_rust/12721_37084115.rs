 diff
--- foo.rs  2014-03-07 17:07:36.000000000 -0800
+++ bar.rs  2014-03-07 17:07:25.000000000 -0800
@@ -57,6 +57,6 @@
   // hamcrest.rs:55:18: 55:35 error: cannot determine a type for this bounded type parameter: unconstrained type
   // hamcrest.rs:55   assertThat(&1, &is(&equalTo(&1)));
   //                                 ^~~~~~~~~~~~~~~~~
-  assertThat(&1, &is(&equalTo(&1)));
+  assertThat(&1, &is(&equalTo(&1) as &Matcher<int>));
 }
