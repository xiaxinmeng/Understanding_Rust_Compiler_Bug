diff
 error[E0277]: the trait bound `{integer}: std::ops::Mul<()>` is not satisfied
-  --> $DIR/binops.rs:14:5
+  --> $DIR/binops.rs:14:7
    |
 14 |     3 * ();
-   |     ^^^^^^ no implementation for `{integer} * ()`
+   |       ^ no implementation for `{integer} * ()`
    |
    = help: the trait `std::ops::Mul<()>` is not implemented for `{integer}`
