
error[E0503]: cannot use `e` because it was mutably borrowed
  --> src/main.rs:13:9
   |
10 |     let f = &mut e;
   |             ------ borrow of `e` occurs here
...
13 |         Xyz::A => println!("a"),
   |         ^^^^^^ use of borrowed `e`
