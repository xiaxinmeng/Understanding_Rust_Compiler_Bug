
diff of stderr:

13   --> $DIR/do-not-attempt-to-add-suggestions-with-no-changes.rs:3:25
14    |
15 LL |     fn into_future() -> Err {}
+    |                         ^^^
+    |                         |
+    |                         not a type
+    |                         not a type
+    |                         help: try using the variant's enum: `std::prelude::rust_2021`
18 error: aborting due to 2 previous errors
19 
