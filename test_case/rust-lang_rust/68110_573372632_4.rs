
error[E0308]: mismatched types
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
   = note: `impl Trait` can only be used in methods that have a single `return`
   = note: for more information on `impl Trait` and trait objects, visit <FOO>
help: consider using trait objects instead of `impl Trait`

9  | fn foo() -> Box<dyn Trait> {
10 |     if true {
11 |         return Box::new(S);
12 |     }
13 |     Box::new(Y)
