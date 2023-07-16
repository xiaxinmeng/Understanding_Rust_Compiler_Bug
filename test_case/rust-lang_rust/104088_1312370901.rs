diff
   --> $DIR/const-match-check.rs:19:26
    |
 LL |     const X: i32 = { let 0 = 0; 0 };
-   |                          ^ patterns `i32::MIN..=-1_i32` and `1_i32..=i32::MAX` not covered
+   |                          ^ identifiers cannot start with a number
+
