
+error[E0623]: lifetime mismatch
+  --> $DIR/ex3-both-anon-regions-both-are-structs.rs:15:12
+   |
+14 | fn foo(mut x: Vec<Ref>, y: Ref) {
+   |                   ---      --- these two types are declared with different lifetimes...
+15 |     x.push(y);
+   |            ^ ...but data from `y` flows into `x` here
+
+error: aborting due to previous error
