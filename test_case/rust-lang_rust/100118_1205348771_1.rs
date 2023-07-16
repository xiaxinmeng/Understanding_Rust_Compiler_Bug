patch
  error[E0432]: unresolved import `two::meth`
    --> src/lib.rs:15:5
     |
  15 | use two::meth;
     |     ^^^^^^^^^ no `meth` in `two`
+    |
+ note: if you intended to use the method `meth` from the impl block in module `two`...
+   --> src/lib.rs:11:12
+    |
+ 11 |         fn meth() {}
+    |            ^^^^
+    |
+ help: ...import the corresponding trait `Tr` and remove the incorrect use item
+   --> src/lib.rs:15:5
+    |
+ 15 | use two::meth;
+    | --------------
+ 15 | use one::Tr;
+    | ++++++++++++
