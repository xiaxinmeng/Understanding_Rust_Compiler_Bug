rust
warning[E0382]: use of moved value: `x`
 --> src/main.rs:6:14
  |
2 |     let x = String::from("sadsadsadsadsa");
  |         - move occurs because `x` has type `std::string::String`, which does not implement the `Copy` trait
...
5 |         0 if { y(x) } => unreachable!(),
  |                  - value moved here
6 |         _ => x,
  |              ^ value used here after move
  |
  = warning: this error has been downgraded to a warning for backwards compatibility with previous releases
  = warning: this represents potential undefined behavior in your code and this warning will become a hard error in the future
