
error[E0382]: use of moved value: `a`
 --> src/main.rs:7:20
  |
4 |     let a = "Hello".to_owned();
  |         - move occurs because `a` has type `std::string::String`, which does not implement the `Copy` trait
...
7 |         let test = move || {
  |                    ^^^^^^^ value moved into closure here, in previous iteration of loop
8 |             consume(a);
  |                     - use occurs due to use in closure
