patch
  error[E0432]: unresolved import `two::meth`
    --> src/lib.rs:16:5
     |
  16 | use two::meth;
-    |     ^^^^^^^^^ no `meth` in `two`
+    | ----^^^^^^^^^ no `meth` in `two`
+    | |
+    | help: remove this use item
+    |
+ note: if you intended to use the method `meth` from the impl block in module `two`...
+   --> src/lib.rs:11:12
+    |
+ 11 |         fn meth() {}
+    |            ^^^^
+    |
+ note: ...nothing else needs to be imported since it is already made available through this import
+   --> src/lib.rs:15:5
+    |
+ 15 | use one::Tr;
+    |     ^^^^^^^

  warning: unused import: `one::Tr`
    --> src/lib.rs:15:5
     |
  15 | use one::Tr;
     |     ^^^^^^^
     |
     = note: `#[warn(unused_imports)]` on by default
