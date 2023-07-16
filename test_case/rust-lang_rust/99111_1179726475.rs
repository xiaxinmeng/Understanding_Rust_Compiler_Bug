rust
    Checking playground v0.0.1 (/playground)
warning: large array defined as const
 --> src/lib.rs:1:1
  |
1 | const BIG_ARRAY: [usize; 65536] = [0; 65536];
  | -----^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  | |
  | help: make this a static item: `static`
  |
  = note: `#[warn(clippy::large_const_arrays)]` on by default
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#large_const_arrays

warning: `playground` (lib) generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.96s
