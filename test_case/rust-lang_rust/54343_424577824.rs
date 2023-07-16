shell
>$ rustc +stage2 ./src/test/ui/liveness/liveness-move-in-while.rs
error[E0382]: borrow of moved value: `y`
  --> ./src/test/ui/liveness/liveness-move-in-while.rs:18:24
   |
18 |         println!("{}", y); //~ ERROR borrow of moved value: `y`
   |                        ^ value moved here in previous iteration of loop
   |
   = note: move occurs because `y` has type `std::boxed::Box<isize>`, which does not implement the `Copy` trait

error: aborting due to previous error

For more information about this error, try `rustc --explain E0382`.
