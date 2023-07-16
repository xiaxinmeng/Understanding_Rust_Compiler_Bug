
warning: literal out of range for i32
 --> src/main.rs:2:20
  |
2 |     let sth: i32 = 100000000000; // or 100000000000i32
  |                    ^^^^^^^^^^^^
  |
  = note: #[warn(overflowing_literals)] on by default
