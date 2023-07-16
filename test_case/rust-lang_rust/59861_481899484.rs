
error[E0382]: use of moved value: `x`
 --> src/main.rs:4:13
  |
2 |     let x = "hello".to_string();
  |         - move occurs because `x` has type `std::string::String`, which does not implement the `Copy` trait
3 |     let z = g(x);
  |               - value moved here
4 |     let y = x;
  |             ^ value used here after move
