
error[E0308]: mismatched types
  --> src/main.rs:21:28
   |
21 |     let arr: [usize; 25] = [1; <Multiply<Five, Five>>::VAL];
   |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an array with a fixed size of 25 elements, found one with 0 elements
   |
   = note: expected type `[usize; 25]`
              found type `[usize; 0]`
