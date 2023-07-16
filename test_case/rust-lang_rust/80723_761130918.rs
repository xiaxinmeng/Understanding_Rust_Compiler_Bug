
+error: call to `.clone()` on a reference in this situation does nothing
+  --> $DIR/unnecessary_clone.rs:89:29
+   |
+LL |         let _ = &mut encoded.clone();
+   |                             ^^^^^^^^ unnecessary method call
+   |
+   = note: `-D noop-method-call` implied by `-D warnings`
+   = note: the type `&[u8]` which `clone` is being called on is the same as the type returned from `clone`, so the method call does not do anything and can be removed
+
+error: call to `.clone()` on a reference in this situation does nothing
+  --> $DIR/unnecessary_clone.rs:90:25
+   |
+LL |         let _ = &encoded.clone();
+   |                         ^^^^^^^^ unnecessary method call
+   |
+   = note: the type `&[u8]` which `clone` is being called on is the same as the type returned from `clone`, so the method call does not do anything and can be removed
