
error[E0308]: mismatched types
  --> ../unknown-vars-resolve-instance/test.rs:26:5
   |
26 |     requires_distinct("str", 12);
   |     ^^^^^^^^^^^^^^^^^ expected `true`, found `false`
   |
   = note: expected type `true`
              found type `false`

error: aborting due to previous error
