
error[E0382]: use of moved value: `a`
 --> test.rs:4:9
  |
4 |         a;
  |         ^ value moved here in previous iteration of loop
  |
  = note: move occurs because `a` has type `std::string::String`, which does not implement the `Copy` trait

error: aborting due to previous error(s)
