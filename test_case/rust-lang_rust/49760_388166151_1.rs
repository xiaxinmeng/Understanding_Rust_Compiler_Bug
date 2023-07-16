`
   Compiling zero1 v0.1.0 (file:///tmp/zero1)
warning: constant evaluation error
 --> src/main.rs:3:17
  |
3 |     println!("{}", 1 / false as u8);
  |                    ^^^^^^^^^^^^^^^ attempt to divide by zero
  |
  = note: #[warn(const_err)] on by default
warning: constant evaluation error
 --> src/main.rs:3:17
  |
3 |     println!("{}", 1 / false as u8);
  |                    ^^^^^^^^^^^^^^^ attempt to divide by zero
    Finished dev [unoptimized + debuginfo] target(s) in 0.48s
