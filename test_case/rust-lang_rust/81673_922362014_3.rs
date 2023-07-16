
error[E0382]: use of moved value: `foo.0`
  --> src/main.rs:13:9
   |
10 |     let mut foo = Movable(42);
   |         ------- move occurs because `foo` has type `Movable`, which does not implement the `Copy` trait
...
13 |         foo.0 += 1;
   |         ^^^^^^^^^^ value used here after move
14 |         println!("{}", foo.into_inner());
   |                            ------------ `foo` moved due to this method call, in previous iteration of loop
   |
note: this function takes ownership of the receiver `self`, which moves `foo`
  --> src/main.rs:4:19
   |
4  |     fn into_inner(self) -> i32 {
   |                   ^^^^
