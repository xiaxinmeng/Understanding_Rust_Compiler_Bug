
> rustc array-hang.rs 
warning: constant is never used: `VALUES`
 --> array-hang.rs:2:1
  |
2 | const VALUES: [usize; std::u32::MAX as usize] = [0; std::u32::MAX as usize];
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

memory allocation of 34359738360 bytes failed
Aborted
