
error[E0XX3]: mismatched types
  --> src/main.rs:13:5
   |
9  | fn foo() -> impl Trait {
   |             ---------- expected because this return type...
10 |     if true {
11 |         return S;
   |                - ...is found to be `S` here
12 |     }
13 |     Y
   |     ^ expected struct `S`, found struct `Y`
