diff
27      error[E0308]: mismatched types
-         --> $DIR/if-let-typo.rs:6:12
-          |
-       LL |     if 3 = foo {}
-          |            ^^^ expected integer, found enum `Option`
-          |
-          = note: expected type `{integer}`
-                     found enum `Option<{integer}>`
-
-       error[E0308]: mismatched types
