bash
diff of stderr:

 error[E0282]: type annotations needed
-  --> $DIR/unknown_type_for_closure.rs:12:14
+  --> $DIR/unknown_type_for_closure.rs:12:17
    |
 12 |     let x = |_| {    };
-   |              ^ consider giving this closure parameter a type
+   |                 ^ cannot infer type for `_`

 error: aborting due to previous error(s)

