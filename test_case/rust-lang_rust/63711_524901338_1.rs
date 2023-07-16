
error[E0308]: mismatched types
  --> src/lib.rs:15:19
   |
15 |             addr: "127.0.0.1:1883",
   |                   ^^^^^^^^^^^^^^^^ expected type parameter, found reference
   |
   = note: expected type `A`
              found type `&'static str`
