
rustc foo.rs 
warning: unused variable: `x`
 --> foo.rs:6:6
  |
6 | fn f(x: S<u32>) {}
  |      ^ help: if this is intentional, prefix it with an underscore: `_x`
  |
  = note: `#[warn(unused_variables)]` on by default

error[E0320]: overflow while adding drop-check rules for S<u32>
 --> foo.rs:6:6
  |
6 | fn f(x: S<u32>) {}
  |      ^
  |
  = note: overflowed on S<fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(u32)))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))>

error: aborting due to previous error; 1 warning emitted
