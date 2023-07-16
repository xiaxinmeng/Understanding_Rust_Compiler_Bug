
error[E0308]: mismatched types
 --> src/main.rs:2:11
  |
2 |     opt = None
  |           ^^^^
  |           |
  |           expected mutable reference, found enum `std::option::Option`
  |           help: consider mutably borrowing here: `&mut None`
  |
  = note: expected type `&mut std::option::Option<std::string::String>`
             found type `std::option::Option<_>`
