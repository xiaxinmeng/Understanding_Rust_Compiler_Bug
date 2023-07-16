
   Compiling playground v0.0.1 (/playground)
error[E0282]: type annotations needed
 --> src/main.rs:2:30
  |
2 |     let _: f64 = "5".parse::<_>().unwrap() / 5.0_f64;
  |                              ^ cannot infer type

For more information about this error, try `rustc --explain E0282`.
