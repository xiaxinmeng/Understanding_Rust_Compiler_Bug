
   Compiling playground v0.0.1 (/playground)
error[E0423]: expected value, found enum `Never`
 --> src/lib.rs:5:14
  |
5 |         _ => Never
  |              ^^^^^
  |
note: the enum is defined here
 --> src/lib.rs:1:1
  |
1 | enum Never {}
  | ^^^^^^^^^^^^^
help: consider importing one of these items instead
  |
1 | use csv::QuoteStyle::Never;
  |
1 | use csv_core::QuoteStyle::Never;
  |
1 | use env_logger::WriteStyle::Never;
  |
1 | use syn::Type::Never;
  |
    and 1 other candidate

error[E0308]: mismatched types
 --> src/lib.rs:6:6
  |
3 | fn test() -> Never {
  |              ----- expected `Never` because of return type
...
6 |     }()
  |      ^^ expected enum `Never`, found `()`

Some errors have detailed explanations: E0308, E0423.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `playground` due to 2 previous errors
