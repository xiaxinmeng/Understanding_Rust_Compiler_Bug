
warning: unused implementer of `std::future::Future` that must be used
 --> src/main.rs:4:5
  |
4 |     foo();
  |     ^^^^^^
  |
  = note: #[warn(unused_must_use)] on by default
  = note: futures do nothing unless you `.await` or poll them

    Finished dev [unoptimized + debuginfo] target(s) in 0.71s
