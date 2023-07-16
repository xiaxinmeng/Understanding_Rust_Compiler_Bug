
error[E0223]: ambiguous associated type
  --> src/main.rs:18:12
   |
18 |     let v: S::M = 0;
   |            ^^^^ help: use fully-qualified syntax: `<S as Trait>::M`

error: aborting due to previous error
