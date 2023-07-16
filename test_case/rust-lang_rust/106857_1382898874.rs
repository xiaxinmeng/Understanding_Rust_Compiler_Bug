
error[E0596]: cannot borrow `*x` as mutable, as it is behind a `&` reference
 --> src/main.rs:5:18
  |
2 |     let x: &[isize] = &mut [1, 2, 3, 4, 5];
  |         - consider changing this binding's type to be: `&mut [isize]`
...
5 |     let _ = &mut x[2..4]; //~ERROR cannot borrow `*x` as mutable, as it is behind a `&` reference
  |                  ^ `x` is a `&` reference, so the data it refers to cannot be borrowed as mutable
