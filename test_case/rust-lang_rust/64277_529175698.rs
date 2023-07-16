
error[E0308]: mismatched types
  --> src/main.rs:12:12
   |
12 |     if let Some(_) = || Some(()) {
   |            ^^^^^^^ expected closure, found enum `std::option::Option`
   |
   = note: expected type `[closure@src/main.rs:12:22: 12:33]`
              found type `std::option::Option<_>`

error[E0308]: mismatched types
  --> src/main.rs:17:5
   |
5  | fn problem() -> String {
   |                 ------ expected `std::string::String` because of return type
...
17 |     1
   |     ^
   |     |
   |     expected struct `std::string::String`, found integer
   |     help: try using a conversion method: `1.to_string()`
   |
   = note: expected type `std::string::String`
              found type `{integer}`

error: aborting due to 2 previous errors
