patch
  error[E0308]: mismatched types
   --> src/main.rs:7:12
    |
  7 |     if let E::One(var1, var2) = var {
    |            ^^^^^^^^^^^^^^^^^^   --- this expression has type `fn(i32, i32) -> E {E::One}`
    |            |
    |            expected fn item, found enum `E`
    |
    = note: expected fn item `fn(i32, i32) -> E {E::One}`
                  found enum `E`
+ help: use parentheses to instantiate this tuple variant
+   |
+ 7 |     if let E::One(var1, var2) = var(_, _) {
+   |                                    ++++++
