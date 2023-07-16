
error[E0308]: mismatched types
  --> src/main.rs:13:25
   |
13 |     const MY_CONST: X = Bar;
   |                         ^^^ expected trait X, found struct `Bar`
   |
   = note: expected type `(dyn X + 'static)`
              found type `Bar`
