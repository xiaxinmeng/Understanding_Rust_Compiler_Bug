
error[E0594]: cannot assign to data in a `&` reference
 --> src/main.rs:4:5
  |
4 |     x.a += 1;
  |     ^^^^^^^^ cannot assign

error[E0594]: cannot assign to `y.a` which is behind a `&` reference
 --> src/main.rs:7:5
  |
6 |     let mut y = &(Foo { a: 1 });
  |                 --------------- help: consider changing this to be a mutable reference: `&mut (Foo { a: 1 })`
7 |     y.a += 1;
  |     ^^^^^^^^ `y` is a `&` reference, so the data it refers to cannot be written
