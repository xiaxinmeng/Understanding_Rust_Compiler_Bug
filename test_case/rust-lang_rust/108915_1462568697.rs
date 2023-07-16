
---- [ui] tests/ui/lazy-type-alias-impl-trait/branches3.rs stdout ----
diff of stderr:

1       error[E0282]: type annotations needed
-         --> $DIR/branches3.rs:8:10
+         --> $DIR/branches3.rs:8:13
3          |
4       LL |         |s| s.len()
-          |          ^  - type must be known at this point
-          |
-       help: consider giving this closure parameter an explicit type
-          |
-       LL |         |s: /* Type */| s.len()
-          |           ++++++++++++
+          |             ^ cannot infer type
11
12      error[E0282]: type annotations needed
-         --> $DIR/branches3.rs:15:10
+         --> $DIR/branches3.rs:15:13
14         |
15      LL |         |s| s.len()
-          |          ^  - type must be known at this point
-          |
-       help: consider giving this closure parameter an explicit type
-          |
-       LL |         |s: /* Type */| s.len()
-          |           ++++++++++++
+          |             ^ cannot infer type
22
23      error[E0282]: type annotations needed
-         --> $DIR/branches3.rs:23:10
+         --> $DIR/branches3.rs:23:13
25         |
26      LL |         |s| s.len()
-          |          ^  - type must be known at this point
-          |
-       help: consider giving this closure parameter an explicit type
-          |
-       LL |         |s: /* Type */| s.len()
-          |           ++++++++++++
+          |             ^ cannot infer type
33
34      error[E0282]: type annotations needed
-         --> $DIR/branches3.rs:30:10
