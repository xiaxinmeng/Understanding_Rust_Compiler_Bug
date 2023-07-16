
error[E0382]: borrow of moved value: `a`
 --> src/main.rs:4:21
  |
3 |     let ref _b = {a.0};
  |                   --- value moved here
4 |     println!("{:?}",a);
  |                     ^ value borrowed here after partial move
  |
  = note: move occurs because `a.0` has type `std::string::String`, which does not implement the `Copy` trait
