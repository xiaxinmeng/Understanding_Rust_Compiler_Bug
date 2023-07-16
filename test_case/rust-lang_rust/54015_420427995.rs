
error[E0382]: borrow of moved value: `y`
 --> src/main.rs:8:24
  |
8 |         println!("{}", y); //~ ERROR use of moved value: `y`
  |                        ^ value borrowed here after move
9 |         while true { while true { while true { x = y; x.clone(); } } }
  |                                                    - value moved here
  |
  = note: move occurs because `y` has type `std::boxed::Box<isize>`, which does not implement the `Copy` trait
