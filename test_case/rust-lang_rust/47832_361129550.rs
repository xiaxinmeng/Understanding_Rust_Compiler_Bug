diff
+ error[E0277]: the trait bound `i32: std::slice::SliceIndex<[{integer}]>` is not satisfied
- error[E0277]: the trait bound `std::vec::Vec<{integer}>: std::ops::Index<i32>` is not satisfied
    --> $DIR/index-help.rs:13:5
     |
  13 |     x[0i32]; //~ ERROR E0277
+    |     ^^^^^^^ slice indices are of type `usize` or ranges of `usize`
-    |     ^^^^^^^ vector indices are of type `usize` or ranges of `usize`
     |
+    = help: the trait `std::slice::SliceIndex<[{integer}]>` is not implemented for `i32`
+    = note: required because of the requirements on the impl of `std::ops::Index<i32>` for `[{integer}]`
-    = help: the trait `std::ops::Index<i32>` is not implemented for `std::vec::Vec<{integer}>`
