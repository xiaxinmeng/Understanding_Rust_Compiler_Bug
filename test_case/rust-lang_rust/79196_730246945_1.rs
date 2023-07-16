
error[E0617]: can't pass `u8` to variadic function
 --> src/main.rs:7:10
  |
7 |         a(123, 0u8);
  |                ^^^ help: cast the value to `c_uint`: `0u8 as c_uint`
