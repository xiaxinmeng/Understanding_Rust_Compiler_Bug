
error[E0382]: use of moved value: `s`
 --> src/main.rs:4:10
  |
2 |     let s = "hello".to_owned();
  |         - move occurs because `s` has type `std::string::String`, which does not implement the `Copy` trait
3 |     dbg!(s);
  |     -------- value moved here
4 |     dbg!(s);
  |          ^ value used here after move
  |
  = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
