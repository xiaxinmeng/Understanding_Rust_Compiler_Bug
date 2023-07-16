
error[[E0282]](https://doc.rust-lang.org/nightly/error-index.html#E0282): type annotations needed
 --> src/main.rs:3:20
  |
3 |     println!("{}", 23u64 + xs.iter().sum());
  |                    ^^^^^^^^^^^^^^^^^^^^^^^ cannot infer type of the type parameter `T` declared on the associated function `new_display`
  |
  = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider specifying the generic argument
  |
3 |     println!("{}", 23u64 + xs.iter().sum()::<T>);
  |
