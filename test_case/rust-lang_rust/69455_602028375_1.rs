
error[E0284]: type annotations needed for `&u64`
 --> src/main.rs:3:39
  |
3 |     let ys: Vec<u64> = xs.iter().map(|i: &u64| i*2).collect();
  |                                       ^ consider giving this closure parameter the explicit type `&u64`, where the type parameter `u64` is specified
  |
  = note: cannot resolve `<u64 as std::ops::Add<_>>::Output == u64`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0284`.
error: could not compile `playground`.
