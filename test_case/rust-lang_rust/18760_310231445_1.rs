
error[E0053]: method `b` has an incompatible type for trait
 --> test.rs:6:13
  |
2 |     fn b(c: &i32);
  |             ---- type in trait
...
6 |     fn b(c: &mut i32) {  }
  |             ^^^^^^^^ types differ in mutability
  |
  = note: expected type `fn(&i32)`
             found type `fn(&mut i32)`

error: aborting due to previous error(s)
