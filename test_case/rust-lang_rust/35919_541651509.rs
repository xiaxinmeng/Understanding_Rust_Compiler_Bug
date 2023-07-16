text
   Compiling playground v0.0.1 (/playground)
error[E0382]: borrow of moved value: `y`
 --> src/main.rs:8:20
  |
3 |     let y = &mut 2i32;
  |         - move occurs because `y` has type `&mut i32`, which does not implement the `Copy` trait
4 |     {
5 |         let x = y;
  |                 - value moved here
...
8 |     println!("{}", y);
  |                    ^ value borrowed here after move

error: aborting due to previous error
