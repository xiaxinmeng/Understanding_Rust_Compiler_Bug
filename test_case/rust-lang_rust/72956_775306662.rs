
--- test_works.hir      2021-02-08 20:05:17.375472239 +0300
+++ test_fails.hir      2021-02-08 20:05:17.603469714 +0300
@@ -46,8 +46,8 @@
                                                                        Peach(Banana::new());
                                                                    let r:
                                                                            &() =
-                                                                       &*peach.index(0);
-                                                                   match baz(r)
+                                                                       &peach[0];
+                                                                   match baz(&r)
                                                                        {
                                                                        mut pinned
                                                                        =>
